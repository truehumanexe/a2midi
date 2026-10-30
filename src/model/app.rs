use std::sync::{
    Arc,
    atomic::{AtomicUsize, AtomicBool, AtomicI32, Ordering}
};




use crate::model::utils::{
    regex::AppRegex,
    dimensions::*,
};

use crate::model::params::*;

pub struct App;

impl App{
    pub const DIM: Dimensions = Dimensions::new(430.0,500.0);
    pub const ANALYSIS_WINDOW:usize = 1024;
    

    pub fn param_to_time_signature(params: &Arc<AppParams>) -> String {
        let numerator = params.time_sig_numerator.load(Ordering::Relaxed).to_string();
        let denominator = params.time_sig_denominator.load(Ordering::Relaxed).to_string();
        format!("{}/{}",numerator, denominator)
    }

    pub fn param_to_tempo(params: &Arc<AppParams>) -> String {
        format!("{:2}",params.tempo.load(Ordering::Relaxed))
    }

    pub fn set_time_signature(params: &Arc<AppParams>, text: &str){
        let captures = AppRegex::TIME_SIG.captures(text);
        if AppRegex::validate_all(&captures) {
            match captures {
                Some(c) => {
                    let num = c[1].parse::<i32>().expect("Invalid time signature numerator").min(99);
                    let denom = c[3].parse::<i32>().expect("Invalid time signature denominator").min(99);

                    params.time_sig_numerator.store(num, Ordering::Relaxed);
                    params.time_sig_denominator.store(denom, Ordering::Relaxed);
                },
                None => {},
            }
        }
    }

    pub fn set_tempo(params: &Arc<AppParams>, text: &str){
        let captures = AppRegex::TEMPO.captures(text);
        if AppRegex::validate_all(&captures) {
            match captures {
                Some(c) => {
                    let integer = c[1].parse::<f64>().expect("Tempo not valid");

                    // User might not input a decimal
                    // use get() to create an option<match>
                    let decimal = c.get(2)      // Str match or None
                    .and_then(|m| m.as_str().parse::<f64>().ok())       // Decimal or None
                    .unwrap_or(0.0)     //Decimal or 0.0 default
                    .min(0.99);     //decimal or  maximum value for a decimal

                    let f = integer + decimal;

                    params.tempo.store(f, Ordering::Relaxed);
                },
                None => {},
            }
        }
    }
}

