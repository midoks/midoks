//! RPC客户端模块
//! 
//! 提供对fastcdn-api服务的RPC客户端封装

mod hello_client;
mod ping_client;

pub use hello_client::HelloClient;
pub use ping_client::PingClient;