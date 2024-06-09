#[derive(Debug, PartialEq)]
pub enum ErrorKind {
    IOError,
    AddrError,
    InvalidSection,
    InvalidKey
}


#[derive(Debug)]
pub struct Error {
    pub kind: ErrorKind,
    pub message: String
}

impl Error {
    pub fn new<S: Into<String>>(kind: ErrorKind, message: S) -> Self {
        Error { kind, message: message.into() }
    }
}

impl std::error::Error for Error {}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error {
            kind: ErrorKind::IOError,
            message: err.to_string()
        }
    }
}

impl From<std::net::AddrParseError> for Error {
    fn from(err: std::net::AddrParseError) -> Self {
        Error {
            kind: ErrorKind::AddrError,
            message: err.to_string()
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Config Error ({:?}): {}", self.kind, self.message)
    }
}


pub type Result<T> = std::result::Result<T, Error>;
