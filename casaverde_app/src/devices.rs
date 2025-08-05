// Copyright 2025 Nicholas Jordan. All Rights Reserved.
// github.com/cvusmo/casaverde/casaverde_app
// src/devices.rs

use serde::{Deserialize, Serialize};
use reqwest::{Client, Certificate};
use std::fs;
use std::time::{Duration, Instant};
use log::{info, warn, error};
use std::error::Error;
use uuid::Uuid;
use std::process::Command as ProcessCommand;

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
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DeviceConfig {
    pub id: String,
    pub r#type: String,
    pub endpoint: String,
    pub interval: u32,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DeviceConfigRoot {
    pub server: String,
    pub configs: Vec<DeviceConfig>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct TempData {
    pub cpu: Option<f32>,
    pub gpu: Option<f32>,
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
}

impl DeviceData {
    pub fn new(config_path: &str) -> Self {
        let config_str = match std::fs::read_to_string(config_path) {
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

        let cert = match fs::read("server.crt") {
            Ok(cert_data) => match Certificate::from_pem(&cert_data) {
                Ok(c) => {
                    info!("Certificate loaded successfully");
                    c
                }
                Err(e) => {
                    error!("Invalid certificate: {e}");
                    panic!("Certificate validation failed");
                }
            },
            Err(e) => {
                error!("Failed to read server.crt: {e}");
                panic!("Certificate read failed");
            }
        };

        let client = Client::builder()
            .add_root_certificate(cert)
            .use_rustls_tls()
            .min_tls_version(reqwest::tls::Version::TLS_1_3)
            .danger_accept_invalid_certs(true) // TEMPORARY BYPASS
            .build()
            .expect("Failed to build secure client");

        let mut states = [false; Sensor::ALL.len()];
        states[Sensor::Temperature as usize] = true;
        let device_values = vec![None; device_count];

        info!("DeviceData initialized with {device_count} devices");
        Self {
            states,
            temp_data: TempData { cpu: None, gpu: None },
            device_values,
            last_updated: Instant::now(),
            client,
            config,
            client_id: Uuid::new_v4().to_string(),
            active_count: device_count,
        }
    }

    pub async fn update_devices(&mut self) {
        let mut devices = Vec::new();

        if self.states[Sensor::Temperature as usize] {
            self.temp_data = TempData {
                cpu: get_cpu_temp(),
                gpu: get_gpu_temp(),
            };

            devices = vec![
                DeviceReading { id: "blackbeard-cpu".to_string(), value: self.temp_data.cpu },
                DeviceReading { id: "blackbeard-gpu".to_string(), value: self.temp_data.gpu },
            ];

            for (i, config) in self.config.configs.iter().enumerate() {
                match config.id.as_str() {
                    "blackbeard-cpu" => self.device_values[i] = self.temp_data.cpu,
                    "blackbeard-gpu" => self.device_values[i] = self.temp_data.gpu,
                    _ => {}
                }
            }
        } else {
            self.temp_data = TempData { cpu: None, gpu: None };
            for value in &mut self.device_values {
                *value = None;
            }
        }

        if self.last_updated.elapsed() >= Duration::from_secs(5) {
            let reading = SensorReading {
                client_id: self.client_id.clone(),
                devices: devices,
            };

            let url = format!("{}/sensor_data", self.config.server);
            info!("Sending JSON: {:?}", serde_json::to_string(&reading).unwrap_or_default());
            match self.client.post(&url).json(&reading).send().await {
                Ok(resp) => {
                    if resp.status().is_success() {
                        info!("Successfully sent temperature data to server");
                        self.last_updated = Instant::now();
                    } else {
                        warn!("Failed to send data to {url}: status {}", resp.status());
                        if let Ok(text) = resp.text().await {
                            warn!("Server response: {text}"); // Fixed with !
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

    pub fn toggle_sensor(&mut self, sensor: Sensor) {
        let index = sensor as usize;
        if index < self.states.len() {
            self.states[index] = !self.states[index];
            info!("Toggled {} to {}", sensor.name(), self.states[index]);
        }
    }
}

fn get_cpu_temp() -> Option<f32> {
    match ProcessCommand::new("sensors").output() {
        Ok(output) => {
            let sensors_str = String::from_utf8_lossy(&output.stdout);
            for line in sensors_str.lines() {
                if line.contains("Package id 0") {
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    for part in parts {
                        if part.ends_with("°C") {
                            if let Ok(temp) = part.trim_end_matches("°C").trim_start_matches('+').parse::<f32>() {
                                return Some(temp);
                            }
                        }
                    }
                }
            }
            None
        }
        Err(e) => {
            error!("Failed to run sensors: {e}");
            None
        }
    }
}

fn get_gpu_temp() -> Option<f32> {
    match ProcessCommand::new("nvidia-smi")
        .arg("--query-gpu=temperature.gpu")
        .arg("--format=csv,noheader")
        .output()
    {
        Ok(output) => {
            let nvidia_str = String::from_utf8_lossy(&output.stdout);
            nvidia_str.trim().parse().ok()
        }
        Err(e) => {
            error!("Failed to run nvidia-smi: {e}");
            None
        }
    }
}

fn get_hostname() -> String {
    std::env::var("HOSTNAME").unwrap_or_else(|_| "unknown".to_string())
}
