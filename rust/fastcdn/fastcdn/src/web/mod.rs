//! Web模块
//!
//! 包含HTTP服务器和相关组件
pub mod server;
pub mod test;

// 重新导出常用组件
pub use server::{DaemonManager, HttpServerManager};
