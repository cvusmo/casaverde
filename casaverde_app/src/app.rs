// Copyright 2025 Nicholas Jordan. All Rights Reserved.
// github.com/cvusmo/casaverde/casaverde_app
// src/app.rs

// Purpose:
// Defines the CasaverdeApp struct and methods for state management (navigation, quitting..)

use crate::sensors::SensorData;
use ratatui::backend::CrosstermBackend;
use std::io;

#[derive(Clone, Copy, PartialEq)]
pub enum Screen {
    Sensors,    // Sensor selection screen
    Monitoring, // New temperature monitoring screen
}

pub struct CasaverdeApp {
    pub sensor_data: SensorData,
    pub selected: usize,
    pub should_quit: bool,
    pub screen: Screen, // Track current screen
}

impl CasaverdeApp {
    pub fn new() -> Self {
        Self {
            sensor_data: SensorData::new(),
            selected: 0,
            should_quit: false,
            screen: Screen::Sensors, // Start on sensor screen
        }
    }

    pub fn move_up(&mut self) {
        if self.screen == Screen::Sensors && self.selected > 0 {
            self.selected -= 1;
        }
    }

    pub fn move_down(&mut self) {
        if self.screen == Screen::Sensors && self.selected + 1 < self.sensor_data.states.len() {
            self.selected += 1;
        }
    }

    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    pub fn switch_screen(&mut self) {
        self.screen = match self.screen {
            Screen::Sensors => Screen::Monitoring,
            Screen::Monitoring => Screen::Sensors,
        };
    }
}

pub async fn run_app(
    terminal: &mut ratatui::Terminal<CrosstermBackend<std::io::Stdout>>,
    mut app: CasaverdeApp,
) -> io::Result<()> {
    use crate::tui::{handle_tui_events, render_tui};
    loop {
        if app.should_quit {
            break;
        }
        app.sensor_data.update_temperatures().await;
        render_tui(terminal, &app)?;
        handle_tui_events(&mut app)?;
    }
    Ok(())
}
