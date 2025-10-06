// Copyright 2025 Acris Software Ltd. Co. All Rights Reserved.
// github.com/cvusmo/casaverde/casaverde_app
// src/devices.rs

use serde::Deserialize;
use std::time::Duration;
use tokio::time::sleep;
use crate::client::AppClient;
use crate::models::{ConfigEntry, ConfigData}; // Added ConfigData import
use casaverde_utils::log::{error, info};

#[derive(Clone, Debug, Default)] // Removed Copy
pub struct DeviceEntry {
    pub id: String, // Added id field
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

#[derive(Clone)]
pub struct DeviceData {
    pub config: Vec<ConfigEntry>, // Updated to Vec<ConfigEntry>
    pub devices: Vec<DeviceEntry>,
    pub active_count: usize,
    pub client: AppClient,
}

impl DeviceData {
    pub fn new(config_path: &str, server: &str) -> Self {
        let content = std::fs::read_to_string(config_path).unwrap_or_default();
        let config: Vec<ConfigEntry> = toml::from_str(&content)
            .unwrap_or_else(|_| vec![ConfigEntry {
                current: ConfigData {
                    server: server.to_string(),
                    controller_id: "blackbeard-pi".to_string(),
                    serial_port: None,
                    light_relay_id: "relay-1".to_string(),
                    light_on_hours: 16,
                    light_off_hours: 8,
                },
                backup: None,
            }]);

        let devices = config.iter().map(|c| DeviceEntry {
            id: c.current.controller_id.clone(),
            value: None,
            active: false,
        }).collect();
        let client = AppClient::new(&config[0].current.server, "casaverde_app".to_string());

        Self {
            config,
            devices,
            active_count: 0,
            client,
        }
    }

    pub async fn update_devices(&mut self) -> std::io::Result<()> {
        for (i, cfg) in self.config.iter().enumerate() {
            let endpoint = match cfg.current.serial_port {
                Some(ref port) if port.contains("temperature") => "/sensor_data".to_string(),
                Some(ref port) if port.contains("relay") => "/commands".to_string(),
                _ => "/sensor_data".to_string(),
            };
            let payload = serde_json::json!({
                "client_id": "casaverde_app",
                "devices": [
                    {"id": cfg.current.controller_id, "action": "GET"}
                ]
            });
            if self.client.send_sensor_data(payload, &endpoint).await {
                info!("Fetched data for device {}", cfg.current.controller_id);
                let value = match cfg.current.controller_id.as_str() {
                    "blackbeard-cpu" => Some(35.0),
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
                if i < self.devices.len() {
                    self.devices[i].value = value;
                    self.devices[i].active = value.is_some();
                }
            } else {
                error!("Failed to fetch data for device {}", cfg.current.controller_id);
                if i < self.devices.len() {
                    self.devices[i].value = None;
                    self.devices[i].active = false;
                }
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
            // Send toggle command to controller (placeholder)
            let cmd = if self.devices[sensor].active {
                format!("SET {} ON", self.devices[sensor].id)
            } else {
                format!("SET {} OFF", self.devices[sensor].id)
            };
            println!("Sending command to controller: {}", cmd); // Replace with client call
        }
    }
}
