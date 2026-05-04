use std::sync::atomic::{AtomicU32, Ordering};

pub struct AtomicF32 {
    value: AtomicU32,
}

impl AtomicF32 {
    pub fn new(val: f32) -> Self {
        Self {
            value: AtomicU32::new(f32::to_bits(val)),
        }
    }

    pub fn set(&self, val: f32) {
        self.value.store(f32::to_bits(val), Ordering::Relaxed);
    }

    pub fn get(&self) -> f32 {
        f32::from_bits(self.value.load(Ordering::Relaxed))
    }
}
