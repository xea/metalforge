use bevy::prelude::Event;

#[derive(Event)]
pub enum DebugEvent {
    ToggleDebugInfo,
}