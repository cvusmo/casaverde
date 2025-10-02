// Copyright 2025 Acris Software Ltd. Co. All Rights Reserved.
// github.com/cvusmo/casaverde/casaverde_app
// src/devices.rs

use crate::{client::AppClient, models::{ConfigEntry, ConfigPayload}};
use log::{error, info};
use serde::{Deserialize, Serialize};
use std::fs;
use uuid::Uuid;
use tokio::time::Duration;

#[derive(Copy, Clone, PartialEq)]
pub enum Sensor {
    Solar,
    Temperature,
    Moisture,
    Humidity,
    Water,
}

impl Sensor {
    pub const ALL: [Sensor; 5] = [
        Sensor::Solar,
        Sensor::Temperature,
        Sensor::Moisture,
        Sensor::Humidity,
        Sensor::Water,
    ];

    pub fn name(self) -> &'static str {
        match self {
            Sensor::Solar => "Solar Sensor",
            Sensor::Temperature => "Temperature Sensor",
            Sensor::Moisture => "Moisture Sensor",
            Sensor::Humidity => "Humidity Sensor",
            Sensor::Water => "Water Sensor",
        }
    }

    pub fn device_id(self) -> &'static str {
        match self {
            Sensor::Solar => "solar-1",
            Sensor::Temperature => "blackbeard-cpu",
            Sensor::Moisture => "moisture-1",
            Sensor::Humidity => "humidity-1",
            Sensor::Water => "water-1",
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DeviceConfig {
    pub id: String,
    pub r#type: String,
    pub endpoint: String,
    pub interval: u32,
    pub serial_port: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DeviceConfigRoot {
    pub server: String,
    pub configs: Vec<DeviceConfig>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct TempData {
    pub cpu: Option<f32>,
}

#[derive(Serialize)]
pub struct SensorReading {
    pub client_id: String,
    pub devices: Vec<DeviceReading>,
}

#[derive(Serialize, Deserialize)]
pub struct DeviceReading {
    pub id: String,
    pub value: Option<f32>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct AppCommand {
    pub action: String,
    pub device_id: String,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct CommandPayload {
    pub controller_id: String,
    pub commands: Vec<AppCommand>,
}

#[derive(Clone)]
pub struct DeviceData {
    pub states: [bool; Sensor::ALL.len()],
    pub temp_data: TempData,
    pub device_values: Vec<Option<f32>>,
    pub client: AppClient,
    pub config: DeviceConfigRoot,
    pub active_count: usize,
}

impl DeviceData {
    pub fn new(config_path: &str) -> Self {
        let config_path = if config_path.is_empty() {
            "/home/echo/.config/casaverde_app/config.toml".to_string()
        } else {
            config_path.to_string()
        };
        let config_str = match std::fs::read_to_string(&config_path) {
            Ok(c) => c,
            Err(e) => {
                error!("Failed to read config file {config_path}: {e}");
                panic!("Config read failed");
            }
        };
        let mut config: DeviceConfigRoot = match toml::from_str(&config_str) {
            Ok(c) => c,
            Err(e) => {
                error!("Invalid TOML config in {config_path}: {e}");
                panic!("Config parse failed");
            }
        };

        let device_count = config.configs.len().min(16);
        config.configs.truncate(device_count);

        let client_id = Uuid::new_v4().to_string();
        let client = AppClient::new(&config.server, client_id.clone());

        let states = [true; Sensor::ALL.len()];
        let device_values = vec![None; device_count];

        info!("DeviceData initialized with {device_count} devices");
        Self {
            states,
            temp_data: TempData { cpu: None },
            device_values,
            client,
            config,
            active_count: device_count,
        }
    }

    pub async fn update_devices(&mut self) {
        let url = format!("{}/temps", self.config.server);
        match self.client.client.get(&url).send().await {
            Ok(resp) => {
                if let Ok(readings) = resp.json::<Vec<(String, Vec<DeviceReading>)>>().await {
                    let mut devices = Vec::new();
                    let configs = self.config.configs.clone();

                    for (i, config) in configs.iter().enumerate() {
                        let sensor = match config.id.as_str() {
                            "blackbeard-cpu" => Some(Sensor::Temperature),
                            "solar-1" => Some(Sensor::Solar),
                            "moisture-1" => Some(Sensor::Moisture),
                            "humidity-1" => Some(Sensor::Humidity),
                            "water-1" => Some(Sensor::Water),
                            _ => None,
                        };

                        let value = readings.iter()
                            .find(|(client_id, _)| client_id == &self.client.client_id)
                            .and_then(|(_, devs)| devs.iter().find(|d| d.id == config.id))
                            .and_then(|d| d.value);

                        if let Some(sensor) = sensor {
                            if self.states[sensor as usize] {
                                self.device_values[i] = value;
                                if config.id == "blackbeard-cpu" {
                                    self.temp_data.cpu = value;
                                }
                                devices.push(DeviceReading {
                                    id: config.id.clone(),
                                    value,
                                });
                            } else {
                                self.device_values[i] = None;
                                if config.id == "blackbeard-cpu" {
                                    self.temp_data.cpu = None;
                                }
                            }
                        } else if config.id == "relay-1" {
                            devices.push(DeviceReading {
                                id: config.id.clone(),
                                value,
                            });
                        }
                    }

                    if self.client.last_updated.elapsed() >= Duration::from_secs(5) {
                        let reading = SensorReading {
                            client_id: self.client.client_id.clone(),
                            devices,
                        };
                        self.client.send_sensor_data(reading, "sensor_data").await;
                    }
                } else {
                    error!("Failed to parse sensor data from server");
                }
            }
            Err(e) => {
                error!("Failed to fetch sensor data from {}: {}", url, e);
            }
        }
    }

    pub fn toggle_sensor(&mut self, sensor: Sensor) {
        let index = sensor as usize;
        if index < self.states.len() {
            self.states[index] = !self.states[index];
            info!("Toggled {} to {}", sensor.name(), self.states[index]);
        }
    }

    pub async fn fetch_controller_config(&self, controller_id: &str) -> Option<ConfigEntry> {
        self.client.fetch_controller_config(controller_id).await
    }

    pub async fn update_controller_config(&self, payload: ConfigPayload) {
        self.client.update_controller_config(payload).await
    }
}
