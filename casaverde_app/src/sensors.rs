// Copyright 2025 Nicholas Jordan. All Rights Reserved.
// github.com/cvusmo/casaverde/casaverde_app
// src/sensors.rs

// Purpose:
// Defines the Sensor enum, manages sensor states, and fetches temperature data from
// casaverde_server

use reqwest::Client;
use serde::Deserialize;
use std::time::{Duration, Instant};

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
}

impl SensorData {
    pub fn new() -> Self {
        let mut states = [false; Sensor::ALL.len()];
        states[Sensor::Temperature as usize] = true; // Enable Temperature by default
        Self {
            states,
            temp_data: TempData { cpu: None, gpu: None },
            last_updated: Instant::now(),
            client: Client::builder().build().unwrap(),
        }
    }

    pub async fn update_temperatures(&mut self) {
        if self.states[Sensor::Temperature as usize] && self.last_updated.elapsed() >= Duration::from_secs(5) {
            match self.client.get("http://10.0.0.50:3000/temps").send().await {
                Ok(resp) => {
                    if let Ok(data) = resp.json::<TempData>().await {
                        self.temp_data = data;
                    }
                    self.last_updated = Instant::now();
                }
                Err(e) => eprintln!("Failed to fetch temperatures: {e}"),
            }
        }
    }

    pub fn toggle_sensor(&mut self, index: usize) {
        self.states[index] = !self.states[index];
    }
}
