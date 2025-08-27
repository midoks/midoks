use crate::rpc::fastcdn::{AdminLoginRequest, AdminLoginResponse, AdminCreateRequest, AdminCreateResponse};
use crate::rpc::fastcdn::admin_client::AdminClient;
use tonic::Request;
use crate::rpc::auth::AuthMiddleware;

pub struct Admin {
    client: AdminClient<tonic::transport::Channel>,
}

impl Admin {
    pub async fn connect(addr: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let client = AdminClient::connect(addr.to_string()).await?;
        Ok(Admin { client })
    }

    pub async fn create(addr: &str) -> Result<Self, Box<dyn std::error::Error>> {
        Self::connect(addr).await
    }

    pub async fn login(
        &mut self,
        req: AdminLoginRequest,
    ) -> Result<AdminLoginResponse, Box<dyn std::error::Error>> {
        // 创建 tonic::Request 并添加认证头
        let request = Request::new(req);
        let authenticated_request = AuthMiddleware::add_auth_headers(request)?;
        
        let response = self.client.login(authenticated_request).await?;
        Ok(response.into_inner())
    }

    pub async fn create_admin(
        &mut self,
        req: AdminCreateRequest,
    ) -> Result<AdminCreateResponse, Box<dyn std::error::Error>> {
        // 创建 tonic::Request 并添加认证头
        let request = Request::new(req);
        let authenticated_request = AuthMiddleware::add_auth_headers(request)?;
        
        let response = self.client.create(authenticated_request).await?;
        Ok(response.into_inner())
    }
}
