// Copyright 2025 Acris Software Ltd. Co. All Rights Reserved.
// github.com/cvusmo/casaverde/casaverde_app
// src/app.rs

use crate::devices::{DeviceData, Sensor};
use ratatui::backend::CrosstermBackend;
use std::io;

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
        Self {
            sensor_data: DeviceData::new("config.toml"),
            selected: 0,
            quit: false,
            screen: Screen::Devices,
        }
    }

    pub fn move_up(&mut self) {
        if self.screen == Screen::Devices && self.selected > 0 {
            self.selected -= 1;
        }
    }

    pub fn move_down(&mut self) {
        if self.screen == Screen::Devices && self.selected + 1 < self.sensor_data.active_count {
            self.selected += 1;
        }
    }

    pub fn quit(&mut self) {
        self.quit = true;
    }

    pub fn switch_screen(&mut self) {
        self.screen = match self.screen {
            Screen::Devices => Screen::Monitoring,
            Screen::Monitoring => Screen::Config,
            Screen::Config => Screen::Devices,
        };
    }

    pub fn toggle_selected_sensor(&mut self) {
        if self.screen == Screen::Devices {
            let sensor = match self.sensor_data.config.configs[self.selected].id.as_str() {
                "blackbeard-cpu" => Sensor::Temperature,
                "solar-1" => Sensor::Solar,
                "moisture-1" => Sensor::Moisture,
                "humidity-1" => Sensor::Humidity,
                "water-1" => Sensor::Water,
                "blackbeard-probe" => Sensor::Probe,
                _ => return,
            };
            self.sensor_data.toggle_sensor(sensor);
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
            break;
        }
        app.sensor_data.update_devices().await;
        render_tui(terminal, &app)?;
        handle_tui_events(&mut app)?;
    }
    Ok(())
}
