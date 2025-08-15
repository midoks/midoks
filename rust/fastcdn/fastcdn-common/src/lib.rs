//! FastCDN 共享库
//!
//! 提供 RPC 客户端和服务器的共享实现

pub mod config;
pub mod rpc;

pub mod db;

// 导出常用的客户端
pub use rpc::client::{HelloClient, PingClient};

// 导出服务器相关
pub use rpc::server::{MyHelloService, MyPingService};
pub use rpc::{HelloServiceServer, PingServiceServer};

// 导出配置读取香港
pub use config::{ConfigApiAdmin, ConfigDb, ConfigServer};
