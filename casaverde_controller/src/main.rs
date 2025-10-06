// Copyright 2025 Acris Software Ltd. Co. All Rights Reserved.
// github.com/cvusmo/casaverde/casaverde_controller
use casaverde_controller::timer::run_light_timer;
use casaverde_controller::{client, models};
use casaverde_utils::dirs::get_home_dir;
use casaverde_utils::fs::{read_to_string, write_all};
use casaverde_utils::io::{new_error, IoError, IoErrorKind};
use casaverde_utils::log::{error, info, LevelFilter};
use casaverde_utils::init_logger;
use std::sync::Arc;
use tokio::{spawn, sync::mpsc, time::interval};
use casaverde_controller::config;
use casaverde_controller::controller::{process_local_rules, process_remote_readings, Command};
use casaverde_controller::gpio;
use casaverde_controller::serial::{init_serial, send_serial_command};
use casaverde_controller::sensors::SensorController;
use toml::Value;
use std::fs::File;

#[tokio::main]
async fn main() -> Result<(), IoError> {
    let mut config_path = get_home_dir()
        .map_err(|e| new_error(IoErrorKind::NotFound, format!("Home directory error: {}", e)))?;
    config_path.push("casaverde/casaverde_controller/config.toml");
    let config_str = read_to_string(&config_path)
        .map_err(|e| new_error(IoErrorKind::Other, format!("Failed to read config.toml: {}", e)))?;
    let config_toml: Value = toml::from_str(&config_str)
        .map_err(|e| new_error(IoErrorKind::Other, format!("Failed to parse config.toml: {}", e)))?;
    let log_level = config_toml.get("logging").and_then(|l| l.get("level")).and_then(|l| l.as_str())
        .map(|s| match s.to_lowercase().as_str() {
            "error" => LevelFilter::Error,
            "warn" => LevelFilter::Warn,
            "info" => LevelFilter::Info,
            "debug" => LevelFilter::Debug,
            "trace" => LevelFilter::Trace,
            _ => LevelFilter::Info,
        })
        .unwrap_or(LevelFilter::Info);
    init_logger("casaverde_controller", log_level)?;
    info!("Starting casaverde_controller on {}", config::get_hostname());

    let mut config = config::load_config()?;
    let client = client::build_secure_client()
        .map_err(|e| new_error(IoErrorKind::Other, format!("Failed to build secure client: {}", e)))?;
    let port: Arc<tokio::sync::Mutex<Box<dyn serialport::SerialPort>>> =
        Arc::new(tokio::sync::Mutex::new(init_serial(&config.serial_port.clone().unwrap_or_default())
            .map_err(|e| new_error(IoErrorKind::Other, format!("Failed to initialize serial: {}", e)))?));

    if let Ok(server_config) = client::fetch_config(&client, &config.server, &config.controller_id).await {
        config = server_config;
        let config_str = toml::to_string(&config)
            .map_err(|e| new_error(IoErrorKind::Other, format!("Failed to serialize config: {}", e)))?;
        let mut file = File::create(&config_path)
            .map_err(|e| new_error(IoErrorKind::Other, format!("Failed to create config.toml: {}", e)))?;
        write_all(&mut file, config_str.as_bytes())
            .map_err(|e| new_error(IoErrorKind::Other, format!("Failed to write config.toml: {}", e)))?;
        info!("Updated local config from server: {:?}", config);
    } else {
        info!("Failed to fetch config from server; using local config.toml: {:?}", config);
    }

    let (cmd_tx, mut cmd_rx) = mpsc::channel::<Command>(100);

    let on_hours = config.light_on_hours;
    let off_hours = config.light_off_hours;
    let tx = cmd_tx.clone();
    spawn(run_light_timer(String::new(), on_hours, off_hours, tx));

    let port_clone = port.clone();
    let client_clone = client.clone();
    let server = config.server.clone();
    let is_simulation = std::env::var("SIMULATION_MODE").map(|v| v == "1").unwrap_or(false);
    spawn(async move {
        while let Some(cmd) = cmd_rx.recv().await {
            info!("Executing command: {:?}", cmd);
            if let Err(e) = client::send_commands(&client_clone, &server, std::slice::from_ref(&cmd)).await {
                error!("send_commands to server error: {}", e);
            }
            let mut guard = port_clone.lock().await;
            let port_ref: &mut dyn serialport::SerialPort = &mut **guard;
            if let Err(e) = send_serial_command(port_ref, &cmd) {
                error!("Error sending command via serial: {}", e);
            }
            if is_simulation {
                if let Err(e) = client::simulation_commands(&client_clone, &server, is_simulation).await {
                    error!("Simulation command error: {}", e);
                }
            }
        }
    });

    let mut interval = interval(tokio::time::Duration::from_secs(5));
    loop {
        interval.tick().await;
        let readings: Vec<models::DeviceReading> = {
            let mut guard = port.lock().await;
            let port_ref: &mut dyn serialport::SerialPort = &mut **guard;
            let mut sensor_controller = SensorController::new(port_ref);
            sensor_controller.process_sensors()
        };
        let cpu_temp = gpio::read_temperature();
        let mut full_readings = readings.clone();
        full_readings.push(models::DeviceReading {
            id: "blackbeard-cpu".to_string(),
            value: cpu_temp,
        });
        let sensor_payload = models::SensorReading {
            client_id: config.controller_id.clone(),
            devices: full_readings.clone(),
        };
        if let Err(e) = client::send_sensor_data(&client, &config.server, &sensor_payload).await {
            error!("Failed to send sensor data: {}", e);
        } else {
            info!("Sent sensor data to server: {:?}", sensor_payload);
        }
        let remote_readings = match client::fetch_readings(&client, &config.server).await {
            Ok(r) => r,
            Err(e) => {
                error!("Failed to fetch remote readings: {}", e);
                continue;
            }
        };
        let remote_commands = process_remote_readings(&remote_readings, &config.controller_id);
        let local_commands = process_local_rules(&config, cpu_temp.unwrap_or_default().into());
        for cmd in remote_commands.into_iter().chain(local_commands.into_iter()) {
            if let Err(e) = cmd_tx.send(cmd).await {
                error!("Failed to enqueue command: {}", e);
            }
        }
    }
}
