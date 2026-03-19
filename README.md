# Rust-native gRPC-Web Client

A Rust-native gRPC-Web client that runs in the browser (WASM), works with Leptos, and supports unary and server streaming RPCs.

## Features

- **WASM Compatible**: Compiles to WebAssembly and runs in any browser
- **Protobuf Support**: Uses `prost` for encoding/decoding protobuf messages
- **gRPC-Web Protocol**: Full support for gRPC-Web transport (binary and text modes)
- **Server Streaming**: Rust `Stream` interface for server-streaming RPCs
- **Error Handling**: Proper gRPC status code mapping
- **Metadata Support**: Custom headers and gRPC metadata

## Prerequisites

- Rust (latest stable)
- `wasm32-unknown-unknown` target
- A gRPC-Web compatible server (e.g., Envoy with `grpc_web` filter or `tonic-web`)

## Usage

This library is designed to run in a browser environment (WASM). It uses `web_sys` for HTTP requests.

### Basic Usage

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

// Create a client (must be called in a browser context)
let client = Client::new("http://localhost:8081");

// Unary call
let request = HelloRequest { name: "World".to_string() };
let response: HelloResponse = client
    .unary("hello.Greeter", "SayHello", request)
    .await?;

println!("Response: {}", response.message);
```

### Server Streaming

```rust
use futures::StreamExt;

let request = HelloRequest { name: "World".to_string() };
let mut stream = client.server_streaming("hello.Greeter", "SayHelloStream", request);

while let Some(result) = stream.next().await {
    match result {
        Ok(data) => println!("Received: {:?}", data),
        Err(e) => eprintln!("Error: {}", e),
    }
}
```

### Using with Leptos

See the [example/client](example/client) directory for a complete Leptos WASM application.

```rust
// example/client/src/lib.rs
use grpc_web_rust::{Client, GrpcWebContentType};
use leptos::*;

#[component]
pub fn App() -> impl IntoView {
    let (response, set_response) = create_signal("".to_string());

    let greet = move |_| {
        wasm_bindgen_futures::spawn_local(async move {
            let client = Client::new("http://localhost:8081")
                .with_content_type(GrpcWebContentType::Binary);
            
            let req = HelloRequest { name: "World".to_string() };
            let resp = client.unary("hello.Greeter", "SayHello", req).await;
            
            if let Ok(msg) = resp {
                set_response.set(msg.message);
            }
        });
    };

    view! {
        <div>
            <button on:click=greet>"Say Hello"</button>
            <p>{response}</p>
        </div>
    }
}
```

## Configuration

### Content Type

The client supports both binary and text modes for gRPC-Web:

```rust
let client = Client::new("http://localhost:8081")
    .with_content_type(GrpcWebContentType::Binary); // or GrpcWebContentType::Text
```

### Metadata

Add custom headers to requests:

```rust
use grpc_web_rust::Metadata;

let mut metadata = Metadata::new();
metadata.insert("authorization", "Bearer token");

let client = Client::new("http://localhost:8081")
    .with_metadata(metadata);
```

## Running

This client requires a gRPC-Web compatible server (e.g., Envoy with `grpc_web` filter or `tonic-web`).

## Building for WASM

```bash
rustup target add wasm32-unknown-unknown
cargo build --target wasm32-unknown-unknown
```

## Example Project

A complete example with Leptos, Tonic gRPC server, and Envoy proxy is available in the [example](example) directory.

To run the example:

```bash
cd example
DOCKER_BUILDKIT=1 docker compose up --build
```

Then open `http://localhost:8082` in your browser.

## Dependencies

- `wasm-bindgen` - For WASM interoperability
- `web-sys` - Browser APIs (Fetch, Request, Response, Headers)
- `js-sys` - JavaScript types
- `wasm-bindgen-futures` - Async/await support
- `prost` - Protobuf encoding/decoding
- `futures` - Async streams
- `base64` - Base64 encoding for gRPC-Web text mode
- `async-stream` - Stream implementation for server streaming

---

*Vibe coded with [opencode](https://opencode.ai)*

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
