use tonic::{Request, Response, Status};

use crate::rpc::ping::ping_server::Ping;
use crate::rpc::ping::{PingRequest, PingResponse};

/// Ping 实现
#[derive(Debug, Default)]
pub struct MyPingService {}

#[tonic::async_trait]
impl Ping for MyPingService {
    async fn ping(&self, request: Request<PingRequest>) -> Result<Response<PingResponse>, Status> {
        println!("收到Ping请求: {:?}", request);

        let reply = PingResponse {};

        Ok(Response::new(reply))
    }
}
