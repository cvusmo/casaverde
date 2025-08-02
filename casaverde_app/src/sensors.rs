// Copyright 2025 Nicholas Jordan. All Rights Reserved.
// github.com/cvusmo/casaverde/casaverde_app
// src/sensors.rs

use reqwest::{Client, Certificate};
use serde::Deserialize;
use std::{fs, time::{Duration, Instant}};

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

#[derive(Clone, Deserialize)]
pub struct TempData {
    pub cpu: Option<f32>,
    pub gpu: Option<f32>,
}

#[derive(Clone)]
pub struct SensorData {
    pub states: [bool; Sensor::ALL.len()],
    pub temp_data: TempData,
    last_updated: Instant,
    client: Client,
    server: String,
}

impl SensorData {
    pub fn new(server: &str) -> Self {
        // Load server certificate for pinning
        let cert_path = "server.crt"; // Expected in casaverde_app directory
        let cert = fs::read(cert_path)
            .map_err(|e| {
                eprintln!("Failed t
 o read server.crt: {e}. Place it in the
 project directory.");                e
            })
            .ok()
            .and_then(|cert_data| Certificate::from_pem(&cert_data).ok())
            .expect("Failed to load or parse server.crt. Ensure it exists and is valid.");

        let client = Client::builder()
            .add_root_certificate(cert)
            .use_rustls_tls()
            .build()
            .expect("Failed to build reqwest client with certificate");

        let mut states = [false; Sensor::ALL.len()];
        states[Sensor::Temperature as usize] = true; // Enable Temperature by default
        Self {
            states,
            temp_data: TempData { cpu: None, gpu: None },
            last_updated: Instant::now(),
            client,
            server: server.to_string(),
        }
    }

    pub async fn update_temperatures(&mut self) {
        if self.states[Sensor::Temperature as usize] && self.last_updated.elapsed() >= Duration::from_secs(5) {
            let url = format!("{}/temps", self.server);
            match self.client.get(&url).send().await {
                Ok(resp) => {
                    if let Ok(data) = resp.json::<TempData>().await {
                        self.temp_data = data;
                    }
                    self.last_updated = Instant::now();
                }
                Err(e) => eprintln!
 ("Failed to fetch temperatures from {url
 }: {e}"),            }
        }
    }

    pub fn toggle_sensor(&mut self, index: usize) {
        self.states[index] = !self.states[index];
    }
}
