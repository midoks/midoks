use crate::rpc::fastcdn::AdminLoginRequest;
use crate::rpc::fastcdn::AdminLoginResponse;
use crate::rpc::fastcdn::admin_client::AdminClient;
// 移除这行未使用的导入
// use tonic::Request;

pub struct Admin {
    client: AdminClient<tonic::transport::Channel>,
}

impl Admin {
    pub async fn connect(addr: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let client = AdminClient::connect(addr.to_string()).await?;
        Ok(Self { client })
    }

    pub async fn create(addr: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let client = AdminClient::connect(addr.to_string()).await?;
        Ok(Self { client })
    }

    // 测试
    pub async fn login(
        &mut self,
        req: AdminLoginRequest,
    ) -> Result<AdminLoginResponse, Box<dyn std::error::Error>> {
        let response = self.client.login(req).await?;
        Ok(response.into_inner())
    }
}
