use clap::{Parser, Subcommand};
use fastcdn_common::daemon::app::Daemon;

mod service;

/// 命令行信息
#[derive(Parser, Debug)]
#[command(
    author = "midoks <midoks@163.com>",
    version = "0.0.1",
    about = "fastcdn-node",
    long_about = "fastcdn node service"
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
    /// start the fastcdn node server
    Start {
        /// run server in daemon mode (background)
        #[arg(short, long)]
        daemon: bool,
    },
    Daemon,
    /// stop the fastcdn node server
    Stop,
    /// reload the fastcdn node server
    Reload,
    /// fastcdn node server Status
    Status,
    /// test function
    Test,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();
    let app = Daemon::new("fastcdn-node.pid");
    let result: Result<&str, Box<dyn std::error::Error>> = match &args.command {
        Some(Commands::Start { daemon }) => {
            if *daemon {
                app.start()?;
                Ok("节点服务启动成功")
            } else {
                service::node::start().await?;
                Ok("成功")
            }
        }
        Some(Commands::Daemon) => {
            app.start()?;
            Ok("start daemon successful")
        }
        Some(Commands::Stop {}) => {
            app.stop()?;
            Ok("停止成功")
        }
        Some(Commands::Reload {}) => {
            app.reload()?;
            Ok("重载成功")
        }
        Some(Commands::Status {}) => {
            app.status()?;
            Ok("")
        }
        Some(Commands::Test {}) => {
            service::test::run().await?;
            println!("测试结束");
            Ok("ok")
        }
        None => {
            println!("欢迎使用 fastcdn 服务！");
            println!("使用 --help 查看可用命令");
            Ok("")
        }
    };

    match result {
        Ok(_) => Ok(()),
        Err(error) => {
            eprintln!("fastcdn-node error: {}", error);
            Err(error)
        }
    }
}
