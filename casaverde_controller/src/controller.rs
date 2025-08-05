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

pub fn process_remote_readings(readings: &[CachedData], controller_id: &str) -> Vec<Command> {
    let mut commands = Vec::new();
    let mut cpu_temp = None;
    let mut gpu_temp = None;

    for reading in readings {
        for device in &reading.devices {
            if let Some(temp) = device.value {
                if device.id == "blackbeard-cpu" {
                    cpu_temp = Some(temp);
                    info!(
                        "Temperature {}°C for {} on {}: {}",
                        temp,
                        device.id,
                        reading.client_id,
                        if temp > 50.0 {
                            "Cooling on (Blue LED)"
                        } else {
                            "Cooling off"
                        }
                    );
                } else if device.id == "blackbeard-gpu" {
                    gpu_temp = Some(temp);
                    info!(
                        "Temperature {}°C for {} on {}: {}",
                        temp,
                        device.id,
                        reading.client_id,
                        if temp > 41.0 {
                            "Cooling on (Red LED)"
                        } else {
                            "Cooling off"
                        }
                    );
                }
            }
        }
    }

    match (cpu_temp, gpu_temp) {
        (Some(cpu), Some(gpu)) => {
            if cpu > 50.0 {
                commands.push(Command::TurnOffCooling("INT1".to_string())); // Red LED off
                commands.push(Command::TurnOnCooling("INT2".to_string())); // Blue LED on
            }
            if gpu > 41.0 {
                commands.push(Command::TurnOnCooling("INT1".to_string())); // Red LED on
                commands.push(Command::TurnOffCooling("INT2".to_string())); // Blue LED off
            }
        }
        _ => {
            info!("Missing temperature data for blackbeard-cpu or blackbeard-gpu");
            commands.push(Command::TurnOffCooling("INT1".to_string())); // Default: Red off
            commands.push(Command::TurnOffCooling("INT2".to_string())); // Default: Blue off
        }
    }

    commands
}
