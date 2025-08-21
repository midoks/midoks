use clap::{Parser, Subcommand};

// 引入模块化的Web服务器和RPC客户端
mod app;
mod web;

use fastcdn_common::daemon::app::Daemon;
use web::HttpServerManager;

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
    /// daemon the fastcdn server
    Daemon,
    /// stop the fastcdn api server
    Stop,
    /// reload the fastcdn api server
    Reload,
    /// fastcdn api server Status
    Status,
    /// test function
    Test,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();
    let app = Daemon::new("fastcdn.pid");

    // 执行相应的操作并返回适当的退出状态码
    let result: Result<&str, Box<dyn std::error::Error>> = match &args.command {
        Some(Commands::Start { daemon }) => {
            if *daemon {
                app.start()?;
                Ok("start successful")
            } else {
                HttpServerManager::start().await?;
                Ok("exit successful!")
            }
        }
        Some(Commands::Daemon {}) => {
            app.start()?;
            Ok("start daemon successful")
        }
        Some(Commands::Stop {}) => {
            app.stop()?;
            Ok("stop server successful")
        }
        Some(Commands::Reload {}) => {
            app.reload()?;
            Ok("服务器重载成功")
        }
        Some(Commands::Status {}) => {
            app.status()?;
            Ok("状态检查完成")
        }
        Some(Commands::Test {}) => {
            let _ = web::test::run().await;
            Ok("测试执行完成")
        }
        None => {
            println!("欢迎使用 fastcdn 服务！");
            println!("使用 --help 查看可用命令");
            Ok("")
        }
    };

    match result {
        Ok(message) => {
            println!("{}", message);
            Ok(())
        }
        Err(error) => {
            eprintln!("✗ 错误: {}", error);
            Err(error)
        }
    }
}
