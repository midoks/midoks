use clap::{Parser, Subcommand};

mod setup;
mod web;

use fastcdn_common::{HelloServiceServer, MyHelloService, MyPingService, PingServiceServer};
use web::ApiDaemonManager;

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
    let daemon_manager = ApiDaemonManager::new("fastcdn-api.pid");

    // 执行相应的操作并返回适当的退出状态码
    let result: Result<&str, Box<dyn std::error::Error>> = match &args.command {
        Some(Commands::Setup {}) => match setup::install_db().await {
            Ok(_) => Ok("Setup completed successfully"),
            Err(e) => {
                eprintln!("Setup failed: {}", e);
                Err(e)
            }
        },
        Some(Commands::Start { daemon }) => {
            if *daemon {
                daemon_manager.start_daemon().await?;
                Ok("后台RPC服务启动成功")
            } else {
                daemon_manager.start_foreground().await?;
                Ok("RPC服务器启动成功")
            }
        }
        Some(Commands::Stop {}) => {
            daemon_manager.stop_daemon()?;
            Ok("RPC服务器停止成功")
        }
        Some(Commands::Reload {}) => {
            daemon_manager.reload_service()?;
            Ok("服务器重载成功")
        }
        Some(Commands::Status {}) => {
            daemon_manager.check_status()?;
            Ok("服务器状态正常")
        }
        Some(Commands::Test {}) => Ok("测试执行完成"),
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
