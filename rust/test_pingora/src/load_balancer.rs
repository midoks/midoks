use async_trait::async_trait;
use pingora_core::server::Server;
use pingora_core::upstreams::peer::HttpPeer;
use pingora_core::Result;
use pingora_proxy::{ProxyHttp, Session};
use pingora_proxy::http_proxy_service;
use pingora_http;

// 定义一个简单的负载均衡器
pub struct LoadBalancer {
    upstreams: Vec<String>,
    current: std::sync::atomic::AtomicUsize,
}

impl LoadBalancer {
    pub fn new(upstreams: Vec<String>) -> Self {
        Self {
            upstreams,
            current: std::sync::atomic::AtomicUsize::new(0),
        }
    }

    // 使用轮询算法选择下一个上游服务器
    fn next_upstream(&self) -> String {
        let current = self.current.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let idx = current % self.upstreams.len();
        self.upstreams[idx].clone()
    }
}

#[async_trait]
impl ProxyHttp for LoadBalancer {
    type CTX = ();
    fn new_ctx(&self) -> Self::CTX {
        ()
    }

    // 处理HTTP请求的方法
    async fn upstream_peer(&self, _session: &mut Session, _ctx: &mut Self::CTX) -> Result<Box<HttpPeer>> {
        // 使用轮询算法选择下一个上游服务器
        let upstream = self.next_upstream();
        println!("Forwarding to upstream: {}", upstream);
        
        // 创建一个HTTP上游连接
        let peer = Box::new(HttpPeer::new(upstream, false, String::new()));
        Ok(peer)
    }

    // 可选：修改请求头
    async fn request_filter(&self, session: &mut Session, _ctx: &mut Self::CTX) -> Result<bool> {
        // 添加一个自定义请求头
        session.req_header_mut().insert_header(
            "X-Forwarded-By",
            "Pingora-LoadBalancer",
        ).unwrap();
        Ok(true) // 继续处理请求
    }

    // 可选：修改响应头
    async fn response_filter(&self, _session: &mut Session, upstream_response: &mut pingora_http::ResponseHeader, _ctx: &mut Self::CTX) -> Result<()> {
        // 添加一个自定义响应头
        upstream_response.insert_header(
            "X-Powered-By",
            "Pingora-LoadBalancer",
        ).unwrap();
        Ok(())
    }
}

// 启动负载均衡器服务
pub fn start_load_balancer() {
    // 初始化日志
    env_logger::init();
    
    // 创建服务器配置
    let mut server = Server::new(None).unwrap();
    server.bootstrap();
    
    // 设置上游服务器列表
    let upstreams = vec![
        "example.com:80".to_string(),
        "httpbin.org:80".to_string(),
    ];
    
    // 创建负载均衡器实例
    let load_balancer = LoadBalancer::new(upstreams);
    
    // 设置负载均衡服务
    let mut lb = http_proxy_service(
        &server.configuration,
        load_balancer,
    );
    
    // 添加TCP监听地址
    lb.add_tcp("127.0.0.1:8081");
    
    // 添加服务到服务器
    server.add_service(lb);
    
    // 运行服务器，阻塞直到服务器关闭
    server.run_forever();
}