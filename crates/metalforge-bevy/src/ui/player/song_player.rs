use std::time::{Duration, Instant};
use bevy::prelude::{Resource, States};

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum PlayerState {
    Playing,
    Paused,
}

#[derive(Resource)]
pub struct SongPlayer {
    pub last_start: Instant,
    pub song_position: Duration,
    pub player_speed: f32,
    pub playing: bool
}

impl SongPlayer {

    pub fn playing(&self) -> bool {
        self.playing
    }

    pub fn rewind(&mut self) {
        self.song_position = Duration::ZERO;
        self.last_start = Instant::now();
    }

    pub fn pause(&mut self) {
        // Ignored for now
        self.playing = false;
    }

    pub fn resume(&mut self) {
        self.last_start = Instant::now();
        self.playing = true;
    }

    pub fn jump_forwards(&mut self, diff: &Duration) {
        self.song_position += *diff;
    }

    pub fn jump_backwards(&mut self, diff: &Duration) {
        self.song_position -= Duration::min(self.song_position, *diff);
    }
}

impl Default for SongPlayer {
    fn default() -> Self {
        Self {
            last_start: Instant::now(),
            song_position: Duration::ZERO,
            playing: false,
            player_speed: 1.0 // Start with normal speed by default
        }
    }
}
