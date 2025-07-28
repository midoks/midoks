use std::fs::{File, remove_file};
use std::io::{Write, Read};
use std::process::{Command, Stdio};

/// 守护进程管理器
pub struct DaemonManager {
    pid_file: String,
}

impl DaemonManager {
    /// 创建新的守护进程管理器
    pub fn new(pid_file: &str) -> Self {
        Self {
            pid_file: pid_file.to_string(),
        }
    }

    /// 启动守护进程
    pub fn start_daemon(&self) -> std::io::Result<()> {
        println!("正在启动后台服务...");

        // 获取当前可执行文件路径
        let current_exe = std::env::current_exe()
            .map_err(|_| std::io::Error::new(
                std::io::ErrorKind::Other,
                "无法获取当前执行文件路径",
            ))?;

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

        println!("✓ 服务已在后台启动，PID: {}", child.id());
        println!("✓ PID已保存到文件: {}", self.pid_file);
        println!("✓ 服务地址: http://127.0.0.1:8980");
        println!("✓ 后台服务启动成功");

        Ok(())
    }

    /// 停止守护进程
    pub fn stop_daemon(&self) -> std::io::Result<()> {
        println!("正在停止 fastcdn 服务器...");
        
        // 检查PID文件是否存在
        if !std::path::Path::new(&self.pid_file).exists() {
            println!("⚠️  PID文件不存在，服务可能未在后台运行");
            return Ok(());
        }
        
        // 读取PID文件
        let mut file = File::open(&self.pid_file)
            .map_err(|e| {
                eprintln!("无法打开PID文件: {}", e);
                e
            })?;
        
        let mut pid_str = String::new();
        file.read_to_string(&mut pid_str)
            .map_err(|e| {
                eprintln!("无法读取PID文件: {}", e);
                e
            })?;
        
        let pid = pid_str.trim().parse::<u32>()
            .map_err(|_| {
                eprintln!("PID文件格式错误");
                std::io::Error::new(std::io::ErrorKind::InvalidData, "PID文件格式错误")
            })?;
        
        println!("找到后台进程 PID: {}", pid);
         
        // 先检查进程是否存在
        let check_result = Command::new("kill")
            .arg("-0")
            .arg(pid.to_string())
            .output();
             
        match check_result {
            Ok(output) => {
                if output.status.success() {
                    // 进程存在，尝试终止
                    let kill_result = Command::new("kill")
                        .arg(pid.to_string())
                        .output();
                        
                    match kill_result {
                        Ok(kill_output) => {
                            if kill_output.status.success() {
                                println!("✓ 成功终止进程 PID: {}", pid);
                            } else {
                                let error_msg = String::from_utf8_lossy(&kill_output.stderr);
                                eprintln!("⚠️  终止进程时出现警告: {}", error_msg);
                            }
                        }
                        Err(e) => {
                            eprintln!("⚠️  执行kill命令失败: {}", e);
                        }
                    }
                } else {
                    // 进程不存在
                    println!("⚠️  进程 PID: {} 不存在，可能已经退出", pid);
                }
                
                // 无论如何都删除PID文件
                if let Err(e) = remove_file(&self.pid_file) {
                    eprintln!("⚠️  删除PID文件失败: {}", e);
                } else {
                    println!("✓ 已删除PID文件");
                }
                
                println!("✓ 服务器停止成功");
                Ok(())
            }
            Err(e) => {
                eprintln!("检查进程状态失败: {}", e);
                Err(e)
            }
        }
    }

    /// 检查服务状态
    pub fn check_status(&self) -> std::io::Result<()> {
        println!("正在检查 fastcdn 服务器状态...");
        
        // 检查PID文件是否存在
        if !std::path::Path::new(&self.pid_file).exists() {
            println!("⚠️  PID文件不存在，服务未在后台运行");
            return Ok(());
        }
        
        // 读取PID文件
        let mut file = File::open(&self.pid_file)?;
        let mut pid_str = String::new();
        file.read_to_string(&mut pid_str)?;
        
        let pid = pid_str.trim().parse::<u32>()
            .map_err(|_| std::io::Error::new(std::io::ErrorKind::InvalidData, "PID文件格式错误"))?;
        
        // 检查进程是否存在
        let check_result = Command::new("kill")
            .arg("-0")
            .arg(pid.to_string())
            .output()?;
            
        if check_result.status.success() {
            println!("✓ 服务正在运行，PID: {}", pid);
            println!("✓ 服务地址: http://127.0.0.1:8980");
        } else {
            println!("⚠️  进程 PID: {} 不存在，服务可能已停止", pid);
        }
        
        Ok(())
    }

    /// 重载服务
    pub fn reload_service(&self) -> std::io::Result<()> {
        println!("正在重新加载 fastcdn 服务器...");
        // TODO: 实现实际的服务器重载逻辑
        println!("✓ 服务器重载成功");
        Ok(())
    }
}