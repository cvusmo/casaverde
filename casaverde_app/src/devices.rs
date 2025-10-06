// Copyright 2025 Acris Software Ltd. Co. All Rights Reserved.
// github.com/cvusmo/casaverde/casaverde_app
// src/devices.rs

use serde::Deserialize;
use std::{fs, io, time::Duration};
use tokio::time::sleep;

/// Represents a single device’s runtime state and measurement.
#[derive(Clone, Copy, Debug, Default)]
pub struct DeviceEntry {
    pub value: Option<f32>,
    pub active: bool,
}

/// Configuration for a single hardware device.
#[derive(Clone, Deserialize)]
pub struct DeviceConfig {
    pub id: String,
    pub r#type: String,
    pub endpoint: String,
    pub interval: u64,
    pub serial_port: String,
}

/// Top-level configuration file parsed from TOML.
#[derive(Clone, Deserialize)]
pub struct ConfigFile {
    pub server: String,
    pub configs: Vec<DeviceConfig>,
}

/// Data-oriented flat structure representing all device states.
#[derive(Clone)]
pub struct DeviceData {
    pub config: ConfigFile,
    pub devices: Vec<DeviceEntry>,
    pub active_count: usize,
}

impl DeviceData {
    /// Initialize device state with defaults or loaded configuration.
    pub fn new(config_path: &str, server: &str) -> Self {
        let content = fs::read_to_string(config_path).unwrap_or_default();
        let mut config: ConfigFile = toml::from_str(&content).unwrap_or(ConfigFile {
            server: server.to_string(),
            configs: Vec::new(),
        });

        if config.server.is_empty() {
            config.server = server.to_string();
        }

        let num_devices = config.configs.len();
        let devices = vec![DeviceEntry::default(); num_devices];

        Self {
            config,
            devices,
            active_count: 0,
        }
    }

    /// Simulates updating device values.
    /// Replace this with hardware or network polling later.
    pub async fn update_devices(&mut self) -> io::Result<()> {
        for (i, _cfg) in self.config.configs.iter().enumerate() {
            // Simulated reading
            let value = (i as f32 * 1.23) + 20.0;
            self.devices[i].value = Some(value);
            self.devices[i].active = i % 2 == 0;

            sleep(Duration::from_millis(50)).await;
        }

        self.active_count = self.config.configs.len();
        Ok(())
    }

    /// Toggle a specific sensor’s active state.
    pub fn toggle_sensor(&mut self, sensor: usize) {
        if sensor < self.devices.len() {
            self.devices[sensor].active = !self.devices[sensor].active;
        }
    }
}
