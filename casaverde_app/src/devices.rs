// Copyright 2025 Acris Software Ltd. Co. All Rights Reserved.
// github.com/cvusmo/casaverde/casaverde_app
// src/devices.rs

use crate::{client::AppClient, models::ConfigEntry};
use log::{error, info};
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, PartialEq)]
pub enum Sensor {
    Solar = 0,
    Temperature = 1,
    Moisture = 2,
    Humidity = 3,
    Water = 4,
    Probe = 5,
}

impl Sensor {
    pub const ALL: [Sensor; 6] = [
        Sensor::Solar,
        Sensor::Temperature,
        Sensor::Moisture,
        Sensor::Humidity,
        Sensor::Water,
        Sensor::Probe,
    ];

    pub fn name(self) -> &'static str {
        match self {
            Sensor::Solar => "Solar Sensor",
            Sensor::Temperature => "CPU Temperature",
            Sensor::Moisture => "Moisture Sensor",
            Sensor::Humidity => "Humidity Sensor",
            Sensor::Water => "Water Sensor",
            Sensor::Probe => "Probe Temperature",
        }
    }

    pub fn device_id(self) -> &'static str {
        match self {
            Sensor::Solar => "solar-1",
            Sensor::Temperature => "blackbeard-cpu",
            Sensor::Moisture => "moisture-1",
            Sensor::Humidity => "humidity-1",
            Sensor::Water => "water-1",
            Sensor::Probe => "blackbeard-probe",
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
    pub probe: Option<f32>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct DeviceReading {
    pub id: String,
    pub value: Option<f32>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SensorReading {
    pub client_id: String,
    pub devices: Vec<DeviceReading>,
}

#[derive(Clone)]
pub struct DeviceData {
    pub states: [bool; Sensor::ALL.len()],
    pub temp_data: TempData,
    pub device_values: [Option<f32>; 10],
    pub client: AppClient,
    pub config: DeviceConfigRoot,
    pub active_count: usize,
}

impl DeviceData {
    pub fn new(config_path: &str) -> Self {
        let config_str = std::fs::read_to_string(config_path)
            .unwrap_or_else(|e| panic!("Failed to read config: {e}"));

        let mut config: DeviceConfigRoot = toml::from_str(&config_str)
            .unwrap_or_else(|e| panic!("Failed to parse config: {e}"));

        let device_count = config.configs.len().min(10);
        config.configs.truncate(device_count);

        let client_id = "blackbeard-pi".to_string();
        let client = AppClient::new(&config.server, client_id.clone());

        let states = [true; Sensor::ALL.len()];
        let device_values = [None; 10];

        info!("DeviceData initialized with {device_count} devices");
        Self {
            states,
            temp_data: TempData { cpu: None, probe: None },
            device_values,
            client,
            config,
            active_count: device_count,
        }
    }

    pub async fn update_devices(&mut self) {
        let url = format!("{}/sensor_data", self.config.server);
        match self.client.client.get(&url).send().await {
            Ok(resp) => match resp.json::<Vec<SensorReading>>().await {
                Ok(sensor_payload) => {
                    for i in 0..self.active_count {
                        let cfg = &self.config.configs[i];
                        let sensor_opt = Sensor::ALL.iter().find(|s| s.device_id() == cfg.id).copied();

                        let value = sensor_payload
                            .iter()
                            .filter(|r| r.client_id == self.client.client_id || r.client_id == "blackbeard-pi")
                            .flat_map(|r| r.devices.iter())
                            .find(|d| d.id == cfg.id)
                            .and_then(|d| d.value);

                        self.device_values[i] = sensor_opt.map_or(value, |s| {
                            if self.states[s as usize] { value } else { None }
                        });

                        if cfg.id == "blackbeard-cpu" {
                            self.temp_data.cpu = self.device_values[i];
                        }
                        if cfg.id == "blackbeard-probe" {
                            self.temp_data.probe = self.device_values[i];
                        }
                    }
                }
                Err(e) => error!("Failed to parse sensor data: {:?}", e),
            },
            Err(_) => error!("Failed to fetch sensor data from {}", url),
        }
    }

    pub fn toggle_sensor(&mut self, sensor: Sensor) {
        self.states[sensor as usize] = !self.states[sensor as usize];
        info!("Toggled {} to {}", sensor.name(), self.states[sensor as usize]);
    }

    pub async fn fetch_controller_config(&self, controller_id: &str) -> Option<ConfigEntry> {
        self.client.fetch_controller_config(controller_id).await
    }
}

