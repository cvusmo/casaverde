// Copyright 2025 Acris Software Ltd. Co. All Rights Reserved.
// github.com/cvusmo/casaverde/casaverde_app
// src/app.rs

use crate::devices::{DeviceData, Sensor};
use ratatui::backend::CrosstermBackend;
use std::io;
use log::{info, error};

#[derive(Clone, Copy, PartialEq)]
pub enum Screen {
    Devices,
    Monitoring,
    Config,
}

pub struct CasaverdeApp {
    pub sensor_data: DeviceData,
    pub selected: usize,
    pub quit: bool,
    pub screen: Screen,
}

impl CasaverdeApp {
    pub fn new() -> Self {
        let app = Self {
            sensor_data: DeviceData::new("config.toml"),
            selected: 0,
            quit: false,
            screen: Screen::Devices,
        };
        info!("CasaverdeApp initialized");
        app
    }

    pub fn move_up(&mut self) {
        if self.screen == Screen::Devices && self.selected > 0 {
            self.selected -= 1;
            info!("Moved selection up to index {}", self.selected);
        }
    }

    pub fn move_down(&mut self) {
        if self.screen == Screen::Devices && self.selected + 1 < self.sensor_data.active_count {
            self.selected += 1;
            info!("Moved selection down to index {}", self.selected);
        }
    }

    pub fn quit(&mut self) {
        self.quit = true;
        info!("Application quit triggered");
    }

    pub fn switch_screen(&mut self) {
        self.screen = match self.screen {
            Screen::Devices => {
                info!("Switched to Monitoring screen");
                Screen::Monitoring
            }
            Screen::Monitoring => {
                info!("Switched to Config screen");
                Screen::Config
            }
            Screen::Config => {
                info!("Switched to Devices screen");
                Screen::Devices
            }
        };
    }

    pub fn toggle_selected_sensor(&mut self) {
        if self.screen == Screen::Devices {
            if let Some(sensor) = self.sensor_data.config.configs.get(self.selected).and_then(|cfg| {
                match cfg.id.as_str() {
                    "blackbeard-cpu" => Some(Sensor::Temperature),
                    "solar-1" => Some(Sensor::Solar),
                    "moisture-1" => Some(Sensor::Moisture),
                    "humidity-1" => Some(Sensor::Humidity),
                    "water-1" => Some(Sensor::Water),
                    "blackbeard-probe" => Some(Sensor::Probe),
                    _ => None,
                }
            }) {
                self.sensor_data.toggle_sensor(sensor);
                info!("Toggled sensor: {}", sensor.name());
            } else {
                error!("Failed to toggle sensor at index {}", self.selected);
            }
        }
    }
}

pub async fn run_app(
    terminal: &mut ratatui::Terminal<CrosstermBackend<std::io::Stdout>>,
    mut app: CasaverdeApp,
) -> io::Result<()> {
    use crate::tui::{handle_tui_events, render_tui};

    loop {
        if app.quit {
            info!("Exiting run loop");
            break;
        }

        app.sensor_data.update_devices().await;

        if let Err(e) = render_tui(terminal, &app) {
            error!("Error rendering TUI: {:?}", e);
        }

        if let Err(e) = handle_tui_events(&mut app) {
            error!("Error handling TUI events: {:?}", e);
        }
    }
    Ok(())
}

