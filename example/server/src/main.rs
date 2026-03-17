mod hello {
    include!("hello.rs");
}

use hello::{greeter_server::GreeterServer, HelloReply, HelloRequest};
use tonic::{Response, Status, async_trait, transport::Server};
use std::net::SocketAddr;
use tokio_stream::Stream;
use std::pin::Pin;

#[derive(Default)]
pub struct GreeterService {}

#[async_trait]
impl hello::greeter_server::Greeter for GreeterService {
    async fn say_hello(
        &self,
        request: tonic::Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        let name = request.into_inner().name;
        let reply = HelloReply {
            message: format!("Hello, {}!", name),
        };
        Ok(Response::new(reply))
    }

    type SayHelloStreamStream = Pin<Box<dyn Stream<Item = Result<HelloReply, Status>> + Send>>;

    async fn say_hello_stream(
        &self,
        request: tonic::Request<HelloRequest>,
    ) -> Result<Response<Self::SayHelloStreamStream>, Status> {
        let name = request.into_inner().name;
        let responses = vec![
            Ok(HelloReply { message: format!("Hello, {}!", name) }),
            Ok(HelloReply { message: format!("Welcome, {}!", name) }),
            Ok(HelloReply { message: format!("Goodbye, {}!", name) }),
        ];
        let stream = tokio_stream::iter(responses);
        Ok(Response::new(Box::pin(stream)))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr: SocketAddr = "0.0.0.0:50051".parse().unwrap();
    let greeter = GreeterService::default();

    println!("gRPC server listening on {}", addr);
    println!("Connect via Envoy at http://localhost:8080");

    Server::builder()
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}
