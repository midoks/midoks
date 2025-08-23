use crate::rpc::fastcdn::PingRequest;
use crate::rpc::fastcdn::ping_client::PingClient;
use tonic::Request;

pub struct Ping {
    client: PingClient<tonic::transport::Channel>,
}

impl Ping {
    // 连接
    pub async fn connect(addr: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let client = PingClient::connect(addr.to_string()).await?;
        Ok(Self { client })
    }
    // 测试
    pub async fn ping(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let request = Request::new(PingRequest {});

        let _response = self.client.ping(request).await?;
        Ok("PingResponse".to_string()) // ping服务返回空响应，这里返回固定字符串
    }
}
