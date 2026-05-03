use crate::audio::{dsp::Oscillator};

pub struct Daw {
    sample_rate: usize,
    instruments: Vec<Instrument>,
}

impl Daw {
    pub fn new(sample_rate: usize) -> Self {
        Self {
            sample_rate,
            instruments: Self::create_dummy_instruments(sample_rate),
        }
    }

    pub fn get_all_instruments(&mut self) -> &mut Vec<Instrument> {
        &mut self.instruments
    }

    pub fn apply_master_output(&mut self, frame: &mut [f32]) {
        // Make sure frames are clean
        for sample in frame.iter_mut() {
            *sample = 0.0;
        }

        // Apply each instrument
        for instrument in self.instruments.iter_mut() {
            instrument.apply_self(frame);
        }
    }

    /// Temporary, for testing.
    pub fn create_dummy_instruments(sample_rate: usize) -> Vec<Instrument> {
        let instrument1 =
            Instrument::new("inst1", Oscillator::new(sample_rate), 0.1, 440.0, 0.0);
        let instrument2 =
            Instrument::new("inst2", Oscillator::new(sample_rate), 0.1, 425.0, 0.0);

        vec![instrument1, instrument2]
    }
}

pub struct Instrument {
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
