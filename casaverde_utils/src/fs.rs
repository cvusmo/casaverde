// Copyright 2025 Acris Software Ltd. Co. All Rights Reserved.
// github.com/cvusmo/casaverde/casaverde_utils
// fs.rs

use crate::io::{new_error, IoError, IoErrorKind};
use std::fs;
use std::io::Write;
use std::path::Path;

/// Reads the contents of a file into a string.
pub fn read_to_string<P: AsRef<Path>>(path: P) -> Result<String, IoError> {
    fs::read_to_string(&path)
        .map_err(|e| new_error(IoErrorKind::Other, format!("Failed to read file: {}", e)))
}

/// Creates a new file at the specified path, or opens it if it exists.
pub fn create_file<P: AsRef<Path>>(path: P) -> Result<fs::File, IoError> {
    fs::File::create(&path)
        .map_err(|e| new_error(IoErrorKind::Other, format!("Failed to create file: {}", e)))
}

/// Writes all bytes to the file.
pub fn write_all(file: &mut fs::File, buf: &[u8]) -> Result<(), IoError> {
    file.write_all(buf).map_err(|e| {
        new_error(
            IoErrorKind::Other,
            format!("Failed to write to file: {}", e),
        )
    })
}
