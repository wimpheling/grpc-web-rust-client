use crate::error::{Error, Result};
use prost::Message;

pub trait ProstMessageExt: Message + Default + 'static {
    fn encode_to_vec_ext(&self) -> Result<Vec<u8>> {
        let mut buf = bytes::BytesMut::new();
        Message::encode(self, &mut buf).map_err(|e| Error::encoding(e.to_string()))?;
        Ok(buf.to_vec())
    }

    fn encode_to_bytes_ext(&self) -> Result<bytes::Bytes> {
        let mut buf = bytes::BytesMut::new();
        Message::encode(self, &mut buf).map_err(|e| Error::encoding(e.to_string()))?;
        Ok(buf.freeze())
    }

    fn decode_from_slice_ext(data: &[u8]) -> Result<Self>
    where
        Self: Sized,
    {
        Message::decode(data).map_err(|e| Error::decoding(e.to_string()))
    }

    fn decode_from_bytes_ext(data: bytes::Bytes) -> Result<Self>
    where
        Self: Sized,
    {
        Message::decode(data).map_err(|e| Error::decoding(e.to_string()))
    }
}

impl<T: Message + Default + 'static> ProstMessageExt for T {}

pub fn encode_grpc_frame(message: &[u8]) -> Vec<u8> {
    let length = message.len() as u32;
    let mut frame = Vec::with_capacity(5 + message.len());
    frame.extend_from_slice(&length.to_be_bytes());
    frame.extend_from_slice(message);
    frame
}

pub fn decode_grpc_frame(data: &[u8]) -> Result<(&[u8], &[u8])> {
    if data.len() < 5 {
        return Err(Error::decoding(format!(
            "Frame too short: expected at least 5 bytes, got {}",
            data.len()
        )));
    }
    let length = u32::from_be_bytes([data[0], data[1], data[2], data[3]]) as usize;
    if data.len() < 5 + length {
        return Err(Error::decoding(format!(
            "Frame incomplete: expected {} bytes, got {}",
            5 + length,
            data.len()
        )));
    }
    Ok((&data[5..5 + length], &data[5 + length..]))
}

pub fn parse_grpc_web_trailers(data: &[u8]) -> Result<(u32, String)> {
    let trailer_prefix = b"grpc-status: ";
    let message_prefix = b"grpc-message: ";

    let mut status_code: Option<u32> = None;
    let mut status_message = String::new();

    for line in data.split(|&b| b == b'\n') {
        let line = std::str::from_utf8(line).map_err(|e| Error::decoding(e.to_string()))?;
        let line = line.trim();
        if line.starts_with("grpc-status: ") {
            let value = &line[trailer_prefix.len()..];
            status_code = Some(
                value
                    .parse()
                    .map_err(|e| Error::decoding(format!("Failed to parse status code: {}", e)))?,
            );
        } else if line.starts_with("grpc-message: ") {
            let value = &line[message_prefix.len()..];
            status_message = percent_decode(value.as_bytes())
                .map_err(|e| Error::decoding(format!("Failed to decode message: {}", e)))?;
        }
    }

    let code = status_code.ok_or_else(|| Error::decoding("Missing grpc-status trailer"))?;

    Ok((code, status_message))
}

fn percent_decode(data: &[u8]) -> Result<String> {
    let mut result = Vec::new();
    let mut i = 0;
    while i < data.len() {
        if data[i] == b'%' && i + 2 < data.len() {
            let hex = &data[i + 1..i + 3];
            let byte = u8::from_str_radix(std::str::from_utf8(hex).unwrap(), 16)
                .map_err(|_| Error::decoding("Invalid percent encoding"))?;
            result.push(byte);
            i += 3;
        } else {
            result.push(data[i]);
            i += 1;
        }
    }
    String::from_utf8(result).map_err(|e| Error::decoding(e.to_string()))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GrpcWebContentType {
    Binary,
    Text,
}

impl GrpcWebContentType {
    pub fn as_str(&self) -> &str {
        match self {
            GrpcWebContentType::Binary => "application/grpc-web+proto",
            GrpcWebContentType::Text => "application/grpc-web-text+proto",
        }
    }
}
