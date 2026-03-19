use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Headers};
use std::pin::Pin;
use futures::Stream;

use crate::encoding::{decode_grpc_frame, parse_grpc_web_trailers, GrpcWebContentType, encode_grpc_frame};
use crate::error::{Error, Result, StatusCode};
use crate::metadata::Metadata;

pub struct GrpcWebTransport {
    base_url: String,
    content_type: GrpcWebContentType,
    default_metadata: Metadata,
}

impl GrpcWebTransport {
    pub fn new(base_url: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into(),
            content_type: GrpcWebContentType::Binary,
            default_metadata: Metadata::new(),
        }
    }

    pub fn with_content_type(mut self, content_type: GrpcWebContentType) -> Self {
        self.content_type = content_type;
        self
    }

    pub fn with_metadata(mut self, metadata: Metadata) -> Self {
        self.default_metadata = metadata;
        self
    }

    pub async fn unary<T, R>(&self, service: &str, method: &str, request: T) -> Result<R>
    where
        T: prost::Message,
        R: prost::Message + Default,
    {
        self.unary_with_metadata(service, method, request, Metadata::new()).await
    }

    pub async fn unary_with_metadata<T, R, M>(&self, service: &str, method: &str, request: T, metadata: M) -> Result<R>
    where
        T: prost::Message,
        R: prost::Message + Default,
        M: Into<Metadata>,
    {
        let body = request.encode_to_vec();
        let mut merged_metadata = self.default_metadata.clone();
        for (k, v) in metadata.into().iter() {
            merged_metadata.insert(k, v);
        }
        let response = self
            .send_request(service, method, body, merged_metadata)
            .await?;

        self.parse_unary_response(response).await
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
        self.server_streaming_with_metadata(service, method, request, Metadata::new())
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
        let base_url = self.base_url.clone();
        let content_type = self.content_type;
        let mut merged_metadata = self.default_metadata.clone();
        for (k, v) in metadata.into().iter() {
            merged_metadata.insert(k, v);
        }
        let service = service.to_string();
        let method = method.to_string();
        let body = request.encode_to_vec();
        let metadata = merged_metadata;

        let stream = async_stream::stream! {
            match send_streaming_request::<R>(base_url, &service, &method, &body, content_type, metadata).await {
                Ok(messages) => {
                    // Messages are received in order, so yield them as-is
                    for msg in messages {
                        yield Ok(msg);
                    }
                }
                Err(e) => {
                    yield Err(e);
                }
            }
        };

        Box::pin(stream) as Pin<Box<dyn Stream<Item = Result<R>> + '_>>
    }

    async fn send_request(
        &self,
        service: &str,
        method: &str,
        body: Vec<u8>,
        metadata: Metadata,
    ) -> Result<Vec<u8>> {
        let window = web_sys::window().ok_or_else(|| Error::transport("No window object"))?;

        let url = format!("{}/{}/{}", self.base_url, service, method);
        let opts = RequestInit::new();
        opts.set_method("POST");
        opts.set_mode(RequestMode::Cors);

        let frame = encode_grpc_frame(&body);

        match self.content_type {
            GrpcWebContentType::Binary => {
                let uint8_array = js_sys::Uint8Array::from(frame.as_slice());
                opts.set_body(&JsValue::from(uint8_array));
            }
            GrpcWebContentType::Text => {
                let encoded = base64::Engine::encode(&base64::engine::general_purpose::STANDARD, &frame);
                opts.set_body(&JsValue::from_str(&encoded));
            }
        }

        let headers = Headers::new().map_err(|_| Error::transport("Headers error"))?;
        
        headers.set("Content-Type", self.content_type.as_str())
            .map_err(|_| Error::transport("Failed to set Content-Type header"))?;
        headers.set("X-Grpc-Web", "1")
            .map_err(|_| Error::transport("Failed to set X-Grpc-Web header"))?;
        headers.set("Accept", self.content_type.as_str())
            .map_err(|_| Error::transport("Failed to set Accept header"))?;

        for (key, value) in metadata.iter() {
            if !key.starts_with("content-") && !key.eq_ignore_ascii_case("content-type") {
                let header_name = format!("grpc-{}", key);
                let _ = headers.set(&header_name, value);
            }
        }

        opts.set_headers(&headers);

        let request = Request::new_with_str_and_init(&url, &opts)
            .map_err(|_| Error::transport("Failed to create request"))?;

        let promise = window.fetch_with_request(&request);
        let response_value = JsFuture::from(promise)
            .await
            .map_err(|e| Error::transport(format!("Fetch failed: {:?}", e)))?;

        let response: web_sys::Response = response_value
            .dyn_into()
            .map_err(|e| Error::transport(format!("Not a Response: {:?}", e)))?;

        if !response.ok() {
            return Err(Error::transport(format!(
                "HTTP error: {}",
                response.status()
            )));
        }

        let body_value = response.array_buffer()
            .map_err(|e| Error::transport(format!("Failed to get array buffer: {:?}", e)))?;
        let body_value = JsFuture::from(body_value)
            .await
            .map_err(|e| Error::transport(format!("Failed to get body: {:?}", e)))?;

        if let Some(array_buffer) = body_value.dyn_ref::<js_sys::ArrayBuffer>() {
            let uint8_array = js_sys::Uint8Array::new(array_buffer);
            let mut full_body = vec![0u8; uint8_array.length() as usize];
            uint8_array.copy_to(&mut full_body);
            return Ok(full_body);
        }

        let js_array = js_sys::Array::from(&body_value);
        let mut full_body = Vec::new();
        for elem in js_array.iter() {
            let uint8_array = js_sys::Uint8Array::new(&elem);
            let mut chunk = vec![0u8; uint8_array.length() as usize];
            uint8_array.copy_to(&mut chunk);
            full_body.extend(chunk);
        }

        Ok(full_body)
    }

    async fn parse_unary_response<R: prost::Message + Default>(&self, body: Vec<u8>) -> Result<R> {
        let decoded_body = match self.content_type {
            GrpcWebContentType::Binary => body,
            GrpcWebContentType::Text => decode_grpc_web_body(&body)?,
        };

        let (message_data, trailing) = split_message_and_trailers(&decoded_body)?;

        if !trailing.is_empty() {
            let (code, message) = parse_grpc_web_trailers(trailing)?;
            let status_code = StatusCode::from_u32(code);
            if status_code != StatusCode::Ok {
                return Err(Error::grpc(status_code, message));
            }
        }

        let (data, _) = decode_grpc_frame(message_data)?;
        let response = R::decode(data).map_err(|e| Error::decoding(e.to_string()))?;

        Ok(response)
    }
}

