use crate::config::api_admin::ApiAdmin;
use crate::rpc::auth::AuthMiddleware;
use lazy_static::lazy_static;
use std::sync::{Arc, Mutex};
use tonic::transport::Channel;
use tonic::{Request, metadata::MetadataValue};
use tonic::{Status, codegen::*};

lazy_static! {
    static ref INSTANCE: Arc<Mutex<Option<Arc<CommonRpc>>>> = Arc::new(Mutex::new(None));
}

pub struct CommonRpc {
    channel: Channel,
}

#[derive(Debug, Clone)]
pub enum RequestAuth {
    ADMIN,
    API,
    Other,
}

impl CommonRpc {
    pub async fn instance() -> Result<Arc<Self>, Box<dyn std::error::Error>> {
        {
            let instance = INSTANCE.lock().unwrap();
            if let Some(rpc) = instance.as_ref() {
                return Ok(rpc.clone());
            }
        }

        // Create new instance if none exists
        let rpc = Self::admin_rpc().await?;
        {
            let mut instance = INSTANCE.lock().unwrap();
            if instance.is_none() {
                *instance = Some(rpc.clone());
            }
        }
        Ok(rpc)
    }

    pub async fn admin_rpc() -> Result<Arc<Self>, Box<dyn std::error::Error>> {
        let api_admin = ApiAdmin::instance()
            .map_err(|e| Status::internal(format!("api_admin loading failed: {}", e)))?;

        let config = api_admin.lock().unwrap();

        let channel = Channel::from_shared(config.rpc_endpoints[0].to_string())?
            .connect()
            .await?;
        let rpc = Arc::new(CommonRpc { channel });
        Ok(rpc)
    }

    pub async fn connect(addr: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let channel = Channel::from_shared(addr.to_string())?.connect().await?;
        Ok(CommonRpc { channel })
    }

    /// 统一的 metadata 处理方法
    pub fn prepare_request_with_metadata<T>(
        &self,
        req: T,
        request_type: RequestAuth,
    ) -> Result<Request<T>, Box<dyn std::error::Error>> {
        let mut request = Request::new(req);

        let version = env!("CARGO_PKG_VERSION");
        request
            .metadata_mut()
            .insert("client-version", MetadataValue::try_from(version)?);

        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
            .to_string();
        request
            .metadata_mut()
            .insert("timestamp", MetadataValue::try_from(&timestamp)?);

        // 根据请求类型设置特定的 metadata
        match request_type {
            RequestAuth::ADMIN => {
                // 管理员 metadata
                request = AuthMiddleware::add_header_admin(request)?;
                request
                    .metadata_mut()
                    .insert("client-id", MetadataValue::try_from("fastcdn-admin")?);
            }
            RequestAuth::API => {
                // api metadata
                request = AuthMiddleware::add_header_api(request)?;
                request
                    .metadata_mut()
                    .insert("client-id", MetadataValue::try_from("fastcdn-api")?);
            }
            RequestAuth::Other => {
                // 其他请求类型的 metadata
                request = AuthMiddleware::add_header_admin(request)?;
            }
        }

        println!(
            "request: {:?}, metadata: {:?}",
            request_type,
            request.metadata()
        );

        Ok(request)
    }

    /// 统一的 gRPC 调用方法
    pub async fn make_grpc_call<T, R>(
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
}
