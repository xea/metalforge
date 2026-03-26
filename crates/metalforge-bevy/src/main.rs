use crossbeam_channel::bounded;
use log::info;
use metalforge_lib::engine::{Engine, EngineChannel, EngineCommand};
use crate::tui::preload_menu;
use crate::ui::UI;

mod config;
mod ui;
mod tui;

const QUEUE_SIZE: usize = 64;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    ratatui::run(preload_menu)?;
    run_gui();
    Ok(())
}

fn run_gui() {
    info!("Initialising application...");

    let (control_tx, _control_rx) = bounded(QUEUE_SIZE);
    let (engine_tx, engine_rx) = bounded(QUEUE_SIZE);
    let (event_tx, event_rx) = bounded(QUEUE_SIZE);

    let mut ui = UI::new(EngineChannel::new(engine_tx.clone(), event_rx.clone()));

    let engine_thread = std::thread::spawn(move || {
        let engine = Engine::new(engine_tx, engine_rx, event_tx, event_rx);
        engine.main_loop();
    });

    ui.run();

    info!("Shutting down...");

    let _ = control_tx.send(EngineCommand::Quit);
    engine_thread.join().expect("Failed to join engine thread");

    info!("Exiting...");
}
