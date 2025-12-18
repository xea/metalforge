use metalforge_lib::engine::{Engine, EngineCommand};
use crate::ui::UI;

mod ui;

fn main() {
    log::info!("Initialising application...");

    let mut engine = Engine::new();

    let control_channel = engine.create_channel();
    let engine_channel = engine.create_channel();

    let mut ui = UI::new(engine_channel);

    let engine_thread = std::thread::spawn(move || engine.main_loop());

    ui.run();

    println!("Shutting down...");
    control_channel.send(EngineCommand::Quit);
    engine_thread.join().expect("Failed to join engine thread");

    log::info!("Exiting...");
}
