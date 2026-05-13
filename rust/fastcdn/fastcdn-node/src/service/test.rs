use fastcdn_common::rpc::client::rpc::CommonRpc;
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
    let mut rpc_client = CommonRpc::connect("http://127.0.0.1:50051").await?;
    let ping_request = PingRequest {};
    let _ping_response = rpc_client.ping(ping_request).await?;
    println!("✓ Ping服务连接成功");

    Ok(())
}

// 定期ping API服务器
pub async fn ping_api_server() -> Result<(), Box<dyn std::error::Error>> {
    let mut rpc_client = CommonRpc::connect("http://127.0.0.1:50051").await?;
    let ping_request = PingRequest {};
    let _ping_response = rpc_client.ping(ping_request).await?;
    Ok(())
}
