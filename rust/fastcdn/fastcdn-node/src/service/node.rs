// 启动节点服务器（前台模式）
pub async fn start() -> Result<(), Box<dyn std::error::Error>> {
    println!("正在启动 fastcdn node 服务器...");

    // 测试与API服务器的连接
    super::test::test_api_connection().await?;

    println!("✓ fastcdn node 服务器启动成功");
    println!("✓ 已连接到 fastcdn-api RPC 服务");

    // 这里可以添加实际的服务器逻辑
    // 为了演示，我们让它保持运行
    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
        println!("time...");
        // 定期ping API服务器
        if let Err(e) = super::test::ping_api_server().await {
            println!("警告: 无法连接到API服务器: {}", e);
        }
    }
}
