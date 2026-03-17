# **Spec: Rust-native gRPC-Web Client for Leptos/WASM**

### **Goal**

Build a **Rust-native gRPC-Web client** that runs in the browser (WASM), can use `.proto` definitions as Rust structs, reuses `tonic` Rust types, and supports **server streaming via Envoy**, with full Rust type safety.

---

### **Core Requirements**

1. **Browser/WASM Compatibility**
   - Client must compile to **WebAssembly** and run in Leptos frontend.
   - No reliance on Node.js or native TCP/HTTP2 libraries.
   - Use **Fetch API** (`web_sys::window().fetch`) or similar WASM HTTP libraries.

2. **Protobuf / Rust Type Integration**
   - Import `.proto` files via `prost` to generate Rust structs.
   - Optional: reuse `tonic` Rust types directly for type consistency.
   - Encode requests and decode responses using **Prost** in WASM.

3. **gRPC-Web Transport**
   - Implement **gRPC-Web transport layer** in Rust:
     - Unary request → single response.
     - Server streaming → multiple messages over one HTTP request.

   - Handle **Envoy gRPC-Web protocol**:
     - `Content-Type: application/grpc-web+proto`
     - 5-byte headers for each message
     - Base64 (text) vs binary framing
     - Trailers for status and errors.

4. **Streaming API**
   - Provide Rust `Stream` interface for server-streaming RPCs:
     - `Stream<Item = Result<T, Error>>`
     - Async-friendly and compatible with `futures` ecosystem.

   - Handle:
     - Chunked responses
     - Frame decoding
     - Stream termination (`end`) and error propagation.

5. **Error Handling**
   - Parse gRPC-Web trailers for:
     - Status code
     - Status message

   - Map to Rust `Result` or custom error type.

6. **Optional / Future Features**
   - Client-streaming or bidirectional streaming:
     - Not supported in browsers with gRPC-Web.
     - Could be implemented via a **WebSocket proxy**.

   - gRPC metadata support for headers.

---

### **Implementation Notes**

- The hardest part is the **WASM-compatible transport + streaming parser**.
- Prost handles **message encoding/decoding**, so `.proto` → Rust types is straightforward.
- Envoy (or `tonic-web`) provides the gRPC-Web protocol layer server-side.
- The Rust client API should feel familiar to `tonic` users (unary + streaming methods).
- Could be designed modularly:
  - `Transport` module → fetch requests + frame parsing
  - `Types` module → Prost-generated types
  - `Client` module → high-level API with unary/streaming calls

---

### **High-level Flow Diagram (Textual)**

```text
Browser (WASM, Rust)
   │
   │ fetch / gRPC-Web request
   ▼
gRPC-Web Transport Layer (Rust)
   │
   │ Parses frames & trailers
   ▼
Futures::Stream<Item = Rust Struct> or unary Result
   │
   ▼
Leptos frontend component
```

- Server streaming yields multiple items in `Stream<Item=Result<T, Error>>`.
- Unary calls return `Result<T, Error>`.

---

### **Deliverables**

1. Rust/WASM library:
   - Unary + server-streaming RPC support
   - Prost-generated types
   - Envoy-compatible gRPC-Web transport
   - Futures streaming API
   - Error handling

2. Example usage in Leptos component:
   - Streaming server updates displayed in the UI

3. Optional example for WebSocket proxy for client/bidi streaming.
