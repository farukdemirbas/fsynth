// owns cpal stream

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{Device, SampleFormat, Stream, StreamConfig};

use crate::audio::daw::{Daw};

pub struct AudioEngine {
    device: Device,
    stream: Option<Stream>,
    config: StreamConfig,
}

impl AudioEngine {
    pub fn new() -> Self {
        let host = cpal::default_host();

        let device = host
            .default_output_device()
            .expect("Could not retrieve output device.");

        let supported_configs_ranges: Vec<_> = device
            .supported_output_configs()
            .expect("Error querying device supported configs.")
            .collect();

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

        let channel_count = config.channels as usize;

        println!("Audio engine initialized.");

        let mut daw = Daw::new(config.sample_rate as usize);

        let stream = device
            .build_output_stream(
                &config,
                move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                    for frame in data.chunks_mut(channel_count) {
                        daw.apply_master_output(frame);
                    }
                },
                move |err| eprintln!("Error: {:?}", err),
                None,
            )
            .expect("Could not build output stream.");

        stream.play().expect("Could not start audio stream.");

        Self {
            device,
            stream: Some(stream),
            config,
        }
    }
}

impl Default for AudioEngine {
    fn default() -> Self {
        Self::new()
    }
}
