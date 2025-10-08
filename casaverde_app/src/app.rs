// Copyright 2025 Acris Software Ltd. Co. All Rights Reserved.
// github.com/cvusmo/casaverde/casaverde_app
// src/app.rs

use ratatui::widgets::ListState;
use crate::devices::DeviceData;
use casaverde_utils::io::{IoError, new_error, IoErrorKind};
use casaverde_utils::log::error;
use casaverde_utils::Logger;
use crossterm::event::{self, Event, KeyCode};
use std::io;
use std::time::{Duration, Instant};

const POLL_INTERVAL: Duration = Duration::from_millis(100);
const DEVICE_REFRESH_INTERVAL: Duration = Duration::from_secs(2);

#[derive(Clone, Copy, PartialEq)]
pub enum Screen {
    Devices,
    Monitoring,
    Config,
}

pub struct App {
    pub sensor_data: DeviceData,
    pub selected: usize,
    pub running: bool,
    pub screen: Screen,
    pub quit: bool,
    pub list_state: ListState,
}

impl App {
    pub fn new(config_path: &str, server: &str, logger: &mut Logger) -> Self {
        let mut list_state = ListState::default();
        list_state.select(Some(0));
        let sensor_data = DeviceData::new(config_path, server, logger);
        Self {
            sensor_data,
            selected: 0,
            running: true,
            screen: Screen::Devices,
            quit: false,
            list_state,
        }
    }

    pub async fn update(&mut self, logger: &mut Logger) -> Result<(), IoError> {
        if let Err(e) = self.sensor_data.update_devices(logger).await {
            error(logger, &format!("Device update failed: {:?}", e))?;
        }
        Ok(())
    }

    pub fn toggle_selected(&mut self, logger: &mut Logger) {
        if let Some(selected) = self.list_state.selected() {
            if selected < self.sensor_data.devices.len() {
                self.sensor_data.toggle_sensor(selected, logger);
            }
        }
    }

    pub fn move_up(&mut self) {
        if let Some(selected) = self.list_state.selected() {
            let new_selected = if selected > 0 {
                selected - 1
            } else {
                self.sensor_data.devices.len().saturating_sub(1)
            };
            self.list_state.select(Some(new_selected));
        }
    }

    pub fn move_down(&mut self) {
        if let Some(selected) = self.list_state.selected() {
            let new_selected = (selected + 1) % self.sensor_data.devices.len();
            self.list_state.select(Some(new_selected));
        }
    }

    pub fn switch_screen(&mut self) {
        self.screen = match self.screen {
            Screen::Devices => Screen::Monitoring,
            Screen::Monitoring => Screen::Config,
            Screen::Config => Screen::Devices,
        };
    }

    pub fn stop(&mut self) {
        self.running = false;
    }
}

pub async fn run_app(tui: &mut crate::tui::Tui, app: &mut App, logger: &mut Logger) -> Result<(), IoError> {
    let mut last_refresh = Instant::now();

    loop {
        tui.draw(app).map_err(|e| new_error(IoErrorKind::Other, format!("Failed to draw TUI: {}", e)))?;

        if event::poll(POLL_INTERVAL).map_err(|e| new_error(IoErrorKind::Other, format!("Poll error: {}", e)))? {
            match event::read().map_err(|e| new_error(IoErrorKind::Other, format!("Read event error: {}", e)))? {
                Event::Key(key) => match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Up => app.move_up(),
                    KeyCode::Down => app.move_down(),
                    KeyCode::Enter => app.toggle_selected(logger),
                    KeyCode::Char('r') => {
                        app.sensor_data.update_devices(logger).await?;
                    }
                    KeyCode::Char('t') => {
                        app.toggle_selected(logger);
                    }
                    KeyCode::Char('m') | KeyCode::Char('c') | KeyCode::Char('s') => {
                        app.switch_screen();
                    }
                    _ => {}
                },
                _ => {},
            }
        }

        if last_refresh.elapsed() >= DEVICE_REFRESH_INTERVAL {
            app.update(logger).await?;
            last_refresh = Instant::now();
        }

        if app.quit {
            break;
        }
    }

    Ok(())
}
