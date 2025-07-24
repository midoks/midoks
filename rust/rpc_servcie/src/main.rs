use clap::Parser;
use tokio::net::TcpListener;

mod client;
mod server;
mod service;

/// Tarpc 示例程序
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// 运行模式 (server/client)
    #[arg(short, long, default_value = "server")]
    mode: String,

    /// 服务器地址 (仅客户端需要)
    #[arg(short, long, default_value = "127.0.0.1:8080")]
    addr: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    match args.mode.to_lowercase().as_str() {
        "server" => {
            println!("Starting server on {}", args.addr);
            let listener = TcpListener::bind(&args.addr).await?;
            server::run_server(listener).await
        }
        "client" => {
            println!("Connecting to server at {}", args.addr);
            client::run_client(&args.addr).await
        }
        _ => {
            eprintln!("Invalid mode. Use 'server' or 'client'.");
            std::process::exit(1);
        }
    }
}
