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
        resp.trim().parse::<f32>().ok()
    }

    pub fn process_sensors(&mut self) -> Vec<DeviceReading> {
        let mut readings = Vec::new();
        let mut moisture_on = false;

        let devices = [
            ("blackbeard-cpu", Command::GetProbeTemp, Some(CPU_THRESHOLD)),
            (
                "blackbeard-probe",
                Command::GetProbeTemp,
                Some(PROBE_THRESHOLD),
            ),
            ("moisture-1", Command::GetMoisture, Some(MOISTURE_THRESHOLD)),
            ("water-1", Command::OpenValve, None),
            ("relay-1", Command::TurnOnCooling, Some(CPU_THRESHOLD)),
            ("relay-2", Command::TurnOnRelay2, Some(PROBE_THRESHOLD)),
            ("relay-3", Command::TurnOnMoisture, Some(MOISTURE_THRESHOLD)),
            ("relay-4", Command::TurnOnHumidity, Some(50.0)),
        ];

        for (id, cmd, threshold) in devices.iter() {
            let value = match *id {
                "relay-3" | "water-1" => Some(rand::random::<f32>() * 100.0), // simulation
                _ => match serial::send_serial_command(self.port, cmd) {
                    Ok(resp) => Self::parse_float(&String::from_utf8_lossy(&resp)),
                    Err(e) => {
                        error!("Failed to read {}: {:?}", id, e);
                        None
                    }
                },
            };

            if let Some(v) = value {
                readings.push(DeviceReading {
                    id: id.to_string(),
                    value: Some(v),
                });

                match *id {
                    "relay-1" => {
                        let cmd_to_send = if v > CPU_THRESHOLD {
                            Command::TurnOnCooling
                        } else {
                            Command::TurnOffCooling
                        };
                        let _ = serial::send_serial_command(self.port, &cmd_to_send);
                    }
                    "relay-2" => {
                        let cmd_to_send = if v > PROBE_THRESHOLD {
                            Command::TurnOnRelay2
                        } else {
                            Command::TurnOffRelay2
                        };
                        let _ = serial::send_serial_command(self.port, &cmd_to_send);
                    }
                    "relay-3" => {
                        moisture_on = v > MOISTURE_THRESHOLD;
                        let cmd_to_send = if moisture_on {
                            Command::TurnOnMoisture
                        } else {
                            Command::TurnOffMoisture
                        };
                        let _ = serial::send_serial_command(self.port, &cmd_to_send);
                    }
                    "relay-4" => {
                        let cmd_to_send = if v > 50.0 {
                            Command::TurnOnHumidity
                        } else {
                            Command::TurnOffHumidity
                        };
                        let _ = serial::send_serial_command(self.port, &cmd_to_send);
                    }
                    _ => {}
                }
            } else {
                error!("Failed to parse value for {}", id);
            }
        }

        readings
    }
}
