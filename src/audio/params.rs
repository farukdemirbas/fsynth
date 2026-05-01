// shared ui -> audio parameters
// this the UI will change the parameters defined here, and the audio engine will read them.

// Eventually use atomics for thread-safe parameters because the UI thread and the Audio thread will both be accessing them.

pub struct AudioParams {
    // frequency
    // gain
    // enabled=true etc.
}
