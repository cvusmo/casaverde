// Copyright 2025 Acris Software Ltd. Co. All Rights Reserved.
// github.com/cvusmo/casaverde/casaverde_controller
// src/config.rs - Configuration loading and hostname utilities

use casaverde_utils::log::{error, info};
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub server: String,
    pub controller_id: String,
    pub serial_port: Option<String>,
    pub light_relay_id: String,
    pub light_on_hours: u64,
    pub light_off_hours: u64,
}

pub fn load_config() -> Result<Config, Box<dyn std::error::Error>> {
    let config_str = fs::read_to_string("config.toml").map_err(|e| {
        error!("Failed to read config.toml: {e}");
        e
    })?;
    let config: Config = toml::from_str(&config_str).map_err(|e| {
        error!("Failed to parse config.toml: {e}");
        e
    })?;
    info!("Configuration loaded successfully");
    Ok(config)
}

pub fn get_hostname() -> String {
    std::env::var("HOSTNAME").unwrap_or_else(|_| {
        info!("HOSTNAME environment variable not set, using 'unknown'");
        "unknown".to_string()
    })
}
