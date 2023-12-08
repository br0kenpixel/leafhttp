use std::{fmt::Display, io};

#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    Io(io::Error),
    EmptyStream,
    Syntax(String),
    UnknownMethod,
    InvalidHttpVersion,
}

impl Error {
    pub fn syntax(description: &str) -> Self {
        Self::Syntax(description.to_string())
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(err) => write!(f, "I/O Error: {err}"),
            Self::EmptyStream => write!(f, "Client sent no data"),
            Self::Syntax(description) => write!(f, "Malformed request: {description}"),
            Self::UnknownMethod => write!(f, "Unknown/unsupported/malformed HTTP method"),
            Self::InvalidHttpVersion => write!(f, "Invalid HTTP protocol version"),
        }
    }
}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Self::Io(value)
    }
}

impl std::error::Error for Error {}
