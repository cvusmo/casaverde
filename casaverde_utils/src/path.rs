// Copyright 2025 Acris Software Ltd. Co. All Rights Reserved.
// github.com/cvusmo/casaverde/casaverde_utils
// src/path.rs

use crate::dirs::get_home_dir;
use crate::io::{new_error, IoError, IoErrorKind};
pub use std::path::PathBuf;
use std::{env, fs};

fn get_project_root() -> Result<PathBuf, IoError> {
    let exe = env::current_exe().map_err(|e| {
        new_error(
            IoErrorKind::Other,
            format!("Failed to get current executable: {}", e),
        )
    })?;
    let path = exe
        .parent()
        .and_then(|p| p.parent())
        .ok_or_else(|| new_error(IoErrorKind::Other, "Failed to resolve project root"))?;
    Ok(path.to_path_buf())
}

pub fn get_config_dir() -> Result<PathBuf, IoError> {
    let cfg_dir = if cfg!(debug_assertions) {
        get_project_root()?.join("build_output/linux")
    } else {
        get_home_dir()?.join(".config/casaverde")
    };

    if !cfg!(debug_assertions) {
        fs::create_dir_all(&cfg_dir).map_err(|e| {
            new_error(
                IoErrorKind::Other,
                format!("Failed to create config directory: {}", e),
            )
        })?;
    }

    Ok(cfg_dir)
}

pub fn get_cert_path(project: &str) -> Result<PathBuf, IoError> {
    // In debug, look in binary directory first
    if cfg!(debug_assertions) {
        let exe_dir = env::current_exe()
            .map_err(|e| new_error(IoErrorKind::Other, format!("Failed to get exe path: {}", e)))?
            .parent()
            .ok_or_else(|| new_error(IoErrorKind::Other, "Failed to get exe parent"))?
            .to_path_buf();

        let path = exe_dir.join("server.crt");
        if path.exists() {
            return Ok(path);
        }
        // fallback to build_output/linux/<project>/server.crt
        return Ok(get_project_root()?
            .join("build_output/linux")
            .join(project)
            .join("server.crt"));
    }

    Ok(get_config_dir()?.join(project).join("server.crt"))
}

pub fn get_key_path(project: &str) -> Result<PathBuf, IoError> {
    if cfg!(debug_assertions) {
        let exe_dir = env::current_exe()
            .map_err(|e| new_error(IoErrorKind::Other, format!("Failed to get exe path: {}", e)))?
            .parent()
            .ok_or_else(|| new_error(IoErrorKind::Other, "Failed to get exe parent"))?
            .to_path_buf();

        let path = exe_dir.join("server.key");
        if path.exists() {
            return Ok(path);
        }

        return Ok(get_project_root()?
            .join("build_output/linux")
            .join(project)
            .join("server.key"));
    }

    Ok(get_config_dir()?.join(project).join("server.key"))
}

pub fn get_config_path(app_name: &str) -> PathBuf {
    if cfg!(debug_assertions) {
        // current exe dir first
        if let Ok(exe_dir) = env::current_exe().map(|p| p.parent().unwrap().to_path_buf()) {
            let candidate = exe_dir.join("config.toml");
            if candidate.exists() {
                return candidate;
            }
        }
        // fallback
        if let Ok(root) = get_project_root() {
            return root
                .join("build_output/linux")
                .join(app_name)
                .join("config.toml");
        }
    }

    // release / fallback
    get_config_dir()
        .unwrap_or_else(|_| PathBuf::from("."))
        .join(app_name)
        .join("config.toml")
}

pub fn get_log_path(project: &str) -> Result<PathBuf, IoError> {
    let base = if cfg!(debug_assertions) {
        get_project_root()?.join("build_output/linux/logs")
    } else {
        get_home_dir()?.join(".local/share/casaverde/logs")
    };
    Ok(base.join(format!("{}.log", project)))
}
