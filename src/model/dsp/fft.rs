
use realfft::{RealFftPlanner, RealToComplex, num_complex::Complex, num_traits::{Zero,Float}};
use std::sync::Arc;

use crate::model::utils::buffer::MirrorBuffer;

pub struct Fft<T>
where
    T: realfft::FftNum + Float,
{
    length: usize,
    complex_spectrum: Vec<Complex<T>>,
    spectrum: Vec<T>,
    signal: MirrorBuffer<T>,
    r2c: Arc<dyn RealToComplex<T>>,
    counter: usize,
    hop_size: usize,
    // Pre-allocated scratch buffer to avoid heap allocation on the audio thread!
    input_scratch: Vec<T>,
}

impl<T> Fft<T>
where
    T: realfft::FftNum + Float,
{
    pub fn new(length: usize, hop_size: usize) -> Self {
        let mut planner = RealFftPlanner::<T>::new();
        let r2c = planner.plan_fft_forward(length);
        let complex_spectrum = r2c.make_output_vec();
        let l = complex_spectrum.len();
        let input_scratch = r2c.make_input_vec(); // Pre-allocated once here

        Self {
            length,
            complex_spectrum,
            spectrum: vec![T::zero();l],
            signal: MirrorBuffer::new(length, T::zero()),
            r2c,
            counter: 0,
            hop_size,
            input_scratch,
        }
    }

    pub fn push(&mut self, sample: T) {
        self.signal.push(sample);
        self.counter += 1;
    }

    pub fn try_spectrum_creation(&mut self) -> bool {
        if self.counter >= self.hop_size {
            
            let signal_slice = self.signal.get_slice(0, self.length); 

            self.input_scratch.copy_from_slice(signal_slice);

            self.r2c
                .process(&mut self.input_scratch, &mut self.complex_spectrum)
                .unwrap();

            self.complex_spectrum
            .iter()
            .enumerate()
            .map(|(i, c)| self.spectrum[i] = c.norm());

            self.counter = 0;
            
            true
        } else {
            false
        }
    }

    pub fn spectrum(&self) -> &[T] {
        &self.spectrum
    }
}