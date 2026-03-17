#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HelloRequest {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}

impl HelloRequest {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HelloReply {
    #[prost(string, tag = "1")]
    pub message: ::prost::alloc::string::String,
}

impl HelloReply {
    pub fn new(message: impl Into<String>) -> Self {
        Self { message: message.into() }
    }
}

pub mod greeter_server {
    use tonic::{server::Grpc, Streaming};
    use super::{Greeter, HelloReply, HelloRequest};

    #[derive(Debug)]
    pub struct GreeterServer<T> {
        inner: T,
    }

    impl<T> GreeterServer<T> {
        pub fn new(inner: T) -> Self {
            Self { inner }
        }
    }

    impl<T, B> Grpc<B> for GreeterServer<T>
    where
        T: Greeter,
        B: tonic::body::Body + Send + Sync + 'static,
        B::Error: std::fmt::Debug,
    {
        type Error = tonic::Status;
        type ResponseBody = B;
        type Future = BoxFuture<Self::ResponseBody, Self::Error>;

        fn unary(&mut self, request: tonic::Request<B>, _path: &str) -> Self::Future {
            let service = self.inner.clone();
            Box::pin(async move {
                let body = request.into_body();
                let bytes = hyper::body::to_bytes(body).await.map_err(tonic::Status::internal)?;
                let request = HelloRequest::decode(bytes).map_err(tonic::Status::internal)?;
                let response = service.say_hello(tonic::Request::new(request)).await?;
                let mut buffer = Vec::new();
                response.into_inner().encode(&mut buffer).map_err(tonic::Status::internal)?;
                Ok(hyper::Body::from(buffer).into())
            })
        }

        fn server_streaming(&mut self, request: tonic::Request<B>, _path: &str) -> Self::Future {
            let service = self.inner.clone();
            Box::pin(async move {
                let body = request.into_body();
                let bytes = hyper::body::to_bytes(body).await.map_err(tonic::Status::internal)?;
                let request = HelloRequest::decode(bytes).map_err(tonic::Status::internal)?;
                let response = service.say_hello_stream(tonic::Request::new(request)).await?;
                Ok(hyper::Body::empty().into())
            })
        }
    }

    impl<T: Clone> Clone for GreeterServer<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
}

pub trait Greeter: Send + Sync + 'static {
    fn say_hello(
        &self,
        request: tonic::Request<HelloRequest>,
    ) -> impl std::future::Future<Output = Result<tonic::Response<HelloReply>, tonic::Status>>
        + Send;

    fn say_hello_stream(
        &self,
        request: tonic::Request<HelloRequest>,
    ) -> impl std::future::Future<Output = Result<tonic::Response<Streaming<HelloReply>>, tonic::Status>>
        + Send;
}

pub type BoxFuture<T, E> = Box<dyn std::future::Future<Output = Result<T, E>> + Send + Sync>;
