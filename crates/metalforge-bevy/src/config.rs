use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub library: LibraryConfig
}

#[derive(Serialize, Deserialize)]
pub struct LibraryConfig {

    pub paths: Vec<String>

}