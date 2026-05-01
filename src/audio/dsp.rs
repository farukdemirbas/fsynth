// oscillator, gain, etc.
// pure audio logic here.
// this file should remain completely testable without opening an audio device

pub struct SineOsc {
    // phase
    // sample_rate
}

impl SineOsc {
    pub fn new(sample_rate: f32) -> Self {
        // ...
    }

    pub fn next_sample(&mut self, frequency: f32) -> f32 {
        // ...
    }
}
