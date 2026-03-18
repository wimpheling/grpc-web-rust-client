# gRPC-Web Example Project

This is an example project demonstrating a gRPC-Web client using Leptos, Tonic, and Envoy proxy.

## Project Structure

- `types/` - Protobuf types generated from proto files
- `server/` - Tonic gRPC server
- `client/` - Leptos frontend that communicates with the server via gRPC-web
- `envoy.yaml` - Envoy proxy configuration for gRPC-web
- `docker-compose.yaml` - Docker setup for server + envoy

## Prerequisites

- Rust (latest stable)
- Node.js (for building the Leptos frontend with wasm-pack)
- Docker & Docker Compose
- Protobuf compiler: `sudo apt install protobuf-compiler`

## Quick Start

```bash
# Generate protobuf types
cargo build -p example-types

# Build and run everything with Docker Compose
docker-compose up --build
```

Then open `http://localhost:8081` in your browser.

## Architecture

```
Browser (WASM) -> Envoy (gRPC-web) -> Tonic gRPC Server
```

- **Client**: Leptos WASM app using grpc-web-rust
- **Envoy**: Translates gRPC-web requests to gRPC
- **Server**: Tonic gRPC server written in Rust
