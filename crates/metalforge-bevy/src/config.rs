use bevy::prelude::Resource;
use serde::{Deserialize, Serialize};

#[derive(Default, Resource, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppConfig {
    pub display: DisplayConfig,
    pub debug: DebugConfig,
    pub library: LibraryConfig,
}

#[derive(Default, Resource, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DisplayConfig {
    // Enable wireframe displays
    pub wireframe: bool,
    pub window_type: WindowType,
}

#[derive(Default, Resource, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DebugConfig {}

#[derive(Default, Resource, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LibraryConfig {
    pub paths: Vec<String>,
}

#[derive(Default, Resource, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum WindowType {
    Desktop,
    #[default]
    Game,
    Mobile,
}
