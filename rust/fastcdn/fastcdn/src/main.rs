use clap::{Parser, Subcommand};

// 引入模块化的Web服务器和RPC客户端
mod app;
mod web;
use web::{DaemonManager, HttpServerManager};

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
    Stop,
    /// reload the fastcdn api server
    Reload,
    /// fastcdn api server Status
    Status,
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
            let _ = web::test::run().await;
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
