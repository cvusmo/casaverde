// Copyright 2025 Acris Software Ltd. Co. All Rights Reserved.
// github.com/cvusmo/casaverde/casaverde_controller
// src/serial.rs

use crate::controller::Command;
use crate::models::DeviceReading;
use log::info;
use serialport;

pub fn init_serial(
    config: &crate::config::Config,
) -> Result<Box<dyn serialport::SerialPort>, serialport::Error> {
    let port_name = config.serial_port.as_ref().ok_or_else(|| {
        log::error!("Serial port not configured in config.toml");
        serialport::Error::new(serialport::ErrorKind::NoDevice, "Serial port not found")
    })?;
    serialport::new(port_name, 9600)
        .timeout(std::time::Duration::from_millis(1000))
        .open()
        .map(|port| {
            info!("Serial port {port_name} initialized at 9600 baud");
            port
        })
}

pub fn send_command(
    port: &mut dyn serialport::SerialPort,
    cmd: &Command,
) -> Result<(), serialport::Error> {
    let (command, msg) = match cmd {
        Command::TurnOnCooling(id) => (format!("SET {id} ON\n"), "turn ON"),
        Command::TurnOffCooling(id) => (format!("SET {id} OFF\n"), "turn OFF"),
        Command::TurnOnMoisture(id) => (format!("SET {id} ON\n"), "TurnOnMoisture"),
        Command::TurnOffMoisture(id) => (format!("SET {id} OFF\n"), "TurnOffMoisture"),
        Command::OpenValve(id) => (format!("SET {id} ON\n"), "OpenValve"),
        Command::CloseValve(id) => (format!("SET {id} OFF\n"), "CloseValve"),
        Command::TurnOnSolar(id) => (format!("SET {id} ON\n"), "TurnOnSolar"),
        Command::TurnOffSolar(id) => (format!("SET {id} OFF\n"), "TurnOffSolar"),
        Command::TurnOnHumidity(id) => (format!("SET {id} ON\n"), "TurnOnHumidity"),
        Command::TurnOffHumidity(id) => (format!("SET {id} OFF\n"), "TurnOffHumidity"),
        Command::SetPWM(id, pwm) => (format!("PWM_{id}_{pwm}\n"), "SetPWM"),
    };
    port.write_all(command.as_bytes())?;
    info!("Sent command {msg} on relay {}", cmd.id());
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
    match_sensor!("SOLAR:", "solar-1", readings);
    match_sensor!("MOISTURE:", "moisture-1", readings);
    match_sensor!("HUMIDITY:", "humidity-1", readings);
    match_sensor!("WATER:", "water-1", readings);
    if response.starts_with("RELAY:") {
        readings.push(DeviceReading {
            id: "relay-1".to_string(),
            value: Some(if response == "RELAY:OK" { 1.0 } else { 0.0 }),
        });
    }
}
