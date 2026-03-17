pub mod client;
pub mod encoding;
pub mod error;
pub mod metadata;
pub mod transport;

pub use client::{Client, StreamingClient};
pub use encoding::{GrpcWebContentType, ProstMessageExt};
pub use error::{Error, Result, StatusCode};
pub use metadata::Metadata;
pub use transport::GrpcWebTransport;

pub mod prelude {
    pub use crate::client::{Client, StreamingClient};
    pub use crate::encoding::ProstMessageExt;
    pub use crate::error::{Error, Result, StatusCode};
    pub use crate::metadata::Metadata;
    pub use crate::GrpcWebContentType;
}
