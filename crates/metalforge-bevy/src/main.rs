use crate::config::Config;
use crate::ui::UI;
use crossbeam_channel::bounded;
use log::info;
use metalforge_lib::engine::{Engine, EngineChannel, EngineCommand};
use metalforge_lib::library::{Library};
use std::fs::File;
use std::io::{Error, Read};

mod config;
mod ui;

const QUEUE_SIZE: usize = 64;

fn main() -> color_eyre::Result<()> {
    let config = load_config()?;

    run_gui(&config);

    Ok(())
}

fn load_config() -> Result<Config, Error> {
    let mut raw_config = String::new();
    let _ = File::open("config/config.yaml")?.read_to_string(&mut raw_config);

    let config: Config = serde_yaml::from_str(raw_config.as_str())
        .unwrap();

    Ok(config)
}

fn run_gui(config: &Config) {
    info!("Initialising application...");

    let song_paths = config.library.paths.iter().map(|s| s.as_str()).collect();
    let library = Library::scan_directories(song_paths);

    let (control_tx, _control_rx) = bounded(QUEUE_SIZE);
    let (engine_tx, engine_rx) = bounded(QUEUE_SIZE);
    let (event_tx, event_rx) = bounded(QUEUE_SIZE);

    let mut ui = UI::new(EngineChannel::new(engine_tx.clone(), event_rx.clone()));

    let engine_thread = std::thread::spawn(move || {
        let engine = Engine::new(engine_tx, engine_rx, event_tx, event_rx, library);
        engine.main_loop();
    });

    ui.run();

    info!("Shutting down...");

    let _ = control_tx.send(EngineCommand::Quit);
    engine_thread.join().expect("Failed to join engine thread");

    info!("Exiting...");
}
