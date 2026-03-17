# Rust-native gRPC-Web Client

A Rust-native gRPC-Web client that runs in the browser (WASM), works with Leptos, and supports unary and server streaming RPCs.

## Features

- **WASM Compatible**: Compiles to WebAssembly and runs in any browser
- **Protobuf Support**: Uses `prost` for encoding/decoding protobuf messages
- **gRPC-Web Protocol**: Full support for gRPC-Web transport (binary and text modes)
- **Server Streaming**: Rust `Stream` interface for server-streaming RPCs
- **Error Handling**: Proper gRPC status code mapping

## Usage

```rust
use grpc_web_rust::prelude::*;
use prost::Message;

// Define your protobuf messages
#[derive(Message, Clone)]
pub struct HelloRequest {
    #[prost(string, tag = "1")]
    pub name: String,
}

#[derive(Message, Clone, Default)]
pub struct HelloResponse {
    #[prost(string, tag = "1")]
    pub message: String,
}

// Create a client
let client = Client::new("http://localhost:8080");

// Unary call
let request = HelloRequest { name: "World".to_string() };
let response: HelloResponse = client
    .unary("HelloService", "SayHello", request)
    .await?;

println!("Response: {}", response.message);

// Server streaming
use futures::StreamExt;

let stream_response = client.server_streaming("StreamService", "StreamData", request);

futures::pin_mut!(stream_response);

while let Some(result) = stream_response.next().await {
    match result {
        Ok(data) => println!("Received: {:?}", data),
        Err(e) => eprintln!("Error: {}", e),
    }
}
```

## Running with Envoy

This client requires a gRPC-Web compatible server (like Envoy with `grpc_web` filter or `tonic-web`).

Example Envoy configuration:

```yaml
static_resources:
  listeners:
  - name: listener_0
    address:
      socket_address:
        address: 0.0.0.0
        port_value: 8080
    filter_chains:
    - filters:
      - name: envoy.filters.network.http_connection_manager
        typed_config:
          "@type": type.googleapis.com/envoy.extensions.filters.network.http_connection_manager.v3.HttpConnectionManager
          codec_type: AUTO
          route_config:
            name: local_route
            virtual_hosts:
            - name: local_service
              routes:
              - match:
                  prefix: "/"
                route:
                  cluster: grpc_service
          http_filters:
          - name: envoy.filters.http.grpc_web
          - name: envoy.filters.http.cors
            typed_config:
              "@type": type.googleapis.com/envoy.extensions.filters.http.cors.v3.CorsPolicy
              allow_origin:
              - "*"
              allow_methods: GET, PUT, DELETE, POST, OPTIONS
              allow_headers: keep-alive,user-agent,cache-control,content-type,content-encoding,grpc-message,grpc-accept-encoding
          - name: envoy.router
  clusters:
  - name: grpc_service
    typed_extension_protocol_options:
      envoy.extensions.upstreams.http.v3.HttpProtocolOptions:
        "@type": type.googleapis.com/envoy.extensions.upstreams.http.v3.HttpProtocolOptions
        explicit_http_config:
          http2_protocol_options: {}
    load_assignment:
      cluster_name: grpc_service
      endpoints:
      - lb_endpoints:
        - endpoint:
            address:
              socket_address:
                address: 127.0.0.1
                port_value: 50051
```

## Building for WASM

```bash
rustup target add wasm32-unknown-unknown
cargo build --target wasm32-unknown-unknown
```

## Dependencies

- `wasm-bindgen` - For WASM interoperability
- `web-sys` - Browser APIs (Fetch, Request, Response, Headers)
- `js-sys` - JavaScript types
- `wasm-bindgen-futures` - Async/await support
- `prost` - Protobuf encoding/decoding
- `futures` - Async streams
- `base64` - Base64 encoding for gRPC-Web text mode
