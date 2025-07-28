use std::net::UdpSocket;
use std::time::Instant;

fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:0")?;
    let server_addr = "127.0.0.1:9090";
    
    println!("正在连接到UDP服务器: {}", server_addr);
    
    // UDP是无连接的，所以没有连接建立时间
    println!("UDP连接建立成功！(无连接协议)");
    println!("目标地址: {}", server_addr);
    
    // 发送多条消息测试
    for i in 1..=5 {
        let message = format!("Hello UDP! 消息 #{}", i);
        
        let start_time = Instant::now();
        
        // 发送消息
        socket.send_to(message.as_bytes(), server_addr)?;
        
        // 接收响应
        let mut buffer = [0; 1024];
        let (size, _) = socket.recv_from(&mut buffer)?;
        let response = String::from_utf8_lossy(&buffer[..size]);
        
        let round_trip_time = start_time.elapsed();
        
        println!("发送: {}", message);
        println!("收到: {}", response);
        println!("往返时间: {:?}\n", round_trip_time);
        
        // 等待一秒再发送下一条消息
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
    
    println!("UDP客户端关闭");
    Ok(())
}