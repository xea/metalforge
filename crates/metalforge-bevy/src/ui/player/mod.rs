use crate::ui::menu::{MenuEvent, MenuState};
use crate::ui::{AppState, EngineView, LibraryView};
use bevy::app::AppExit;
use bevy::color::palettes::basic::{PURPLE, RED};
use bevy::input::ButtonInput;
use bevy::prelude::{default, in_state, App, AssetServer, Assets, Camera2d, Color, ColorMaterial, Commands, Component, Event, EventReader, EventWriter, IntoScheduleConfigs, KeyCode, Mesh, Mesh2d, MeshMaterial2d, NextState, OnEnter, Query, Rectangle, Res, ResMut, Resource, Sprite, State, Text2d, TextFont, Time, Transform, Update, Vec2, Vec3, With};
use bevy::text::TextBounds;
use metalforge_lib::asset::{load_asset, load_instrument_part};

/// `PlayerEvent` describes the various events that may happen during song play.
#[derive(Event, Copy, Clone)]
pub enum PlayerEvent {
    ResumeSong,
    PauseSong
}

#[derive(Resource, Default)]
struct PlayerState {
    song_playing: bool
}

pub fn player_plugin(app: &mut App) {
    app
        .insert_resource(PlayerState::default())
        .add_event::<PlayerEvent>()
        .add_systems(OnEnter(AppState::Player), setup_player)
        .add_systems( Update,
            handle_player_events.run_if(in_state(AppState::Player)),
        )
        .add_systems(Update, handle_keyboard.run_if(in_state(AppState::Player)))
        .add_systems(Update, scroll_nodes.run_if(in_state(AppState::Player)))
        .add_systems(Update, move_camera.run_if(in_state(AppState::Player)))
        .add_systems(Update, move_cursor.run_if(in_state(AppState::Player)))
        .add_systems(Update, update_color.run_if(in_state(AppState::Player)));
}

fn setup_player(
    mut commands: Commands,
    mut _meshes: ResMut<Assets<Mesh>>,
    mut _materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
    menu_state: Res<MenuState>,
    engine: Res<EngineView>,
) {
    let song = engine.0
        .song_library
        .song(menu_state.selected_song_idx)
        .expect("Unable to open selected song");

    let arrangement = song
        .header
        .arrangements
        .get(menu_state.selected_arrangement_idx)
        .expect("Unable to find selected arrangement");


    let asset_id = unimplemented!();

    let part = load_instrument_part(&asset_id)?;

    // Text 2D
    let font = asset_server.load("fonts/FiraMono-Medium.ttf");
    let text_font = TextFont {
        font: font.clone(),
        ..default()
    };

    // Create the white line cursor on the screen
    commands.spawn((
        Cursor,
        Sprite::from_color(Color::srgb(1.0, 1.0, 1.0), Vec2::new(1.0, 60.0)),
        Transform::from_xyz(0., 0., 1.0)
        ));

    const BOX_WIDTH: f32 = 15.0;
    const BOX_HEIGHT: f32 = 30.0;

    for note in &part.notes {
        let box_size = Vec2::new(BOX_WIDTH, BOX_HEIGHT);
        let string = note.string as f32 * BOX_HEIGHT;

        commands.spawn((
            Note2d { kind: format!("{}", note.fret) },
            //Note2d { kind: note.class.to_string() },
            Sprite::from_color(Color::srgb(0.2, 0.3, 0.7), box_size),
            Transform::from_xyz(note.time * BOX_WIDTH, string, 0.)
        )).with_children(|builder| {
            builder.spawn((
                Text2d::new(format!("{}", note.fret)),
                //Text2d::new(note.class.to_string()),
                text_font.clone(),
                TextBounds::from(box_size),
                Transform::from_translation(Vec3::Z),
            ));
        });
        /*
        commands.spawn((
            Note2d { kind: note.class.to_string(), },
            Mesh2d(meshes.add(Rectangle::default())),
            MeshMaterial2d(materials.add(Color::from(PURPLE))),
            // Transform::default().with_scale(Vec3::new(64., 32., 16.)),
            Transform::from_xyz(0., 0., 0.).with_scale(Vec3::splat(32.))
        ));
         */
    }
}

fn handle_keyboard(
    input: Res<ButtonInput<KeyCode>>,
    current_app_state: Res<State<AppState>>,
    mut menu_events: EventWriter<crate::ui::menu::MenuEvent>,
    mut player_events: EventWriter<PlayerEvent>,
    player_state: Res<PlayerState>,
) {
    if input.just_pressed(KeyCode::Escape) {
        match current_app_state.get() {
            AppState::Player => {
                menu_events.write(MenuEvent::OpenMainMenu);
            }
            _ => {
                println!("Ignored escape");
            }
        }
    } else if input.just_pressed(KeyCode::Space) {
        if player_state.song_playing {
            player_events.write(PlayerEvent::PauseSong);
        } else {
            player_events.write(PlayerEvent::ResumeSong);
        }
    }
}

// TODO refactor event handler to make it more modular
fn handle_player_events(
    mut events: EventReader<PlayerEvent>,
    mut player_state: ResMut<PlayerState>,
    mut engine: ResMut<EngineView>,
    mut app_state: ResMut<NextState<AppState>>,
    mut app_exit_events: EventWriter<AppExit>,
) {
    for event in events.read() {
        match event {
            PlayerEvent::ResumeSong => {
                player_state.song_playing = true;
                engine.0.start()
            }
            PlayerEvent::PauseSong => {
                player_state.song_playing = false;
                engine.0.stop()
            }
        }
    }
}

#[derive(Component)]
pub struct Cursor;

#[derive(Component)]
pub struct Note2d {
    pub kind: String,
}

fn scroll_nodes(mut query: Query<&mut Transform, With<Note2d>>, _time: Res<Time>) {
    for mut transform in query.iter_mut() {
        // transform.translation.x -= 0.3;
    }
}

fn move_cursor(mut query: Query<&mut Transform, With<Cursor>>, player_state: Res<PlayerState>) {
    let Ok(mut cursor) = query.single_mut() else {
        return;
    };

    if player_state.song_playing {
        cursor.translation.x += 1.0;
    }
}

fn move_camera(mut query: Query<&mut Transform, With<Camera2d>>, player_state: Res<PlayerState>) {
    let Ok(mut camera) = query.single_mut() else {
        return;
    };

    if player_state.song_playing {
        camera.translation.x += 1.0;
    }
}

fn update_color(mut query: Query<(&mut Sprite, &Note2d)>) {
    for (mut sprite, _) in query.iter_mut() {
       // sprite.color = Color::srgb(0.2, 0.2, 0.2);
    }
}

#[cfg(test)]
mod tests {}
