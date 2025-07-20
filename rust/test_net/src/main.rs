use std::{
    net::SocketAddr,
    sync::{atomic::AtomicUsize, Arc},
};
use hyper::{
    body::Bytes,
    Request, Response,
    service::service_fn,
};
use hyper_util::{
    rt::{TokioExecutor, TokioIo},
    client::legacy::{
        Client,
        connect::HttpConnector,
    },
    server::conn::auto::Builder as ServerBuilder,
};
use tokio::net::TcpListener;
use lazy_static::lazy_static;
use rand::seq::SliceRandom;

// 后端服务器列表
lazy_static! {
    static ref BACKEND_SERVERS: Vec<&'static str> = vec![
        "http://127.0.0.1:8081",
        "http://127.0.0.1:8082",
        "http://127.0.0.1:8083",
    ];
}

// 负载均衡策略
#[derive(Clone)]
enum LoadBalancer {
    RoundRobin(Arc<AtomicUsize>),
    Random,
}

// 反向代理核心逻辑
async fn proxy_request(
    req: Request<hyper::body::Incoming>,
    client: Client<HttpConnector>,
    balancer: &LoadBalancer,
) -> Result<Response<hyper::body::Incoming>, hyper::Error> {
    // 选择后端服务器
    let backend = match balancer {
        LoadBalancer::RoundRobin(counter) => {
            let idx = counter.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            BACKEND_SERVERS[idx % BACKEND_SERVERS.len()]
        }
        LoadBalancer::Random => BACKEND_SERVERS.choose(&mut rand::thread_rng()).unwrap(),
    };

    // 构建新请求
    let (mut parts, body) = req.into_parts();
    let path = parts.uri.path_and_query().map(|pq| pq.as_str()).unwrap_or("/");
    let new_uri = format!("{}{}", backend, path).parse().unwrap();
    parts.uri = new_uri;
    parts.headers.remove("host");

    // 转发请求
    let proxy_req = Request::from_parts(parts, body);
    client.request(proxy_req).await
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化负载均衡器
    let balancer = LoadBalancer::RoundRobin(Arc::new(AtomicUsize::new(0)));

    // 创建 HTTP 客户端
    let client = Client::builder(TokioExecutor::new())
        .build(HttpConnector::new());

    // 监听地址
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let listener = TcpListener::bind(addr).await?;
    println!("🚀 Reverse Proxy running on http://{}", addr);

    loop {
        let (stream, _) = listener.accept().await?;
        let io = TokioIo::new(stream);
        
        let client = client.clone();
        let balancer = balancer.clone();

        tokio::task::spawn(async move {
            let service = service_fn(move |req| {
                proxy_request(req, client.clone(), &balancer)
            });

            if let Err(err) = ServerBuilder::new(TokioExecutor::new())
                .serve_connection(io, service)
                .await
            {
                eprintln!("Failed to serve connection: {:?}", err);
            }
        });
    }
}