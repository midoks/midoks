use tonic::{Request, Response, Status};

use crate::rpc::ping::ping_service_server::PingService;
use crate::rpc::ping::{PingRequest, PingResponse};

/// PingService 实现
#[derive(Debug, Default)]
pub struct FcPingService {}

#[tonic::async_trait]
impl PingService for FcPingService {
    async fn ping(&self, request: Request<PingRequest>) -> Result<Response<PingResponse>, Status> {
        println!("收到Ping请求: {:?}", request);

        let reply = PingResponse {};

        Ok(Response::new(reply))
    }
}
