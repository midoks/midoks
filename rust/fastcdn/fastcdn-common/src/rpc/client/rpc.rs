use crate::rpc::auth::AuthMiddleware;
use crate::rpc::fastcdn::{
    AdminCreateRequest, AdminCreateResponse, AdminLoginRequest, AdminLoginResponse,
};
use tonic::codegen::*;
use tonic::transport::Channel;
use tonic::{Request, metadata::MetadataValue};

pub struct Rpc {
    channel: Channel,
}

#[derive(Debug, Clone)]
pub enum RequestType {
    ADMIN,
    API,
    Other(String),
}

impl Rpc {
    pub async fn connect(addr: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let channel = Channel::from_shared(addr.to_string())?.connect().await?;
        Ok(Rpc { channel })
    }

    /// 统一的 metadata 处理方法
    fn prepare_request_with_metadata<T>(
        &self,
        req: T,
        request_type: RequestType,
    ) -> Result<Request<T>, Box<dyn std::error::Error>> {
        let mut request = Request::new(req);

        // 根据请求类型设置特定的 metadata
        match request_type {
            RequestType::ADMIN => {
                // 管理员 metadata
                request
                    .metadata_mut()
                    .insert("request-type", MetadataValue::try_from("login")?);
                request
                    .metadata_mut()
                    .insert("client-version", MetadataValue::try_from("1.0.0")?);
            }
            RequestType::API => {
                // api metadata
                request
                    .metadata_mut()
                    .insert("request-type", MetadataValue::try_from("create-admin")?);
                request
                    .metadata_mut()
                    .insert("admin-operation", MetadataValue::try_from("create")?);
            }
            RequestType::Other(ref op) => {
                // 其他请求类型的 metadata
                request
                    .metadata_mut()
                    .insert("request-type", MetadataValue::try_from("other")?);
                request
                    .metadata_mut()
                    .insert("operation", MetadataValue::try_from(op.as_str())?);
            }
        }

        // 添加认证头
        let auth_request = AuthMiddleware::add_auth_headers(request)?;

        // 添加通用的 metadata
        let mut final_request = auth_request;
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
            .to_string();
        final_request
            .metadata_mut()
            .insert("timestamp", MetadataValue::try_from(&timestamp)?);
        final_request
            .metadata_mut()
            .insert("client-id", MetadataValue::try_from("fastcdn-client")?);

        println!(
            "准备请求 - 类型: {:?}, metadata: {:?}",
            request_type,
            final_request.metadata()
        );

        Ok(final_request)
    }

    /// 统一的 gRPC 调用方法
    async fn make_grpc_call<T, R>(
        &mut self,
        request: Request<T>,
        url: &str,
    ) -> Result<R, Box<dyn std::error::Error>>
    where
        T: prost::Message + Default + 'static,
        R: prost::Message + Default + 'static,
    {
        let mut client = tonic::client::Grpc::new(self.channel.clone());
        client.ready().await?;

        let codec = tonic::codec::ProstCodec::default();
        let path = url.parse::<http::uri::PathAndQuery>()?;

        let response = client.unary(request, path, codec).await?;
        Ok(response.into_inner())
    }

    pub async fn login(
        &mut self,
        req: AdminLoginRequest,
    ) -> Result<AdminLoginResponse, Box<dyn std::error::Error>> {
        let request = self.prepare_request_with_metadata(req, RequestType::ADMIN)?;
        self.make_grpc_call(request, "/fastcdn.Admin/login").await
    }
    pub async fn create(
        &mut self,
        req: AdminCreateRequest,
    ) -> Result<AdminCreateResponse, Box<dyn std::error::Error>> {
        let request = self.prepare_request_with_metadata(req, RequestType::ADMIN)?;
        self.make_grpc_call(request, "/fastcdn.Admin/create").await
    }
}
