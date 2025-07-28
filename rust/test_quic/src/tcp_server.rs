use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Instant;

fn handle_client(mut stream: TcpStream) {
    println!("新的TCP连接: {}", stream.peer_addr().unwrap());
    
    let mut buffer = [0; 1024];
    loop {
        match stream.read(&mut buffer) {
            Ok(0) => {
                println!("客户端断开连接");
                break;
            }
            Ok(n) => {
                let message = String::from_utf8_lossy(&buffer[..n]);
                println!("收到消息: {}", message);
                
                // 回复消息
                let response = format!("TCP服务器收到: {}", message);
                if let Err(e) = stream.write_all(response.as_bytes()) {
                    println!("发送响应失败: {}", e);
                    break;
                }
            }
            Err(e) => {
                println!("读取数据失败: {}", e);
                break;
            }
        }
    }
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    println!("TCP服务器启动在 127.0.0.1:8080");
    println!("等待客户端连接...");
    
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || {
                    handle_client(stream);
                });
            }
            Err(e) => {
                println!("连接失败: {}", e);
            }
        }
    }
    
    Ok(())
}