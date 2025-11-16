use persisten_tui::events;
use persisten_tui::events::AppEvent;
use persisten_tui::events::Event;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let mut event_handler = events::EventHandler::new();

    loop {
        let event = event_handler.next().await?;
        match event {
            Event::App(AppEvent::Quit) => break,
            Event::Tick => {}
            _ => println!("Event: {:?}", event),
        }
    }
    Ok(())
}
