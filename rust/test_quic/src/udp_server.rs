use std::net::UdpSocket;
use std::time::Instant;

fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:9090")?;
    println!("UDP服务器启动在 127.0.0.1:9090");
    println!("等待客户端连接...");
    
    let mut buf = [0; 1024];
    
    loop {
        match socket.recv_from(&mut buf) {
            Ok((size, src)) => {
                let message = String::from_utf8_lossy(&buf[..size]);
                println!("收到来自 {} 的消息: {}", src, message);
                
                // 回复消息
                let response = format!("UDP服务器收到: {}", message);
                if let Err(e) = socket.send_to(response.as_bytes(), src) {
                    println!("发送响应失败: {}", e);
                }
            }
            Err(e) => {
                println!("接收数据失败: {}", e);
            }
        }
    }
}