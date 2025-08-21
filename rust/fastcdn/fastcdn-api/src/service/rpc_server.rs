// use std::fs::{File, remove_file};
// use std::io::{Read, Write};
// use std::process::{Command, Stdio};
use tonic::transport::Server;

use fastcdn_common::rpc::hello::hello_service_server::HelloServiceServer;
use fastcdn_common::rpc::ping::ping_service_server::PingServiceServer;
use fastcdn_common::rpc::server::hello::MyHelloService;
use fastcdn_common::rpc::server::ping::MyPingService;

pub struct RpcServerManager;

impl RpcServerManager {
    /// 创建并启动HTTP服务器
    pub async fn start() -> std::io::Result<()> {
        println!("starting fastcdn api RPC server...");

        let addr = "127.0.0.1:10001".parse().map_err(|e| {
            eprintln!("address resolution failed: {}", e);
            std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "address resolution failed",
            )
        })?;

        let hello_service = MyHelloService::default();
        let ping_service = MyPingService::default();

        Server::builder()
            .add_service(HelloServiceServer::new(hello_service))
            .add_service(PingServiceServer::new(ping_service))
            .serve(addr)
            .await
            .map_err(|e| {
                eprintln!("astcdn grpc server failed to start: {}", e);
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "fastcdn grpc server failed to start",
                )
            })?;

        Ok(())
    }
}
