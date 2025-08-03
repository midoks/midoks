//! Web模块
//! 
//! 包含HTTP服务器和相关组件

pub mod server;
pub mod config;  // 添加config模块

// 重新导出常用组件
pub use server::{HttpServerManager, DaemonManager};
pub use config::{ConfigManager, Server};  // 导出配置相关类型