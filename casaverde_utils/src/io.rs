use std::io::{self, Error, ErrorKind};

pub use std::io::{Error as IoError, ErrorKind as IoErrorKind};

/// Creates a new I/O error with the given kind and message.
pub fn new_error(kind: IoErrorKind, message: impl Into<String>) -> IoError {
    IoError::new(kind, message.into())
}
