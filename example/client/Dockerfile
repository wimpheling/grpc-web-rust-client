FROM rust:1.94 as builder

RUN apt-get update && apt-get install -y protobuf-compiler && rm -rf /var/lib/apt/lists/*
RUN cargo install wasm-pack

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY proto ./proto
COPY example ./example

RUN wasm-pack build --target web --out-dir /app/dist/example_client --release example/client

FROM nginx:alpine

COPY --from=builder /app/dist /usr/share/nginx/html
COPY example/client/index.html /usr/share/nginx/html/index.html
COPY example/client/nginx.conf /etc/nginx/conf.d/default.conf

EXPOSE 80

CMD ["nginx", "-g", "daemon off;"]
