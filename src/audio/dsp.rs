// oscillator, gain, etc.
// pure audio logic here.
// this file should remain completely testable independently

use std::{f32::consts::PI, sync::Arc};

use crate::audio::params::AudioParams;

pub struct Oscillator {
    sample_rate: f32,
    phase: f32,
    audio_params: Arc<AudioParams>
}

impl Oscillator {
    pub fn new(sample_rate: usize) -> Self {
        Self {
            phase: 0.0,
            sample_rate: sample_rate as f32,
            audio_params: Arc::new(AudioParams::default())
        }
    }

    pub fn next_value(&mut self) -> f32 {
        let amplitude = self.audio_params.amplitude.get();
        let frequency = self.audio_params.frequency.get();

        self.phase += 2.0 * PI * frequency / self.sample_rate;
        if self.phase > 2.0 * PI {
            self.phase -= 2.0 * PI;
        }
        amplitude * self.phase.sin()
    }

    pub fn reset_phase(&mut self) {
        self.phase = 0.0;
    }
}
