//! FastCDN 共享库
//!
//! 提供 RPC 客户端和服务器的共享实现

pub mod config;
pub mod daemon;
pub mod rpc;
pub mod db;
pub mod utils;

// 导出配置读取
pub use config::{ConfigApiAdmin, ConfigDb, ConfigServer};
