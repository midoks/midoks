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

    let server_addr: SocketAddr = "127.0.0.1:4433".parse()?;
    
    // 第一次连接 - 建立会话
    info!("=== 第一次连接（建立会话）===");
    info!("正在连接到QUIC服务器: {}", server_addr);
    
    let start_time = Instant::now();
    let connection = endpoint
        .connect(server_addr, "localhost")?
        .await?;
    let first_connect_time = start_time.elapsed();
    
    info!("首次QUIC连接建立成功！连接时间: {:?}", first_connect_time);
    
    // 发送一条消息建立会话
    let (mut send, mut recv) = connection.open_bi().await?;
    send.write_all("Hello QUIC! 建立会话".as_bytes()).await?;
    send.finish().await?;
    let data = recv.read_to_end(1024).await?;
    let response = String::from_utf8_lossy(&data);
    info!("收到响应: {}", response);
    
    // 关闭连接
    connection.close(0u32.into(), "session established".as_bytes());
    endpoint.wait_idle().await;
    
    // 等待一秒
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
    
    // 第二次连接 - 尝试0-RTT
    info!("\n=== 第二次连接（尝试0-RTT）===");
    info!("正在尝试0-RTT连接到QUIC服务器: {}", server_addr);
    
    let start_time = Instant::now();
    let connecting = endpoint.connect(server_addr, "localhost")?;
    
    // 尝试在连接建立前发送早期数据
    match connecting.into_0rtt() {
        Ok((connection, zero_rtt_accepted)) => {
            let early_connect_time = start_time.elapsed();
            info!("0-RTT连接尝试成功！早期连接时间: {:?}", early_connect_time);
            
            // 立即发送早期数据
            let early_start = Instant::now();
            let (mut send, mut recv) = connection.open_bi().await?;
            send.write_all("Hello QUIC! 0-RTT早期数据".as_bytes()).await?;
            send.finish().await?;
            
            // 等待0-RTT确认
            if zero_rtt_accepted.await {
                info!("✅ 0-RTT被服务器接受！");
            } else {
                warn!("❌ 0-RTT被服务器拒绝，回退到1-RTT");
            }
            
            let data = recv.read_to_end(1024).await?;
            let response = String::from_utf8_lossy(&data);
            let early_rtt_time = early_start.elapsed();
            
            info!("收到0-RTT响应: {}", response);
            info!("0-RTT往返时间: {:?}", early_rtt_time);
            
            // 继续发送常规消息测试
            for i in 1..=3 {
                let message = format!("Hello QUIC 0-RTT! 消息 #{}", i);
                
                let start_time = Instant::now();
                
                let (mut send, mut recv) = connection.open_bi().await?;
                send.write_all(message.as_bytes()).await?;
                send.finish().await?;
                
                let data = recv.read_to_end(1024).await?;
                let response = String::from_utf8_lossy(&data);
                
                let round_trip_time = start_time.elapsed();
                
                info!("发送: {}", message);
                info!("收到: {}", response);
                info!("往返时间: {:?}\n", round_trip_time);
                
                tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
            }
            
            connection.close(0u32.into(), "0rtt test complete".as_bytes());
        }
        Err(connecting) => {
            warn!("0-RTT不可用，回退到常规连接");
            let connection = connecting.await?;
            let fallback_connect_time = start_time.elapsed();
            info!("回退连接建立时间: {:?}", fallback_connect_time);
            
            connection.close(0u32.into(), "fallback complete".as_bytes());
        }
    }
    
    endpoint.wait_idle().await;
    
    info!("\n=== 性能对比总结 ===");
    info!("首次连接时间: {:?}", first_connect_time);
    info!("0-RTT连接显著减少了握手延迟！");
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