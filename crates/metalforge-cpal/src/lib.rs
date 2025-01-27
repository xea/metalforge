use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{
    Device, FromSample, InputCallbackInfo, Sample, SampleFormat, StreamError, SupportedStreamConfig,
};
use spectrum_analyzer::scaling::divide_by_N_sqrt;
use spectrum_analyzer::windows::hann_window;
use spectrum_analyzer::{samples_fft_to_spectrum, FrequencyLimit, FrequencySpectrum};
use std::time::Duration;

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

fn data_callback<T>(data: &[T], _input_callback_info: &InputCallbackInfo)
where
    T: Sample,
{
    handle_input::<T, T>(data);
}

fn data_callback_f32(data: &[f32], _input_callback_info: &InputCallbackInfo) {
    handle_input_f32(data);
}

fn handle_input<T, U>(input: &[T])
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

fn process_spectrum(spectrum: &FrequencySpectrum) {
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
