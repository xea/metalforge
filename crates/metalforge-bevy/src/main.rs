use crossbeam_channel::bounded;
use metalforge_lib::engine::{Engine, EngineChannel, EngineCommand};
use crate::ui::UI;

mod ui;

fn main() {
    log::info!("Initialising application...");

    let (control_tx, _control_rx) = bounded(64);
    let (engine_tx, engine_rx) = bounded(64);

    let mut ui = UI::new(EngineChannel::new(engine_tx.clone()));

    let engine_thread = std::thread::spawn(move || {
        let engine = Engine::new(engine_tx, engine_rx);
        engine.main_loop();
    });

    ui.run();

    println!("Shutting down...");

    let _ = control_tx.send(EngineCommand::Quit);
    engine_thread.join().expect("Failed to join engine thread");

    log::info!("Exiting...");
}
