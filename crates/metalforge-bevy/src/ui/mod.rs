pub mod menu;
pub mod debug;
pub mod keyboard;
mod player;
pub mod event;

use crate::config::Config;
use crate::ui::menu::{MenuState, MenuStructure};
use bevy::app::{App, PluginGroup, Startup, Update};
use bevy::camera::Camera2d;
use bevy::image::ImagePlugin;
use bevy::math::Vec2;
use bevy::prelude::{AppExtStates, Commands, Component, Entity, FixedUpdate, MessageReader, Query, ResMut, Resource, States, With};
use bevy::utils::default;
use bevy::window::{PrimaryWindow, Window, WindowCloseRequested, WindowPlugin, WindowTheme};
use bevy::winit::WinitSettings;
use bevy::DefaultPlugins;
use bevy_dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin, FrameTimeGraphConfig};
use log::info;
use metalforge_lib::engine::{EngineChannel, EngineCommand};
use crate::ui::event::handle_engine_event;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum AppState {
    MainMenu,
    Player
}

#[derive(Resource)]
pub struct UIEngine {
    channel: EngineChannel,
    window_size: Vec2,
    config: Config,
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
            .insert_resource(MenuStructure::default())
            .insert_resource(UIEngine {
                channel: engine,
                config,
                window_size: Vec2::new(0.0, 0.0),
            })
            .add_systems(Startup, (update_window_size, create_camera))
            .add_systems(Update, (update_window_size, handle_window_closed))
            .add_systems(FixedUpdate, handle_engine_event)
            .add_plugins(keyboard::handle_key_input)
            .add_plugins(debug::debug)
            .add_plugins(menu::main_menu)
            .add_plugins(player::player_plugin);

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

fn handle_window_closed(mut window_close_events: MessageReader<WindowCloseRequested>, engine: ResMut<UIEngine>) {
    for _event in window_close_events.read() {
        info!("Window closed, sending quit event to engine");
        engine.send(EngineCommand::Quit);
    }
}

fn update_window_size(window_q: Query<&Window, With<PrimaryWindow>>, mut engine: ResMut<UIEngine>) {
    for window in window_q {
        engine.window_size.x = window.width();
        engine.window_size.y = window.height();
    }
}

pub fn exit_menu<T: Component>(mut commands: Commands, query: Query<Entity, With<T>>, mut menu: ResMut<MenuStructure>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }

    menu.selected_idx = 0;
}

pub fn despawn_screen<T: Component>(mut commands: Commands, query: Query<Entity, With<T>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}
