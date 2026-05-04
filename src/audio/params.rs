use std::sync::atomic::AtomicBool;

use crate::utils::sync::AtomicF32;

// audio parameters shared between the audio thread and the ui thread.
// Each Arc<AudioParam> will be cloned, one for UI and one for the Oscillator.

pub struct AudioParams {
    pub enabled: AtomicBool,
    pub amplitude: AtomicF32,
    pub frequency: AtomicF32,
    pub pan: AtomicF32,
}

impl Default for AudioParams {
    fn default() -> Self {
        Self {
            enabled: AtomicBool::new(true),
            amplitude: AtomicF32::new(0.1),
            frequency: AtomicF32::new(440.0),
            pan: AtomicF32::new(0.0),
        }
    }
}
