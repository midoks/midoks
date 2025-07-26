use clap::{Parser, Subcommand};

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
    command: Commands,
}
/// subcommand operation mode
#[derive(Subcommand, Debug)]
enum Commands {
    /// start the fastcdn node server
    Start {},
    /// stop the fastcdn node server
    Stop {},
    /// reload the fastcdn node server
    Reload {},

    /// fastcdn node server Status
    Status {},

    /// test function
    Test {},
}

fn main() {
    let args = Cli::parse();

    println!("命令行参数解析结果:");
    println!("{:#?}", args);

    // 在实际应用中，这里会根据解析的参数执行相应的操作
    match &args.command {
        Commands::Start {} => {
            println!("start server!!!");
        }
        Commands::Stop {} => {
            println!("stop server!!!");
        }
        Commands::Reload {} => {
            println!("reload server!!!");
        }
        Commands::Status {} => {
            println!("reload server!!!");
        }

        Commands::Test {} => {
            println!("test...");
        }
    }
}
