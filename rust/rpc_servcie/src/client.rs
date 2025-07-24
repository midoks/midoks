use crate::service::MathServiceClient;
use anyhow::Result;
use tarpc::context;
use tarpc::serde_transport::tcp;
use tarpc::tokio_serde::formats::Json;

pub async fn run_client(addr: &str) -> Result<()> {
    let transport = tcp::connect(addr, Json::default).await?;
    let client = MathServiceClient::new(tarpc::client::Config::default(), transport).spawn();

    // 创建上下文
    let ctx = context::current();

    // 调用加法服务
    let sum = client.add(ctx.clone(), 5, 3).await?;
    println!("5 + 3 = {}", sum);

    // 调用阶乘服务
    for n in [5, 10, 25] {
        match client.factorial(ctx.clone(), n).await {
            Ok(val) => println!("{}! = {}", n, val),
            Err(e) => println!("Error calculating {}!: {}", n, e),
        }
    }

    Ok(())
}
