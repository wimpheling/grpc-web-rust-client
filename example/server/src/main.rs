use example_common::hello::{HelloReply, HelloRequest};
use prost::Message;
use std::net::SocketAddr;
use hyper::body::Bytes;
use axum::{
    response::IntoResponse,
    http::{Response, StatusCode},
};

#[derive(Default, Clone)]
pub struct GreeterService {}

impl GreeterService {
    pub fn say_hello(&self, name: &str) -> HelloReply {
        HelloReply::new(format!("Hello, {}!", name))
    }
}

#[tokio::main]
async fn main() {
    let greeter = GreeterService::default();

    async fn handle_hello(
        axum::extract::State(greeter): axum::extract::State<GreeterService>,
        body: Bytes,
    ) -> impl IntoResponse {
        let request = match HelloRequest::decode(&body[..]) {
            Ok(req) => req,
            Err(e) => return Err((StatusCode::BAD_REQUEST, e.to_string())),
        };

        let reply = greeter.say_hello(&request.name);
        let mut buf = Vec::new();
        if let Err(e) = reply.encode(&mut buf) {
            return Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()));
        }

        let body_str = String::from_utf8_lossy(&buf).to_string();
        Ok((StatusCode::OK, body_str))
    }

    let app = axum::Router::new()
        .route("/hello.Greeter/SayHello", axum::routing::post(handle_hello))
        .with_state(greeter);

    let addr = SocketAddr::from(([127, 0, 0, 1], 50051));
    println!("Greeter server listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
