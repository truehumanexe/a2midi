
use crate::model::{
    clock::*,
    dimensions::*,
};

pub struct App{
    pub dimensions:Dimensions,
}

impl Default for App{
    fn default() -> Self{
        Self { 
            dimensions: Dimensions::new(500.0,430.0),
        }
    }
}

// pub fn param_to_time_signature(params: &Arc<AppParams>) -> String {
    //     let numerator = params.time_sig_numerator.load(Ordering::Relaxed).to_string();
    //     let denominator = params.time_sig_denominator.load(Ordering::Relaxed).to_string();
    //     format!("{}/{}",numerator, denominator)
    // }

    // pub fn param_to_tempo(params: &Arc<AppParams>) -> String {
    //     format!("{:2}",params.tempo.load(Ordering::Relaxed))
    // }

    // pub fn valid_time_signature(&self, text: &str) -> bool {
    //     AppRegex::validate_all(&self.re.time_signature, text)
    // }

    // pub fn valid_tempo(&self, text: &str) -> bool {
    //     // only integer portion needs to be validated
    //     // if decimal portion passes regex, it is automatically valid
    //     AppRegex::validate_n(&self.re.tempo, text, 1)
    // }

    // pub fn set_time_signature(&mut self, text: &str){
    //     if self.valid_time_signature(text) {
    //         let caps = self.re.time_signature.captures(text).expect("Invalid time signature");
            
    //         let num = caps[1].parse::<i32>().expect("Invalid time signature numerator").min(99);
    //         let denom = caps[3].parse::<i32>().expect("Invalid time signature denominator").min(99);

    //         self.params.time_sig_numerator.store(num, Ordering::Relaxed);
    //         self.params.time_sig_denominator.store(denom, Ordering::Relaxed);
    //     }
    // }

    // pub fn set_tempo(&mut self, text: &str){
    //     if self.valid_tempo(text) {

    //         let caps = self.re.tempo.captures(text).expect("Invalid tempo");
    //         let integer = caps[1].parse::<f64>().expect("Tempo not valid");
    //         let decimal = caps.get(2) //option match to protect against none group
    //         .map(|m| m.as_str().parse::<f64>().unwrap_or(0.0)) //match to string for parse functionality
    //         .unwrap_or(0.0)
    //         .min(0.99);

    //         let f = integer + decimal;

    //         self.params.tempo.store(f, Ordering::Relaxed);
            
    //     }
    // }