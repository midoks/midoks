// use std::fs::{File, remove_file};
// use std::io::{Read, Write};
// use std::process::{Command, Stdio};
use tonic::transport::Server;

use fastcdn_common::rpc::server::admin::FcAdmin;
use fastcdn_common::rpc::server::hello::MyHelloService;
use fastcdn_common::rpc::server::ping::FcPing;

// fastcdn
use fastcdn_common::rpc::fastcdn::admin_server::AdminServer;
use fastcdn_common::rpc::fastcdn::hello_service_server::HelloServiceServer;
use fastcdn_common::rpc::fastcdn::ping_server::PingServer;

pub struct RpcServerManager;

impl RpcServerManager {
    /// 创建并启动HTTP服务器
    pub async fn start() -> std::io::Result<()> {
        println!("starting fastcdn-api rpc server...");

        let addr = "127.0.0.1:10001".parse().map_err(|e| {
            eprintln!("address resolution failed: {}", e);
            std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "address resolution failed",
            )
        })?;

        Server::builder()
            .add_service(HelloServiceServer::new(MyHelloService::default()))
            .add_service(PingServer::new(FcPing::default()))
            .add_service(AdminServer::new(FcAdmin::default()))
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
