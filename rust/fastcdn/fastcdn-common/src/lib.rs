//! FastCDN 共享库
//!
//! 提供 RPC 客户端和服务器的共享实现

pub mod config;
pub mod daemon;
pub mod rpc; // 添加这行

pub mod db;

// 导出配置读取香港
pub use config::{ConfigApiAdmin, ConfigDb, ConfigServer};
