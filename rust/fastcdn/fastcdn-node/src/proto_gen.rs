// 手动生成的proto代码

pub mod hello {
    use tonic::{Request, Response, Status};
    
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct HelloRequest {
        #[prost(string, tag = "1")]
        pub name: ::prost::alloc::string::String,
    }
    
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct HelloResponse {
        #[prost(string, tag = "1")]
        pub message: ::prost::alloc::string::String,
    }
    
    pub mod hello_service_client {
        use tonic::codegen::*;
        
        #[derive(Debug, Clone)]
        pub struct HelloServiceClient<T> {
            inner: tonic::client::Grpc<T>,
        }
        
        impl HelloServiceClient<tonic::transport::Channel> {
            pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
            where
                D: std::convert::TryInto<tonic::transport::Endpoint>,
                D::Error: Into<StdError>,
            {
                let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
                Ok(Self::new(conn))
            }
        }
        
        impl<T> HelloServiceClient<T>
        where
            T: tonic::client::GrpcService<tonic::body::BoxBody>,
            T::Error: Into<StdError>,
            T::ResponseBody: Body<Data = Bytes> + Send + 'static,
            <T::ResponseBody as Body>::Error: Into<StdError> + Send,
        {
            pub fn new(inner: T) -> Self {
                let inner = tonic::client::Grpc::new(inner);
                Self { inner }
            }
            
            pub async fn say_hello(
                &mut self,
                request: impl tonic::IntoRequest<super::HelloRequest>,
            ) -> Result<tonic::Response<super::HelloResponse>, tonic::Status> {
                self.inner
                    .ready()
                    .await
                    .map_err(|e| {
                        tonic::Status::new(
                            tonic::Code::Unknown,
                            format!("Service was not ready: {}", e.into()),
                        )
                    })?;
                let codec = tonic::codec::ProstCodec::default();
                let path = http::uri::PathAndQuery::from_static("/hello.HelloService/SayHello");
                self.inner.unary(request.into_request(), path, codec).await
            }
        }
    }
}

pub mod service_ping {
    use tonic::{Request, Response, Status};
    
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PingRequest {
    }
    
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PingResponse {
    }
    
    pub mod ping_service_client {
        use tonic::codegen::*;
        
        #[derive(Debug, Clone)]
        pub struct PingServiceClient<T> {
            inner: tonic::client::Grpc<T>,
        }
        
        impl PingServiceClient<tonic::transport::Channel> {
            pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
            where
                D: std::convert::TryInto<tonic::transport::Endpoint>,
                D::Error: Into<StdError>,
            {
                let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
                Ok(Self::new(conn))
            }
        }
        
        impl<T> PingServiceClient<T>
        where
            T: tonic::client::GrpcService<tonic::body::BoxBody>,
            T::Error: Into<StdError>,
            T::ResponseBody: Body<Data = Bytes> + Send + 'static,
            <T::ResponseBody as Body>::Error: Into<StdError> + Send,
        {
            pub fn new(inner: T) -> Self {
                let inner = tonic::client::Grpc::new(inner);
                Self { inner }
            }
            
            pub async fn ping(
                &mut self,
                request: impl tonic::IntoRequest<super::PingRequest>,
            ) -> Result<tonic::Response<super::PingResponse>, tonic::Status> {
                self.inner
                    .ready()
                    .await
                    .map_err(|e| {
                        tonic::Status::new(
                            tonic::Code::Unknown,
                            format!("Service was not ready: {}", e.into()),
                        )
                    })?;
                let codec = tonic::codec::ProstCodec::default();
                let path = http::uri::PathAndQuery::from_static("/pb.PingService/ping");
                self.inner.unary(request.into_request(), path, codec).await
            }
        }
    }
}