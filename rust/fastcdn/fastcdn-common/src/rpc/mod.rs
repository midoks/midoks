//! RPC模块
//! 
//! 包含proto定义、服务器实现和客户端实现

pub mod server;
pub mod client;

// 引入生成的proto代码
pub mod hello {
    tonic::include_proto!("hello");
}

pub mod ping {
    tonic::include_proto!("ping");
}

// 重新导出常用类型
pub use hello::hello_service_server::HelloServiceServer;
pub use ping::ping_service_server::PingServiceServer;

// 重新导出服务实现
pub use server::{MyHelloService, MyPingService};

// 重新导出客户端
pub use client::{HelloClient, PingClient};