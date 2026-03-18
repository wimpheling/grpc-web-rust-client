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
        Self {
            message: message.into(),
        }
    }
}
