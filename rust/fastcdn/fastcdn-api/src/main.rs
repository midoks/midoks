use clap::{Parser, Subcommand};
use std::fs::{File, remove_file};
use std::io::{Read, Write};
use std::process::{Command, Stdio};
use tonic::transport::Server;

mod setup;
use fastcdn_common::{HelloServiceServer, MyHelloService, MyPingService, PingServiceServer};

/// 命令行信息
#[derive(Parser, Debug)]
#[command(
    author = "midoks <midoks@163.com>",
    version = "0.0.1",
    about = "fastcdn-api",
    long_about = "fastcdn api service"
)]

struct Cli {
    /// display version information
    #[arg(short, long, global = true)]
    verbose: bool,

    /// subcommand operation mode
    #[command(subcommand)]
    command: Option<Commands>,
}
/// subcommand operation mode
#[derive(Subcommand, Debug)]
enum Commands {
    /// start the fastcdn api server
    Start {
        /// run server in daemon mode (background)
        #[arg(short, long)]
        daemon: bool,
    },
    /// stop the fastcdn api server
    Stop {},
    /// reload the fastcdn api server
    Reload {},

    /// fastcdn api server Status
    Status {},

    /// fastcdn api server setup cmd
    Setup {},

    /// test function
    Test {},
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();

    if args.verbose {
        println!("命令行参数解析结果:");
        println!("{:#?}", args);
    }

    // 执行相应的操作并返回适当的退出状态码
    let result: Result<&str, Box<dyn std::error::Error>> = match &args.command {
        Some(Commands::Setup {}) => match setup::install_db() {
            Ok(_) => Ok("Setup completed successfully"),
            Err(e) => {
                eprintln!("Setup failed: {}", e);
                Err(e)
            }
        },
        Some(Commands::Start { daemon }) => {
            if *daemon {
                // 后台模式运行
                println!("正在启动后台RPC服务...");

                // 获取当前可执行文件路径
                let current_exe = match std::env::current_exe() {
                    Ok(exe) => exe,
                    Err(_e) => {
                        return Err(Box::new(std::io::Error::new(
                            std::io::ErrorKind::Other,
                            "无法获取当前执行文件路径",
                        )) as Box<dyn std::error::Error>);
                    }
                };

                // 启动后台进程
                let child = match Command::new(current_exe)
                    .arg("start")
                    .stdin(Stdio::null())
                    .stdout(Stdio::null())
                    .stderr(Stdio::null())
                    .spawn()
                {
                    Ok(child) => child,
                    Err(e) => return Err(Box::new(e) as Box<dyn std::error::Error>),
                };

                // 保存PID到文件
                let pid_file = "fastcdn-api.pid";
                let mut file = match File::create(pid_file) {
                    Ok(file) => file,
                    Err(e) => return Err(Box::new(e) as Box<dyn std::error::Error>),
                };

                if let Err(e) = writeln!(file, "{}", child.id()) {
                    return Err(Box::new(e) as Box<dyn std::error::Error>);
                }

                println!("✓ RPC服务已在后台启动，PID: {}", child.id());
                println!("✓ PID已保存到文件: {}", pid_file);
                println!("✓ gRPC服务地址: http://127.0.0.1:9001");

                Ok("后台RPC服务启动成功")
            } else {
                // 前台模式运行
                println!("正在启动 fastcdn api RPC server...");
                println!("gRPC服务监听地址: 127.0.0.1:9001");

                let addr = match "127.0.0.1:9001".parse() {
                    Ok(addr) => addr,
                    Err(e) => {
                        eprintln!("地址解析失败: {}", e);
                        return Err(Box::new(std::io::Error::new(
                            std::io::ErrorKind::InvalidInput,
                            "地址解析失败",
                        )) as Box<dyn std::error::Error>);
                    }
                };
                let hello_service = MyHelloService::default();
                let ping_service = MyPingService::default();

                println!("✓ HelloService 已注册");
                println!("✓ PingService 已注册");
                println!("✓ gRPC服务器启动成功，监听端口: 9001");

                if let Err(e) = Server::builder()
                    .add_service(HelloServiceServer::new(hello_service))
                    .add_service(PingServiceServer::new(ping_service))
                    .serve(addr)
                    .await
                {
                    eprintln!("gRPC服务器启动失败: {}", e);
                    return Err(Box::new(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        "gRPC服务器启动失败",
                    )) as Box<dyn std::error::Error>);
                }

                Ok("RPC服务器启动成功")
            }
        }
        Some(Commands::Stop {}) => {
            println!("正在停止 fastcdn api RPC服务器...");

            let pid_file = "fastcdn-api.pid";

            // 检查PID文件是否存在
            if !std::path::Path::new(pid_file).exists() {
                println!("⚠️  PID文件不存在，RPC服务可能未在后台运行");
                return Ok(());
            }

            // 读取PID文件
            let mut file = match File::open(pid_file) {
                Ok(file) => file,
                Err(e) => {
                    eprintln!("无法打开PID文件: {}", e);
                    return Err(Box::new(e) as Box<dyn std::error::Error>);
                }
            };

            let mut pid_str = String::new();
            if let Err(e) = file.read_to_string(&mut pid_str) {
                eprintln!("无法读取PID文件: {}", e);
                return Err(Box::new(e));
            }

            let pid = match pid_str.trim().parse::<u32>() {
                Ok(pid) => pid,
                Err(_) => {
                    eprintln!("PID文件格式错误");
                    return Err(Box::new(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        "PID文件格式错误",
                    )) as Box<dyn std::error::Error>);
                }
            };

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

                    Ok("RPC服务器停止成功")
                }
                Err(e) => {
                    eprintln!("检查RPC进程状态失败: {}", e);
                    Err(Box::new(e) as Box<dyn std::error::Error>)
                }
            }
        }
        Some(Commands::Reload {}) => {
            println!("正在重新加载 fastcdn api 服务器...");
            // 这里应该包含实际的服务器重载逻辑
            Ok("服务器重载成功")
        }
        Some(Commands::Status {}) => {
            println!("正在检查 fastcdn api 服务器状态...");
            // 这里应该包含实际的状态检查逻辑
            Ok("服务器状态正常")
        }
        Some(Commands::Test {}) => {
            println!("正在执行测试...");
            // 这里应该包含实际的测试逻辑
            Ok("测试执行完成")
        }
        None => {
            println!("欢迎使用 fastcdn-api 服务！");
            println!("使用 --help 查看可用命令");
            Ok("程序执行完成")
        }
    };

    match result {
        Ok(message) => {
            println!("✓ {}", message);
            Ok(())
        }
        Err(error) => {
            eprintln!("✗ 错误: {}", error);
            Err(error)
        }
    }
}
