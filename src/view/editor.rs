

use std::sync::{
    Arc,
    atomic::{AtomicUsize, AtomicBool, AtomicI32, Ordering}
};

use nice_plug::{editor::dpi::LogicalSize, prelude::*};

use rtrb::{RingBuffer, PushError, PopError, Producer, Consumer};
use triple_buffer::{TripleBuffer, Input, Output};

use crate::model::params::*;
use crate::model::regex::*;




/// AppEditor holds persistent editor and gui data 
/// that should remain even when the gui is closed in the DAW
pub struct AppEditor{
    pub params: Arc<AppParams>,
    pub probe_rx: Arc<Output<Vec<f32>>>,
    pub time_signature: String,
    pub tempo: String,
    pub re: AppRegex,

}

impl AppEditor{

    pub fn param_to_time_signature(params: &Arc<AppParams>) -> String {
        let numerator = params.time_sig_numerator.load(Ordering::Relaxed).to_string();
        let denominator = params.time_sig_denominator.load(Ordering::Relaxed).to_string();
        format!("{}/{}",numerator, denominator)
    }

    pub fn param_to_tempo(params: &Arc<AppParams>) -> String {
        format!("{:2}",params.tempo.load(Ordering::Relaxed))
    }

    pub fn valid_time_signature(&self, text: &str) -> bool {
        AppRegex::validate(&self.re.time_signature, text)
    }

    pub fn valid_tempo(&self, text: &str) -> bool {
        AppRegex::validate(&self.re.tempo, text)
    }

    pub fn set_time_signature(&mut self, text: &str){
        if self.valid_time_signature(text) {
            let caps = self.re.time_signature.captures(text).expect("Invalid time signature");
            
            let num = caps[1].parse::<i32>().expect("Invalid time signature numerator");
            let denom = caps[3].parse::<i32>().expect("Invalid time signature denominator");

            self.params.time_sig_numerator.store(num, Ordering::Relaxed);
            self.params.time_sig_denominator.store(denom, Ordering::Relaxed);
        }
    }

    pub fn set_tempo(&mut self, text: &str){
        if self.valid_tempo(text) {

            let caps = self.re.time_signature.captures(text).expect("Invalid tempo");
            let f = caps[0].parse::<f64>().expect("Time signature numerator not valid");

            self.params.tempo.store(f, Ordering::Relaxed);
            
        }
    }



    pub fn new(params: Arc<AppParams>, probe_rx: Arc<Output<Vec<f32>>>) -> Self{

        let tempo = AppEditor::param_to_tempo(&params);

        let time_signature = AppEditor::param_to_time_signature(&params);
    
        Self{
            params,
            probe_rx,
            time_signature,
            tempo,
            re: AppRegex::new(),
        }
    }


}

