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
    frame.push(0); // compression flag: no compression
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
    // Skip compression flag byte (data[0])
    let length = u32::from_be_bytes([data[1], data[2], data[3], data[4]]) as usize;
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
    // Parse gRPC-web binary trailer frames
    // Each frame: 1 byte compression flag + 4 bytes length + data
    let mut pos = 0;
    let mut trailers_text = String::new();

    while pos + 5 <= data.len() {
        let _compression = data[pos];
        let length =
            u32::from_be_bytes([data[pos + 1], data[pos + 2], data[pos + 3], data[pos + 4]])
                as usize;
        pos += 5;

        if pos + length > data.len() {
            break;
        }

        let frame_data = &data[pos..pos + length];
        if let Ok(text) = std::str::from_utf8(frame_data) {
            trailers_text.push_str(text);
        }
        pos += length;
    }

    // Parse the concatenated trailer text
    let mut status_code: Option<u32> = None;
    let mut status_message = String::new();

    for line in trailers_text.split(|c| c == '\n' || c == '\r') {
        let line = line.trim();
        if line.starts_with("grpc-status:") {
            let value = line["grpc-status:".len()..].trim();
            status_code = Some(
                value
                    .parse()
                    .map_err(|e| Error::decoding(format!("Failed to parse status code: {}", e)))?,
            );
        } else if line.starts_with("grpc-message:") {
            let value = line["grpc-message:".len()..].trim();
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
