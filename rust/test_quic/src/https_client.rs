use std::sync::Arc;
use std::net::SocketAddr;
use std::time::Instant;
use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio_rustls::{TlsConnector, rustls};
use tracing::{info, warn};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化日志
    tracing_subscriber::fmt::init();

    // 配置TLS客户端（接受自签名证书）
    let config = rustls::ClientConfig::builder()
        .with_safe_defaults()
        .with_custom_certificate_verifier(Arc::new(SkipServerVerification))
        .with_no_client_auth();
    
    let connector = TlsConnector::from(Arc::new(config));
    let server_addr: SocketAddr = "127.0.0.1:8443".parse()?;
    
    info!("正在连接到HTTPS服务器: {}", server_addr);
    
    let start_time = Instant::now();
    
    // TCP连接
    let tcp_stream = TcpStream::connect(server_addr).await?;
    let tcp_connect_time = start_time.elapsed();
    
    // TLS握手
    let tls_start = Instant::now();
    let domain = rustls::ServerName::try_from("localhost")?;
    let tls_stream = connector.connect(domain, tcp_stream).await?;
    let tls_handshake_time = tls_start.elapsed();
    let total_connect_time = start_time.elapsed();
    
    info!("TCP连接时间: {:?}", tcp_connect_time);
    info!("TLS握手时间: {:?}", tls_handshake_time);
    info!("HTTPS连接建立成功！总连接时间: {:?}", total_connect_time);
    
    let (mut reader, mut writer) = tokio::io::split(tls_stream);
    
    // 发送多个HTTP请求测试
    for i in 1..=5 {
        let request_body = format!("Hello HTTPS! 消息 #{}", i);
        let request = format!(
            "POST /test HTTP/1.1\r\n\
             Host: localhost:8443\r\n\
             Content-Type: text/plain\r\n\
             Content-Length: {}\r\n\
             Connection: keep-alive\r\n\
             \r\n\
             {}",
            request_body.len(),
            request_body
        );
        
        let start_time = Instant::now();
        
        // 发送HTTP请求
        writer.write_all(request.as_bytes()).await?;
        
        // 接收HTTP响应
        let mut buffer = vec![0; 2048];
        let n = reader.read(&mut buffer).await?;
        let response = String::from_utf8_lossy(&buffer[..n]);
        
        let round_trip_time = start_time.elapsed();
        
        // 解析HTTP响应
        let response_lines: Vec<&str> = response.lines().collect();
        let status_line = response_lines.get(0).unwrap_or(&"Unknown");
        let body_start = response.find("\r\n\r\n").map(|i| i + 4).unwrap_or(0);
        let response_body = &response[body_start..].trim();
        
        info!("发送: {}", request_body);
        info!("状态: {}", status_line);
        info!("收到: {}", response_body);
        info!("往返时间: {:?}\n", round_trip_time);
        
        // 等待一秒再发送下一个请求
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }
    
    info!("HTTPS客户端关闭");
    Ok(())
}

// 跳过服务器证书验证（仅用于测试）
struct SkipServerVerification;

impl rustls::client::ServerCertVerifier for SkipServerVerification {
    fn verify_server_cert(
        &self,
        _end_entity: &rustls::Certificate,
        _intermediates: &[rustls::Certificate],
        _server_name: &rustls::ServerName,
        _scts: &mut dyn Iterator<Item = &[u8]>,
        _ocsp_response: &[u8],
        _now: std::time::SystemTime,
    ) -> Result<rustls::client::ServerCertVerified, rustls::Error> {
        Ok(rustls::client::ServerCertVerified::assertion())
    }
}