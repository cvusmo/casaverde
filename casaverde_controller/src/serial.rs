// Copyright 2025 Nicholas Jordan. All Rights Reserved.
// github.com/cvusmo/casaverde/casaverde_controller
// src/serial.rs - USB serial communication with UNO R3

use crate::controller::Command;
use log::info;
use serialport;

pub fn init_serial(
    port_name: &str,
    baud_rate: u32,
) -> Result<Box<dyn serialport::SerialPort>, Box<dyn std::error::Error>> {
    let port = serialport::new(port_name, baud_rate)
        .timeout(std::time::Duration::from_millis(1000))
        .open()
        .map_err(|e| {
            log::error!("Failed to open serial port {port_name}: {e}");
            e
        })?;
    info!("Serial port {port_name} initialized at {baud_rate} baud");
    Ok(port)
}

pub fn send_command(
    port: &mut dyn serialport::SerialPort,
    cmd: Command,
) -> Result<(), Box<dyn std::error::Error>> {
    match cmd {
        Command::TurnOnCooling(id) => {
            let command = format!("COOL_ON_{id}\n");
            port.write_all(command.as_bytes())?;
            info!("Sent: {}", command.trim());
        }
        Command::TurnOffCooling(id) => {
            let command = format!("COOL_OFF_{id}\n");
            port.write_all(command.as_bytes())?;
            info!("Sent: {}", command.trim());
        }
    }
    Ok(())
}
