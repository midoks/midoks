use std::sync::Arc;
use std::net::SocketAddr;
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio_rustls::{TlsAcceptor, rustls};
use tracing::{info, warn};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化日志
    tracing_subscriber::fmt::init();

    // 生成自签名证书
    let cert = rcgen::generate_simple_self_signed(vec!["localhost".into()])?;
    let cert_der = cert.serialize_der()?;
    let priv_key = cert.serialize_private_key_der();

    // 配置TLS
    let certs = vec![rustls::Certificate(cert_der)];
    let key = rustls::PrivateKey(priv_key);
    
    let config = rustls::ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth()
        .with_single_cert(certs, key)?;
    
    let acceptor = TlsAcceptor::from(Arc::new(config));

    // 绑定地址
    let addr: SocketAddr = "127.0.0.1:8443".parse()?;
    let listener = TcpListener::bind(&addr).await?;
    
    info!("HTTPS服务器启动在 {}", addr);
    info!("等待客户端连接...");

    // 处理连接
    while let Ok((stream, peer_addr)) = listener.accept().await {
        let acceptor = acceptor.clone();
        tokio::spawn(async move {
            if let Err(e) = handle_connection(acceptor, stream, peer_addr).await {
                warn!("处理连接失败: {}", e);
            }
        });
    }

    Ok(())
}

async fn handle_connection(
    acceptor: TlsAcceptor,
    stream: TcpStream,
    peer_addr: SocketAddr,
) -> Result<(), Box<dyn std::error::Error>> {
    info!("新的TCP连接: {}", peer_addr);
    
    // TLS握手
    let tls_stream = acceptor.accept(stream).await?;
    info!("TLS握手完成: {}", peer_addr);
    
    let (mut reader, mut writer) = tokio::io::split(tls_stream);
    
    loop {
        let mut buffer = vec![0; 1024];
        match reader.read(&mut buffer).await {
            Ok(0) => {
                info!("客户端断开连接: {}", peer_addr);
                break;
            }
            Ok(n) => {
                let request = String::from_utf8_lossy(&buffer[..n]);
                info!("收到HTTPS请求: {}", request.lines().next().unwrap_or(""));
                
                // 简单的HTTP响应
                let response_body = format!("HTTPS服务器收到请求: {}", request.trim());
                let response = format!(
                    "HTTP/1.1 200 OK\r\n\
                     Content-Type: text/plain\r\n\
                     Content-Length: {}\r\n\
                     Connection: keep-alive\r\n\
                     \r\n\
                     {}",
                    response_body.len(),
                    response_body
                );
                
                if let Err(e) = writer.write_all(response.as_bytes()).await {
                    warn!("发送响应失败: {}", e);
                    break;
                }
            }
            Err(e) => {
                warn!("读取数据失败: {}", e);
                break;
            }
        }
    }
    
    Ok(())
}