//! FastCDN 共享库
//! 
//! 提供 RPC 客户端和服务器的共享实现

pub mod rpc;

// 重新导出常用的客户端
pub use rpc::client::{HelloClient, PingClient};

// 重新导出服务器相关
pub use rpc::server::{MyHelloService, MyPingService};
pub use rpc::{HelloServiceServer, PingServiceServer};