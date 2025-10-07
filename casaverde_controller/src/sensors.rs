// Copyright 2025 Acris Software Ltd. Co. All Rights Reserved.
// github.com/cvusmo/casaverde/casaverde_controller
// src/sensors.rs

use crate::controller::Command;
use crate::models::DeviceReading;
use crate::serial;
use casaverde_utils::log::{error, info};

const PROBE_THRESHOLD: f32 = 24.0;
const CPU_THRESHOLD: f32 = 40.0;
const MOISTURE_THRESHOLD: f32 = 25.0;

pub struct SensorController<'a> {
    port: &'a mut dyn serialport::SerialPort,
}

impl<'a> SensorController<'a> {
    pub fn new(port: &'a mut dyn serialport::SerialPort) -> Self {
        Self { port }
    }

    fn parse_float(resp: &str) -> Option<f32> {
        resp.trim()
            .split(':')
            .nth(1)
            .and_then(|s| s.trim().parse::<f32>().ok())
    }

    pub fn process_sensors(&mut self) -> Vec<DeviceReading> {
        let mut readings = Vec::new();
        let devices = [
            (
                "blackbeard-probe",
                Command::GetProbeTemp,
                Some(PROBE_THRESHOLD),
            ),
            ("moisture-1", Command::GetMoisture, Some(MOISTURE_THRESHOLD)),
            ("nutrients-1", Command::GetMoisture, None),
            ("humidity-1", Command::GetHumidity, Some(50.0)),
            ("solar-1", Command::GetSolar, Some(100.0)),
            ("water-1", Command::GetWater, None),
            ("relay-1", Command::TurnOnCooling, Some(CPU_THRESHOLD)),
            ("relay-2", Command::TurnOnRelay2, Some(PROBE_THRESHOLD)),
            ("relay-3", Command::TurnOnMoisture, Some(MOISTURE_THRESHOLD)),
            ("relay-4", Command::TurnOnHumidity, Some(50.0)),
        ];

        for (id, cmd, _threshold) in devices.iter() {
            // Fixed unused
            let value = match serial::send_serial_command(self.port, cmd) {
                Ok(resp) => {
                    let resp_str = String::from_utf8_lossy(&resp).to_string();
                    info!("Received response for {}: {}", id, resp_str);
                    Self::parse_float(&resp_str)
                }
                Err(e) => {
                    error!("Failed to read {}: {:?}", id, e);
                    None
                }
            };

            if let Some(v) = value {
                readings.push(DeviceReading {
                    id: id.to_string(),
                    value: Some(v),
                });

                let cmd_to_send = match *id {
                    "relay-1" if v > CPU_THRESHOLD => Command::TurnOnCooling,
                    "relay-1" => Command::TurnOffCooling,
                    "relay-2" if v > PROBE_THRESHOLD => Command::TurnOnRelay2,
                    "relay-2" => Command::TurnOffRelay2,
                    "relay-3" if v > MOISTURE_THRESHOLD => Command::TurnOnMoisture,
                    "relay-3" => Command::TurnOffMoisture,
                    "relay-4" if v > 50.0 => Command::TurnOnHumidity,
                    "relay-4" => Command::TurnOffHumidity,
                    _ => continue,
                };
                if let Err(e) = serial::send_serial_command(self.port, &cmd_to_send) {
                    error!("Failed to send command for {}: {:?}", id, e);
                } else {
                    info!("Sent command for {}: {:?}", id, cmd_to_send);
                }
            } else {
                error!("Failed to parse value for {}", id);
            }
        }

        readings
    }
}
