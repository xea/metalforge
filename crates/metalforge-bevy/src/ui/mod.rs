mod event;
mod player;

use bevy::app::{App, PluginGroup, Startup};
use bevy::camera::Camera2d;
use bevy::prelude::{AppExtStates, Commands, Resource, States};
use bevy::utils::default;
use bevy::window::{PresentMode, Window, WindowPlugin, WindowTheme};
use bevy::winit::WinitSettings;
use bevy::DefaultPlugins;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy_dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin, FrameTimeGraphConfig};
use bevy_framepace::{debug, FramepacePlugin};
use metalforge_lib::engine::{EngineChannel};

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum AppState {
    Player
}

#[derive(Resource)]
pub struct UIEngine {
    channel: EngineChannel
}

pub struct UI {
    app: App,
}

impl UI {
    pub fn new(engine: EngineChannel) -> Self {
        let mut app = App::new();

        app.add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                //mode: WindowMode::BorderlessFullscreen(MonitorSelection::Current),
                window_theme: Some(WindowTheme::Dark),
                ..default()
            }),
            ..default()
        }));

        /*
        app.add_plugins(FrameTimeDiagnosticsPlugin {
            max_history_length: 60,
            smoothing_factor: 0.0,
        });
        
        app.add_plugins((FramepacePlugin, debug::DiagnosticsPlugin));
         */

        app.add_plugins(FpsOverlayPlugin {
            config: FpsOverlayConfig {
                enabled: true,
                frame_time_graph_config: FrameTimeGraphConfig {
                    enabled: true,
                    ..default()
                },
                ..default()
            }
        });

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
