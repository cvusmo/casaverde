// Copyright 2025 Acris Software Ltd. Co. All Rights Reserved.
// github.com/cvusmo/casaverde/casaverde_utils
// io.rs

use std::io;

/// Custom error type for casaverde_utils.
#[derive(Debug)]
pub enum IoError {
    Io(io::Error),
    Toml(toml::de::Error),
    SerdeJson(serde_json::Error),
}

impl std::fmt::Display for IoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IoError::Io(err) => write!(f, "IO error: {}", err),
            IoError::Toml(err) => write!(f, "TOML error: {}", err),
            IoError::SerdeJson(err) => write!(f, "JSON error: {}", err),
        }
    }
}

impl std::error::Error for IoError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            IoError::Io(err) => Some(err),
            IoError::Toml(err) => Some(err),
            IoError::SerdeJson(err) => Some(err),
        }
    }
}

impl From<io::Error> for IoError {
    fn from(err: io::Error) -> Self {
        IoError::Io(err)
    }
}

impl From<toml::de::Error> for IoError {
    fn from(err: toml::de::Error) -> Self {
        IoError::Toml(err)
    }
}

impl From<serde_json::Error> for IoError {
    fn from(err: serde_json::Error) -> Self {
        IoError::SerdeJson(err)
    }
}

/// Re-export std::io::ErrorKind for compatibility.
pub use std::io::ErrorKind as IoErrorKind;

/// Creates a new I/O error with the given kind and message.
pub fn new_error(kind: IoErrorKind, message: impl Into<String>) -> IoError {
    IoError::Io(io::Error::new(kind, message.into()))
}
