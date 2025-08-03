// Copyright 2025 Nicholas Jordan. All Rights Reserved.
// github.com/cvusmo/casaverde/casaverde_app
// src/tui.rs

use crate::{
    app::{CasaverdeApp, Screen},
    ui::create_layout,
};
use crossterm::event::{self, Event, KeyCode};
use log::info;
use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
};
use std::io;

pub fn render_tui(
    terminal: &mut ratatui::Terminal<ratatui::backend::CrosstermBackend<std::io::Stdout>>,
    app: &CasaverdeApp,
) -> io::Result<()> {
    terminal.draw(|frame| {
        let chunks = create_layout(frame.area());

        let title = Paragraph::new("Casaverde")
            .block(Block::new().borders(Borders::ALL))
            .style(Style::default().fg(Color::Green))
            .alignment(ratatui::layout::Alignment::Center);
        frame.render_widget(title, chunks[0]);

        match app.screen {
            Screen::Devices => {
                let mut items = Vec::with_capacity(app.sensor_data.active_count);
                for i in 0..app.sensor_data.active_count {
                    let id = app.sensor_data.config.configs[i]
                        .id
                        .trim_matches(char::from(0)); // Direct String access
                    let value = app.sensor_data.device_values[i];
                    let flag = if value.is_some() { "[ON]  " } else { "[OFF] " };
                    let value_str = value.map_or("N/A".to_string(), |v| format!("{v:.1}"));
                    items.push(ListItem::new(Span::raw(format!(
                        "{} {}: {}",
                        flag, id, value_str
                    ))));
                }

                let mut list_state = ListState::default();
                list_state.select(Some(app.selected.min(items.len().saturating_sub(1))));

                let devices = List::new(items.clone())
                    .block(
                        Block::new()
                            .borders(Borders::ALL)
                            .title("Devices")
                            .title_alignment(ratatui::layout::Alignment::Center)
                            .style(Style::default().fg(Color::Yellow)),
                    )
                    .highlight_symbol(">> ")
                    .highlight_style(Style::default().bg(Color::DarkGray));

                frame.render_stateful_widget(devices, chunks[1], &mut list_state);
                info!("Rendered Devices screen with {} items", items.len());
            }
            Screen::Monitoring => {
                let mut temp_text = Vec::with_capacity(app.sensor_data.active_count);
                for i in 0..app.sensor_data.active_count {
                    let id = app.sensor_data.config.configs[i]
                        .id
                        .trim_matches(char::from(0));
                    let value = app.sensor_data.device_values[i];
                    temp_text.push(
                        Line::from(format!(
                            "{}: {}",
                            id,
                            value.map_or("N/A".to_string(), |v| format!("{v:.1}"))
                        ))
                        .centered(),
                    );
                }
                if temp_text.is_empty() {
                    temp_text.push(Line::from("No devices configured").centered());
                }

                let monitor = Paragraph::new(temp_text.clone())
                    .block(
                        Block::new()
                            .borders(Borders::ALL)
                            .title("Monitoring")
                            .title_alignment(ratatui::layout::Alignment::Center)
                            .style(Style::default().fg(Color::Yellow)),
                    )
                    .alignment(ratatui::layout::Alignment::Center);

                frame.render_widget(monitor, chunks[1]);
                info!("Rendered Monitoring screen with {} items", temp_text.len());
            }
        }

        let status_text = match app.screen {
            Screen::Devices => "Navigate with Up/Down, Switch to Monitor with m, Quit with q",
            Screen::Monitoring => "Switch to Devices with s, Quit with q",
        };
        let status = Paragraph::new(status_text)
            .block(Block::new().borders(Borders::ALL))
            .style(Style::default().fg(Color::Cyan))
            .alignment(ratatui::layout::Alignment::Center);
        frame.render_widget(status, chunks[2]);
    })?;
    Ok(())
}

pub fn handle_tui_events(app: &mut CasaverdeApp) -> io::Result<()> {
    if event::poll(std::time::Duration::from_millis(200))? {
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => {
                    app.quit();
                    info!("Quit command received");
                }
                KeyCode::Up if app.screen == Screen::Devices => app.move_up(),
                KeyCode::Down if app.screen == Screen::Devices => app.move_down(),
                KeyCode::Char('m') => {
                    app.switch_screen();
                    info!("Switched to Monitoring screen");
                }
                KeyCode::Char('s') if app.screen == Screen::Monitoring => {
                    app.switch_screen();
                    info!("Switched to Devices screen");
                }
                _ => {}
            }
        }
    }
    Ok(())
}
