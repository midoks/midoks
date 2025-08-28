use crate::rpc::auth::AuthMiddleware;
use crate::rpc::fastcdn::ping_client::PingClient;
use crate::rpc::fastcdn::{PingRequest, PingResponse};
use tonic::Request;
use tonic::transport::Channel;

pub struct Ping {
    client: PingClient<Channel>,
}

impl Ping {
    pub async fn connect(addr: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let client = PingClient::connect(addr.to_string()).await?;
        Ok(Ping { client })
    }

    pub async fn ping(
        &mut self,
        req: PingRequest,
    ) -> Result<PingResponse, Box<dyn std::error::Error>> {
        // 创建 tonic::Request 并添加认证头
        let request = Request::new(req);
        let authenticated_request = AuthMiddleware::add_header_api(request)?;

        let response = self.client.ping(authenticated_request).await?;
        Ok(response.into_inner())
    }
}
