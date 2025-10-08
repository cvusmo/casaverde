// Copyright 2025 Acris Software Ltd. Co. All Rights Reserved.
// github.com/cvusmo/casaverde/casaverde_app
// src/devices.rs

use serde::Deserialize;
use std::time::Duration;
use tokio::time::sleep;
use crate::client::AppClient;
use crate::models::{ConfigEntry, ConfigData};
use casaverde_utils::log::{error, info};
use casaverde_utils::Logger;

#[derive(Clone, Debug, Default)]
pub struct DeviceEntry {
    pub id: String,
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
    pub config: Vec<ConfigEntry>,
    pub devices: Vec<DeviceEntry>,
    pub active_count: usize,
    pub client: AppClient,
}

impl DeviceData {
    pub fn new(config_path: &str, server: &str, logger: &mut Logger) -> Self {
        let content = std::fs::read_to_string(config_path).unwrap_or_default();
        let config: Vec<ConfigEntry> = toml::from_str(&content)
            .unwrap_or_else(|_| {
                info(logger, "Using default config due to parsing error").expect("Failed to log");
                vec![ConfigEntry {
                    current: ConfigData {
                        server: server.to_string(),
                        controller_id: "blackbeard-pi".to_string(),
                        serial_port: None,
                        light_relay_id: "relay-1".to_string(),
                        light_on_hours: 16,
                        light_off_hours: 8,
                    },
                    backup: None,
                }]
            });

        let devices = config.iter().map(|c| DeviceEntry {
            id: c.current.controller_id.clone(),
            value: None,
            active: false,
        }).collect();
        let client = AppClient::new(&config[0].current.server, "casaverde_app".to_string(), logger);

        Self {
            config,
            devices,
            active_count: 0,
            client,
        }
    }

    pub async fn update_devices(&mut self, logger: &mut Logger) -> Result<(), casaverde_utils::IoError> {
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
            if self.client.send_sensor_data(payload, &endpoint, logger).await {
                info(logger, &format!("Fetched data for device {}", cfg.current.controller_id))?;
                let value = self.client.fetch_device_value(&cfg.current.controller_id).await;
                if i < self.devices.len() {
                    self.devices[i].value = value;
                    self.devices[i].active = value.is_some();
                }
            } else {
                error(logger, &format!("Failed to fetch data for device {}", cfg.current.controller_id))?;
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

    pub fn toggle_sensor(&mut self, sensor: usize, logger: &mut Logger) {
        if sensor < self.devices.len() {
            self.devices[sensor].active = !self.devices[sensor].active;
            self.active_count = self.devices.iter().filter(|d| d.active).count();
            let action = if self.devices[sensor].active { "ON" } else { "OFF" };
            let payload = serde_json::json!({
                "controller_id": "blackbeard-pi",
                "commands": [{"action": action.to_string(), "device_id": self.devices[sensor].id.clone()}]
            });
            info(logger, &format!("Sending toggle for {}", action)).expect("Failed to log toggle send");
            tokio::spawn({
                let mut client = self.client.clone();
                let mut logger_clone = logger.clone();
                async move {
                    if client.send_sensor_data(payload, "/commands", &mut logger_clone).await {
                        info(&mut logger_clone, &format!("Toggle sent successfully for {}", action)).ok();
                    } else {
                        error(&mut logger_clone, &format!("Failed to send toggle for {}", action)).ok();
                    }
                }
            });
        }
    }
}
