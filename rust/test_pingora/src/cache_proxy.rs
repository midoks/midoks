use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use async_trait::async_trait;
use pingora_core::server::Server;
use pingora_core::upstreams::peer::HttpPeer;
use pingora_core::Result;
use pingora_proxy::{ProxyHttp, Session};
use pingora_proxy::http_proxy_service;
use pingora_http;

// 定义一个简单的缓存条目
struct CacheEntry {
    #[allow(dead_code)]
    content: Vec<u8>,
    headers: HashMap<String, String>,
    created_at: Instant,
    ttl: Duration,
}

impl CacheEntry {
    fn is_expired(&self) -> bool {
        self.created_at.elapsed() > self.ttl
    }
}

// 定义一个带缓存的HTTP代理服务
pub struct CachingProxy {
    upstream: String,
    cache: Arc<Mutex<HashMap<String, CacheEntry>>>,
    cache_ttl: Duration,
}

impl CachingProxy {
    pub fn new(upstream: String, cache_ttl_secs: u64) -> Self {
        Self {
            upstream,
            cache: Arc::new(Mutex::new(HashMap::new())),
            cache_ttl: Duration::from_secs(cache_ttl_secs),
        }
    }

    // 检查请求是否可缓存（只缓存GET请求）
    fn is_cacheable(&self, session: &Session) -> bool {
        session.req_header().method == "GET"
    }

    // 生成缓存键
    fn cache_key(&self, session: &Session) -> String {
        // 从 URI 中获取主机名，如果没有则使用空字符串
        let host = session.req_header().uri.host().unwrap_or_default().to_string();
        format!("{}{}", host, session.req_header().uri.path())
    }
}

#[async_trait]
impl ProxyHttp for CachingProxy {
    type CTX = ();
    fn new_ctx(&self) -> Self::CTX {
        ()
    }

    // 处理HTTP请求的方法
    async fn upstream_peer(&self, session: &mut Session, _ctx: &mut Self::CTX) -> Result<Box<HttpPeer>> {
        // 检查是否可以从缓存中获取响应
        if self.is_cacheable(session) {
            let cache_key = self.cache_key(session);
            
            // 尝试从缓存中获取响应
            let mut cache = self.cache.lock().unwrap();
            if let Some(entry) = cache.get(&cache_key) {
                if !entry.is_expired() {
                    println!("Cache hit for: {}", cache_key);
                    
                    // 即使是缓存命中，我们也需要返回一个有效的 HttpPeer
                    // 我们将在 response_filter 中设置缓存的响应
                    // 但是我们仍然需要一个有效的上游连接，以防我们需要获取新的内容
                    let peer = Box::new(HttpPeer::new(self.upstream.clone(), false, String::new()));
                    return Ok(peer);
                } else {
                    // 缓存已过期，移除
                    cache.remove(&cache_key);
                }
            }
        }
        
        // 缓存未命中，创建一个HTTP上游连接
        println!("Cache miss, forwarding to: {}", self.upstream);
        let peer = Box::new(HttpPeer::new(self.upstream.clone(), false, String::new()));
        Ok(peer)
    }

    // 处理响应，可能将其存储到缓存中
    async fn response_filter(&self, session: &mut Session, upstream_response: &mut pingora_http::ResponseHeader, _ctx: &mut Self::CTX) -> Result<()> {
        // 检查是否可以缓存响应
        if self.is_cacheable(session) {
            let cache_key = self.cache_key(session);
            
            // 检查是否是缓存命中的情况
            let mut cache = self.cache.lock().unwrap();
            if let Some(entry) = cache.get(&cache_key) {
                if !entry.is_expired() {
                    // 缓存命中，设置响应头
                    for (name, value) in &entry.headers {
                        upstream_response.insert_header(name.clone(), value.clone()).unwrap();
                    }
                    
                    // 添加缓存标记
                    upstream_response.insert_header("X-Cache", "HIT").unwrap();
                    return Ok(());
                }
            }
            
            // 缓存未命中，但响应可缓存
            if upstream_response.status.is_success() {
                // 提取响应头
                let mut headers = HashMap::new();
                for (name, value) in upstream_response.headers.iter() {
                    headers.insert(name.to_string(), value.to_str().unwrap_or_default().to_string());
                }
                
                // 创建缓存条目（注意：我们不能获取响应体，所以这里使用空内容）
                let entry = CacheEntry {
                    content: Vec::new(), // 空内容，因为我们不能获取响应体
                    headers,
                    created_at: Instant::now(),
                    ttl: self.cache_ttl,
                };
                
                // 存储到缓存中
                cache.insert(cache_key, entry);
                
                // 添加缓存标记
                upstream_response.insert_header("X-Cache", "MISS").unwrap();
            }
        }
        
        Ok(())
    }
}

// 注意：我们不再需要EmptyPeer结构体，因为HttpPeer是一个结构体而不是trait
// 对于缓存命中的情况，我们直接使用HttpPeer但设置一个不会被使用的地址

// 启动缓存代理服务
pub fn start_cache_proxy() {
    // 初始化日志
    env_logger::init();
    
    // 创建服务器配置
    let mut server = Server::new(None).unwrap();
    server.bootstrap();
    
    // 创建缓存代理实例
    let caching_proxy = CachingProxy::new(
        "example.com:80".to_string(),
        60, // 缓存TTL为60秒
    );
    
    // 创建HTTP代理服务
    let mut cache_proxy = http_proxy_service(
        &server.configuration,
        caching_proxy,
    );
    
    // 添加TCP监听地址
    cache_proxy.add_tcp("127.0.0.1:8082");
    
    // 添加服务到服务器
    server.add_service(cache_proxy);
    
    // 运行服务器，阻塞直到服务器关闭
    server.run_forever();
}