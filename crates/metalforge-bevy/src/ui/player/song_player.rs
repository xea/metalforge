use std::time::{Duration, Instant};
use bevy::prelude::{Resource, States};
use metalforge_lib::song::Song;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum PlayerState {
    Playing,
    Paused,
    Menu
}

#[derive(Resource)]
pub struct SongPlayer {
    pub current_song: Option<Song>,
    pub start_position: Duration,
    pub loop_position: Duration,
    pub last_start: Instant,
    pub song_position: Duration,
    pub song_duration: Duration,
    pub player_speed: f32,
    pub playing: bool
}

impl SongPlayer {

    pub fn new(song: Song) -> Self {
        let duration = song.metadata.length;

        // By default, the player should loop when the song ends
        let loop_position = duration;
        let song_duration = duration;

        Self {
            current_song: Some(song),
            start_position: Duration::ZERO,
            loop_position,
            last_start: Instant::now(),
            song_position: Duration::ZERO,
            song_duration,
            playing: false,
            player_speed: 1.0 // Start with normal speed by default
        }
    }

    pub fn reset(&mut self, song: Song) {
        let length = song.metadata.length;
        self.current_song = Some(song);
        self.start_position = Duration::ZERO;
        self.song_position = Duration::ZERO;
        self.song_duration = length;
        self.loop_position = length;
        self.last_start = Instant::now();
    }

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

    pub fn seek(&mut self, location: &Duration) {
        self.song_position = *location;
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
            current_song: None,
            start_position: Duration::ZERO,
            loop_position: Duration::ZERO,
            last_start: Instant::now(),
            song_position: Duration::ZERO,
            song_duration: Duration::ZERO,
            playing: false,
            player_speed: 1.0 // Start with normal speed by default
        }
    }
}
