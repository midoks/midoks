use tonic::Request;
use crate::rpc::ping::ping_service_client::PingServiceClient;
use crate::rpc::ping::PingRequest;

pub struct PingClient {
    client: PingServiceClient<tonic::transport::Channel>,
}

impl PingClient {
    /// 连接到PingService
    pub async fn connect(addr: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let client = PingServiceClient::connect(addr.to_string()).await?;
        Ok(Self { client })
    }
    
    /// 调用ping方法
    pub async fn ping(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let request = Request::new(PingRequest {});
        
        let _response = self.client.ping(request).await?;
        Ok("PingResponse".to_string()) // ping服务返回空响应，这里返回固定字符串
    }
}