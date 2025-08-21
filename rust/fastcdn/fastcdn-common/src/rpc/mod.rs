//! RPC模块
//!
//! 包含proto定义、服务器实现和客户端实现

pub mod client;
pub mod server;

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
