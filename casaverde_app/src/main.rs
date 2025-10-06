use casaverde_app::app::run_app;
use casaverde_app::tui::Tui;
use casaverde_utils::logger;
use tokio::runtime::Builder;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    logger::init();

    let mut tui = Tui::new()?;
    tui.enter()?;

    if let Err(e) = run_app(&mut tui).await {
        eprintln!("Error: {:?}", e);
    }

    tui.exit()?;
    Ok(())
}
