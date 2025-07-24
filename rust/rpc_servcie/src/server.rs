use crate::service::{MathService, MathServiceServer};
use tarpc::context;
use tarpc::tokio_serde::formats::Json;
use tokio::net::TcpListener;

#[derive(Clone)]
struct MathServer;

#[tarpc::server]
impl MathService for MathServer {
    async fn add(self, _: context::Context, a: i32, b: i32) -> i32 {
        a + b
    }

    async fn factorial(self, _: context::Context, n: u64) -> Result<u64, String> {
        if n > 20 {
            return Err("Number too large!".to_string());
        }
        Ok((1..=n).product())
    }
}

pub async fn run_server(listener: TcpListener) -> anyhow::Result<()> {
    println!("Server running on {}", listener.local_addr()?);

    loop {
        let (socket, addr) = listener.accept().await?;
        println!("New client connected: {}", addr);

        let transport = tarpc::serde_transport::new(
            tarpc::tokio_util::codec::length_delimited::LengthDelimitedCodec::builder()
                .new_framed(socket),
            Json::default(),
        );

        let server = MathServiceServer::new(MathServer);

        tokio::spawn(async move {
            if let Err(e) = server.serve(transport).await {
                eprintln!("Server error: {}", e);
            }
        });
    }
}
