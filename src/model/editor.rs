

use std::sync::{Arc, atomic::AtomicUsize, atomic::Ordering};
use regex::Regex;
use nice_plug::{editor::dpi::LogicalSize, prelude::*};

use rtrb::{RingBuffer, PushError, PopError, Producer, Consumer};
use triple_buffer::{TripleBuffer, Input, Output};

use crate::model::params::*;

pub const MIN_WINDOW_SIZE: LogicalSize<f32> = LogicalSize::new(640.0, 360.0);
pub const RESIZE_HINT: ResizeHint = ResizeHint::resizable().with_min_logical_size(MIN_WINDOW_SIZE);
pub const INITIAL_SCALE_FACTOR: f32 = 1.0;


/// AppEditor holds persistent editor and gui data 
/// that should remain even when the gui is closed in the DAW
pub struct AppEditor{
    pub params: Arc<AppParams>,
    pub probe_rx: Output<Vec<f32>>,
    pub numerator_text: String,
    pub denominator_text: String,
    pub tempo_text: String,
    tempo_regex: Regex,
    time_sig_regex: Regex,

}

impl AppEditor{
    pub fn new(params: Arc<AppParams>, probe_rx: Output<Vec<f32>>) -> Self{

        //match 2 or 3 digits, followed optionally by a period and 1 or two digits from the start of the string
        let tempo_regex = Regex::new(r"^(\d{2,3})(\.\d{2}){0,1}").expect("Tempo regex not compiled"); 

        //match 2 digits from the start of a string
        let time_sig_regex = Regex::new(r"^\d{1,2}").expect("Time signature regex not compiled"); 

        let tempo_text = format!("{:2}",params.tempo.load(Ordering::Relaxed));
        let numerator_text = params.time_sig_numerator.load(Ordering::Relaxed).to_string();
        let denominator_text = params.time_sig_denominator.load(Ordering::Relaxed).to_string();

        Self{
            params,
            probe_rx,
            numerator_text,
            denominator_text,
            tempo_text,
            tempo_regex,
            time_sig_regex,
        }
    }


}

