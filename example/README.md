# gRPC-Web Example Project

This is an example project demonstrating a gRPC-Web client using Leptos, Tonic, and Envoy proxy.

## Project Structure

- `types/` - Protobuf types generated from proto files
- `server/` - Tonic gRPC server
- `client/` - Leptos frontend that communicates with the server via gRPC-web
- `envoy.yaml` - Envoy proxy configuration for gRPC-web
- `docker-compose.yaml` - Docker setup for server, envoy, and frontend

## Prerequisites

- Rust (latest stable)
- Docker & Docker Compose

## Quick Start

```bash
# Build and run everything with Docker Compose
cd example
docker compose up --build
```

Then open `http://localhost:8082` in your browser.

That's it! Docker Compose will:
1. Build and start the Tonic gRPC server on port 50051
2. Start Envoy proxy on port 8081 (translates gRPC-web to gRPC)
3. Build the Leptos WASM client and serve it with nginx on port 8082

## Architecture

```
Browser (WASM, served on :8082) -> Envoy (gRPC-web, :8081) -> Tonic gRPC Server (:50051)
```

- **Frontend**: Leptos WASM app built with wasm-pack, served by nginx
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
