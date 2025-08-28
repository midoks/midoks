use tonic::Request;
use crate::rpc::fastcdn::hello_service_client::HelloServiceClient;
use crate::rpc::fastcdn::HelloRequest;
use crate::impl_base_rpc_client;

pub struct HelloClient {
    client: HelloServiceClient<tonic::transport::Channel>,
}

// 使用宏实现基础功能
impl_base_rpc_client!(HelloClient, HelloServiceClient<tonic::transport::Channel>);

impl HelloClient {
    /// 调用SayHello方法
    pub async fn say_hello(&mut self, name: &str) -> Result<String, Box<dyn std::error::Error>> {
        let request = Request::new(HelloRequest {
            name: name.to_string(),
        });
        
        let response = self.client.say_hello(request).await?;
        Ok(response.into_inner().message)
    }
}