use std::fs::File;
use std::time::Duration;
use crossbeam_channel::{Receiver, Sender};
use rodio::{Decoder, MixerDeviceSink, Player};

/// `Engine` is responsible for handling input and output devices and managing playback.
pub struct Engine {
    command_rx: Receiver<EngineCommand>,
    command_tx: Sender<EngineCommand>,
    _output_sink: MixerDeviceSink,
    output_player: Player
}

impl Engine {
    pub fn new(command_tx: Sender<EngineCommand>, command_rx: Receiver<EngineCommand>) -> Self {
        let output_sink = rodio::DeviceSinkBuilder::open_default_sink()
            .expect("Cannot open audio stream");

        let player = rodio::Player::connect_new(&output_sink.mixer());

        Self {
            command_rx,
            command_tx,
            _output_sink: output_sink,
            output_player: player
        }
    }

    pub fn create_channel(&self) -> EngineChannel {
        EngineChannel::new(self.command_tx.clone())
    }

    pub fn command(&self, command: EngineCommand) {
        self.command_tx.send(command).expect("Failed to send engine command");
    }

    pub fn main_loop(&self) {
        loop {
            if let Ok(command) = self.command_rx.recv() {
                if !self.handle_command(&command) {
                    break;
                }
            } else {
                // Oh no, something bad.
                break;
            }
        }
    }

    fn handle_command(&self, event: &EngineCommand) -> bool {
        match *event {
            EngineCommand::Quit => return self.quit(),
            EngineCommand::PlaySong => self.play_song(),
            EngineCommand::Pause => self.pause(),
            EngineCommand::Resume => self.resume(),
            EngineCommand::Seek(duration) => self.seek(duration),
            EngineCommand::LoadSong => todo!(),
        }
        true
    }

    fn play_song(&self) {
        // Add a source to the sink
        let path = "./examples/sample_song/Sandbox-24bit-44k.ogg";

        let file = File::open(path)
            .expect("Failed to open OGG file");

        let file_source = Decoder::try_from(file)
            .expect("Failed to decode sound file");
            // .amplify(1.0)
            // .take_duration(Duration::from_millis(10_000));

        self.output_player.append(file_source);
    }

    fn pause(&self) {
        self.output_player.pause();
    }

    fn resume(&self) {
        if self.output_player.is_paused() {
            self.output_player.play();
        } else {
            self.play_song();
        }
    }

    fn seek(&self, duration: Duration) {
        if let Err(err) = self.output_player.try_seek(duration) {
            println!("Failed to seek in song: {:?}", err);
        }
    }

    fn quit(&self) -> bool {
        self.output_player.stop();
        false
    }
}

pub enum EngineCommand {
    LoadSong,
    PlaySong,
    Seek(Duration),
    Pause,
    Resume,
    Quit
}

pub struct EngineChannel {
    tx: Sender<EngineCommand>
}

impl EngineChannel {
    pub fn new(tx: Sender<EngineCommand>) -> Self {
        Self {
            tx
        }
    }

    pub fn send(&self, command: EngineCommand) {
        self.tx.send(command).expect("Failed to send engine command");
    }
}