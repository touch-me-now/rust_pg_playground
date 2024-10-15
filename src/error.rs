use std::{fmt, io};

#[derive(Debug)]
pub enum PgError {
    DecodeError(String),
    EncodeError(String),
    IOError(io::Error),
    ErrorResp(String),
}

impl fmt::Display for PgError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PgError::DecodeError(msg) => write!(f, "Decode error: {}", msg),
            PgError::EncodeError(msg) => write!(f, "Encode error: {}", msg),
            PgError::IOError(err) => write!(f, "Io error: {}", err),
            PgError::ErrorResp(err) => write!(f, "PSQL server error: {}", err),
        }
    }
}

impl std::error::Error for PgError {}

impl From<DecodeError<'_>> for PgError {
    fn from(err: DecodeError) -> Self {
        PgError::DecodeError(err.0.to_string())
    }
}

impl From<EncodeError<'_>> for PgError {
    fn from(err: EncodeError) -> Self {
        PgError::EncodeError(err.0.to_string())
    }
}

#[derive(Debug)]
pub struct EncodeError<'a>(pub &'a str);

impl fmt::Display for EncodeError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for EncodeError<'_> {}

#[derive(Debug)]
pub struct DecodeError<'a>(pub &'a str);

impl fmt::Display for DecodeError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for DecodeError<'_> {}