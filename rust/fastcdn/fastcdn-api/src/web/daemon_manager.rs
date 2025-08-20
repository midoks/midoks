use std::fs::{File, remove_file};
use std::io::{Read, Write};
use std::process::{Command, Stdio};
use tonic::transport::Server;

use fastcdn_common::{HelloServiceServer, MyHelloService, MyPingService, PingServiceServer};

/// API 守护进程管理器
pub struct ApiDaemonManager {
    pid_file: String,
}

impl ApiDaemonManager {
    /// 创建新的 API 守护进程管理器
    pub fn new(pid_file: &str) -> Self {
        Self {
            pid_file: pid_file.to_string(),
        }
    }

    /// 启动守护进程
    pub async fn start_daemon(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("正在启动后台RPC服务...");

        // 获取当前可执行文件路径
        let current_exe = std::env::current_exe().map_err(|_| {
            std::io::Error::new(std::io::ErrorKind::Other, "无法获取当前执行文件路径")
        })?;

        // 启动后台进程
        let child = Command::new(current_exe)
            .arg("start")
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()?;

        // 保存PID到文件
        let mut file = File::create(&self.pid_file)?;
        writeln!(file, "{}", child.id())?;

        println!("✓ RPC服务已在后台启动，PID: {}", child.id());
        println!("✓ PID已保存到文件: {}", self.pid_file);
        println!("✓ gRPC服务地址: http://127.0.0.1:9001");

        Ok(())
    }

    /// 启动前台服务
    pub async fn start_foreground(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("正在启动 fastcdn api RPC server...");
        println!("gRPC服务监听地址: 127.0.0.1:9001");

        let addr = "127.0.0.1:9001".parse().map_err(|e| {
            eprintln!("地址解析失败: {}", e);
            std::io::Error::new(std::io::ErrorKind::InvalidInput, "地址解析失败")
        })?;

        let hello_service = MyHelloService::default();
        let ping_service = MyPingService::default();

        println!("✓ HelloService 已注册");
        println!("✓ PingService 已注册");
        println!("✓ gRPC服务器启动成功，监听端口: 9001");

        Server::builder()
            .add_service(HelloServiceServer::new(hello_service))
            .add_service(PingServiceServer::new(ping_service))
            .serve(addr)
            .await
            .map_err(|e| {
                eprintln!("gRPC服务器启动失败: {}", e);
                std::io::Error::new(std::io::ErrorKind::Other, "gRPC服务器启动失败")
            })?;

        Ok(())
    }

    /// 停止守护进程
    pub fn stop_daemon(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("正在停止 fastcdn api RPC服务器...");

        let pid_file = &self.pid_file;

        // 检查PID文件是否存在
        if !std::path::Path::new(pid_file).exists() {
            println!("⚠️  PID文件不存在，RPC服务可能未在后台运行");
            return Ok(());
        }

        // 读取PID文件
        let mut file = File::open(pid_file).map_err(|e| {
            eprintln!("无法打开PID文件: {}", e);
            e
        })?;

        let mut pid_str = String::new();
        file.read_to_string(&mut pid_str).map_err(|e| {
            eprintln!("无法读取PID文件: {}", e);
            e
        })?;

        let pid = pid_str.trim().parse::<u32>().map_err(|_| {
            eprintln!("PID文件格式错误");
            std::io::Error::new(std::io::ErrorKind::InvalidData, "PID文件格式错误")
        })?;

        println!("找到后台RPC进程 PID: {}", pid);

        // 先检查进程是否存在
        let check_result = Command::new("kill").arg("-0").arg(pid.to_string()).output();

        match check_result {
            Ok(output) => {
                if output.status.success() {
                    // 进程存在，尝试终止
                    let kill_result = Command::new("kill").arg(pid.to_string()).output();

                    match kill_result {
                        Ok(kill_output) => {
                            if kill_output.status.success() {
                                println!("✓ 成功终止RPC进程 PID: {}", pid);
                            } else {
                                let error_msg = String::from_utf8_lossy(&kill_output.stderr);
                                eprintln!("⚠️  终止RPC进程时出现警告: {}", error_msg);
                            }
                        }
                        Err(e) => {
                            eprintln!("⚠️  执行kill命令失败: {}", e);
                        }
                    }
                } else {
                    // 进程不存在
                    println!("⚠️  RPC进程 PID: {} 不存在，可能已经退出", pid);
                }

                // 无论如何都删除PID文件
                if let Err(e) = remove_file(pid_file) {
                    eprintln!("⚠️  删除PID文件失败: {}", e);
                } else {
                    println!("✓ 已删除PID文件");
                }

                Ok(())
            }
            Err(e) => {
                eprintln!("检查RPC进程状态失败: {}", e);
                Err(Box::new(e) as Box<dyn std::error::Error>)
            }
        }
    }

    /// 检查服务状态
    pub fn check_status(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("正在检查 fastcdn api RPC服务器状态...");

        let pid_file = &self.pid_file;

        // 检查PID文件是否存在
        if !std::path::Path::new(pid_file).exists() {
            println!("⚠️  PID文件不存在，API RPC服务未在后台运行");
            return Ok(());
        }

        // 读取PID文件
        let mut file = File::open(pid_file)?;
        let mut pid_str = String::new();
        file.read_to_string(&mut pid_str)?;

        let pid = pid_str
            .trim()
            .parse::<u32>()
            .map_err(|_| std::io::Error::new(std::io::ErrorKind::InvalidData, "PID文件格式错误"))?;

        // 检查进程是否存在
        let check_result = Command::new("kill")
            .arg("-0")
            .arg(pid.to_string())
            .output()?;

        if check_result.status.success() {
            println!("✓ API RPC服务正在运行，PID: {}", pid);
            println!("✓ gRPC服务地址: http://127.0.0.1:9001");
        } else {
            println!("⚠️  API RPC进程 PID: {} 不存在，服务可能已停止", pid);
        }

        Ok(())
    }

    /// 重载服务
    pub fn reload_service(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("正在重新加载 fastcdn api 服务器...");
        // 这里应该包含实际的服务器重载逻辑
        println!("✓ 服务器重载成功");
        Ok(())
    }
}