async fn send_streaming_request<R>(
    base_url: String,
    service: &str,
    method: &str,
    body: &[u8],
    content_type: GrpcWebContentType,
    metadata: Metadata,
) -> Result<Vec<R>>
where
    R: prost::Message + Default + 'static,
{
    let window = web_sys::window().ok_or_else(|| Error::transport("No window object"))?;

    let url = format!("{}/{}/{}", base_url, service, method);
    let opts = RequestInit::new();
    opts.set_method("POST");
    opts.set_mode(RequestMode::Cors);

    let frame = encode_grpc_frame(body);

    match content_type {
        GrpcWebContentType::Binary => {
            let uint8_array = js_sys::Uint8Array::from(frame.as_slice());
            opts.set_body(&JsValue::from(uint8_array));
        }
        GrpcWebContentType::Text => {
            let encoded = base64::Engine::encode(&base64::engine::general_purpose::STANDARD, &frame);
            opts.set_body(&JsValue::from_str(&encoded));
        }
    }

    let headers = Headers::new().map_err(|_| Error::transport("Headers error"))?;
    headers.set("Content-Type", content_type.as_str())
        .map_err(|_| Error::transport("Failed to set Content-Type header"))?;
    headers.set("X-Grpc-Web", "1")
        .map_err(|_| Error::transport("Failed to set X-Grpc-Web header"))?;
    headers.set("Accept", content_type.as_str())
        .map_err(|_| Error::transport("Failed to set Accept header"))?;

    for (key, value) in metadata.iter() {
        if !key.starts_with("content-") && !key.eq_ignore_ascii_case("content-type") {
            let header_name = format!("grpc-{}", key);
            let _ = headers.set(&header_name, value);
        }
    }

    opts.set_headers(&headers);

    let request = Request::new_with_str_and_init(&url, &opts)
        .map_err(|_| Error::transport("Failed to create request"))?;

    let promise = window.fetch_with_request(&request);
    let response_value = JsFuture::from(promise)
        .await
        .map_err(|e| Error::transport(format!("Fetch failed: {:?}", e)))?;

    let response: web_sys::Response = response_value
        .dyn_into()
        .map_err(|e| Error::transport(format!("Not a Response: {:?}", e)))?;

    if !response.ok() {
        return Err(Error::transport(format!(
            "HTTP error: {}",
            response.status()
        )));
    }

    let body_value = response.array_buffer()
        .map_err(|e| Error::transport(format!("Failed to get array buffer: {:?}", e)))?;
    let body_value = JsFuture::from(body_value)
        .await
        .map_err(|e| Error::transport(format!("Failed to get body: {:?}", e)))?;

    let full_body = if let Some(array_buffer) = body_value.dyn_ref::<js_sys::ArrayBuffer>() {
        let uint8_array = js_sys::Uint8Array::new(array_buffer);
        let mut buf = vec![0u8; uint8_array.length() as usize];
        uint8_array.copy_to(&mut buf);
        buf
    } else {
        let js_array = js_sys::Array::from(&body_value);
        let mut buf = Vec::new();
        for elem in js_array.iter() {
            let uint8_array = js_sys::Uint8Array::new(&elem);
            let mut chunk = vec![0u8; uint8_array.length() as usize];
            uint8_array.copy_to(&mut chunk);
            buf.extend(chunk);
        }
        buf
    };

    let decoded_body = match content_type {
        GrpcWebContentType::Binary => full_body,
        GrpcWebContentType::Text => decode_grpc_web_body(&full_body)?,
    };

    // Parse all frames from the response body
    let mut messages = Vec::new();
    let mut pos = 0;
    let mut has_trailing = false;

    while pos + 5 <= decoded_body.len() {
        let _compression = decoded_body[pos];
        let length = u32::from_be_bytes([
            decoded_body[pos + 1],
            decoded_body[pos + 2],
            decoded_body[pos + 3],
            decoded_body[pos + 4],
        ]) as usize;
        
        // Check if this is a trailing frame (empty data or contains grpc-status)
        // The frame data starts at pos + 5
        let frame_start = pos + 5;
        if frame_start + length > decoded_body.len() {
            break;
        }
        
        let frame_data = &decoded_body[frame_start..frame_start + length];

        if length == 0 || frame_data.starts_with(b"grpc-status:") {
            has_trailing = true;
            // Parse trailing status - pass the entire frame (including header)
            let (code, message) = parse_grpc_web_trailers(&decoded_body[pos..])?;
            let status_code = StatusCode::from_u32(code);
            if status_code != StatusCode::Ok {
                return Err(Error::grpc(status_code, message));
            }
            break;
        }

        // Decode message frame - frame_data is the raw message bytes
        let message = R::decode(frame_data).map_err(|e| Error::decoding(e.to_string()))?;
        messages.push(message);

        pos = frame_start + length;
    }

    if !has_trailing && pos < decoded_body.len() {
        // Try to parse trailing data at the end
        let trailing_data = &decoded_body[pos..];
        if !trailing_data.is_empty() {
            let (code, message) = parse_grpc_web_trailers(trailing_data)?;
            let status_code = StatusCode::from_u32(code);
            if status_code != StatusCode::Ok {
                return Err(Error::grpc(status_code, message));
            }
        }
    }

    Ok(messages)
}

fn decode_grpc_web_body(body: &[u8]) -> Result<Vec<u8>> {
    base64::Engine::decode(&base64::engine::general_purpose::STANDARD, body)
        .map_err(|e| Error::decoding(format!("Base64 decode error: {}", e)))
}

fn split_message_and_trailers(data: &[u8]) -> Result<(&[u8], &[u8])> {
    if data.len() < 5 {
        return Err(Error::invalid_response("Response too short"));
    }

    let length = u32::from_be_bytes([data[1], data[2], data[3], data[4]]) as usize;

    if data.len() < 5 + length {
        return Err(Error::invalid_response("First frame incomplete"));
    }

    let message_end = 5 + length;
    Ok((&data[..message_end], &data[message_end..]))
}
