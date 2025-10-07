// Copyright 2025 Acris Software Ltd. Co. All Rights Reserved.
// github.com/cvusmo/casaverde/casaverde_utils
// src/path.rs

use crate::dirs::get_home_dir;
use crate::io::{new_error, IoError, IoErrorKind};
use std::fs;
use std::path::PathBuf as StdPathBuf;

pub use std::path::PathBuf;

pub fn get_config_dir() -> Result<PathBuf, IoError> {
    let cfg_dir = if cfg!(debug_assertions) {
        PathBuf::from("./build_output/linux")
    } else {
        get_home_dir()?.join(".config/casaverde")
    };
    fs::create_dir_all(&cfg_dir).map_err(|e| {
        new_error(
            IoErrorKind::Other,
            format!("Failed to create config directory: {}", e),
        )
    })?;
    Ok(cfg_dir)
}

pub fn get_config_path(project: &str) -> Result<PathBuf, IoError> {
    let cfg_dir = get_config_dir()?;
    Ok(cfg_dir.join(project).join("config.toml"))
}

pub fn get_cert_path(project: &str) -> Result<PathBuf, IoError> {
    let cfg_dir = get_config_dir()?;
    Ok(cfg_dir.join(project).join("server.crt"))
}

pub fn get_key_path(project: &str) -> Result<PathBuf, IoError> {
    let cfg_dir = get_config_dir()?;
    Ok(cfg_dir.join(project).join("server.key"))
}
