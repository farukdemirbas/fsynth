// owns cpal stream

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{SampleFormat, Stream, StreamConfig};

pub struct AudioEngine {
    stream: Stream,
    config: StreamConfig,
}

impl AudioEngine {
    pub fn new() -> Self {
        // prepare audio device/config/stream
        println!("Getting the default host...");

        let host = cpal::default_host();

        println!("Retrieving output device...");

        let device = host
            .default_output_device()
            .expect("Could not retrieve output device.");

        println!("Setting device config...");

        let supported_configs_ranges: Vec<_> = device
            .supported_output_configs()
            .expect("Error querying device supported configs.")
            .collect();

        // println!("Supported output config ranges:");
        // for supported_config_range in &supported_configs_ranges {
        //     println!("  {:?}", supported_config_range);
        // }

        let config = supported_configs_ranges
            .iter()
            .find(|supported_config_range| {
                supported_config_range.channels() == 2
                    && supported_config_range.min_sample_rate() <= 48_000
                    && supported_config_range.max_sample_rate() >= 48_000
                    && supported_config_range.sample_format() == SampleFormat::F32
            })
            .expect("Could not find supported stereo 48 kHz f32 output config.")
            .with_sample_rate(48_000)
            .config();

        println!("Building output stream...");

        let stream = device
            .build_output_stream(
                &config,
                fill_output_buffer,
                move |err| eprintln!("Error: {:?}", err),
                None,
            )
            .expect("Could not build output stream.");

        let _ = stream.play();

        println!("Audio engine initialized.");

        AudioEngine { stream, config }
    }
}

fn fill_output_buffer(data: &mut [f32], _: &cpal::OutputCallbackInfo) {
    for frame in data.chunks_mut(2) {
        // frame[0] -> left
        // frame[1] -> right
    }
}
