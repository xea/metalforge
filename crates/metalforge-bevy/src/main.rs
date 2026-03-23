use crossbeam_channel::bounded;
use log::info;
use metalforge_lib::engine::{Engine, EngineChannel, EngineCommand};
use crate::ui::UI;

mod ui;

const QUEUE_SIZE: usize = 64;

fn main() {
    log::info!("Initialising application...");

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

    log::info!("Exiting...");
}
