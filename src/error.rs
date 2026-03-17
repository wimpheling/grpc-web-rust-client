use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StatusCode {
    Ok = 0,
    Canceled = 1,
    Unknown = 2,
    InvalidArgument = 3,
    DeadlineExceeded = 4,
    NotFound = 5,
    AlreadyExists = 6,
    PermissionDenied = 7,
    ResourceExhausted = 8,
    FailedPrecondition = 9,
    Aborted = 10,
    OutOfRange = 11,
    Unimplemented = 12,
    Internal = 13,
    Unavailable = 14,
    DataLoss = 15,
    Unauthenticated = 16,
}

impl StatusCode {
    pub fn from_u32(value: u32) -> Self {
        match value {
            0 => StatusCode::Ok,
            1 => StatusCode::Canceled,
            2 => StatusCode::Unknown,
            3 => StatusCode::InvalidArgument,
            4 => StatusCode::DeadlineExceeded,
            5 => StatusCode::NotFound,
            6 => StatusCode::AlreadyExists,
            7 => StatusCode::PermissionDenied,
            8 => StatusCode::ResourceExhausted,
            9 => StatusCode::FailedPrecondition,
            10 => StatusCode::Aborted,
            11 => StatusCode::OutOfRange,
            12 => StatusCode::Unimplemented,
            13 => StatusCode::Internal,
            14 => StatusCode::Unavailable,
            15 => StatusCode::DataLoss,
            16 => StatusCode::Unauthenticated,
            _ => StatusCode::Unknown,
        }
    }

    pub fn as_u32(&self) -> u32 {
        *self as u32
    }
}

impl std::fmt::Display for StatusCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StatusCode::Ok => write!(f, "OK"),
            StatusCode::Canceled => write!(f, "CANCELED"),
            StatusCode::Unknown => write!(f, "UNKNOWN"),
            StatusCode::InvalidArgument => write!(f, "INVALID_ARGUMENT"),
            StatusCode::DeadlineExceeded => write!(f, "DEADLINE_EXCEEDED"),
            StatusCode::NotFound => write!(f, "NOT_FOUND"),
            StatusCode::AlreadyExists => write!(f, "ALREADY_EXISTS"),
            StatusCode::PermissionDenied => write!(f, "PERMISSION_DENIED"),
            StatusCode::ResourceExhausted => write!(f, "RESOURCE_EXHAUSTED"),
            StatusCode::FailedPrecondition => write!(f, "FAILED_PRECONDITION"),
            StatusCode::Aborted => write!(f, "ABORTED"),
            StatusCode::OutOfRange => write!(f, "OUT_OF_RANGE"),
            StatusCode::Unimplemented => write!(f, "UNIMPLEMENTED"),
            StatusCode::Internal => write!(f, "INTERNAL"),
            StatusCode::Unavailable => write!(f, "UNAVAILABLE"),
            StatusCode::DataLoss => write!(f, "DATA_LOSS"),
            StatusCode::Unauthenticated => write!(f, "UNAUTHENTICATED"),
        }
    }
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("Transport error: {0}")]
    Transport(String),

    #[error("gRPC error: {code} - {message}")]
    Grpc { code: StatusCode, message: String },

    #[error("Encoding error: {0}")]
    Encoding(String),

    #[error("Decoding error: {0}")]
    Decoding(String),

    #[error("Stream error: {0}")]
    Stream(String),

    #[error("WASM error: {0}")]
    Wasm(String),

    #[error("Invalid response: {0}")]
    InvalidResponse(String),
}

impl Error {
    pub fn transport<S: Into<String>>(msg: S) -> Self {
        Error::Transport(msg.into())
    }

    pub fn grpc(code: StatusCode, message: impl Into<String>) -> Self {
        Error::Grpc {
            code,
            message: message.into(),
        }
    }

    pub fn encoding<S: Into<String>>(msg: S) -> Self {
        Error::Encoding(msg.into())
    }

    pub fn decoding<S: Into<String>>(msg: S) -> Error {
        Error::Decoding(msg.into())
    }

    pub fn stream<S: Into<String>>(msg: S) -> Error {
        Error::Stream(msg.into())
    }

    pub fn wasm<S: Into<String>>(msg: S) -> Error {
        Error::Wasm(msg.into())
    }

    pub fn invalid_response<S: Into<String>>(msg: S) -> Error {
        Error::InvalidResponse(msg.into())
    }

    pub fn code(&self) -> Option<StatusCode> {
        match self {
            Error::Grpc { code, .. } => Some(*code),
            _ => None,
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;
