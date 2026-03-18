FROM rust:1.94 as builder

RUN apt-get update && apt-get install -y protobuf-compiler && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY example/server ./example/server
COPY example/types ./example/types
COPY example/client ./example/client
COPY example/proto ./example/proto

RUN cargo build --release -p example-server

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/server /usr/local/bin/

CMD ["server"]
