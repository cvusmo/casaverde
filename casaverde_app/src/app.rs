use crate::devices::{self, Device};
use crate::tui::Tui;
use crossterm::event::{self, Event, KeyCode};
use std::time::{Duration, Instant};

const POLL_INTERVAL: Duration = Duration::from_millis(100);
const DEVICE_REFRESH_INTERVAL: Duration = Duration::from_secs(2);

pub async fn run_app(tui: &mut Tui) -> anyhow::Result<()> {
    let mut devices = devices::fetch_devices().await.unwrap_or_default();
    let mut last_refresh = Instant::now();

    loop {
        tui.draw(&devices)?;

        if event::poll(POLL_INTERVAL)? {
            match event::read()? {
                Event::Key(key) => match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Char('r') => {
                        devices = devices::fetch_devices().await?;
                    }
                    KeyCode::Char('t') => {
                        devices::toggle_random_device(&mut devices).await?;
                    }
                    _ => {}
                },
                _ => {}
            }
        }

        if last_refresh.elapsed() >= DEVICE_REFRESH_INTERVAL {
            devices = devices::update_devices(devices).await?;
            last_refresh = Instant::now();
        }
    }

    Ok(())
}
