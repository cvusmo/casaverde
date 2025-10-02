// Copyright 2025 Acris Software Ltd. Co. All Rights Reserved.
// github.com/cvusmo/casaverde/casaverde_controller
// src/serial.rs

use crate::controller::Command;
use crate::models::DeviceReading;
use log::info;
use serialport;

pub fn init_serial(
    config: &crate::config::Config,
) -> Result<Box<dyn serialport::SerialPort>, Box<dyn std::error::Error>> {
    let port_name = config.serial_port.as_ref().ok_or_else(|| {
        log::error!("Serial port not configured in config.toml");
        std::io::Error::new(std::io::ErrorKind::NotFound, "Serial port not found")
    })?;
    let port = serialport::new(port_name, 9600)
        .timeout(std::time::Duration::from_millis(1000))
        .open()
        .map_err(|e| {
            log::error!("Failed to open serial port {port_name}: {e}");
            e
        })?;
    info!("Serial port {port_name} initialized at 9600 baud");
    Ok(port)
}

pub fn send_command(
    port: &mut dyn serialport::SerialPort,
    cmd: &Command,
) -> Result<(), Box<dyn std::error::Error>> {
    match cmd {
        Command::TurnOnCooling(id) => {
            let command = format!("SET {id} ON\n");
            port.write_all(command.as_bytes())?;
            info!("Sent command to turn ON relay {id}");
        }
        Command::TurnOffCooling(id) => {
            let command = format!("SET {id} OFF\n");
            port.write_all(command.as_bytes())?;
            info!("Sent command to turn OFF relay {id}");
        }
        Command::TurnOnMoisture(id) => {
            let command = format!("SET {id} ON\n");
            port.write_all(command.as_bytes())?;
            info!("Sent command TurnOnMoisture to relay {id}");
        }
        Command::TurnOffMoisture(id) => {
            let command = format!("SET {id} OFF\n");
            port.write_all(command.as_bytes())?;
            info!("Sent command TurnOffMoisture to relay {id}");
        }
        Command::OpenValve(id) => {
            let command = format!("SET {id} ON\n");
            port.write_all(command.as_bytes())?;
            info!("Sent command OpenValve to relay {id}");
        }
        Command::CloseValve(id) => {
            let command = format!("SET {id} OFF\n");
            port.write_all(command.as_bytes())?;
            info!("Sent command CloseValve on relay {id}");
        }
        Command::TurnOnSolar(id) => {
            let command = format!("SET {id} ON\n");
            port.write_all(command.as_bytes())?;
            info!("Sent command TurnOnSolar on relay {id}");
        }
        Command::TurnOffSolar(id) => {
            let command = format!("SET {id} OFF\n");
            port.write_all(command.as_bytes())?;
            info!("Sent command TurnOffSolar on relay {id}");
        }
        Command::TurnOnHumidity(id) => {
            let command = format!("SET {id} ON\n");
            port.write_all(command.as_bytes())?;
            info!("Sent command TurnOnHumidity on relay {id}");
        }
        Command::TurnOffHumidity(id) => {
            let command = format!("SET {id} OFF\n");
            port.write_all(command.as_bytes())?;
            info!("Sent command TurnOffHumidity on relay {id}");
        }
        Command::SetPWM(id, pwm) => {
            let command = format!("PWM_{id}_{pwm}\n");
            port.write_all(command.as_bytes())?;
            info!("Sent command SetPWM {pwm} on relay {id}");
        }
    }
    Ok(())
}

pub fn read_sensor_data(port: &mut dyn serialport::SerialPort) -> Option<Vec<DeviceReading>> {
    let mut buffer = [0u8; 128];
    port.set_timeout(std::time::Duration::from_millis(500))
        .ok()?;
    match port.read(&mut buffer) {
        Ok(n) if n > 0 => {
            let response = String::from_utf8_lossy(&buffer[..n]).trim().to_string();
            info!("Received from simulator: {}", response);
            let mut readings = Vec::new();
            if response.starts_with("TEMP:") {
                if let Ok(value) = response[5..].parse::<f32>() {
                    readings.push(DeviceReading {
                        id: "blackbeard-cpu".to_string(),
                        value: Some(value),
                    });
                }
            } else if response.starts_with("SOLAR:") {
                if let Ok(value) = response[6..].parse::<f32>() {
                    readings.push(DeviceReading {
                        id: "solar-1".to_string(),
                        value: Some(value),
                    });
                }
            } else if response.starts_with("MOISTURE:") {
                if let Ok(value) = response[9..].parse::<f32>() {
                    readings.push(DeviceReading {
                        id: "moisture-1".to_string(),
                        value: Some(value),
                    });
                }
            } else if response.starts_with("HUMIDITY:") {
                if let Ok(value) = response[9..].parse::<f32>() {
                    readings.push(DeviceReading {
                        id: "humidity-1".to_string(),
                        value: Some(value),
                    });
                }
            } else if response.starts_with("WATER:") {
                if let Ok(value) = response[6..].parse::<f32>() {
                    readings.push(DeviceReading {
                        id: "water-1".to_string(),
                        value: Some(value),
                    });
                }
            } else if response.starts_with("RELAY:") {
                let value = if response == "RELAY:OK" { 1.0 } else { 0.0 }; // Simplified
                readings.push(DeviceReading {
                    id: "relay-1".to_string(),
                    value: Some(value),
                });
            }
            Some(readings)
        }
        _ => None,
    }
}
