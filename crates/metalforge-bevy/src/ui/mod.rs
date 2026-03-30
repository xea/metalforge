mod player;
pub mod menu;

use bevy::app::{App, FixedUpdate, PluginGroup, Plugins, Startup};
use bevy::camera::Camera2d;
use bevy::image::ImagePlugin;
use bevy::prelude::{AppExtStates, Commands, Component, Entity, MessageReader, Query, ResMut, Resource, States, With};
use bevy::utils::default;
use bevy::window::{Window, WindowCloseRequested, WindowPlugin, WindowTheme};
use bevy::winit::WinitSettings;
use bevy::DefaultPlugins;
use bevy_dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin, FrameTimeGraphConfig};
use metalforge_lib::engine::{EngineChannel, EngineCommand};
use crate::config::Config;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum AppState {
    MainMenu,
    Player
}

#[derive(Resource)]
pub struct UIEngine {
    channel: EngineChannel,
    config: Config
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
    pub fn new(engine: EngineChannel, config: Config) -> Self {
        let mut app = App::new();

        app.add_plugins(DefaultPlugins
            .set(window_plugin())
            .set(image_plugin()));

        if config.debug.show_fps {
            app.add_plugins(fps_overlay_plugin());
        }
        /*
        app.add_plugins(FrameTimeDiagnosticsPlugin {
            max_history_length: 60,
            smoothing_factor: 0.0,
        });

        app.add_plugins((FramepacePlugin, debug::DiagnosticsPlugin));
         */

        // engine.send(EngineCommand::LoadSong(unimplemented!()));

        app
            .insert_state(AppState::MainMenu)
            .insert_resource(WinitSettings::game())
            .insert_resource(UIEngine { channel: engine, config })
            .add_systems(Startup, create_camera)
            .add_systems(FixedUpdate, handle_window_events)
            .add_plugins(menu::main_menu);
            //.add_plugins(player::player_plugin);

        Self {
            app
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

/// Tweak the image plugin to disable antialiasing.
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

fn create_camera(mut commands: Commands) {
    commands.spawn( Camera2d);
}

pub fn handle_window_events(mut window_close_events: MessageReader<WindowCloseRequested>, engine: ResMut<UIEngine>) {
    for _event in window_close_events.read() {
        engine.send(EngineCommand::Quit);
    }
}

pub fn despawn_screen<T: Component>(mut commands: Commands, query: Query<Entity, With<T>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}
