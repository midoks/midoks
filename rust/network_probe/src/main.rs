mod modules;
mod utils;
mod api;
mod websocket;
mod cli;

use clap::Parser;
use env_logger;

use crate::cli::{Cli, handle_command};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    
    // 设置日志级别
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", &cli.log_level);
    }
    
    // 初始化日志
    env_logger::Builder::from_default_env()
        .format_timestamp_secs()
        .init();
    
    log::info!("Network Probe starting...");
    
    // 处理命令
    handle_command(cli).await?;
    
    Ok(())
}