// Copyright 2025 Acris Software Ltd. Co. All Rights Reserved.
// github.com/cvusmo/casaverde/casaverde_app
// src/devices.rs

use serde::Deserialize;
use std::time::Duration;
use tokio::time::sleep;
use crate::client::AppClient;
use casaverde_utils::log::{error, info};

#[derive(Clone, Copy, Debug, Default)]
pub struct DeviceEntry {
    pub value: Option<f32>,
    pub active: bool,
}

#[derive(Clone, Deserialize)]
pub struct DeviceConfig {
    pub id: String,
    pub r#type: String,
    pub endpoint: String,
    pub interval: u64,
    pub serial_port: String,
}

#[derive(Clone, Deserialize)]
pub struct ConfigFile {
    pub server: String,
    pub configs: Vec<DeviceConfig>,
}

#[derive(Clone)]
pub struct DeviceData {
    pub config: ConfigFile,
    pub devices: Vec<DeviceEntry>,
    pub active_count: usize,
    pub client: AppClient,
}

impl DeviceData {
    pub fn new(config_path: &str, server: &str) -> Self {
        let content = std::fs::read_to_string(config_path).unwrap_or_default();
        let mut config: ConfigFile = toml::from_str(&content).unwrap_or(ConfigFile {
            server: server.to_string(),
            configs: Vec::new(),
        });

        if config.server.is_empty() {
            config.server = server.to_string();
        }

        let num_devices = config.configs.len();
        let devices = vec![DeviceEntry::default(); num_devices];
        let client = AppClient::new(&config.server, "casaverde_app".to_string());

        Self {
            config,
            devices,
            active_count: 0,
            client,
        }
    }

    pub async fn update_devices(&mut self) -> std::io::Result<()> {
        for (i, cfg) in self.config.configs.iter().enumerate() {
            let endpoint = match cfg.r#type.as_str() {
                "temperature" | "moisture" | "nutrients" | "humidity" | "solar" | "water" => "/sensor_data",
                "relay" => "/commands",
                _ => "/sensor_data",
            };
            let payload = serde_json::json!({
                "client_id": "casaverde_app",
                "devices": [
                    {"id": cfg.id, "action": "GET"}
                ]
            });
            if self.client.send_sensor_data(payload, endpoint).await {
                info!("Fetched data for device {}", cfg.id);
                // Fetch data from server response (temporary placeholders)
                let value = match cfg.id.as_str() {
                    "blackbeard-cpu" => Some(35.0), // Replace with server response
                    "blackbeard-probe" => Some(20.0),
                    "moisture-1" => Some(50.0),
                    "nutrients-1" => Some(0.5),
                    "humidity-1" => Some(70.0),
                    "solar-1" => Some(100.0),
                    "water-1" => Some(30.0),
                    "relay-1" => Some(1.0),
                    "relay-2" => Some(0.0),
                    "relay-3" => Some(1.0),
                    "relay-4" => Some(0.0),
                    _ => None,
                };
                self.devices[i].value = value;
                self.devices[i].active = value.is_some();
            } else {
                error!("Failed to fetch data for device {}", cfg.id);
                self.devices[i].value = None;
                self.devices[i].active = false;
            }
            sleep(Duration::from_millis(50)).await;
        }
        self.active_count = self.devices.iter().filter(|d| d.active).count();
        Ok(())
    }

    pub fn toggle_sensor(&mut self, sensor: usize) {
        if sensor < self.devices.len() {
            self.devices[sensor].active = !self.devices[sensor].active;
            self.active_count = self.devices.iter().filter(|d| d.active).count();
        }
    }
}
