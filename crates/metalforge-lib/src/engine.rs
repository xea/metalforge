use crate::library::SongLibrary;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{Device, FromSample, InputCallbackInfo, OutputCallbackInfo, Sample, SampleFormat, StreamConfig, StreamError, SupportedStreamConfig};
use fundsp::hacker::{sine_hz, AudioUnit};
use spectrum_analyzer::scaling::divide_by_N_sqrt;
use spectrum_analyzer::windows::hann_window;
use spectrum_analyzer::{samples_fft_to_spectrum, FrequencyLimit, FrequencySpectrum};
use std::time::Duration;
use crate::loader::scan_libraries;

#[derive(Default)]
pub struct EngineBuilder {
    library_paths: Vec<String>
}

impl EngineBuilder {
    pub fn with_library_paths(&mut self, paths: &[String]) -> &Self {
        paths.iter().for_each(|path| self.library_paths.push(path.to_string()));
        self
    }

    pub fn build(&self) -> Engine {
        let song_library = scan_libraries(&self.library_paths)
            .map(SongLibrary::from)
            .unwrap_or_else(|_| SongLibrary::empty());

        Engine {
            song_library,
        }
    }
}

#[derive(Default)]
pub struct Engine {
    pub song_library: SongLibrary
}

impl Engine {
    pub fn start(&mut self) {
        let _default_host = cpal::default_host();
        let all_host_ids = cpal::available_hosts();

        // Debug info
        for host_id in all_host_ids {
            println!("Host: {}", host_id.name())
        }


    }

    pub fn stop(&mut self) {

    }
}

pub fn run_dsp() {
    let host = cpal::default_host();

    let device_out = host
        .default_output_device()
        .expect("No output device is available");

    let config = device_out
        .default_output_config()
        .expect("No default output device is available");

    run_dsp0(&device_out, &config.into());
}

fn run_dsp0(device: &Device, config: &StreamConfig) {

    let sample_rate = config.sample_rate.0 as f64;
    let channels = config.channels as usize;

    //let mut c = 0.2 * (organ_hz(midi_hz(57.0)) + organ_hz(midi_hz(61.0)) + organ_hz(midi_hz(64.0)));
    //let mut c = hammond_hz(57.0) + hammond_hz(midi_hz(61.0)) + hammond_hz(midi_hz(64.0));

    let mut c = sine_hz(440.0);

    c.set_sample_rate(sample_rate);
    c.allocate();

    let mut next_value = move || c.get_stereo();

    let err_fn = |err| eprintln!("Error on stream: {}", err);

    let stream = device.build_output_stream(config, move |output: &mut [f32], _: &OutputCallbackInfo| {
        // write_data
        for frame in output.chunks_mut(channels) {
            let sample = next_value();
            let left = f32::from_sample(sample.0);
            let right = f32::from_sample(sample.1);

            for (channel, sample) in frame.iter_mut().enumerate() {
                if channel & 1 == 0 {
                    *sample = left;
                } else {
                    *sample = right;
                }
            }
        }
    },
    err_fn,
    None).expect("Failed to build output stream");

    // Fail catastrophically if playing does not start
    stream.play().expect("Failed to play stream");

    std::thread::sleep(Duration::from_millis(50000));
}

pub fn run_demo() {
    let host = cpal::default_host();

    let device_in = host
        .default_input_device()
        .expect("No input device is available");
    let config_in = find_best_config(&device_in);

    //println!("{}{}", termion::clear::All, termion::cursor::Goto(1, 1));

    println!("Using audio host: {}", host.id().name());
    println!(
        "Using device: {}",
        device_in
            .name()
            .expect("Failed to get name of audio device")
    );

    let error_callback = move |error: StreamError| {
        eprintln!("Error: {}", error);
    };

    let stream_in = match config_in.sample_format() {
        SampleFormat::F32 => device_in
            .build_input_stream(&config_in.into(), data_callback_f32, error_callback, None)
            .expect("Failed to build input stream"),
        _sample_format => return,
    };

    stream_in.play().expect("Failed to play input stream");

    std::thread::sleep(Duration::from_millis(3000));

    drop(stream_in);

    println!("Recording is complete");
}

fn _data_callback<T>(data: &[T], _input_callback_info: &InputCallbackInfo)
where
    T: Sample,
{
    _handle_input::<T, T>(data);
}

fn data_callback_f32(data: &[f32], _input_callback_info: &InputCallbackInfo) {
    handle_input_f32(data);
}

fn _handle_input<T, U>(input: &[T])
where
    T: Sample,
    U: Sample + FromSample<T>,
{
    for &sample in input.iter() {
        let _sample: U = U::from_sample(sample);
        // handle sample
        //let r = samples_fft_to_spectrum(&hann_window, 44100, FrequencyLimit::Max(4400.0), Some(&divide_by_N_sqrt));
    }
}

fn handle_input_f32(input: &[f32]) {
    for &_sample in input.iter() {
        // handle sample
        let hann_window = hann_window(input);
        let r = samples_fft_to_spectrum(
            &hann_window,
            44100,
            FrequencyLimit::Max(4400.0),
            Some(&divide_by_N_sqrt),
        );

        match r {
            Ok(freq_spectrum) => process_spectrum(&freq_spectrum),
            Err(_) => {}
        }
    }
}

fn process_spectrum(_spectrum: &FrequencySpectrum) {
    // print!("{}Max: {:10} {:10}", termion::cursor::Goto(1, 1), spectrum.max().0, spectrum.max_fr().val());
}

fn find_best_config(device: &Device) -> SupportedStreamConfig {
    let _default_config = device
        .default_input_config()
        .expect("Failed to get default configuration");
    let supported_configs = device
        .supported_input_configs()
        .expect("Failed to get supported configurations");

    for config in supported_configs {
        return config.with_max_sample_rate();
    }

    unimplemented!()
}
