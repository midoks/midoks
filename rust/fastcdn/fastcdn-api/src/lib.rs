//! FastCDN API库
//! 
//! 提供RPC客户端和proto定义供其他项目使用

pub mod rpc;

// 重新导出常用的RPC客户端
pub use rpc::{HelloClient, PingClient};

// 重新导出proto定义
pub use rpc::{hello, ping};