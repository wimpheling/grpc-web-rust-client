# syntax=docker/dockerfile:1.5
FROM rust:1.94 as builder

RUN apt-get update && apt-get install -y protobuf-compiler && rm -rf /var/lib/apt/lists/*
RUN --mount=type=cache,target=/usr/local/cargo/registry,sharing=locked --mount=type=cache,target=/usr/local/cargo/git cargo install wasm-pack

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY example ./example

RUN --mount=type=cache,target=/usr/local/cargo/registry,sharing=locked --mount=type=cache,target=/usr/local/cargo/git --mount=type=cache,target=/app/target wasm-pack build --target web --out-dir /app/dist/example_client --release example/client

FROM nginx:alpine

COPY --from=builder /app/dist /usr/share/nginx/html
COPY example/client/index.html /usr/share/nginx/html/index.html
COPY example/client/nginx.conf /etc/nginx/conf.d/default.conf

EXPOSE 80

CMD ["nginx", "-g", "daemon off;"]
