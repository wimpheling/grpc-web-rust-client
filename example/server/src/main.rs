use example_common::hello::{HelloReply, HelloRequest};
use prost::Message;
use std::net::SocketAddr;
use hyper::body::{Bytes, Body};
use axum::{
    response::IntoResponse,
    http::{StatusCode, header, Response},
    routing::get,
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

    async fn serve_index() -> impl IntoResponse {
        let html = std::fs::read_to_string("dist/index.html").unwrap_or_default();
        Response::builder()
            .status(StatusCode::OK)
            .header(header::CONTENT_TYPE, "text/html")
            .body(Body::from(html))
            .unwrap()
    }

    async fn serve_static(path: axum::extract::Path<String>) -> Response<Body> {
        let path = path.0;
        let full_path = format!("dist/{}", path);
        
        if let Ok(contents) = std::fs::read(&full_path) {
            let mime = if path.ends_with(".wasm") {
                "application/wasm"
            } else if path.ends_with(".js") {
                "application/javascript"
            } else if path.ends_with(".html") {
                "text/html"
            } else {
                "application/octet-stream"
            };
            
            return Response::builder()
                .status(StatusCode::OK)
                .header(header::CONTENT_TYPE, mime)
                .body(Body::from(contents))
                .unwrap();
        }
        
        Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::from("File not found"))
            .unwrap()
    }

    let app = axum::Router::new()
        .route("/hello.Greeter/SayHello", axum::routing::post(handle_hello))
        .route("/", get(serve_index))
        .route("/:file", get(serve_static))
        .with_state(greeter);

    let addr = SocketAddr::from(([127, 0, 0, 1], 50051));
    println!("Greeter server listening on {}", addr);
    println!("Serving static files from ../dist");

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
