

use std::sync::{Arc, atomic::AtomicUsize};

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
    params: Arc<AppParams>,
    probe_rx: Output<Vec<f32>>,

}


