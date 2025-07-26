use tonic::{transport::Server, Request, Response, Status};
use tokio_stream::wrappers::ReceiverStream;
use tokio::sync::mpsc;

pub mod hello {
    tonic::include_proto!("hello");
}

use hello::hello_service_server::{HelloService, HelloServiceServer};
use hello::{HelloRequest, HelloResponse};

#[derive(Debug, Default)]
pub struct MyHelloService {}

#[tonic::async_trait]
impl HelloService for MyHelloService {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloResponse>, Status> {
        println!("Got a request: {:?}", request);

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
        println!("Got a stream request: {:?}", request);

        let (tx, rx) = mpsc::channel(4);
        let name = request.into_inner().name;

        tokio::spawn(async move {
            for i in 0..5 {
                let response = HelloResponse {
                    message: format!("Hello {} - Message {}", name, i + 1),
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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let hello_service = MyHelloService::default();

    println!("HelloService Server listening on {}", addr);

    Server::builder()
        .add_service(HelloServiceServer::new(hello_service))
        .serve(addr)
        .await?;

    Ok(())
}