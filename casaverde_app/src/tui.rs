use crate::devices::Device;
use crossterm::{
    event, execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Frame, Terminal,
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, Borders, List, ListItem},
};
use std::io::{self, Stdout};

pub struct Tui {
    terminal: Terminal<CrosstermBackend<Stdout>>,
}

impl Tui {
    pub fn new() -> anyhow::Result<Self> {
        let stdout = io::stdout();
        let backend = CrosstermBackend::new(stdout);
        Ok(Self {
            terminal: Terminal::new(backend)?,
        })
    }

    pub fn enter(&mut self) -> anyhow::Result<()> {
        enable_raw_mode()?;
        execute!(io::stdout(), EnterAlternateScreen)?;
        Ok(())
    }

    pub fn exit(&mut self) -> anyhow::Result<()> {
        disable_raw_mode()?;
        execute!(io::stdout(), LeaveAlternateScreen)?;
        Ok(())
    }

    pub fn draw(&mut self, devices: &[Device]) -> anyhow::Result<()> {
        self.terminal.draw(|f| Self::ui(f, devices))?;
        Ok(())
    }

    fn ui(f: &mut Frame, devices: &[Device]) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([Constraint::Percentage(100)])
            .split(f.size());

        let items: Vec<ListItem> = devices
            .iter()
            .map(|d| {
                let status = if d.active {
                    Span::styled("ON", Style::default().fg(Color::Green))
                } else {
                    Span::styled("OFF", Style::default().fg(Color::Red))
                };
                ListItem::new(format!("{:<10} [{}]", d.name, status.content))
            })
            .collect();

        let list = List::new(items)
            .block(Block::default().title("Devices").borders(Borders::ALL))
            .highlight_style(Style::default().add_modifier(Modifier::BOLD));

        f.render_widget(list, chunks[0]);
    }
}
