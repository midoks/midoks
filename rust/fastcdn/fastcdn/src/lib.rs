//! FastCDN库模块

pub mod rpc;

// 重新导出proto定义
pub use rpc::{hello, ping};