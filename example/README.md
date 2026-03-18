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
- wasm-pack: `cargo install wasm-pack`
- Protobuf compiler: `sudo apt install protobuf-compiler`

## Quick Start

```bash
# Generate protobuf types
cargo build -p example-types

# Build the Leptos WASM client
cd example/client
wasm-pack build --target web --out-dir ../../dist/example_client --release
cd ../..

# Start Envoy + gRPC server with Docker Compose
cd example
docker compose up --build
```

Then in a separate terminal, serve the frontend:
```bash
cd dist
python3 -m http.server 8082
```

Open `http://localhost:8082` in your browser.

## Architecture

```
Browser (WASM) -> Envoy (gRPC-web, port 8081) -> Tonic gRPC Server (port 50051)
Browser loads Leptos app from HTTP server (port 8082)
```

- **Client**: Leptos WASM app using grpc-web-rust
- **Envoy**: Translates gRPC-web requests to gRPC
- **Server**: Tonic gRPC server written in Rust

## Running Tests

```bash
cd example/client/tests
npm install
npm test
```

The test will:
1. Start Envoy and the gRPC server via docker-compose
2. Build the WASM client
3. Serve the frontend on port 8082
4. Use Playwright to verify the gRPC-web call works correctly
