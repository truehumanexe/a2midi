use std::sync::{Arc, atomic::AtomicUsize, atomic::Ordering};
use std::cell::Cell;
use triple_buffer::{Input, Output, TripleBuffer, triple_buffer};


/// struct holding variables and containers that
/// help transmit data from the audio thread
/// to the Oscilloscope and Spectrum Analyzer
/// GUI components
pub struct ProbeData{
    tx: Input<Vec<f32>>,
    pub rx: Arc<Output<Vec<f32>>>,
    staging: Vec<f32>,
    current_size: usize,
    counter: usize,
}

impl Default for ProbeData{
    fn default() -> Self {
        Self::new(1024)
    }
}

impl ProbeData{

    pub fn new(size: usize) -> Self {
        let (tx, rx) = triple_buffer(vec![0.0f32;size].as_ref());
        Self{
            tx, 
            rx: Arc::new(rx),
            staging: vec![0.0f32;size],
            current_size: size,
            counter: 0
        }
    }

    pub fn push(&mut self, val: f32) {
        self.staging[self.counter] = val;
        self.counter += 1;
        
        if self.counter >= self.current_size{
            let input = self.tx.input_buffer_mut();
            input.copy_from_slice(self.staging.as_slice());
            self.tx.publish();
            self.counter = 0;
        }

    }

    //add resize function to change current_size and clear out buffers
}