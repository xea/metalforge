mod arrangements;
mod main_menu;
mod settings;
mod song_library;

use crate::ui::menu::arrangements::{setup_arrangement, OnArrangements};
use crate::ui::menu::main_menu::{setup_main_menu, OnMainMenu};
use crate::ui::menu::settings::{setup_settings, OnSettingsMenu};
use crate::ui::menu::song_library::{setup_song_library, OnSongLibrary};
use crate::ui::AppState;
use bevy::app::{App, AppExit, Update};
use bevy::color::Color;
use bevy::input::ButtonInput;
use bevy::prelude::{default, in_state, Changed, Commands, Component, Entity, Event, EventReader, EventWriter, IntoScheduleConfigs, KeyCode, NextState, OnEnter, OnExit, Query, Res, ResMut, Resource, State, With};
use bevy::text::TextColor;
use bevy_ui::prelude::{Button, Text};
use bevy_ui::{FlexDirection, Interaction, Node, Val};

pub const NORMAL_COLOR: Color = Color::srgb(1., 1., 1.);
pub const HOVERED_COLOR: Color = Color::srgb(0.6, 0.6, 1.);

pub fn menu_plugin(app: &mut App) {
    let menu_handlers = (
        highlight_menu,
        handle_menu_mouse,
        handle_menu_keys,
        handle_menu_events,
    );

    app.insert_resource(MenuState::default())
        .add_event::<MenuEvent>()
        .add_systems(OnEnter(AppState::MainMenu), setup_main_menu)
        .add_systems(OnExit(AppState::MainMenu), despawn_screen::<OnMainMenu>)
        .add_systems(OnEnter(AppState::SongLibrary), setup_song_library)
        .add_systems(
            OnExit(AppState::SongLibrary),
            despawn_screen::<OnSongLibrary>,
        )
        .add_systems(OnEnter(AppState::SettingsMenu), setup_settings)
        .add_systems(
            OnExit(AppState::SettingsMenu),
            despawn_screen::<OnSettingsMenu>,
        )
        .add_systems(OnEnter(AppState::Arrangements), setup_arrangement)
        .add_systems(
            OnExit(AppState::Arrangements),
            despawn_screen::<OnArrangements>,
        )
        .add_systems(Update, menu_handlers.run_if(in_state(AppState::MainMenu)))
        .add_systems(
            Update,
            menu_handlers.run_if(in_state(AppState::SettingsMenu)),
        )
        .add_systems(
            Update,
            menu_handlers.run_if(in_state(AppState::SongLibrary)),
        )
        .add_systems(
            Update,
            menu_handlers.run_if(in_state(AppState::Arrangements)),
        );
}

#[derive(Event, Component, Copy, Clone, Default)]
pub enum MenuEvent {
    #[default]
    OpenMainMenu,
    OpenSettingsMenu,
    PrevMenuItem,
    NextMenuItem,
    FocusMenuItem(usize),
    ChooseSong(usize),
    ChooseArrangement(usize),
    Play,
    Quit,
    Todo,
    Ignore,
}

#[derive(Component, Debug)]
struct MenuIdx(usize);

#[derive(Resource, Default)]
pub(crate) struct MenuState {
    previous_idx: usize,
    selected_idx: usize,
    menu_len: usize,
    current_action: MenuEvent,
    pub selected_song_idx: usize,
    pub selected_arrangement_idx: usize,
    menu_stack: Vec<(usize, MenuEvent)>,
}

impl MenuState {
    pub fn idx_changed(&self) -> bool {
        self.selected_idx != self.previous_idx
    }

    pub fn update_selection(&mut self, new_idx: usize, new_action: MenuEvent) {
        self.previous_idx = new_idx;
        self.current_action = new_action;
    }

    pub fn select_idx(&mut self, idx: usize) {
        self.selected_idx = idx.max(0).min(self.menu_len.max(1) - 1);
    }

    pub fn select_next(&mut self) {
        self.select_idx(self.selected_idx + 1);
    }

    pub fn select_prev(&mut self) {
        self.select_idx(self.selected_idx.max(1) - 1);
    }

    pub fn push(&mut self) {
        self.menu_stack
            .push((self.selected_idx, self.current_action));
        self.select_idx(0);
    }

    pub fn pop(&mut self) {
        let (new_idx, new_action) = self.menu_stack.pop().unwrap_or((0, MenuEvent::Ignore));
        self.select_idx(new_idx);
        self.current_action = new_action;
    }
}

