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
        Command::TurnOnRelay2 => "SET RELAY2 ON\n",
        Command::TurnOffRelay2 => "SET RELAY2 OFF\n",
    };

    port.write_all(command.as_bytes())?;
    let mut buffer = [0u8; 1024];
    let n = port.read(&mut buffer)?;
    Ok(buffer[..n].to_vec())
}

/// Fully data-driven, DOD/DOP compliant sensor reading
pub fn read_sensor_data(port: &mut dyn serialport::SerialPort) -> Option<Vec<DeviceReading>> {
    let mut readings = Vec::new();

    // Device table
    struct Device<'a> {
        cmd: Command,
        prefix: &'a str,
        id: &'a str,
        parser: fn(&str) -> Option<f32>,
    }

    // Parsers
    fn float_parser(resp: &str) -> Option<f32> {
        resp.trim().parse::<f32>().ok()
    }

    fn binary_parser(on: &str, off: &str, resp: &str) -> Option<f32> {
        if resp == on {
            Some(1.0)
        } else if resp == off {
            Some(0.0)
        } else {
            None
        }
    }

    // Define all devices in a single table
    let devices: &[Device] = &[
        // Sensors
        Device {
            cmd: Command::GetProbeTemp,
            prefix: "TEMP_PROBE:",
            id: "blackbeard-probe",
            parser: float_parser,
        },
        Device {
            cmd: Command::GetMoisture,
            prefix: "MOISTURE:",
            id: "moisture-1",
            parser: float_parser,
        },
        Device {
            cmd: Command::GetHumidity,
            prefix: "HUMIDITY:",
            id: "humidity-1",
            parser: float_parser,
        },
        Device {
            cmd: Command::GetSolar,
            prefix: "SOLAR:",
            id: "solar-1",
            parser: float_parser,
        },
        Device {
            cmd: Command::GetWater,
            prefix: "WATER:",
            id: "water-1",
            parser: float_parser,
        },
        // Relays / fans
        Device {
            cmd: Command::TurnOnRelay2,
            prefix: "RELAY2:",
            id: "relay-2",
            parser: |s| binary_parser("RELAY2:ON", "RELAY2:OFF", s),
        },
        Device {
            cmd: Command::TurnOnCooling,
            prefix: "FAN:",
            id: "FAN1",
            parser: |s| binary_parser("FAN:ON", "FAN:OFF", s),
        },
        Device {
            cmd: Command::TurnOnMoisture,
            prefix: "RELAY:",
            id: "relay-1",
            parser: |s| binary_parser("RELAY:ON", "RELAY:OFF", s),
        },
    ];

    // Process each device
    for device in devices {
        match send_serial_command(port, &device.cmd) {
            Ok(response) => {
                let line = String::from_utf8_lossy(&response).trim().to_string();
                info!("Processing response: {}", line);

                if line.starts_with(device.prefix) {
                    match (device.parser)(&line) {
                        Some(value) => readings.push(DeviceReading {
                            id: device.id.to_string(),
                            value: Some(value),
                        }),
                        None => error!("Failed to parse value from response: {}", line),
                    }
                } else {
                    error!(
                        "Unexpected response prefix: {} (expected {})",
                        line, device.prefix
                    );
                }
            }
            Err(e) => error!(
                "Failed to read sensor data for command {:?}: {}",
                device.cmd, e
            ),
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
