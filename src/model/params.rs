use std::sync::atomic::{AtomicBool, AtomicI32, Ordering};
use nice_plug::prelude::*;

#[derive(Params)]
pub struct AppParams {

    #[persist = "tempo"]
    pub tempo: util::AtomicF64,

    #[persist = "time_sig_numerator"]
    pub time_sig_numerator: AtomicI32,

    #[persist = "time_sig_denominator"]
    pub time_sig_denominator: AtomicI32,

    #[persist = "armed"]
    pub armed: AtomicBool,
    
    #[persist = "playing"]
    pub playing: AtomicBool,

    #[persist = "metronome"]
    pub metronome_on: AtomicBool,

    #[persist = "daw_lock"]
    pub daw_lock: AtomicBool,

    #[id = "probe_exp"]
    pub probe_exponent: IntParam,

}

impl Default for AppParams{
    fn default() -> Self {
        Self {
            tempo: util::AtomicF64::new(120.0),
            time_sig_numerator: AtomicI32::new(4),
            time_sig_denominator: AtomicI32::new(4),
            armed: AtomicBool::new(false),
            playing: AtomicBool::new(false),
            metronome_on: AtomicBool::new(false),
            daw_lock: AtomicBool::new(false),
            probe_exponent: IntParam::new( "Probe Exponent",  10, IntRange::Linear{min:9, max:11}),
            
        }
    }
}