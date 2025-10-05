// Copyright 2025 Acris Software Ltd. Co. All Rights Reserved.
// github.com/cvusmo/casaverde/casaverde_controller
// src/serial.rs

use crate::controller::Command;
use crate::models::DeviceReading;
use log::{error, info};
use serialport::{ClearBuffer, DataBits, Parity, StopBits};
use std::time::Duration;

pub fn init_serial(
    config: &crate::config::Config,
) -> Result<Box<dyn serialport::SerialPort>, serialport::Error> {
    let port_name = config.serial_port.as_ref().ok_or_else(|| {
        error!("Serial port not configured in config.toml");
        serialport::Error::new(serialport::ErrorKind::NoDevice, "Serial port not found")
    })?;
    let port = serialport::new(port_name, 9600)
        .timeout(Duration::from_secs(5))
        .data_bits(DataBits::Eight)
        .parity(Parity::None)
        .stop_bits(StopBits::One)
        .open();
    match port {
        Ok(p) => {
            p.clear(ClearBuffer::Input)?;
            p.clear(ClearBuffer::Output)?;
            info!("Serial port {} initialized at 9600 baud", port_name);
            Ok(p)
        }
        Err(e) => {
            error!("Failed to open serial port {}: {:?}", port_name, e);
            Err(e)
        }
    }
}

pub fn send_serial_command(
    port: &mut dyn serialport::SerialPort,
    cmd: &Command,
) -> Result<Vec<u8>, serialport::Error> {
    // Map command enum to string
    let command = match cmd {
        Command::TurnOnCooling => "SET FAN1 ON\n",
        Command::TurnOffCooling => "SET FAN1 OFF\n",
        Command::TurnOnMoisture => "SET moisture-1 ON\n",
        Command::TurnOffMoisture => "SET moisture-1 OFF\n",
        Command::OpenValve => "SET water-1 OPEN\n",
        Command::CloseValve => "SET water-1 CLOSE\n",
        Command::TurnOnSolar => "SET solar-1 ON\n",
        Command::TurnOffSolar => "SET solar-1 OFF\n",
        Command::TurnOnHumidity => "SET humidity-1 ON\n",
        Command::TurnOffHumidity => "SET humidity-1 OFF\n",
        Command::SetPWM(pwm) => Box::leak(format!("SET FAN1 PWM_{}\n", pwm).into_boxed_str()),
        Command::GetProbeTemp => "GET blackbeard-probe\n",
        Command::GetMoisture => "GET moisture-1\n",
        Command::GetHumidity => "GET humidity-1\n",
        Command::GetSolar => "GET solar-1\n",
        Command::GetWater => "GET water-1\n",
        Command::TurnOnRelay2 => "ON_INT2\n",
        Command::TurnOffRelay2 => "OFF_INT2\n",
    };

    // Send command
    port.write_all(command.as_bytes())?;

    // Read response (up to 1024 bytes)
    let mut buffer = [0u8; 1024];
    let n = port.read(&mut buffer)?;

    Ok(buffer[..n].to_vec())
}

pub fn read_sensor_data(port: &mut dyn serialport::SerialPort) -> Option<Vec<DeviceReading>> {
    let mut readings = Vec::new();
    let sensor_commands = vec![
        Command::GetProbeTemp,
        Command::GetMoisture,
        Command::GetHumidity,
        Command::GetSolar,
        Command::GetWater,
    ];

    for cmd in sensor_commands {
        match send_serial_command(port, &cmd) {
            Ok(response) => {
                let line = String::from_utf8_lossy(&response).trim().to_string();
                info!("Processing response: {}", line);
                match_response(&line, &mut readings);
            }
            Err(e) => {
                error!("Failed to read sensor data for command {:?}: {}", cmd, e);
            }
        }
    }

    if !readings.is_empty() {
        info!("Collected readings: {:?}", readings);
        Some(readings)
    } else {
        error!("No valid sensor readings collected");
        None
    }
}

fn match_response(response: &str, readings: &mut Vec<DeviceReading>) {
    macro_rules! match_sensor {
        ($prefix:expr, $id:expr, $readings:expr) => {
            if let Some(value) = response
                .strip_prefix($prefix)
                .and_then(|v| v.trim().parse::<f32>().ok())
            {
                $readings.push(DeviceReading {
                    id: $id.to_string(),
                    value: Some(value),
                });
                return;
            }
        };
    }
    match_sensor!("TEMP:", "blackbeard-cpu", readings);
    match_sensor!("TEMP_PROBE:", "blackbeard-probe", readings);
    match_sensor!("SOLAR:", "solar-1", readings);
    match_sensor!("MOISTURE:", "moisture-1", readings);
    match_sensor!("HUMIDITY:", "humidity-1", readings);
    match_sensor!("WATER:", "water-1", readings);
    if response.starts_with("RELAY:") {
        readings.push(DeviceReading {
            id: "relay-1".to_string(),
            value: Some(if response == "RELAY:ON" { 1.0 } else { 0.0 }),
        });
    } else if response.starts_with("FAN:") {
        readings.push(DeviceReading {
            id: "FAN1".to_string(),
            value: Some(if response == "FAN:ON" { 1.0 } else { 0.0 }),
        });
    } else {
        error!("Unrecognized response format: {}", response);
    }
}
