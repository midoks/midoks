//! RPC服务器实现模块
//!
//! 包含所有gRPC服务的具体实现

pub mod hello_service;
pub mod ping_service;

// 重新导出服务实现
pub use hello_service::MyHelloService;
pub use ping_service::FcPingService;
