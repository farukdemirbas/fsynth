// owns cpal stream

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{Device, SampleFormat, Stream, StreamConfig};

use crate::audio::dsp::Oscillator;

struct Instrument {
    name: String,
    oscillator: Oscillator,
    pan: f32,
    volume: f32,
    frequency: f32,
}

impl Instrument {
    pub fn new(name: &str, oscillator: Oscillator, volume: f32, frequency: f32, pan: f32) -> Self {
        Self {
            name: name.to_string(),
            oscillator,
            volume,
            frequency,
            pan,
        }
    }

    fn next_value(&mut self) -> f32 {
        // TODO: For each oscillator, get next value, accumulate them, return
        self.oscillator.next_value(self.volume, self.frequency)
    }

    pub fn apply_self(&mut self, frame: &mut [f32]) {
        let val = self.next_value();
        let pan = (self.pan + 1.0) / 2.0; // Converting [-1..1] to [0..1]
        frame[0] += val * (1.0 - pan);
        frame[1] += val * pan;
    }
}

pub struct AudioEngine {
    device: Device,
    stream: Option<Stream>,
    config: StreamConfig,
}

impl AudioEngine {
    pub fn init() -> Self {
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

        let sample_rate = config.sample_rate;
        let channels = config.channels as usize;

        println!("Audio engine initialized.");

        let mut obj = Self {
            device,
            stream: None,
            config,
        };

        obj.start_stream(sample_rate, channels);

        obj
    }

    pub fn start_stream(&mut self, sample_rate: u32, channels: usize) {
        // This state is mutated on the realtime audio thread, so the callback owns it.
        // Later UI controls should communicate with this thread via atomics or messages.
        let mut instruments = Self::create_instruments(sample_rate);

        let stream = self
            .device
            .build_output_stream(
                &self.config,
                move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                    Self::write_output_buffer(data, channels, &mut instruments);
                },
                move |err| eprintln!("Error: {:?}", err),
                None,
            )
            .expect("Could not build output stream.");

        stream.play().expect("Could not start audio stream.");
        self.stream = Some(stream);
    }

    fn create_instruments(sample_rate: u32) -> Vec<Instrument> {
        // Hardcoding some instruments for testing.
        let instrument1 = Instrument::new(
            "inst1",
            Oscillator::new(sample_rate as usize),
            0.1,
            440.0,
            0.0,
        );
        let instrument2 = Instrument::new(
            "inst2",
            Oscillator::new(sample_rate as usize),
            0.1,
            425.0,
            0.0,
        );

        vec![instrument1, instrument2]
    }

    fn write_output_buffer(data: &mut [f32], channels: usize, instruments: &mut [Instrument]) {
        for frame in data.chunks_mut(channels) {
            // Make sure frames are clean
            for sample in frame.iter_mut() {
                *sample = 0.0;
            }

            // Apply each instrument
            for instrument in instruments.iter_mut() {
                instrument.apply_self(frame);
            }
        }
    }
}

impl Default for AudioEngine {
    fn default() -> Self {
        Self::init()
    }
}
