// Copyright 2025 Acris Software Ltd. Co. All Rights Reserved.
// github.com/cvusmo/casaverde/casaverde_controller
// src/controller.rs

use serde_json::Value;

#[derive(Debug, Clone)]
pub enum Command {
    TurnOnCooling,
    TurnOffCooling,
    TurnOnMoisture,
    TurnOffMoisture,
    OpenValve,
    CloseValve,
    TurnOnSolar,
    TurnOffSolar,
    TurnOnHumidity,
    TurnOffHumidity,
    SetPWM(u8),
    GetProbeTemp,
    GetMoisture,
    GetHumidity,
    GetSolar,
    GetWater,
    TurnOnRelay2,
    TurnOffRelay2,
}

impl Command {
    pub fn id(&self) -> &str {
        match self {
            Command::TurnOnCooling | Command::TurnOffCooling => "FAN1",
            Command::TurnOnMoisture | Command::TurnOffMoisture | Command::GetMoisture => {
                "moisture-1"
            }
            Command::OpenValve | Command::CloseValve | Command::GetWater => "water-1",
            Command::TurnOnSolar | Command::TurnOffSolar | Command::GetSolar => "solar-1",
            Command::TurnOnHumidity | Command::TurnOffHumidity | Command::GetHumidity => {
                "humidity-1"
            }
            Command::SetPWM(_) => "FAN1",
            Command::GetProbeTemp => "blackbeard-probe",
            Command::TurnOnRelay2 | Command::TurnOffRelay2 => "relay-2",
        }
    }
}

pub fn process_remote_readings(readings: &Value, controller_id: &str) -> Vec<Command> {
    readings
        .as_array()
        .map(|client_readings| {
            client_readings
                .iter()
                .filter_map(|client_reading| {
                    let arr: &[Value] = client_reading.as_array()?.as_slice();
                    let [id, devices] = arr else { return None };
                    (id.as_str() == Some(controller_id)).then(|| {
                        devices
                            .as_array()
                            .map(|devices_array| {
                                devices_array
                                    .iter()
                                    .filter_map(|device| {
                                        device.as_object().and_then(|obj| {
                                            let id = obj.get("id")?.as_str()?;
                                            let value = obj.get("value")?.as_f64()?;
                                            match id {
                                                "blackbeard-cpu" if value > 40.0 => {
                                                    Some(Command::TurnOnCooling)
                                                }
                                                "blackbeard_cpu" if value < 40.0 => {
                                                    Some(Command::TurnOffCooling)
                                                }
                                                "blackbeard-probe" if value > 15.0 => {
                                                    Some(Command::TurnOnRelay2)
                                                }
                                                "blackbeard-probe" if value <= 15.0 => {
                                                    Some(Command::TurnOffRelay2)
                                                }
                                                "moisture-1" if value < 30.0 => {
                                                    Some(Command::TurnOnMoisture)
                                                }
                                                "moisture-1" if value >= 50.0 => {
                                                    Some(Command::TurnOffMoisture)
                                                }
                                                "humidity-1" if value < 40.0 => {
                                                    Some(Command::TurnOnHumidity)
                                                }
                                                "humidity-1" if value >= 60.0 => {
                                                    Some(Command::TurnOffHumidity)
                                                }
                                                "solar-1" if value < 50.0 => {
                                                    Some(Command::TurnOnSolar)
                                                }
                                                "solar-1" if value >= 100.0 => {
                                                    Some(Command::TurnOffSolar)
                                                }
                                                "water-1" if value < 20.0 => {
                                                    Some(Command::OpenValve)
                                                }
                                                "water-1" if value >= 40.0 => {
                                                    Some(Command::CloseValve)
                                                }
                                                _ => None,
                                            }
                                        })
                                    })
                                    .collect::<Vec<_>>()
                            })
                            .unwrap_or_default()
                    })
                })
                .flatten()
                .collect::<Vec<_>>()
        })
        .unwrap_or_default()
}

pub fn process_local_rules(_config: &crate::config::Config, local_temp: f64) -> Vec<Command> {
    let mut commands = Vec::new();
    if local_temp > 40.0 {
        commands.push(Command::TurnOnCooling);
    } else if local_temp < 40.0 {
        commands.push(Command::TurnOffCooling);
    }
    // Periodic queries for all sensors
    commands.push(Command::GetProbeTemp);
    commands.push(Command::GetMoisture);
    commands.push(Command::GetHumidity);
    commands.push(Command::GetSolar);
    commands.push(Command::GetWater);
    commands
}
