mod config;
mod ui;

use crate::config::{AppConfig, WindowType};
use crate::ui::{AppState, EngineView};
use bevy::prelude::{App, AppExtStates, Camera2d, Commands, Startup};
use bevy::winit::WinitSettings;
use bevy::DefaultPlugins;
use metalforge_lib::engine::EngineBuilder;
use std::env::current_dir;

fn main() -> std::io::Result<()> {
    // Load configuration
    let cfg = load_config();

    // Initialise audio engine
    let engine = init_engine(&cfg);

    // Initialise application
    let mut app = init_app(&cfg);

    // Make app config available as a resource
    app.insert_resource(cfg);
    app.insert_resource(engine);

    // Finally, run the main loop
    app.run();

    Ok(())
}

fn init_engine(app_config: &AppConfig) -> EngineView {
    let engine = EngineBuilder::default()
        .with_library_paths(&app_config.library.paths)
        .build();
    
    EngineView(engine)
}

fn load_config() -> AppConfig {
    let path = current_dir()
        .map(|cwd| cwd.join("config").join("config.yaml"))
        .expect("Unable to load current directory");

    let file = std::fs::File::open(path).expect("Unable to open config.yaml");

    serde_yaml::from_reader(file).expect("Unable to parse config.yaml")
}

fn init_app(cfg: &AppConfig) -> App {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins);

    // State initialisation has to come after DefaultPlugins
    app.init_state::<AppState>();
    app.add_systems(Startup, setup);
    app.add_plugins((ui::menu::menu_plugin, ui::player::player_plugin));

    if cfg.display.wireframe {
        // app.add_plugins(Wireframe2dPlugin);
    }

    // Display mode
    match cfg.display.window_type {
        WindowType::Desktop => app.insert_resource(WinitSettings::desktop_app()),
        WindowType::Game => app.insert_resource(WinitSettings::game()),
        WindowType::Mobile => app.insert_resource(WinitSettings::mobile()),
    };

    app
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}
