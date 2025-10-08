// Copyright 2025 Acris Software Ltd. Co. All Rights Reserved.
// github.com/cvusmo/casaverde/casaverde_utils
// fs.rs

use crate::io::{new_error, IoError, IoErrorKind};
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::Path;

pub fn read_to_string<P: AsRef<Path>>(path: P) -> Result<String, IoError> {
    fs::read_to_string(&path)
        .map_err(|e| new_error(IoErrorKind::Other, format!("Failed to read file: {}", e)))
}

pub fn create_file<P: AsRef<Path>>(path: P) -> Result<fs::File, IoError> {
    OpenOptions::new()
        .create(true) // Create the file if it doesn't exist
        .append(true) // Open in append mode
        .open(&path)
        .map_err(|e| {
            new_error(
                IoErrorKind::Other,
                format!("Failed to open/create file: {}", e),
            )
        })
}

pub fn write_all(file: &mut fs::File, buf: &[u8]) -> Result<(), IoError> {
    file.write_all(buf).map_err(|e| {
        new_error(
            IoErrorKind::Other,
            format!("Failed to write to file: {}", e),
        )
    })
}
