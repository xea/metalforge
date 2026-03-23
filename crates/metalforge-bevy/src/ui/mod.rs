mod player;

use bevy::app::{App, PluginGroup, Plugins};
use bevy::prelude::{AppExtStates, Resource, States};
use bevy::utils::default;
use bevy::window::{Window, WindowPlugin, WindowTheme};
use bevy::winit::WinitSettings;
use bevy::DefaultPlugins;
use bevy::image::ImagePlugin;
use bevy_dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin, FrameTimeGraphConfig};
use metalforge_lib::engine::{EngineChannel, EngineCommand};

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum AppState {
    Player
}

#[derive(Resource)]
pub struct UIEngine {
    channel: EngineChannel
}

impl UIEngine {
    pub fn send(&self, event: EngineCommand) {
        self.channel.send(event);
    }
}

pub struct UI {
    app: App,
}

impl UI {
    pub fn new(engine: EngineChannel) -> Self {
        let mut app = App::new();

        app.add_plugins(DefaultPlugins
            .set(window_plugin())
            .set(image_plugin()))
            .add_plugins(fps_overlay_plugin());
        /*
        app.add_plugins(FrameTimeDiagnosticsPlugin {
            max_history_length: 60,
            smoothing_factor: 0.0,
        });

        app.add_plugins((FramepacePlugin, debug::DiagnosticsPlugin));
         */

        engine.send(EngineCommand::LoadSong);

        app
            .insert_state(AppState::Player)
            .insert_resource(WinitSettings::game())
            .insert_resource(UIEngine { channel: engine })
            .add_plugins(player::player_plugin);

        Self {
            app,
    //        engine
        }
    }

    pub fn run(&mut self) {
        self.app.run();
    }
}

fn window_plugin() -> WindowPlugin {
    WindowPlugin {
        primary_window: Some(Window {
            //mode: WindowMode::BorderlessFullscreen(MonitorSelection::Current),
            window_theme: Some(WindowTheme::Dark),
            ..default()
        }),
        ..default()
    }
}

fn image_plugin() -> ImagePlugin {
    ImagePlugin::default_nearest()
}

fn fps_overlay_plugin() -> FpsOverlayPlugin {
    FpsOverlayPlugin {
        config: FpsOverlayConfig {
            enabled: true,
            frame_time_graph_config: FrameTimeGraphConfig {
                enabled: true,
                ..default()
            },
            ..default()
        }
    }
}
