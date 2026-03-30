use std::fs::File;
use std::path::Path;
use std::time::Duration;
use crossbeam_channel::{Receiver, Sender};
use log::{debug, error};
use rodio::{MixerDeviceSink, Player};
use rodio::decoder::DecoderBuilder;
use crate::library::Library;
use crate::library::songfile::SongFile;
use crate::song::song::Song;

/// `Engine` is responsible for handling input and output devices and managing playback.
pub struct Engine {
    command_rx: Receiver<EngineCommand>,
    command_tx: Sender<EngineCommand>,
    event_rx: Receiver<EngineEvent>,
    event_tx: Sender<EngineEvent>,
    _output_sink: MixerDeviceSink,
    output_player: Player,
    library: Library
}

impl Engine {
    pub fn new(command_tx: Sender<EngineCommand>, command_rx: Receiver<EngineCommand>, event_tx: Sender<EngineEvent>, event_rx: Receiver<EngineEvent>, library: Library) -> Self {
        let output_sink = rodio::DeviceSinkBuilder::open_default_sink()
            .expect("Cannot open audio stream");

        let player = Player::connect_new(output_sink.mixer());

        Self {
            command_rx,
            command_tx,
            event_rx,
            event_tx,
            _output_sink: output_sink,
            output_player: player,
            library
        }
    }

    pub fn create_channel(&self) -> EngineChannel {
        EngineChannel::new(self.command_tx.clone(), self.event_rx.clone())
    }

    pub fn command(&self, command: EngineCommand) {
        self.command_tx.send(command).expect("Failed to send engine command");
    }

    pub fn main_loop(&self) {
        while let Ok(command) = self.command_rx.recv() {
            if !self.handle_command(&command) {
                break;
            }
        }
    }

    fn handle_command(&self, event: &EngineCommand) -> bool {
        match event {
            EngineCommand::Quit => return self.quit(),
            EngineCommand::Pause => self.pause(),
            EngineCommand::Resume => self.resume(),
            EngineCommand::Seek(duration) => self.seek(*duration),
            EngineCommand::ChangeSpeed(speed) => self.change_speed(*speed),
            EngineCommand::LoadSong(songfile) => self.load_songfile(songfile), //self.load_song("./examples/sample_song/Sandbox-24bit-44k.ogg"),
        }
        true
    }

    fn load_songfile(&self, songfile: &SongFile) {
        self.load_song(songfile.song_path.as_str());
    }

    fn load_song<P: AsRef<Path>>(&self, path: P) {
        let file = File::open(path)
            .expect("Failed to open OGG file");

        let len = file.metadata()
            .expect("Failed to open file metadata")
            .len();

        let file_source = DecoderBuilder::new()
            .with_data(file)
            .with_byte_len(len)
            .with_gapless(true)
            .with_seekable(true)
            .build()
            .expect("Failed to create decoder for file");

        // let file_source = Decoder::try_from(file)
        //     .expect("Failed to decode sound file");

        self.output_player.clear();
        self.output_player.append(file_source);
    }

    fn pause(&self) {
        self.output_player.pause();
    }

    fn resume(&self) {
        if self.output_player.is_paused() {
            self.output_player.play();
        }
    }

    fn seek(&self, duration: Duration) {
        if let Err(err) = self.output_player.try_seek(duration) {
            error!("Failed to seek in song: {:?}", err);
        } else {
            debug!("Seeked song: {:?} and {:?}", duration, self.output_player.get_pos());
        }
    }

    fn change_speed(&self, speed: f32) {
        self.output_player.set_speed(speed);
    }

    fn quit(&self) -> bool {
        self.output_player.stop();
        false
    }
}

pub enum EngineCommand {
    LoadSong(SongFile),
    Seek(Duration),
    Pause,
    Resume,
    ChangeSpeed(f32),
    Quit
}

pub enum EngineEvent {
    LibraryReady(Library),
    SongLoaded(Song)
}

pub struct EngineChannel {
    tx: Sender<EngineCommand>,
    rx: Receiver<EngineEvent>
}

impl EngineChannel {
    pub fn new(tx: Sender<EngineCommand>, rx: Receiver<EngineEvent>) -> Self {
        Self {
            tx, rx
        }
    }

    pub fn send(&self, command: EngineCommand) {
        self.tx.send(command).expect("Failed to send engine command");
    }

    pub fn receive(&self) -> Option<EngineEvent> {
        self.rx.recv().ok()
    }

    pub fn try_receive(&self) -> Option<EngineEvent> {
        self.rx.try_recv().ok()
    }
}