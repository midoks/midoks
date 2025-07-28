//! RPC模块
//! 
//! 包含proto定义和服务器实现

pub mod server;

// 引入生成的proto代码
pub mod hello {
    tonic::include_proto!("hello");
}

pub mod ping {
    tonic::include_proto!("ping");
}

// 重新导出常用类型
pub use hello::hello_service_server::{HelloService, HelloServiceServer};
pub use hello::{HelloRequest, HelloResponse};
pub use ping::ping_service_server::{PingService, PingServiceServer};
pub use ping::{PingRequest, PingResponse};

// 重新导出服务实现
pub use server::{MyHelloService, MyPingService};