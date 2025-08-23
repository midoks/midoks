use tonic::{Request, Response, Status};

use crate::rpc::fastcdn::ping_server::Ping;
use crate::rpc::fastcdn::{PingRequest, PingResponse};

/// Ping 实现
#[derive(Debug, Default)]
pub struct FcPingService {}

#[tonic::async_trait]
impl Ping for FcPingService {
    async fn ping(&self, request: Request<PingRequest>) -> Result<Response<PingResponse>, Status> {
        println!("收到Ping请求: {:?}", request);

        let reply = PingResponse {};

        Ok(Response::new(reply))
    }
}
