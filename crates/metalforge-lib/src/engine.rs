use std::time::Duration;
use crossbeam_channel::{bounded, Receiver, Sender};
use rodio::{OutputStream, Sink, Source};
use rodio::source::SineWave;

pub struct Engine {
    command_rx: Receiver<EngineCommand>,
    command_tx: Sender<EngineCommand>,
    _output_stream: OutputStream,
    output_sink: Sink
}

impl Engine {
    pub fn new() -> Self {
        let stream_handle = rodio::OutputStreamBuilder::open_default_stream().expect("Cannot open audio stream");
        let sink = rodio::Sink::connect_new(&stream_handle.mixer());

        let (tx, rx) = bounded(64);
        Self {
            command_rx: rx,
            command_tx: tx,
            _output_stream: stream_handle,
            output_sink: sink
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
            EngineCommand::LoadSong => todo!(),
        }
        true
    }

    fn play_song(&self) {
        // Add a source to the sink
        let source = SineWave::new(440.0).take_duration(Duration::from_secs(10)).amplify(0.2);
        self.output_sink.append(source);

        self.output_sink.set_speed(0.5);

        // And an overlay
        let alternative = SineWave::new(880.0).take_duration(Duration::from_secs(1)).amplify(0.2);
        self._output_stream.mixer().add(alternative);
    }


    fn quit(&self) -> bool {
        self.output_sink.stop();
        false
    }
}

pub enum EngineCommand {
    LoadSong,
    PlaySong,
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