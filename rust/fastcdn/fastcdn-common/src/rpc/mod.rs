//! RPC模块
//!
//! 包含proto定义、服务器实现和客户端实现

pub mod auth;
pub mod client;
pub mod server;

// 引入生成的proto代码 - 所有proto文件都在同一个package中
pub mod fastcdn {
    tonic::include_proto!("fastcdn");
}

// 重新导出常用类型以便使用
pub use fastcdn::*;
