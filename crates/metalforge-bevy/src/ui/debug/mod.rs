pub mod event;

use crate::ui::debug::event::DebugEvent;
use bevy::color::Color;
use bevy::prelude::{in_state, percent, px, App, AppExtStates, BorderColor, Commands, Component, Entity, IntoScheduleConfigs, JustifyContent, NextState, On, OnEnter, Query, Res, ResMut, State, States, Text, UiRect, Update, With};
use bevy::ui::{AlignItems, Node};
use bevy::utils::default;
use crate::ui::AppState;
use crate::ui::menu::{MenuState, MenuStructure};

#[derive(Default, States, Copy, Clone, Debug, Hash, Ord, PartialOrd, PartialEq, Eq)]
pub enum DebugState {
    #[default]
    HideDebug,
    ShowDebug,
}

#[derive(Component)]
pub struct DebugInfo;

impl DebugInfo {
    pub fn new() -> Self {
        Self
    }
}

pub fn debug(app: &mut App) {
    app
        .insert_state(DebugState::HideDebug)
        .add_systems(OnEnter(DebugState::ShowDebug), show_debug_info)
        .add_systems(OnEnter(DebugState::HideDebug), hide_debug_info)
        .add_systems(Update, update_debug_info.run_if(in_state(DebugState::ShowDebug)))
        .add_observer(handle_debug_events);
}

fn show_debug_info(mut commands: Commands) {
    let debug_info = (
        Node {
            border: UiRect::all(px(1.0)),
            width: percent(100.0),
            height: percent(100.0),
            align_items: AlignItems::FlexEnd,
            justify_content: JustifyContent::FlexEnd,
            ..default()
        },
        BorderColor::all(Color::WHITE),
        DebugInfo::new()
    );

    commands.spawn(debug_info)
        .with_children(|parent| {
            parent.spawn((
                Node {
                    ..default()
                },
                BorderColor::all(Color::srgba(1.0, 0.0, 0.0, 0.5)),
            )).with_children(|node_parent| {
                node_parent.spawn(
                    (
                    Text::new("Debug"), DebugInfo)
                );
            });
        });
}

fn hide_debug_info(mut commands: Commands, debug_info: Query<Entity, With<DebugInfo>>) {
    for entity in debug_info.iter() {
        commands.entity(entity).despawn();
    }
}

fn update_debug_info(
    app_state: Res<State<AppState>>,
    menu_state: Res<State<MenuState>>,
    menu_struct: Res<MenuStructure>,
    mut debug_info_q: Query<&mut Text, With<DebugInfo>>
) {
    for mut debug_info in &mut debug_info_q {
        debug_info.0.clear();
        debug_info.0.push_str(
            format!("{:?} {:?} {:?}",
                    app_state.get(),
                    menu_state.get(),
                    menu_struct.current_menu_id()
            ).as_str());
    }
}

fn handle_debug_events(
    event: On<DebugEvent>,
    debug_state: Res<State<DebugState>>,
    mut next_debug_state: ResMut<NextState<DebugState>>,
) {
    match event.event() {
        DebugEvent::ToggleDebugInfo =>
            toggle_debug_state(debug_state.get(), &mut next_debug_state),
    }
}

fn toggle_debug_state(
    current_state: &DebugState,
    next_state: &mut ResMut<NextState<DebugState>>,
) {
    match current_state {
        DebugState::HideDebug => {
            next_state.set(DebugState::ShowDebug);
        }
        DebugState::ShowDebug => {
            next_state.set(DebugState::HideDebug);
        }
    }
}