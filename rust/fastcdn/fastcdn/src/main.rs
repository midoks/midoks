use clap::{Parser, Subcommand};

// 引入模块化的Web服务器和RPC客户端
mod app;
mod web;

use web::{DaemonManager, HttpServerManager};

// 引入自动生成的proto代码


use fastcdn::hello::hello_service_client::HelloServiceClient;
use fastcdn::hello::{HelloRequest, HelloResponse};
use fastcdn::ping::ping_service_client::PingServiceClient;
use fastcdn::ping::{PingRequest, PingResponse};

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

    if args.verbose {
        println!("命令行参数解析结果:");
        println!("{:#?}", args);
    }

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
            match PingServiceClient::connect("http://127.0.0.1:50051").await {
                Ok(mut client) => {
                    let request = tonic::Request::new(PingRequest {});
                    match client.ping(request).await {
                        Ok(response) => println!("✓ Ping服务连接成功: {:?}", response.into_inner()),
                        Err(e) => println!("✗ Ping服务调用失败: {}", e),
                    }
                }
                Err(e) => println!("✗ Ping服务连接失败: {}", e),
            }

            // 测试Hello服务
            match HelloServiceClient::connect("http://127.0.0.1:50051").await {
                Ok(mut client) => {
                    let request = tonic::Request::new(HelloRequest {
                        name: "FastCDN Web".to_string(),
                    });
                    match client.say_hello(request).await {
                        Ok(response) => println!("✓ Hello服务响应: {}", response.get_ref().message),
                        Err(e) => println!("✗ Hello服务调用失败: {}", e),
                    }
                }
                Err(e) => println!("✗ Hello服务连接失败: {}", e),
            }

            println!("✓ 所有gRPC连接测试完成");
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
