use tonic::{Request, Response, Status};

use crate::rpc::auth::AuthMiddleware;
use crate::rpc::fastcdn::ping_server::Ping;
use crate::rpc::fastcdn::{PingRequest, PingResponse};

/// Ping 实现
#[derive(Debug, Default)]
pub struct FcPing {}

#[tonic::async_trait]
impl Ping for FcPing {
    async fn ping(&self, request: Request<PingRequest>) -> Result<Response<PingResponse>, Status> {
        // 验证请求头认证
        AuthMiddleware::verify_request(&request)?;

        println!("收到Ping请求: {:?}", request);

        let reply = PingResponse {};

        Ok(Response::new(reply))
    }
}
