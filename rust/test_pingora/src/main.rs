use clap::{Parser, Subcommand};
use pingora_core::server::Server;
use pingora_core::upstreams::peer::HttpPeer;
use pingora_core::Result;
use pingora_proxy::{ProxyHttp, Session};
use pingora_proxy::http_proxy_service;
use async_trait::async_trait;

// 导入自定义模块
use test_pingora::{start_load_balancer, start_cache_proxy};

// 定义一个简单的HTTP代理服务
struct SimpleProxy {
    upstream: String,
}

#[async_trait]
impl ProxyHttp for SimpleProxy {
    type CTX = ();
    fn new_ctx(&self) -> Self::CTX {
        ()
    }

    // 处理HTTP请求的方法
    async fn upstream_peer(&self, _session: &mut Session, _ctx: &mut Self::CTX) -> Result<Box<HttpPeer>> {
        // 创建一个HTTP上游连接
        let peer = Box::new(HttpPeer::new(self.upstream.clone(), false, String::new()));
        Ok(peer)
    }
}

// 启动简单代理服务
fn start_simple_proxy() {
    // 初始化日志
    env_logger::init();
    
    // 创建服务器配置
    let mut server = Server::new(None).unwrap();
    server.bootstrap();
    
    // 创建简单代理实例
    let simple_proxy = SimpleProxy {
        upstream: "example.com:80".to_string(),
    };
    
    // 设置代理服务
    let mut proxy = http_proxy_service(
        &server.configuration,
        simple_proxy,
    );
    
    // 添加TCP监听地址
    proxy.add_tcp("127.0.0.1:8080");
    
    // 添加服务到服务器
    server.add_service(proxy);
    
    // 运行服务器，阻塞直到服务器关闭
    server.run_forever();
}

// 定义命令行参数
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// 启动简单代理服务器
    Simple,
    /// 启动负载均衡代理服务器
    LoadBalancer,
    /// 启动带缓存的代理服务器
    Cache,
}

fn main() {
    // 解析命令行参数
    let cli = Cli::parse();

    // 根据命令行参数启动不同类型的代理服务器
    match cli.command.unwrap_or(Commands::Simple) {
        Commands::Simple => {
            println!("启动简单代理服务器，监听端口 8080");
            start_simple_proxy();
        },
        Commands::LoadBalancer => {
            println!("启动负载均衡代理服务器，监听端口 8081");
            start_load_balancer();
        },
        Commands::Cache => {
            println!("启动带缓存的代理服务器，监听端口 8082");
            start_cache_proxy();
        },
    }
}