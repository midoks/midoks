// 导出模块
pub mod load_balancer;
pub mod cache_proxy;

// 重新导出启动函数
pub use load_balancer::start_load_balancer;
pub use cache_proxy::start_cache_proxy;