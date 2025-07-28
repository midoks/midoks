use std::io::{Read, Write};
use std::net::TcpStream;
use std::time::Instant;

fn main() -> std::io::Result<()> {
    println!("正在连接到TCP服务器: 127.0.0.1:8080");
    
    let start_time = Instant::now();
    let mut stream = TcpStream::connect("127.0.0.1:8080")?;
    let connect_time = start_time.elapsed();
    
    println!("TCP连接建立成功！连接时间: {:?}", connect_time);
    println!("远程地址: {}", stream.peer_addr()?);
    
    // 发送多条消息测试
    for i in 1..=5 {
        let message = format!("Hello TCP! 消息 #{}", i);
        
        let start_time = Instant::now();
        
        // 发送消息
        stream.write_all(message.as_bytes())?;
        
        // 接收响应
        let mut buffer = [0; 1024];
        let n = stream.read(&mut buffer)?;
        let response = String::from_utf8_lossy(&buffer[..n]);
        
        let round_trip_time = start_time.elapsed();
        
        println!("发送: {}", message);
        println!("收到: {}", response);
        println!("往返时间: {:?}\n", round_trip_time);
        
        // 等待一秒再发送下一条消息
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
    
    println!("TCP客户端关闭");
    Ok(())
}