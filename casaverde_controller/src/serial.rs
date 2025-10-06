// src/serial.rs
use crate::controller::Command;
use casaverde_utils::log::{error, info};
use serialport::{ClearBuffer, DataBits, Parity, StopBits};
use std::time::Duration;

pub fn init_serial(port_name: &str) -> Result<Box<dyn serialport::SerialPort>, serialport::Error> {
    let port = serialport::new(port_name, 9600)
        .timeout(Duration::from_secs(5))
        .data_bits(DataBits::Eight)
        .parity(Parity::None)
        .stop_bits(StopBits::One)
        .open();

    match port {
        Ok(mut p) => {
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
    let command_str = match cmd {
        Command::TurnOnCooling => "SET FAN1 ON\n",
        Command::TurnOffCooling => "SET FAN1 OFF\n",
        Command::TurnOnMoisture => "SET moisture-1 ON\n",
        Command::TurnOffMoisture => "SET moisture-1 OFF\n",
        Command::OpenValve => "SET water-1 OPEN\n",
        Command::CloseValve => "SET water-1 CLOSE\n",
        Command::TurnOnRelay2 => "SET RELAY2 ON\n",
        Command::TurnOffRelay2 => "SET RELAY2 OFF\n",
        Command::TurnOnSolar => "SET solar-1 ON\n",
        Command::TurnOffSolar => "SET solar-1 OFF\n",
        Command::TurnOnHumidity => "SET humidity-1 ON\n",
        Command::TurnOffHumidity => "SET humidity-1 OFF\n",
        Command::GetProbeTemp => "GET blackbeard-probe\n",
        Command::GetMoisture => "GET moisture-1\n",
        Command::GetWater => "GET water-1\n",
        Command::GetHumidity => "GET humidity-1\n",
        Command::GetSolar => "GET solar-1\n",
        Command::SetPWM(pwm) => Box::leak(format!("SET FAN1 PWM_{}\n", pwm).into_boxed_str()),
        _ => {
            return Err(serialport::Error::new(
                serialport::ErrorKind::InvalidInput,
                "Unknown command",
            ))
        }
    };

    port.write_all(command_str.as_bytes())?;
    let mut buffer = [0u8; 1024];
    let n = port.read(&mut buffer)?;
    Ok(buffer[..n].to_vec())
}
