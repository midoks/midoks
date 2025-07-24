// 定义 RPC 服务接口
#[tarpc::service]
pub trait MathService {
    // 加法
    async fn add(a: i32, b: i32) -> i32;

    // 阶乘计算
    async fn factorial(n: u64) -> Result<u64, String>;
}

// tarpc::service宏会自动生成客户端和服务器类型
