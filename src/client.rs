use futures::Stream;
use std::pin::Pin;
use crate::encoding::GrpcWebContentType;
use crate::error::Result;
use crate::metadata::Metadata;
use crate::transport::GrpcWebTransport;

pub struct Client {
    transport: GrpcWebTransport,
}

impl Client {
    pub fn new(base_url: impl Into<String>) -> Self {
        Self {
            transport: GrpcWebTransport::new(base_url),
        }
    }

    pub fn with_content_type(mut self, content_type: GrpcWebContentType) -> Self {
        self.transport = self.transport.with_content_type(content_type);
        self
    }

    pub fn with_metadata(mut self, metadata: Metadata) -> Self {
        self.transport = self.transport.with_metadata(metadata);
        self
    }

    pub async fn unary<T, R>(
        &self,
        service: &str,
        method: &str,
        request: T,
    ) -> Result<R>
    where
        T: prost::Message,
        R: prost::Message + Default,
    {
        self.transport.unary(service, method, request).await
    }

    pub async fn unary_with_metadata<T, R, M>(
        &self,
        service: &str,
        method: &str,
        request: T,
        metadata: M,
    ) -> Result<R>
    where
        T: prost::Message,
        R: prost::Message + Default,
        M: Into<Metadata>,
    {
        self.transport.unary_with_metadata(service, method, request, metadata).await
    }

    pub fn server_streaming<T, R>(
        &self,
        service: &str,
        method: &str,
        request: T,
    ) -> Pin<Box<dyn Stream<Item = Result<R>> + '_>>
    where
        T: prost::Message,
        R: prost::Message + Default + 'static,
    {
        self.transport.server_streaming(service, method, request)
    }

    pub fn server_streaming_with_metadata<T, R, M>(
        &self,
        service: &str,
        method: &str,
        request: T,
        metadata: M,
    ) -> Pin<Box<dyn Stream<Item = Result<R>> + '_>>
    where
        T: prost::Message,
        R: prost::Message + Default + 'static,
        M: Into<Metadata>,
    {
        self.transport.server_streaming_with_metadata(service, method, request, metadata)
    }
}

pub struct StreamingClient<T: prost::Message + Default + 'static> {
    client: Client,
    _phantom: std::marker::PhantomData<T>,
}

impl<T: prost::Message + Default + 'static> StreamingClient<T> {
    pub fn new(base_url: impl Into<String>) -> Self {
        Self {
            client: Client::new(base_url),
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn with_content_type(mut self, content_type: GrpcWebContentType) -> Self {
        self.client = self.client.with_content_type(content_type);
        self
    }

    pub fn with_metadata(mut self, metadata: Metadata) -> Self {
        self.client = self.client.with_metadata(metadata);
        self
    }

    pub async fn call<R>(
        &self,
        service: &str,
        method: &str,
        request: T,
    ) -> Result<R>
    where
        R: prost::Message + Default,
    {
        self.client.unary(service, method, request).await
    }

    pub async fn call_with_metadata<R, M>(
        &self,
        service: &str,
        method: &str,
        request: T,
        metadata: M,
    ) -> Result<R>
    where
        R: prost::Message + Default,
        M: Into<Metadata>,
    {
        self.client.unary_with_metadata(service, method, request, metadata).await
    }
}
