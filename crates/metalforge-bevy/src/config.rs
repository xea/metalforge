use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub debug: DebugConfig,
    pub library: LibraryConfig
}

#[derive(Serialize, Deserialize)]
pub struct DebugConfig {
    pub show_fps: bool
}

#[derive(Serialize, Deserialize)]
pub struct LibraryConfig {
    pub paths: Vec<String>
}
