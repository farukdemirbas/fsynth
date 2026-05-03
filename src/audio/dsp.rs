// oscillator, gain, etc.
// pure audio logic here.
// this file should remain completely testable independently

use std::f32::consts::PI;

pub struct Oscillator {
    sample_rate: f32,
    phase: f32,
    // freq: f32,
    // amplitude: f32,
}

impl Oscillator {
    pub fn new(sample_rate: usize) -> Self {
        Self {
            phase: 0.0,
            sample_rate: sample_rate as f32,
        }
    }

    pub fn next_value(&mut self, amplitude: f32, frequency: f32) -> f32 {
        self.phase += 2.0 * PI * frequency / self.sample_rate;
        if self.phase > 2.0 * PI {
            self.phase -= 2.0 * PI;
        }
        amplitude * self.phase.sin()
    }

    pub fn set_sample_rate(&mut self, sample_rate: usize) {
        self.sample_rate = sample_rate as f32;
    }

    pub fn reset_phase(&mut self) {
        self.phase = 0.0;
    }
}
