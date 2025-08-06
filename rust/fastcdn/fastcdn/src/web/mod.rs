//! Web模块
//!
//! 包含HTTP服务器和相关组件

pub mod config;
pub mod database;
pub mod server;

// 重新导出常用组件
pub use config::{ConfigApiAdmin, ConfigServer};
pub use database::DatabaseManager;
pub use server::{DaemonManager, HttpServerManager};
