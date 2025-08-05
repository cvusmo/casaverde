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

#[derive(Debug, Serialize, Deserialize)]
pub enum Command {
    TurnOnCooling(String),
    TurnOffCooling(String),
}

// Process local DS18B20 temperature readings
//pub fn process_local_readings(local_temp: Option<f32>, controller_id: &str) -> Vec<Command> {
//let mut commands = Vec::new();

//if let Some(temp) = local_temp {
//let local_device_id = "local_sensor".to_string();
//if temp > 40.0 {
//info!(
//"Local temperature above 40°C for {local_device_id} on {controller_id}: {temp}. Turning on INT1 (Red LED)."
//);
//commands.push(Command::TurnOnCooling("INT1".to_string()));
//commands.push(Command::TurnOffCooling("INT2".to_string()));
//} else if temp < 39.0 {
//info!(
//"Local temperature below 39°C for {local_device_id} on {controller_id}: {temp}. Turning on INT2 (Blue LED)."
//);
//commands.push(Command::TurnOffCooling("INT1".to_string()));
//commands.push(Command::TurnOnCooling("INT2".to_string()));
//} else {
//info!(
//"Local temperature stable for {local_device_id} on {controller_id}: {temp}. Both OFF."
//);
//commands.push(Command::TurnOffCooling("INT1".to_string()));
//commands.push(Command::TurnOffCooling("INT2".to_string()));
//}
//} //else {
// Testing mode since no sensor is wired
//info!("No temperature sensor detected for {controller_id}. Testing mode:");
//static mut CYCLE: bool = false;
//unsafe {
//if CYCLE {
//info!("TEST ON - Simulating INT1 (Red LED) on");
//commands.push(Command::TurnOnCooling("INT1".to_string()));
//commands.push(Command::TurnOffCooling("INT2".to_string()));
//} else {
//info!("TEST ON - Simulating INT2 (Blue LED) on");
//commands.push(Command::TurnOffCooling("INT1".to_string()));
//commands.push(Command::TurnOnCooling("INT2".to_string()));
//}
//CYCLE = !CYCLE;
//}
//}

//commands
//}

// Process remote server temperature readings
pub fn process_remote_readings(readings: &[CachedData], controller_id: &str) -> Vec<Command> {
    let mut commands = Vec::new();

    for reading in readings {
        for device in &reading.devices {
            if let Some(temp) = device.value {
                info!(
                    "Temperature {}°C for {} on {}: {}",
                    temp,
                    device.id,
                    reading.client_id,
                    if temp > 50.0 {
                        "Cooling on (Blue LED)"
                    } else {
                        "Cooling off (Red LED)"
                    }
                );

                if device.id == "blackbeard-cpu" {
                    if temp > 50.0 {
                        commands.push(Command::TurnOffCooling("INT1".to_string())); // Red LED off
                        commands.push(Command::TurnOnCooling("INT2".to_string()));
                    // Blue LED on
                    } else {
                        commands.push(Command::TurnOnCooling("INT1".to_string())); // Red LED on
                        commands.push(Command::TurnOffCooling("INT2".to_string()));
                        // Blue LED off
                    }
                }
            }
        }
    }
    commands
}
