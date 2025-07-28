use quinn::{ClientConfig, Endpoint};
use std::error::Error;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Instant;
use tracing::{info, warn};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // 初始化日志
    tracing_subscriber::fmt::init();

    // 配置客户端TLS支持0-RTT（接受自签名证书）
    let mut crypto = rustls::ClientConfig::builder()
        .with_safe_defaults()
        .with_custom_certificate_verifier(Arc::new(SkipServerVerification))
        .with_no_client_auth();
    crypto.alpn_protocols = vec![b"hq-29".to_vec()];
    
    // 启用0-RTT支持
    crypto.enable_early_data = true;

    // 配置传输参数以优化0-RTT性能
    let mut transport_config = quinn::TransportConfig::default();
    transport_config.max_concurrent_bidi_streams(100u32.into());
    transport_config.max_concurrent_uni_streams(100u32.into());
    transport_config.max_idle_timeout(Some(std::time::Duration::from_secs(30).try_into()?));
    
    // 创建QUIC客户端配置并启用0-RTT
    let mut client_config = ClientConfig::new(Arc::new(crypto));
    client_config.transport_config(Arc::new(transport_config));

    // 创建端点
    let mut endpoint = Endpoint::client("0.0.0.0:0".parse()?)?;
    endpoint.set_default_client_config(client_config);

    // 连接到服务器
    let server_addr: SocketAddr = "127.0.0.1:4433".parse()?;
    info!("正在连接到QUIC服务器: {}", server_addr);
    
    let start_time = Instant::now();
    let connection = endpoint
        .connect(server_addr, "localhost")?
        .await?;
    let connect_time = start_time.elapsed();
    
    info!("QUIC连接建立成功！连接时间: {:?}", connect_time);
    info!("远程地址: {}", connection.remote_address());

    // 发送多条消息测试
    for i in 1..=5 {
        let message = format!("Hello QUIC! 消息 #{}", i);
        
        let start_time = Instant::now();
        
        // 打开双向流
        let (mut send, mut recv) = connection.open_bi().await?;
        
        // 发送消息
        send.write_all(message.as_bytes()).await?;
        send.finish().await?;
        
        // 接收响应
        let data = recv.read_to_end(1024).await?;
        let response = String::from_utf8_lossy(&data);
        
        let round_trip_time = start_time.elapsed();
        
        info!("发送: {}", message);
        info!("收到: {}", response);
        info!("往返时间: {:?}\n", round_trip_time);
        
        // 等待一秒再发送下一条消息
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }

    // 关闭连接
    connection.close(0u32.into(), b"client closing normally");
    
    // 等待连接完全关闭
    endpoint.wait_idle().await;
    
    info!("客户端关闭");
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