fn highlight_menu(
    mut query: Query<(&MenuEvent, &mut TextColor, &MenuIdx)>,
    mut menu_state: ResMut<MenuState>,
) {
    if menu_state.idx_changed() {
        for (event, mut color, menu_idx) in query.iter_mut() {
            if menu_idx.0 == menu_state.selected_idx {
                color.0 = HOVERED_COLOR;
                menu_state.update_selection(menu_idx.0, *event);
            } else {
                color.0 = NORMAL_COLOR;
            }
        }
    }
}

fn handle_menu_mouse(
    mut events: EventWriter<MenuEvent>,
    mut interactions: Query<
        (&Interaction, &MenuIdx, &MenuEvent),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, idx, event) in interactions.iter_mut() {
        match interaction {
            Interaction::Pressed => {
                events.send(*event);
            }
            Interaction::Hovered => {
                events.send(MenuEvent::FocusMenuItem(idx.0));
            }
            _ => { /* Ignore */ }
        }
    }
}

fn handle_menu_keys(
    input: Res<ButtonInput<KeyCode>>,
    current_app_state: Res<State<AppState>>,
    mut events: EventWriter<MenuEvent>,
    mut menu_state: ResMut<MenuState>,
) {
    if input.just_pressed(KeyCode::Escape) {
        match current_app_state.get() {
            AppState::MainMenu => {
                menu_state.pop();
                events.send(MenuEvent::Quit);
            }
            AppState::SettingsMenu => {
                menu_state.pop();
                events.send(MenuEvent::OpenMainMenu);
            }
            AppState::SongLibrary => {
                menu_state.pop();
                events.send(MenuEvent::OpenMainMenu);
            }
            AppState::Player => {
                unimplemented!()
            }
            AppState::Arrangements => {
                menu_state.pop();
                events.send(MenuEvent::Play);
            }
        }
    } else if input.just_pressed(KeyCode::Enter) {
        match &menu_state.current_action {
            MenuEvent::OpenMainMenu
            | MenuEvent::OpenSettingsMenu
            | MenuEvent::ChooseSong(_)
            | MenuEvent::Play => {
                menu_state.push();
            }
            _ => {}
        }
        events.send(menu_state.current_action);
    } else if input.just_pressed(KeyCode::ArrowDown) {
        events.send(MenuEvent::NextMenuItem);
    } else if input.just_pressed(KeyCode::ArrowUp) {
        events.send(MenuEvent::PrevMenuItem);
    }
}

fn handle_menu_events(
    mut events: EventReader<MenuEvent>,
    mut menu_state: ResMut<MenuState>,
    mut app_state: ResMut<NextState<AppState>>,
    mut app_exit_events: EventWriter<AppExit>,
) {
    for event in events.read() {
        match event {
            MenuEvent::OpenMainMenu => {
                app_state.set(AppState::MainMenu);
            }
            MenuEvent::OpenSettingsMenu => {
                app_state.set(AppState::SettingsMenu);
            }
            MenuEvent::PrevMenuItem => {
                menu_state.select_prev();
            }
            MenuEvent::NextMenuItem => {
                menu_state.select_next();
            }
            MenuEvent::FocusMenuItem(idx) => {
                menu_state.select_idx(*idx);
            }
            MenuEvent::ChooseSong(song_idx) => {
                menu_state.selected_song_idx = *song_idx;
                app_state.set(AppState::Arrangements);
            }
            MenuEvent::ChooseArrangement(arrangement_idx) => {
                menu_state.selected_arrangement_idx = *arrangement_idx;
                app_state.set(AppState::Player);
            }
            MenuEvent::Play => {
                menu_state.selected_idx = 0;
                app_state.set(AppState::SongLibrary);
            }
            MenuEvent::Quit => {
                println!("Exiting app...");
                app_exit_events.send(AppExit::Success);
            }
            MenuEvent::Todo => {
                println!("TODO: Feature not implemented yet");
            }
            MenuEvent::Ignore => { /* Ignore */ }
        }
    }
}

// fn switch_state_fwd(mut current_state: ResMut<NextState<AppState>>, new_state: AppState, mut menu_state: ResMut<MenuState>) {
//     menu_state.push();
// }
//
// fn switch_state_back(mut current_state: ResMut<NextState<AppState>>, new_state: AppState, mut menu_state: ResMut<MenuState>) {
//     menu_state.push();
// }

