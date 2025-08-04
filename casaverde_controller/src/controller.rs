// Copyright 2025 Nicholas Jordan. All Rights Reserved.
// github.com/cvusmo/casaverde/casaverde_controller
// src/controller.rs - Data processing and command generation

use log::info;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DeviceReading {
    pub id: String,
    pub value: Option<f32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CachedData {
    pub client_id: String,
    pub devices: Vec<DeviceReading>,
}

#[derive(Debug)]
pub enum Command {
    TurnOnCooling(String),
    TurnOffCooling(String),
}

// Process local DS18B20 temperature readings
pub fn process_local_readings(local_temp: Option<f32>, controller_id: &str) -> Vec<Command> {
    let mut commands = Vec::new();

    if let Some(temp) = local_temp {
        let local_device_id = "local_sensor".to_string();
        if temp > 45.0 {
            info!(
                "Local temperature above 45°C for {local_device_id} on {controller_id}: {temp}. Turn on cooling."
            );
            commands.push(Command::TurnOnCooling(local_device_id.clone()));
        } else if temp < 44.0 {
            info!(
                "Local temperature below 44°C for {local_device_id} on {controller_id}: {temp}. Turn off cooling."
            );
            commands.push(Command::TurnOffCooling(local_device_id));
        } else {
            info!("Local temperature stable for {local_device_id} on {controller_id}: {temp}.");
            // No command for stable state, but can be added if needed
        }
    }

    commands
}

// Process remote server temperature readings
pub fn process_remote_readings(readings: &[CachedData], _controller_id: &str) -> Vec<Command> {
    let mut commands = Vec::new();

    for data in readings {
        for device in &data.devices {
            if let Some(temp) = device.value {
                if temp > 50.0 {
                    info!(
                        "Temperature above 50°C for {} on {}: {}. Turn on cooling.",
                        device.id, data.client_id, temp
                    );
                    commands.push(Command::TurnOnCooling(device.id.clone()));
                } else {
                    info!(
                        "Temperature below 50°C for {} on {}: {}. Cooling off.",
                        device.id, data.client_id, temp
                    );
                    commands.push(Command::TurnOffCooling(device.id.clone()));
                    // TODO: Implement energy optimization (500 kWh/30 days)
                }
            }
        }
    }
    commands
}

// TODO: create algorithm to update process_reading based off cached values and analyzing the data
// this algorithm will be done by the server, not by the controller.
