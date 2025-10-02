// Copyright 2025 Acris Software Ltd. Co. All Rights Reserved.
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

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Command {
    TurnOnCooling(String),  // Red LED turn on/off CPU cooling
    TurnOffCooling(String), // Green LED turn on/off GPU cooling
    OpenValve(String),      // Blue LED
    CloseValve(String),     // Yellow LED reverse polarity
    TurnOnLight(String),    // Red
    TurnOffLight(String),   // Orange
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

// FIX: Solenoid Valve needs to be a mutable device
pub fn process_remote_readings(readings: &[CachedData], controller_id: &str) -> Vec<Command> {
    let mut commands = Vec::new();
    let mut cpu_temp = None;
    let mut gpu_temp = None;

    for reading in readings {
        for device in &reading.devices {
            if let Some(temp) = device.value {
                if device.id == "blackbeard-cpu" {
                    cpu_temp = Some(cpu_temp.unwrap_or(f32::MIN).max(temp));
                    //cpu_temp = Some(temp);
                    info!(
                        "Temperature {}°C for {} on {}: {}",
                        temp,
                        device.id,
                        reading.client_id,
                        if temp > 40.0 {
                            "Cooling on (Red LED)"
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
                        if temp > 40.0 {
                            "Cooling on (Blue LED)"
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
            if cpu > 40.0 {
                commands.push(Command::TurnOnCooling("INT1".to_string())); // Red ON
                commands.push(Command::OpenValve("VALVE1".to_string())); // Yellow ON
                info!("Cooling activate: solenoid valve opened.");
            } else if cpu <= 40.0 {
                commands.push(Command::TurnOffCooling("INT1".to_string())); // Red off
                commands.push(Command::CloseValve("VALVE2".to_string())); // Green ON
                info!("Cooling deactivated: solenoid valve closed.");
            }

            if gpu > 40.0 {
                commands.push(Command::TurnOnCooling("INT2".to_string()));
                info!("GPU temp above 40");
            } else {
                commands.push(Command::TurnOffCooling("INT2".to_string()));
                info!("GPU temp below 40");
            }
        }
        _ => {
            info!("Missing temperature data for blackbeard-cpu or blackbeard-gpu");
            commands.push(Command::TurnOffCooling("INT1".to_string())); // Red off
            commands.push(Command::TurnOffCooling("INT2".to_string())); // Blue off
            commands.push(Command::CloseValve("VALVE2".to_string())); // Green ON
            info!("No valid data: Closing solenoid valve");
        }
    }

    commands
}
