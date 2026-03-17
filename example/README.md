# gRPC-Web Example Project

This is an example project demonstrating a gRPC-Web client using Leptos and a simple server.

## Project Structure

- `common/` - Shared proto definitions and message types
- `server/` - Simple HTTP server (not a full gRPC server, for demo purposes)
- `client/` - Leptos frontend that communicates with the server

## Prerequisites

- Rust (latest stable)
- Node.js (for building the Leptos frontend with wasm-pack)

## Running the Example

### 1. Build the Client (WASM)

```bash
# Build the WASM client
cd example/client
wasm-pack build --target web --out-dir ../../dist
```

### 2. Run the Server

```bash
# Start the server (serves both API and static files)
cargo run -p example-server
```

The server will listen on `http://127.0.0.1:50051` and serve:
- gRPC-web API at `/hello.Greeter/SayHello`
- Static files from `dist/` directory
- Index page at `/`

Open `http://127.0.0.1:50051` in your browser.

## Testing

### Server Tests

Run server unit tests:

```bash
cargo test -p example-server
```

### Client Tests

The client is a WebAssembly application. Tests can be run with:

```bash
# Run WASM tests
cd example/client
wasm-pack test --node

# Or run in browser
wasm-pack test --chrome
```

### Common Tests

```bash
cargo test -p example-common
```

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

#### Run Browser Tests

```bash
# Build the client first
cd ../..
wasm-pack build --target web --out-dir dist

# Run the browser tests (server starts automatically)
cd example/client/tests
npm test
```

#### Test Coverage

- Page loads correctly
- Input field works
- Button click works
- Server response is displayed

## Note

This example demonstrates a simplified setup. The server is a basic HTTP server that responds with string data for demonstration purposes. For a full gRPC-Web experience with proper binary encoding/decoding:

1. Install protobuf compiler: `apt-get install protobuf-compiler`
2. Enable the build.rs files in each crate to generate code from proto files
3. Configure proper gRPC-web headers and trailers

## Development

To rebuild after making changes:

```bash
# Rebuild everything
cargo build --workspace

# Or rebuild specific crate
cargo build -p example-client
```
