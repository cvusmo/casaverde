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
    pub device_count: usize,
    pub configs: Vec<DeviceConfig>,
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

        config.device_count = config.configs.len().min(16);
        config.configs.truncate(config.device_count);

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

        let device_values = vec![None; config.device_count];
        info!("DeviceData initialized with {} devices", config.device_count);
        Self {
            device_values,
            last_updated: Instant::now(),
            client,
            config: config.clone(),
            active_count: config.device_count,
        }
    }

    pub async fn update_devices(&mut self) {
        for i in 0..self.config.device_count {
            let interval = Duration::from_secs(self.config.configs[i].interval as u64);
            if self.last_updated.elapsed() >= interval {
                let url = &self.config.configs[i].endpoint;
                match self.client.get(url).send().await {
                    Ok(resp) if resp.status().is_success() => {
                        if let Ok(value) = resp.json::<f32>().await {
                            self.device_values[i] = Some(value);
                            info!("Updated device {} value: {}", i, value);
                        }
                    }
                    Ok(resp) => warn!("Non-success status from {}: {}", url, resp.status()),
                    Err(e) => error!("Failed to fetch from {}: {}", url, e),
                }
            }
        }
        if self.active_count > 0 {
            self.last_updated = Instant::now();
        }
    }
}
