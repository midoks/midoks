use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status};

use crate::rpc::hello::hello_service_server::HelloService;
use crate::rpc::hello::{HelloRequest, HelloResponse};

/// HelloService 实现
#[derive(Debug, Default)]
pub struct MyHelloService {}

#[tonic::async_trait]
impl HelloService for MyHelloService {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloResponse>, Status> {
        println!("收到Hello请求: {:?}", request);

        let reply = HelloResponse {
            message: format!("Hello {}!", request.into_inner().name),
        };

        Ok(Response::new(reply))
    }

    type SayHelloStreamStream = ReceiverStream<Result<HelloResponse, Status>>;

    async fn say_hello_stream(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<Self::SayHelloStreamStream>, Status> {
        println!("收到HelloStream请求: {:?}", request);

        let (tx, rx) = mpsc::channel(4);
        let name = request.into_inner().name;

        tokio::spawn(async move {
            for i in 0..5 {
                let response = HelloResponse {
                    message: format!("Hello {} - 消息 {}", name, i + 1),
                };

                if tx.send(Ok(response)).await.is_err() {
                    break;
                }

                tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
            }
        });

        Ok(Response::new(ReceiverStream::new(rx)))
    }
}