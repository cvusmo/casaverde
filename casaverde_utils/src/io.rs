// Copyright 2025 Acris Software Ltd. Co. All Rights Reserved.
// github.com/cvusmo/casaverde/casaverde_utils

use std::io;

#[derive(Debug)]
pub enum IoError {
    Io(io::Error),
    Generic(Box<dyn std::error::Error>),
}

impl std::fmt::Display for IoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IoError::Io(err) => write!(f, "IO error: {}", err),
            IoError::Generic(err) => write!(f, "Generic error: {}", err),
        }
    }
}

impl std::error::Error for IoError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            IoError::Io(err) => Some(err),
            IoError::Generic(err) => Some(err.as_ref()),
        }
    }
}

impl From<io::Error> for IoError {
    fn from(err: io::Error) -> Self {
        IoError::Io(err)
    }
}

impl From<Box<dyn std::error::Error>> for IoError {
    fn from(err: Box<dyn std::error::Error>) -> Self {
        IoError::Generic(err)
    }
}

pub use std::io::ErrorKind as IoErrorKind;

pub fn new_error(kind: IoErrorKind, message: impl Into<String>) -> IoError {
    IoError::Io(io::Error::new(kind, message.into()))
}
