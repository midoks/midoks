use std::fs::{File, remove_file};
use std::io::{Read, Write};
use std::process::{Command, Stdio};
use tonic::transport::Server;

use fastcdn_common::{HelloServiceServer, MyHelloService, MyPingService, PingServiceServer};
/// HTTP服务器配置和启动
pub struct RpcServerManager;

impl RpcServerManager {
    /// 创建并启动HTTP服务器
    pub async fn start() -> std::io::Result<()> {
        println!("正在启动 fastcdn api RPC server...");
        println!("gRPC服务监听地址: 127.0.0.1:9001");

        let addr = "127.0.0.1:9001".parse().map_err(|e| {
            eprintln!("地址解析失败: {}", e);
            std::io::Error::new(std::io::ErrorKind::InvalidInput, "地址解析失败")
        })?;

        let hello_service = MyHelloService::default();
        let ping_service = MyPingService::default();

        println!("✓ HelloService 已注册");
        println!("✓ PingService 已注册");
        println!("✓ gRPC服务器启动成功，监听端口: 9001");

        Server::builder()
            .add_service(HelloServiceServer::new(hello_service))
            .add_service(PingServiceServer::new(ping_service))
            .serve(addr)
            .await
            .map_err(|e| {
                eprintln!("gRPC服务器启动失败: {}", e);
                std::io::Error::new(std::io::ErrorKind::Other, "gRPC服务器启动失败")
            })?;

        Ok(())
    }
}
