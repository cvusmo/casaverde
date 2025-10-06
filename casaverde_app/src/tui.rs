// Copyright 2025 Acris Software Ltd. Co. All Rights Reserved.
// github.com/cvusmo/casaverde/casaverde_app
// src/tui.rs

use crate::app::Screen;
use crate::devices::DeviceData;
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use ratatui::{
    backend::CrosstermBackend,
    crossterm,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame, Terminal,
};
use std::io::{self, Stdout};

pub struct Tui {
    terminal: Terminal<CrosstermBackend<Stdout>>,
}

impl Tui {
    pub fn new() -> io::Result<Self> {
        let stdout = io::stdout();
        let backend = CrosstermBackend::new(stdout);
        Ok(Self {
            terminal: Terminal::new(backend)?,
        })
    }

    pub fn enter(&mut self) -> io::Result<()> {
        enable_raw_mode()?;
        execute!(io::stdout(), EnterAlternateScreen)?;
        Ok(())
    }

    pub fn exit(&mut self) -> io::Result<()> {
        disable_raw_mode()?;
        execute!(io::stdout(), LeaveAlternateScreen)?;
        Ok(())
    }

    pub fn draw(&mut self, app: &crate::app::App) -> io::Result<()> {
        let sensor_data = &app.sensor_data;
        let screen = app.screen;
        self.terminal.draw(|f| render_ui(f, sensor_data, screen))?;
        Ok(())
    }
}

fn render_ui(f: &mut Frame, sensor_data: &DeviceData, screen: Screen) {
    let area = f.area();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Percentage(80),
            Constraint::Length(3),
        ])
        .split(area);

    let title = Paragraph::new("Casaverde")
        .block(Block::default().borders(Borders::ALL))
        .style(Style::default().fg(Color::Green))
        .alignment(Alignment::Center);
    f.render_widget(title, chunks[0]);

    match screen {
        Screen::Devices => render_devices(f, chunks[1], sensor_data),
        Screen::Monitoring => render_monitoring(f, chunks[1], sensor_data),
        Screen::Config => render_config(f, chunks[1], sensor_data),
    }

    let status = Paragraph::new("q:quit ↑/↓:navigate Enter:toggle m/c/s:switch")
        .block(Block::default().borders(Borders::ALL))
        .style(Style::default().fg(Color::Cyan))
        .alignment(Alignment::Center);
    f.render_widget(status, chunks[2]);
}

fn render_devices(f: &mut Frame, area: Rect, sensor_data: &DeviceData) {
    let items: Vec<ListItem> = sensor_data
        .devices
        .iter()
        .enumerate()
        .map(|(i, d)| {
            let status = if d.active {
                Span::styled("ON", Style::default().fg(Color::Green))
            } else {
                Span::styled("OFF", Style::default().fg(Color::Red))
            };
            ListItem::new(format!(
                "{:<10} [{}]",
                format!("Sensor {}", i + 1),
                status.content
            ))
        })
        .collect();

    let list = List::new(items)
        .block(Block::default().title("Devices").borders(Borders::ALL))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD));

    f.render_widget(list, area);
}

fn render_monitoring(f: &mut Frame, area: Rect, sensor_data: &DeviceData) {
    let lines: Vec<Line> = sensor_data
        .devices
        .iter()
        .enumerate()
        .map(|(i, d)| {
            let s = format!("Sensor {}: {:.1}", i + 1, d.value.unwrap_or_default());
            Line::from(Span::raw(s))
        })
        .collect();

    let paragraph =
        Paragraph::new(lines).block(Block::default().borders(Borders::ALL).title("Monitoring"));
    f.render_widget(paragraph, area);
}

fn render_config(f: &mut Frame, area: Rect, sensor_data: &DeviceData) {
    let lines: Vec<Line> = sensor_data
        .config
        .configs
        .iter()
        .map(|cfg| {
            Line::from(Span::raw(format!(
                "{}: type={} endpoint={} interval={}s",
                cfg.id, cfg.r#type, cfg.endpoint, cfg.interval
            )))
        })
        .collect();

    let paragraph =
        Paragraph::new(lines).block(Block::default().borders(Borders::ALL).title("Config"));
    f.render_widget(paragraph, area);
}
