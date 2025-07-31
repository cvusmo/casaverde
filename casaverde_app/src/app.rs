// Copyright 2025 Nicholas Jordan. All Rights Reserved.
// github.com/cvusmo/casaverde/casaverde_app
// src/app.rs

// Purpose:
// Defines the CasaverdeApp struct and methods for state management (navigation, quitting..)

use crate::sensors::SensorData;
use ratatui::backend::CrosstermBackend;
use std::io;

pub struct CasaverdeApp {
    pub sensor_data: SensorData,
    pub selected: usize,
    pub should_quit: bool,
}

impl CasaverdeApp {
    pub fn new() -> Self {
        Self {
            sensor_data: SensorData::new(),
            selected: 0,
            should_quit: false,
        }
    }

    pub fn move_up(&mut self) {
        if self.selected > 0 {
            self.selected -= 1;
        }
    }

    pub fn move_down(&mut self) {
        if self.selected + 1 < self.sensor_data.states.len() {
            self.selected += 1;
        }
    }

    pub fn quit(&mut self) {
        self.should_quit = true
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
