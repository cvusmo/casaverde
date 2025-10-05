// Copyright 2025 Acris Software Ltd. Co. All Rights Reserved.
// github.com/cvusmo/casaverde/casaverde_controller
// src/serial.rs

use crate::controller::Command;
use crate::models::DeviceReading;
use log::{error, info};
use serialport;

pub fn init_serial(
    config: &crate::config::Config,
) -> Result<Box<dyn serialport::SerialPort>, serialport::Error> {
    let port_name = config.serial_port.as_ref().ok_or_else(|| {
        log::error!("Serial port not configured in config.toml");
        serialport::Error::new(serialport::ErrorKind::NoDevice, "Serial port not found")
    })?;
    let port = serialport::new(port_name, 9600)
        .timeout(std::time::Duration::from_millis(1000))
        .open();
    match port {
        Ok(p) => {
            info!("Serial port {} initialized at 9600 baud", port_name);
            Ok(p)
        }
        Err(e) => {
            error!("Failed to open serial port {}: {:?}", port_name, e);
            Err(e)
        }
    }
}

pub fn send_command(
    port: &mut dyn serialport::SerialPort,
    cmd: &Command,
) -> Result<(), serialport::Error> {
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
        Command::SetPWM(pwm) => &format!("SET FAN1 PWM_{pwm}\n"),

        Command::GetProbeTemp => "GET blackbeard-probe\n",
        Command::TurnOnRelay2 => "ON_INT2\n",
        Command::TurnOffRelay2 => "OFF_INT2\n",
    };
    port.write_all(command.as_bytes())?;
    info!("Sent command on device {}", cmd.id());
    Ok(())
}

pub fn read_sensor_data(port: &mut dyn serialport::SerialPort) -> Option<Vec<DeviceReading>> {
    let mut buffer = Vec::new();
    let mut temp = [0u8; 1];
    let mut readings = Vec::new();
    while port.read(&mut temp).ok()? > 0 {
        buffer.push(temp[0]);
        if temp[0] == b'\n' {
            let line = String::from_utf8_lossy(&buffer).trim().to_string();
            match_response(&line, &mut readings);
            buffer.clear();
        }
    }
    if !readings.is_empty() {
        Some(readings)
    } else {
        None
    }
}

fn match_response(response: &str, readings: &mut Vec<DeviceReading>) {
    macro_rules! match_sensor {
        ($prefix:expr, $id:expr, $readings:expr) => {
            if let Some(value) = response
                .strip_prefix($prefix)
                .and_then(|v| v.parse::<f32>().ok())
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
    }
}
