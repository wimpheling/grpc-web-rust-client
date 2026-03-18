# gRPC-Web Example Project

This is an example project demonstrating a gRPC-Web client using Leptos, Tonic, and Envoy proxy.

## Project Structure

- `common/` - Shared proto definitions and message types
- `server/` - Simple Tonic gRPC server
- `client/` - Leptos frontend that communicates with the server via gRPC-web
- `envoy.yaml` - Envoy proxy configuration for gRPC-web
- `docker-compose.yaml` - Docker setup for server + envoy

## Prerequisites

- Rust (latest stable)
- Node.js (for building the Leptos frontend with wasm-pack)
- Docker (for running Envoy proxy)
- Protobuf compiler: `sudo apt install protobuf-compiler`

## Building

The proto types are generated from `.proto` files. Build them first:

```bash
# Generate types from proto
cargo build -p example-types
```

## Running the Example

### Option 1: With Docker Compose (Recommended)

```bash
# Build the client WASM first
cd example/client
wasm-pack build --target web --out-dir ../../dist

# Start everything with docker-compose
cd ../..
docker-compose -f example/docker-compose.yaml up --build
```

Then open `http://localhost:8081` in your browser.

### Option 2: Manual Setup

#### 1. Build the Client

```bash
cd example/client
wasm-pack build --target web --out-dir ../../dist
```

#### 2. Run the Server

```bash
cargo run -p example-server
```

#### 3. Run Envoy Proxy

```bash
docker run -d -p 8081:8080 -v $PWD/example/envoy.yaml:/etc/envoy/envoy.yaml envoyproxy/envoy
```

#### 4. Serve Static Files

```bash
npx http-server dist -p 8080
```

Then open `http://localhost:8081` in your browser.

## Testing

### Browser Tests (Chrome + Playwright)

The browser tests use Chrome with Playwright for end-to-end testing.

#### Prerequisites

- Node.js 18+
- Chromium browser (installed via Playwright)

#### Install Dependencies

```bash
cd example/client/tests
npm install
npx playwright install chromium
```

#### Run Tests

```bash
# Build the client first
cd ../..
wasm-pack build --target web --out-dir dist

# Run the browser tests
cd example/client/tests
npm test
```

## Architecture

```
Browser (WASM) -> Envoy (gRPC-web) -> Tonic gRPC Server
```

- **Client**: Leptos WASM app that uses grpc-web-rust to call gRPC-web
- **Envoy**: Acts as a proxy, translating gRPC-web requests to gRPC
- **Server**: Tonic gRPC server written in Rust
