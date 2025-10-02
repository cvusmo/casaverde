// Copyright 2025 Nicholas Jordan. All Rights Reserved.
// github.com/cvusmo/casaverde/casaverde_controller
// src/serial.rs

use crate::controller::Command;
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
            log::error!("Failed to open serial port {}: {e}", port_name);
            e
        })?;
    info!("Serial port {} initialized at 9600 baud", port_name);
    Ok(port)
}

pub fn send_command(
    port: &mut dyn serialport::SerialPort,
    cmd: &Command,
) -> Result<(), Box<dyn std::error::Error>> {
    match cmd {
        Command::TurnOnCooling(id) => {
            let command = format!("ON_{id}\n");
            port.write_all(command.as_bytes())?;
            info!("Sent: {}", command.trim());
        }
        Command::TurnOffCooling(id) => {
            let command = format!("OFF_{id}\n");
            port.write_all(command.as_bytes())?;
            info!("Sent: {}", command.trim());
        }
        Command::OpenValve(id) => {
            let command = format!("OPEN_{id}\n");
            port.write_all(command.as_bytes())?;
            info!("Sent: {}", command.trim());
        }
        Command::CloseValve(id) => {
            let command = format!("CLOSE_{id}\n");
            port.write_all(command.as_bytes())?;
            info!("Sent: {}", command.trim());
        }
        Command::TurnOnLight(id) => {
            let command = format!("ON_{id}\n");
            port.write_all(command.as_bytes())?;
            info!("Sent: {}", command.trim());
        }
        Command::TurnOffLight(id) => {
            let command = format!("OFF_{id}\n");
            port.write_all(command.as_bytes())?;
            info!("Sent: {}", command.trim());
        }
    }
    Ok(())
}
