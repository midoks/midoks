use async_trait::async_trait;
use pingora::prelude::*;
use pingora::proxy::{http_proxy_service, ProxyHttp};
use pingora::protocols::http::server::Session;
use pingora::server::Server;
use pingora::upstreams::peer::HttpPeer;

fn main() {
    // 1. 创建服务器配置
    let mut server = Server::new(Some(Opt {
        daemon: false,
        upgrade: false,
        ..Default::default()
    })).unwrap();

    // 2. 创建 HTTP 代理服务
    let proxy = http_proxy_service(
        &server.configuration,
        MyProxyHandler::new(),
    );

    // 3. 添加服务到服务器
    server.add_service(proxy);

    // 4. 启动服务器
    server.run_forever();
}

// 自定义代理处理器
pub struct MyProxyHandler;
impl MyProxyHandler {
    pub fn new() -> Self {
        MyProxyHandler
    }
}

#[async_trait]
impl ProxyHttp for MyProxyHandler {
    async fn upstream_peer(&self, _session: &mut Session) -> pingora::Result<Box<HttpPeer>> {
        // 定义上游服务器（例如代理到 example.com）
        Ok(Box::new(HttpPeer::new("example.com:80", false, "example.com".to_string())))
    }
}