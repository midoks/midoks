use clap::{Parser, Subcommand};
use std::env;

// 引入模块化的Web服务器和RPC客户端
mod app;
mod web;

use web::{DaemonManager, HttpServerManager, config_server};
// 引入共享的RPC客户端
use fastcdn_api::{HelloClient, PingClient};

/// 命令行信息
#[derive(Parser, Debug)]
#[command(
    author = "midoks <midoks@163.com>",
    version = "0.0.1",
    about = "fastcdn",
    long_about = "fastcdn service"
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
    /// test function
    Test {},
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args = Cli::parse();
    let daemon_manager = DaemonManager::new("fastcdn.pid");

    // 执行相应的操作并返回适当的退出状态码
    let result: Result<&str, std::io::Error> = match &args.command {
        Some(Commands::Start { daemon }) => {
            if *daemon {
                // 后台模式运行
                daemon_manager.start_daemon()?;
                Ok("后台服务启动成功")
            } else {
                // 前台模式运行
                HttpServerManager::start().await?;
                Ok("服务器启动成功")
            }
        }
        Some(Commands::Stop {}) => {
            daemon_manager.stop_daemon()?;
            Ok("服务器停止成功")
        }
        Some(Commands::Reload {}) => {
            daemon_manager.reload_service()?;
            Ok("服务器重载成功")
        }
        Some(Commands::Status {}) => {
            daemon_manager.check_status()?;
            Ok("状态检查完成")
        }
        Some(Commands::Test {}) => {
            println!("正在测试gRPC连接...");

            // 测试Ping服务
            match PingClient::connect("http://127.0.0.1:50051").await {
                Ok(mut client) => match client.ping().await {
                    Ok(response) => println!("✓ Ping服务连接成功: {}", response),
                    Err(e) => println!("✗ Ping服务调用失败: {}", e),
                },
                Err(e) => println!("✗ Ping服务连接失败: {}", e),
            }

            // 测试Hello服务
            match HelloClient::connect("http://127.0.0.1:50051").await {
                Ok(mut client) => match client.say_hello("FastCDN Web").await {
                    Ok(response) => println!("✓ Hello服务响应: {}", response),
                    Err(e) => println!("✗ Hello服务调用失败: {}", e),
                },
                Err(e) => println!("✗ Hello服务连接失败: {}", e),
            }

            println!("✓ 所有gRPC连接测试完成");

            // 测试配置加载功能
            match env::current_dir() {
                Ok(path) => {
                    println!("当前运行目录: {}", path.display());

                    // 使用web/config模块的配置管理器
                    match config_server::Manager::new() {
                        Ok(config_manager) => {
                            let server_config = config_manager.server();
                            println!("✓ 配置文件加载成功: {:#?}", server_config);

                            // 显示配置信息
                            println!("环境: {}", server_config.env);
                            println!(
                                "HTTP服务: {}",
                                if server_config.http.on {
                                    "启用"
                                } else {
                                    "禁用"
                                }
                            );
                            if server_config.http.on {
                                println!("HTTP监听地址: {:?}", server_config.get_http_addresses());
                            }
                            println!(
                                "HTTPS服务: {}",
                                if server_config.https.on {
                                    "启用"
                                } else {
                                    "禁用"
                                }
                            );
                            if server_config.https.on {
                                println!(
                                    "HTTPS监听地址: {:?}",
                                    server_config.get_https_addresses()
                                );
                            }
                        }
                        Err(e) => {
                            println!("✗ 配置文件加载失败: {}", e);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("获取目录失败: {}", e);
                }
            }

            Ok("测试执行完成")
        }
        None => {
            println!("欢迎使用 fastcdn 服务！");
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
