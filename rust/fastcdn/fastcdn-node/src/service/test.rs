use fastcdn_common::rpc::client::hello::HelloClient;
use fastcdn_common::rpc::client::ping::Ping;
use fastcdn_common::rpc::fastcdn::PingRequest;

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    println!("开始测试...");
    let _ = test_grpc_connection().await;
    Ok(())
}

// 测试gRPC连接
async fn test_grpc_connection() -> Result<(), Box<dyn std::error::Error>> {
    println!("测试gRPC连接...");

    test_api_connection().await?;

    println!("✓ 所有gRPC连接测试通过");
    Ok(())
}

// 测试API连接
pub async fn test_api_connection() -> Result<(), Box<dyn std::error::Error>> {
    // 测试Ping服务
    let mut ping_client = Ping::connect("http://127.0.0.1:50051").await?;
    let ping_request = PingRequest {};
    let _ping_response = ping_client.ping(ping_request).await?;
    println!("✓ Ping服务连接成功");

    // 测试Hello服务
    let mut hello_client = HelloClient::connect("http://127.0.0.1:50051").await?;
    let hello_response = hello_client.say_hello("fastcdn-node").await?;
    println!("✓ Hello服务响应: {}", hello_response);

    Ok(())
}

// 定期ping API服务器
pub async fn ping_api_server() -> Result<(), Box<dyn std::error::Error>> {
    let mut ping_client = Ping::connect("http://127.0.0.1:50051").await?;
    let ping_request = PingRequest {};
    let _ping_response = ping_client.ping(ping_request).await?;
    Ok(())
}
