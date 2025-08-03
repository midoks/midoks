//! Web模块
//!
//! 包含HTTP服务器和相关组件

pub mod config;
pub mod server;

// 重新导出常用组件
pub use config::server as config_server;
pub use server::{DaemonManager, HttpServerManager};
