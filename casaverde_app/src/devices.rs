// Copyright 2025 Nicholas Jordan. All Rights Reserved.
// github.com/cvusmo/casaverde/casaverde_app
// src/devices.rs

use serde::{Deserialize, Serialize};
use reqwest::{Client, Certificate};
use std::fs;
use std::time::{Duration, Instant};
use log::{info, warn, error};

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

#[derive(Clone)]
pub struct DeviceData {
    pub device_values: Vec<Option<f32>>,
    pub last_updated: Instant,
    pub client: Client,
    pub config: DeviceConfigRoot,
    pub active_count: usize,
}

impl DeviceData {
    pub fn new(config_path: &str) -> Self {
        let config_str = match std::fs::read_to_string(config_path) {
            Ok(c) => c,
            Err(e) => {
                error!("Failed to read config file {}: {}", config_path, e);
                panic!("Config read failed");
            }
        };
        let mut config: DeviceConfigRoot = match toml::from_str(&config_str) {
            Ok(c) => c,
            Err(e) => {
                error!("Invalid TOML config in {}: {}", config_path, e);
                panic!("Config parse failed");
            }
        };

        let device_count = config.configs.len().min(16);
        config.configs.truncate(device_count);

        let cert = match fs::read("server.crt") {
            Ok(cert_data) => match Certificate::from_pem(&cert_data) {
                Ok(c) => c,
                Err(e) => {
                    error!("Invalid certificate: {}", e);
                    panic!("Certificate validation failed");
                }
            },
            Err(e) => {
                error!("Failed to read server.crt: {}", e);
                panic!("Certificate read failed");
            }
        };

        let client = Client::builder()
            .add_root_certificate(cert)
            .use_rustls_tls()
            .min_tls_version(reqwest::tls::Version::TLS_1_3)
            .build()
            .expect("Failed to build secure client");

        let mut device_values = vec![None; device_count];
        info!("DeviceData initialized with {} devices", device_count);
        Self {
            device_values,
            last_updated: Instant::now(),
            client,
            config,
            active_count: device_count,
        }
    }

    pub async fn update_devices(&mut self) {
        let client_id = "casaverde_app".to_string(); // Hardcoded for now
        let readings = self.device_values
            .iter()
            .enumerate()
            .filter_map(|(i, value)| {
                self.config.configs.get(i).map(|config| DeviceReading {
                    id: config.id.clone(),
                    value: *value,
                })
            })
            .collect::<Vec<_>>();

        let reading = SensorReading {
            client_id: client_id.clone(),
            devices: readings,
        };

        for i in 0..self.config.configs.len() {
            let interval = Duration::from_secs(self.config.configs[i].interval as u64);
            if self.last_updated.elapsed() >= interval {
                let url = &self.config.configs[i].endpoint;
                match self.client.post(url)
                    .json(&reading)
                    .send()
                    .await
                {
                    Ok(resp) if resp.status().is_success() => {
                        if let Ok(value) = resp.json::<f32>().await {
                            self.device_values[i] = Some(value);
                            info!("Updated device {} value: {}", i, value);
                        }
                    }
                    Ok(resp) => warn!("Non-success status from {}: {}", url, resp.status()),
                    Err(e) => error!("Failed to post to {}: {}", url, e),
                }
            }
        }
        if self.active_count > 0 {
            self.last_updated = Instant::now();
        }
    }
}
