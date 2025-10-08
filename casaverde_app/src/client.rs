// Copyright 2025 Acris Software Ltd. Co. All Rights Reserved.
// github.com/cvusmo/casaverde/casaverde_app
// src/client.rs

use std::error::Error;
use crate::models::ConfigEntry;
use casaverde_utils::log::{error, info};
use casaverde_utils::path::get_cert_path;
use casaverde_utils::Logger;
use reqwest::{Client, Certificate};
use serde::Serialize;
use std::fs;
use std::time::Instant;

#[derive(Clone)]
pub struct AppClient {
    pub client: Client,
    pub server: String,
    pub client_id: String,
    pub last_updated: Instant,
}

impl AppClient {
    pub fn new(server: &str, client_id: String, logger: &mut Logger) -> Self {
        let cert_path = get_cert_path("casaverde_app").expect("Failed to get certificate path");
        let cert_data = fs::read(&cert_path).expect("Failed to read server.crt");
        let cert = Certificate::from_pem(&cert_data).expect("Invalid certificate");
        info(logger, &format!("Certificate loaded successfully from {:?}", cert_path)).expect("Failed to log");

        let client = Client::builder()
            .add_root_certificate(cert)
            .use_rustls_tls()
            .min_tls_version(reqwest::tls::Version::TLS_1_3)
            .build()
            .expect("Failed to build secure client");

        Self {
            client,
            server: server.to_string(),
            client_id,
            last_updated: Instant::now(),
        }
    }

    pub async fn send_sensor_data<T: Serialize>(
        &mut self,
        data: T,
        endpoint: &str,
        logger: &mut Logger,
    ) -> bool {
        let url = format!("{}/{}", self.server, endpoint.trim_start_matches('/'));
        let data_str = serde_json::to_string(&data).unwrap_or_default();
        info(logger, &format!("Sending JSON to {}: {:?}", url, data_str))?;
        match self.client.post(&url).json(&data).send().await {
            Ok(resp) => {
                if resp.status().is_success() {
                    info(logger, "Successfully sent data to server")?;
                    self.last_updated = Instant::now();
                    true
                } else {
                    error(logger, &format!("Failed to send data to {}: status {}", url, resp.status()))?;
                    if let Ok(text) = resp.text().await {
                        error(logger, &format!("Server response: {}", text))?;
                    }
                    false
                }
            }
            Err(e) => {
                error(logger, &format!("Failed to send data to {}: {}", url, e))?;
                if let Some(source) = e.source() {
                    error(logger, &format!("Error source: {}", source))?;
                }
                if e.is_connect() {
                    error(logger, "Connection error: check server availability or network")?;
                }
                if e.is_timeout() {
                    error(logger, "Request timed out")?;
                }
                false
            }
        }
    }

    pub async fn fetch_controller_config(&self, controller_id: &str) -> Option<ConfigEntry> {
        let url = format!("{}/configs/{}", self.server, controller_id);
        self.client.get(&url).send().await.ok()?.json().await.ok()
    }

    pub async fn fetch_device_value(&self, id: &str) -> Option<f32> {
        let url = format!("{}/sensor_data", self.server);
        if let Ok(resp) = self.client.get(&url).send().await {
            if let Ok(data) = resp.json::<serde_json::Value>().await {
                if let Some(devices) = data.as_array() {
                    for device in devices {
                        if let Some(dev) = device.as_object() {
                            if dev.get("id").and_then(|v| v.as_str()) == Some(id) {
                                return dev.get("value").and_then(|v| v.as_f64()).map(|v| v as f32);
                            }
                        }
                    }
                }
            }
        }
        None
    }
}
