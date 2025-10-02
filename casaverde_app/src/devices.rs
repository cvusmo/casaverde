// Copyright 2025 Acris Software Ltd. Co. All Rights Reserved.
// github.com/cvusmo/casaverde/casaverde_app
// src/devices.rs

use crate::models::{ConfigEntry, ConfigPayload};
use log::{error, info, warn};
use reqwest::{Certificate, Client};
use serde::{Deserialize, Serialize};
use serialport::{self, SerialPort};
use std::error::Error;
use std::fs;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use uuid::Uuid;

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
struct SensorReading {
    client_id: String,
    devices: Vec<DeviceReading>,
}

#[derive(Serialize)]
struct DeviceReading {
    id: String,
    value: Option<f32>,
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
    pub last_updated: Instant,
    pub client: Client,
    pub config: DeviceConfigRoot,
    pub client_id: String,
    pub active_count: usize,
    pub serial_ports: Vec<Option<Arc<Mutex<Box<dyn SerialPort>>>>>,
}

impl DeviceData {
    pub fn new(config_path: &str) -> Self {
        let config_path = if config_path.is_empty() {
            "/home/echo/tmp/config.toml".to_string()
        } else {
            config_path.to_string()
        };
        let cert_path = "/home/echo/tmp/server.crt";
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

        let cert = match fs::read(&cert_path) {
            Ok(cert_data) => match Certificate::from_pem(&cert_data) {
                Ok(c) => {
                    info!("Certificate loaded successfully from {}", cert_path);
                    c
                }
                Err(e) => {
                    error!("Invalid certificate at {}: {}", cert_path, e);
                    panic!("Certificate validation failed");
                }
            },
            Err(e) => {
                error!("Failed to read server.crt at {}: {}", cert_path, e);
                panic!("Certificate read failed");
            }
        };

        let client = Client::builder()
            .add_root_certificate(cert)
            .use_rustls_tls()
            .min_tls_version(reqwest::tls::Version::TLS_1_3)
            .danger_accept_invalid_certs(true)
            .build()
            .expect("Failed to build secure client");

        let states = [true; Sensor::ALL.len()];
        let serial_ports = config
            .configs
            .iter()
            .map(|config| {
                match serialport::new(&config.serial_port, 9600)
                    .timeout(Duration::from_secs(1))
                    .open()
                {
                    Ok(port) => {
                        info!(
                            "Opened serial port {} for device {}",
                            config.serial_port, config.id
                        );
                        Some(Arc::new(Mutex::new(port)))
                    }
                    Err(e) => {
                        error!(
                            "Failed to open serial port {} for device {}: {}",
                            config.serial_port, config.id, e
                        );
                        None
                    }
                }
            })
            .collect::<Vec<_>>();

        let device_values = vec![None; device_count];

        info!("DeviceData initialized with {device_count} devices");
        Self {
            states,
            temp_data: TempData { cpu: None },
            device_values,
            last_updated: Instant::now(),
            client,
            config,
            client_id: Uuid::new_v4().to_string(),
            active_count: device_count,
            serial_ports,
        }
    }

    pub async fn update_devices(&mut self) {
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

            if let Some(sensor) = sensor {
                if self.states[sensor as usize] {
                    let value = self.read_from_serial(i, &config.id);
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
            }
        }

        if self.last_updated.elapsed() >= Duration::from_secs(5) {
            let reading = SensorReading {
                client_id: self.client_id.clone(),
                devices,
            };

            let url = format!("{}/sensor_data", self.config.server);
            info!(
                "Sending JSON: {:?}",
                serde_json::to_string(&reading).unwrap_or_default()
            );
            match self.client.post(&url).json(&reading).send().await {
                Ok(resp) => {
                    if resp.status().is_success() {
                        info!("Successfully sent sensor data to server");
                        self.last_updated = Instant::now();
                    } else {
                        warn!("Failed to send data to {url}: status {}", resp.status());
                        if let Ok(text) = resp.text().await {
                            warn!("Server response: {text}");
                        }
                    }
                }
                Err(e) => {
                    error!("Failed to send data to {url}: {e}");
                    if let Some(source) = e.source() {
                        error!("Error source: {source}");
                    }
                    if e.is_connect() {
                        error!("Connection error: check server availability or network");
                    }
                    if e.is_timeout() {
                        error!("Request timed out");
                    }
                }
            }
        }
    }

    fn read_from_serial(&mut self, index: usize, device_id: &str) -> Option<f32> {
        if let Some(Some(port)) = self.serial_ports.get(index) {
            let mut port = port.lock().unwrap();
            let command = format!("{}\n", device_id);
            if let Err(e) = port.write(command.as_bytes()) {
                error!("Failed to write to serial port for {}: {}", device_id, e);
                return None;
            }

            let mut buffer = [0u8; 128];
            match port.read(&mut buffer) {
                Ok(bytes_read) => {
                    let response = String::from_utf8_lossy(&buffer[..bytes_read])
                        .trim()
                        .to_string();
                    info!("Received from {}: {}", device_id, response);
                    match response {
                        s if s.starts_with("TEMP:") => s
                            .strip_prefix("TEMP:")
                            .and_then(|v| v.strip_suffix("°C"))
                            .and_then(|v| v.parse::<f32>().ok()),
                        s if s.starts_with("SOLAR:") => s
                            .strip_prefix("SOLAR:")
                            .and_then(|v| v.strip_suffix("W"))
                            .and_then(|v| v.parse::<f32>().ok()),
                        s if s.starts_with("MOISTURE:") => s
                            .strip_prefix("MOISTURE:")
                            .and_then(|v| v.parse::<f32>().ok()),
                        s if s.starts_with("HUMIDITY:") => s
                            .strip_prefix("HUMIDITY:")
                            .and_then(|v| v.strip_suffix("%"))
                            .and_then(|v| v.parse::<f32>().ok()),
                        s if s.starts_with("WATER:") => {
                            s.strip_prefix("WATER:").and_then(|v| v.parse::<f32>().ok())
                        }
                        _ => {
                            error!("Invalid response for {}: {}", device_id, response);
                            None
                        }
                    }
                }
                Err(e) => {
                    error!("Failed to read from serial port for {}: {}", device_id, e);
                    None
                }
            }
        } else {
            error!(
                "No serial port available for device {} at index {}",
                device_id, index
            );
            None
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
        let url = format!("{}/configs/{}", self.config.server, controller_id);
        self.client.get(&url).send().await.ok()?.json().await.ok()
    }

    pub async fn update_controller_config(&self, payload: ConfigPayload) {
        let url = format!("{}/configs", self.config.server);
        self.client.post(&url).json(&payload).send().await.ok();
    }
}

fn get_hostname() -> String {
    std::env::var("HOSTNAME").unwrap_or_else(|_| "unknown".to_string())
}
