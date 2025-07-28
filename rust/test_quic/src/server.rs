use quinn::{Endpoint, ServerConfig};
use std::error::Error;
use std::net::SocketAddr;
use std::sync::Arc;
use tracing::{info, warn};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // 初始化日志
    tracing_subscriber::fmt::init();

    // 生成自签名证书
    let cert = rcgen::generate_simple_self_signed(vec!["localhost".into()])?;
    let cert_der = cert.serialize_der()?;
    let priv_key = cert.serialize_private_key_der();

    // 配置TLS
    let mut server_crypto = rustls::ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth()
        .with_single_cert(
            vec![rustls::Certificate(cert_der.clone())],
            rustls::PrivateKey(priv_key),
        )?;
    server_crypto.alpn_protocols = vec![b"hq-29".to_vec()];

    // 创建QUIC服务器配置
    let server_config = ServerConfig::with_crypto(Arc::new(server_crypto));

    // 绑定地址
    let addr: SocketAddr = "127.0.0.1:4433".parse()?;
    let endpoint = Endpoint::server(server_config, addr)?;
    
    info!("QUIC服务器启动在 {}", addr);
    info!("等待客户端连接...");

    // 处理连接
    while let Some(conn) = endpoint.accept().await {
        tokio::spawn(async move {
            match conn.await {
                Ok(connection) => {
                    info!("新的QUIC连接建立: {}", connection.remote_address());
                    
                    // 处理双向流
                    while let Ok((mut send, mut recv)) = connection.accept_bi().await {
                        tokio::spawn(async move {
                            match recv.read_to_end(1024).await {
                                Ok(data) => {
                                    let msg = String::from_utf8_lossy(&data);
                                    info!("收到消息: {}", msg);
                                    
                                    // 回复消息
                                    let response = format!("服务器收到: {}", msg);
                                    if let Err(e) = send.write_all(response.as_bytes()).await {
                                        warn!("发送响应失败: {}", e);
                                    }
                                    
                                    if let Err(e) = send.finish().await {
                                        warn!("关闭发送流失败: {}", e);
                                    }
                                }
                                Err(e) => warn!("读取数据失败: {}", e),
                            }
                        });
                    }
                }
                Err(e) => warn!("连接失败: {}", e),
            }
        });
    }

    Ok(())
}