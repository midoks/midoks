use tonic::Request;
use crate::rpc::hello::hello_service_client::HelloServiceClient;
use crate::rpc::hello::HelloRequest;

pub struct HelloClient {
    client: HelloServiceClient<tonic::transport::Channel>,
}

impl HelloClient {
    /// 连接到HelloService
    pub async fn connect(addr: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let client = HelloServiceClient::connect(addr.to_string()).await?;
        Ok(Self { client })
    }
    
    /// 调用SayHello方法
    pub async fn say_hello(&mut self, name: &str) -> Result<String, Box<dyn std::error::Error>> {
        let request = Request::new(HelloRequest {
            name: name.to_string(),
        });
        
        let response = self.client.say_hello(request).await?;
        Ok(response.into_inner().message)
    }
}