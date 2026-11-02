use realfft::{RealFftPlanner, RealToComplex};
use std::sync::Arc;

/// Generates a periodic Hann window of a given size.
pub fn generate_hann_window(frame_size: usize) -> Vec<f32> {
    (0..frame_size)
        .map(|i| {
            0.5 * (1.0 - (2.0 * std::f32::consts::PI * i as f32 / frame_size as f32).cos())
        })
        .collect()
}


pub fn saw(
    freq_hz: f32,
    sample_rate: f32,
    frame_size: usize,
) -> Vec<f32> {
    let nyquist = sample_rate / 2.0;
    
    //if sample as a float is fractional, take the full next sample for accuracy
    let period_samples = (sample_rate / freq_hz).ceil() as usize;

    assert!(
        period_samples <= frame_size,
        "Frequency {} Hz: Sample period ({} samples) exceeds the frame size ({})",
        freq_hz,
        period_samples,
        frame_size
    );

    //if harmonic is fractional, take floor to avoid rolling over nyquist
    let max_harmonic = (nyquist / freq_hz).floor() as usize;
    let mut waveform = vec![0.0 as f32; frame_size];

    for i in 0..frame_size {
        let t = i as f32 / sample_rate;
        let mut sample = 0.0;
        
        for h in 1..=max_harmonic {
            let sign = if h % 2 == 1 { 1.0 } else { -1.0 };
            let amplitude = sign / h as f32;
            sample += amplitude * (2.0 * std::f32::consts::PI * h as f32 * freq_hz * t).sin();
        }
        waveform[i] = sample;
    }

    waveform
}



pub fn generate_waveform_array(
    frequencies: &[f32],
    sample_rate: f32,
    frame_size: usize,
) -> Vec<f32> {
    let mut stacked_waveforms = Vec::with_capacity(frequencies.len() * frame_size);

    for &freq in frequencies {
        let wave = saw(freq, sample_rate, frame_size);
        stacked_waveforms.extend(wave);
    }

    stacked_waveforms
}



pub fn generate_template_array(
    waveform_array: &[f32],
    frame_size: usize,
    apply_hann: bool,
    normalize: bool,
    mut normalize_fn: impl FnMut(&mut [f32]),
) -> Vec<f32> {
    let num_frames = waveform_array.len() / frame_size;
    let nyquist_bin = frame_size / 2;

    // Setup realfft planner
    let mut planner = RealFftPlanner::<f32>::new();
    let r2c = planner.plan_fft_forward(frame_size);
    
    let window = if apply_hann {
        generate_hann_window(frame_size)
            .into_iter()
            .map(|v| v as f32)
            .collect::<Vec<_>>()
    } else {
        vec![1.0; frame_size]
    };

    let mut spectra = Vec::with_capacity(num_frames* (frame_size/2 + 1));

    for f in 0..num_frames {
        let start = f * frame_size;
        let end = start + frame_size;
        let frame_slice = &waveform_array[start..end];

        // Prepare input scratch and apply window (converting f64 input to f32 for realfft)
        let mut input_scratch: Vec<f32> = frame_slice
            .iter()
            .zip(window.iter())
            .map(|(&sample, &w)| (sample) * w)
            .collect();

        let mut output_spectrum = r2c.make_output_vec();

        // Run FFT
        r2c.process(&mut input_scratch, &mut output_spectrum).unwrap();

        // Extract positive magnitude bins up to Nyquist
        let mut mag_spectrum: Vec<f32> = output_spectrum[..nyquist_bin]
            .iter()
            .map(|c| c.norm())
            .collect();

        // Optional normalization step using the mutable closure placeholder
        if normalize {
            normalize_fn(&mut mag_spectrum);
        }

        spectra.extend(mag_spectrum);
    }

    spectra
}