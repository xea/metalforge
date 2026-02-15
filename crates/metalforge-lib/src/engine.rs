use std::fs::File;
use std::time::Duration;
use crossbeam_channel::{Receiver, Sender};
use rodio::{Decoder, OutputStream, Sink, Source};
use rodio::source::SineWave;

pub struct Engine {
    command_rx: Receiver<EngineCommand>,
    command_tx: Sender<EngineCommand>,
    output_stream: OutputStream,
    output_sink: Sink
}

impl Engine {
    pub fn new(command_tx: Sender<EngineCommand>, command_rx: Receiver<EngineCommand>) -> Self {
        let stream_handle = rodio::OutputStreamBuilder::open_default_stream().expect("Cannot open audio stream");
        let sink = Sink::connect_new(&stream_handle.mixer());

        Self {
            command_rx,
            command_tx,
            output_stream: stream_handle,
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
            EngineCommand::Pause => self.pause(),
            EngineCommand::Resume => self.resume(),
            EngineCommand::Seek(duration) => self.seek(duration),
            EngineCommand::LoadSong => todo!(),
        }
        true
    }

    fn play_song(&self) {
        // Add a source to the sink
        /*
        let source = SineWave::new(440.0)
            .take_duration(Duration::from_secs(10))
            .amplify(0.2);

        self.output_sink.append(source);
        self.output_sink.set_speed(0.5);

        // And an overlay
        let alternative = SineWave::new(880.0)
            .take_duration(Duration::from_secs(1))
            .amplify(0.2);

        self.output_stream.mixer().add(alternative);
         */

        let file = File::open("./examples/sample_song/Sandbox.ogg")
            .expect("Failed to open OGG file");

        let ogg_source = Decoder::try_from(file)
            .expect("Failed to decode sound file");

        self.output_sink.append(ogg_source);
        //self.output_stream.mixer().add(ogg_source);
    }

    fn pause(&self) {
        self.output_sink.pause();
    }

    fn resume(&self) {
        if self.output_sink.is_paused() {
            self.output_sink.play();
        } else {
            self.play_song();
        }
    }

    fn seek(&self, duration: Duration) {
        if let Err(err) = self.output_sink.try_seek(duration) {
            println!("Failed to seek in song: {:?}", err);
        }
    }

    fn quit(&self) -> bool {
        self.output_sink.stop();
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