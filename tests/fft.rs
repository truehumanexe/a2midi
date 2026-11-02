use a2midi::model::{dsp::fft::*, utils::array::argmax_pair};

/*
test all
 cargo test

test specific file
cargo test fft_spectrum

test and println
 cargo test -- --nocapture

*/

#[test]
fn test_fft_spectrum_output() {
    // 1. Generate a test signal (e.g., a simple 440 Hz tone or synthetic saw wave)
    let sample_rate = 48000.0;
    let num_samples = 1024;
    let freq = 375.0; // 1024 / 48000 / 375 = 1024 / 128 = 8; max should be at 8th index

    let mut fft = Fft::new(num_samples,num_samples/2 as usize);
    
    let mut signal = vec![0.0f32; num_samples];
    for (i, sample) in signal.iter_mut().enumerate() {
        let t = i as f32 / sample_rate;
        // Simple sine wave test first
        *sample = (2.0 * std::f32::consts::PI * freq * t).sin();
    }
    
    for value in signal.iter(){
        fft.push(*value);
    };

    fft.create_spectrum();

    let _max = argmax_pair(fft.spectrum()).unwrap();

    assert!(_max.0 == 8);
    assert!(_max.1 >= 512.0);

}