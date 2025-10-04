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
        Command::TurnOnMoisture => "GET moisture-1\n",
        Command::TurnOffMoisture => "GET moisture-1\n",
        Command::OpenValve => "GET water-1\n",
        Command::CloseValve => "GET water-1\n",
        Command::TurnOnSolar => "GET solar-1\n",
        Command::TurnOffSolar => "GET solar-1\n",
        Command::TurnOnHumidity => "GET humidity-1\n",
        Command::TurnOffHumidity => "GET humidity-1\n",
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
    let mut buffer = [0u8; 128];
    if port
        .set_timeout(std::time::Duration::from_millis(500))
        .is_ok()
    {
        if let Ok(n) = port.read(&mut buffer) {
            if n > 0 {
                let response = String::from_utf8_lossy(&buffer[..n]).trim().to_string();
                info!("Received from simulator: {}", response);
                let mut readings = Vec::new();
                match_response(&response, &mut readings);
                return Some(readings);
            }
        }
    }
    None
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
