use std::sync::atomic::{AtomicBool, AtomicI32, Ordering};
use std::time::{Duration, Instant};


pub struct Clock {
    last_time: Instant,
    tick_period: Duration, 
}

impl Default for Clock {
    fn default() -> Self {
        Self::new(16)
    }
}

impl Clock {
    pub fn new(tick_period_ms: u64) -> Self {
        Self {
            last_time: Instant::now(),
            tick_period: Duration::from_millis(tick_period_ms), // Fixed conversion
        }
    }

    pub fn ready(&mut self) -> bool {
        let t = Instant::now();

        if t.duration_since(self.last_time) > self.tick_period{
            self.last_time = t;
            true
        }
        else{
            false
        }
    }
}