fn despawn_screen<T: Component>(mut commands: Commands, query: Query<Entity, With<T>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub struct MenuItem {
    idx: MenuIdx,
    title: String,
    event: MenuEvent,
}

impl From<(usize, &str, MenuEvent)> for MenuItem {
    fn from(value: (usize, &str, MenuEvent)) -> Self {
        MenuItem::from((value.0, value.1.to_string(), value.2))
    }
}

impl From<(usize, String, MenuEvent)> for MenuItem {
    fn from(value: (usize, String, MenuEvent)) -> Self {
        MenuItem {
            idx: MenuIdx(value.0),
            title: value.1,
            event: value.2,
        }
    }
}

pub(crate) fn setup_menu<T: Component>(
    menu_title: &str,
    menu_items: Vec<MenuItem>,
    tag: T,
    mut commands: Commands,
    mut state: ResMut<MenuState>,
) {
    state.current_action = menu_items
        .get(state.selected_idx)
        .map(|item| item.event)
        .unwrap_or(MenuEvent::Ignore);
    state.menu_len = menu_items.len();

    // Container defining the overall outline of the menu, including tagging required for screen de-spawning
    commands
        .spawn((
            tag,
            Node {
                height: Val::Percent(100.0),
                width: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                ..default()
            },
        ))
        .with_children(|parent| {
            // Draw title
            parent.spawn(Text(menu_title.to_string()));

            // Create a button for each menu item, highlighting the first element in the list
            for menu_item in menu_items.into_iter() {
                let text_color = if menu_item.idx.0 == state.selected_idx {
                    HOVERED_COLOR
                } else {
                    NORMAL_COLOR
                };

                parent.spawn((
                    Button,
                    menu_item.idx,
                    menu_item.event,
                    Text(menu_item.title),
                    TextColor(text_color),
                    //BackgroundColor(Color::from(RED)),
                    Node {
                        width: Val::Percent(70.0),
                        ..default()
                    },
                ));
            }
        });
}

#[cfg(test)]
mod tests {
    use crate::ui::menu::{handle_menu_events, handle_menu_keys, MenuEvent, MenuState};
    use crate::ui::AppState;
    use bevy::app::Update;
    use bevy::input::ButtonInput;
    use bevy::prelude::{App, AppExtStates, KeyCode};
    use bevy::state::app::StatesPlugin;

    #[test]
    fn default_menu_idx_is_zero() {
        let mut app = App::new();

        app.add_event::<MenuEvent>()
            .insert_resource(MenuState::default());

        assert_eq!(0, app.world().resource::<MenuState>().selected_idx);
    }

    fn test_handler() {}

    #[test]
    fn arrow_key_down_increments_the_currently_selected_menu_item_until_last_item() {
        let mut app = App::new();

        let mut menu_state = MenuState::default();
        menu_state.menu_len = 3;

        app.add_plugins(StatesPlugin)
            .add_event::<MenuEvent>()
            .add_systems(Update, (test_handler, handle_menu_keys, handle_menu_events))
            .init_state::<AppState>()
            .insert_resource(menu_state);

        let mut input = ButtonInput::<KeyCode>::default();

        input.press(KeyCode::ArrowDown);
        app.insert_resource(input);

        // First update, expect an increment
        app.update();

        assert_eq!(1, app.world().resource::<MenuState>().selected_idx);

        // Second update, expect an increment to the third, last item
        app.update();

        assert_eq!(2, app.world().resource::<MenuState>().selected_idx);

        // Final update, no increment is expected as we're already on the last item
        app.update();

        assert_eq!(2, app.world().resource::<MenuState>().selected_idx);

        // Clear key
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .clear();

        app.update();

        assert_eq!(2, app.world().resource::<MenuState>().selected_idx);
    }

    #[test]
    fn arrow_key_up_decrements_the_current_menu_item_until_first_item() {
        let mut app = App::new();

        let mut menu_state = MenuState::default();
        menu_state.menu_len = 3;

        app.add_plugins(StatesPlugin)
            .add_event::<MenuEvent>()
            .add_systems(Update, (test_handler, handle_menu_keys, handle_menu_events))
            .init_state::<AppState>()
            .insert_resource(menu_state);

        let mut input_down = ButtonInput::<KeyCode>::default();
        let mut input_up = ButtonInput::<KeyCode>::default();

        input_down.press(KeyCode::ArrowDown);
        input_up.press(KeyCode::ArrowUp);

        // Go down to the last item
        app.insert_resource(input_down);

        app.update();
        app.update();

        // Clear arrow down and change to arrow up
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .clear();

        app.update();

        // Iterate upwards
        app.insert_resource(input_up);

        app.update();
        assert_eq!(1, app.world().resource::<MenuState>().selected_idx);

        // Move to top element
        app.update();
        assert_eq!(0, app.world().resource::<MenuState>().selected_idx);

        // Keep moving up but expect no change as we're already at the top
        app.update();

        assert_eq!(0, app.world().resource::<MenuState>().selected_idx);

        // Clear key
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .clear();

        app.update();

        assert_eq!(0, app.world().resource::<MenuState>().selected_idx);
    }
}
