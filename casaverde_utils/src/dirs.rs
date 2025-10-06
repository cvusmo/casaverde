// Copyright 2025 Acris Software Ltd. Co. All Rights Reserved.
// github.com/cvusmo/casaverde/casaverde_utils
// dirs.rs

use std::env;
use std::fs;
use std::io;
use std::path::PathBuf;

pub fn get_home_dir() -> Result<PathBuf, io::Error> {
    if let Ok(home) = env::var("HOME") {
        return Ok(PathBuf::from(home));
    }
    if cfg!(target_os = "windows") {
        if let Ok(userprofile) = env::var("USERPROFILE") {
            return Ok(PathBuf::from(userprofile));
        }
    }
    Err(io::Error::new(
        io::ErrorKind::NotFound,
        "Unable to determine home directory",
    ))
}

/// Returns ~/casaverde/target/build/log and ensures it exists.
pub fn get_casaverde_log_dir() -> Result<PathBuf, io::Error> {
    let mut log_dir = get_home_dir()?;
    log_dir.push("casaverde/target/build/log");
    fs::create_dir_all(&log_dir)?;
    Ok(log_dir)
}
