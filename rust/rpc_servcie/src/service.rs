use tarpc::context;

// 定义 RPC 服务接口
#[tarpc::service]
pub trait MathService {
    // 加法
    async fn add(context: context::Context, a: i32, b: i32) -> i32;

    // 阶乘计算
    async fn factorial(context: context::Context, n: u64) -> Result<u64, String>;
}
