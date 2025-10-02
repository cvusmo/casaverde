// Copyright 2025 Acris Software Ltd. Co. All Rights Reserved.
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
            log::error!("Failed to open serial port {}: {}", port_name, e);
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
            let command = format!("ON_{}\n", id);
            port.write_all(command.as_bytes())?;
            info!(
                "Sent command to turn ON relay {} (e.g., INT1 for CPU cooling)",
                id
            );
        }
        Command::TurnOffCooling(id) => {
            let command = format!("OFF_{}\n", id);
            port.write_all(command.as_bytes())?;
            info!(
                "Sent command to turn OFF relay {} (e.g., INT2 for GPU cooling)",
                id
            );
        }
        Command::OpenValve(id) => {
            let command = format!("OPEN_{}\n", id);
            port.write_all(command.as_bytes())?;
            info!("Sent command to open valve on relay {}", id);
        }
        Command::CloseValve(id) => {
            let command = format!("CLOSE_{}\n", id);
            port.write_all(command.as_bytes())?;
            info!("Sent command to close valve on relay {}", id);
        }
        Command::TurnOnLight(id) => {
            let command = format!("ON_{}\n", id);
            port.write_all(command.as_bytes())?;
            info!("Sent command to turn ON light relay {} (e.g., INT3)", id);
        }
        Command::TurnOffLight(id) => {
            let command = format!("OFF_{}\n", id);
            port.write_all(command.as_bytes())?;
            info!("Sent command to turn OFF light relay {} (e.g., INT4)", id);
        }
    }
    Ok(())
}
