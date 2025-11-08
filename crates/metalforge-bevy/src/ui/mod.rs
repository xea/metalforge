mod event;
mod player;

use bevy::app::{App, PluginGroup, Startup, Update};
use bevy::camera::Camera2d;
use bevy::prelude::{AppExtStates, Commands, MonitorSelection, States};
use bevy::utils::default;
use bevy::window::{Window, WindowMode, WindowPlugin, WindowTheme};
use bevy::winit::WinitSettings;
use bevy::DefaultPlugins;
use metalforge_lib::engine::Engine;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum AppState {
    Player
}

pub struct UI<'e> {
    app: App,
    engine: &'e mut Engine
}

impl<'e> UI<'e> {
    pub fn new(engine: &'e mut Engine) -> Self {
        let mut app = App::new();

        app.add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                //mode: WindowMode::BorderlessFullscreen(MonitorSelection::Current),
                window_theme: Some(WindowTheme::Dark),
                ..default()
            }),
            ..default()
        }));

        app
            .insert_state(AppState::Player)
            .insert_resource(WinitSettings::game())
            .add_plugins(player::player_plugin)
            .add_systems(Startup, setup);

        Self {
            app,
            engine
        }
    }

    pub fn run(&mut self) {
        self.app.run();
    }

    fn play(&mut self) {
        log::info!("Playing...");
    }
}

/// Sets up the initial state of the application. This is where we create a camera that defines our main view
fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}
