use std::sync::atomic::{AtomicBool, AtomicI32, Ordering};
use nice_plug::prelude::*;

// #[derive(Debug, Enum, PartialEq, Copy, Clone)]
// pub enum ChannelConfig{
//     Mono,
//     Stereo,
// }




#[derive(Params)]
pub struct A2MParams {

    #[persist = "tempo"]
    tempo: util::AtomicF64,

    #[persist = "time_sig_numerator"]
    time_sig_numerator: AtomicI32,

    #[persist = "time_sig_denominator"]
    time_sig_denominator: AtomicI32,

    #[persist = "armed"]
    armed: AtomicBool,
    
    #[persist = "playing"]
    playing: AtomicBool,

    #[persist = "metronome"]
    metronome_on: AtomicBool

}

impl Default for A2MParams{
    fn default() -> Self {
        Self {
            tempo: util::AtomicF64::new(120.0),
            time_sig_numerator: AtomicI32::new(4),
            time_sig_denominator: AtomicI32::new(4),
            armed: AtomicBool::new(false),
            playing: AtomicBool::new(false),
            metronome_on: AtomicBool::new(false),
            
        }
    }
}