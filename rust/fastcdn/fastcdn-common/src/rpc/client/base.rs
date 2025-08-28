use crate::rpc::auth::AuthMiddleware;
use std::error::Error;
use tonic::Request;

/// 基础RPC客户端trait
/// 定义所有RPC客户端的通用行为
pub trait BaseRpcClient {
    type Client;
    type Error: Error + Send + Sync + 'static;

    /// 连接到RPC服务
    fn connect(
        addr: &str,
    ) -> impl std::future::Future<Output = Result<Self, Box<dyn Error>>> + Send
    where
        Self: Sized;

    /// 获取内部客户端引用
    fn client(&self) -> &Self::Client;

    /// 获取内部客户端可变引用
    fn client_mut(&mut self) -> &mut Self::Client;
}

/// 基础RPC客户端结构体
/// 提供通用的连接和认证功能
pub struct BaseClient<T> {
    client: T,
    addr: String,
}

impl<T> BaseClient<T> {
    /// 创建新的基础客户端实例
    pub fn new(client: T, addr: String) -> Self {
        Self { client, addr }
    }

    /// 获取客户端地址
    pub fn addr(&self) -> &str {
        &self.addr
    }

    /// 获取内部客户端引用
    pub fn client(&self) -> &T {
        &self.client
    }

    /// 获取内部客户端可变引用
    pub fn client_mut(&mut self) -> &mut T {
        &mut self.client
    }

    /// 为请求添加认证头
    pub fn add_auth_headers<R>(&self, request: Request<R>) -> Result<Request<R>, Box<dyn Error>> {
        AuthMiddleware::add_auth_headers(request).map_err(|e| Box::new(e) as Box<dyn Error>)
    }
}

/// 连接助手trait
/// 为具体的gRPC客户端提供连接功能
pub trait ConnectableClient<T> {
    fn connect_to(
        addr: String,
    ) -> impl std::future::Future<Output = Result<T, Box<dyn Error>>> + Send;
}

/// 为tonic生成的客户端实现连接功能的宏
#[macro_export]
macro_rules! impl_connectable_client {
    ($client_type:ty) => {
        impl ConnectableClient<$client_type> for $client_type {
            async fn connect_to(addr: String) -> Result<$client_type, Box<dyn std::error::Error>> {
                let client = <$client_type>::connect(addr).await?;
                Ok(client)
            }
        }
    };
}

/// 实现基础RPC客户端的宏
#[macro_export]
macro_rules! impl_base_rpc_client {
    ($struct_name:ident, $client_type:ty) => {
        impl $struct_name {
            /// 连接到RPC服务
            pub async fn connect(addr: &str) -> Result<Self, Box<dyn std::error::Error>> {
                let client = <$client_type>::connect(addr.to_string()).await?;
                Ok(Self { client })
            }

            /// 为请求添加认证头
            pub fn add_auth_headers<R>(
                &self,
                request: tonic::Request<R>,
            ) -> Result<tonic::Request<R>, Box<dyn std::error::Error>> {
                crate::rpc::auth::AuthMiddleware::add_auth_headers(request)
                    .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
            }
        }
    };
}
