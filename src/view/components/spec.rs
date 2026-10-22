use nice_plug::{editor::dpi::LogicalSize, editor::ResizeHint};
use nice_plug_iced::iced::Length;


/// This file define constants that represent ideal sizes
/// for various gui components based on a desktop
/// size of 1280 x 720
/// It also contains helper functions

// Main window specs
pub const BASE_HEIGHT:u32 = 500;
pub const BASE_WIDTH:u32  = 430;
pub const ASPECT_RATIO:f32 = BASE_WIDTH as f32 / BASE_HEIGHT as f32;

pub const MIN_WINDOW_SIZE: LogicalSize<f32> = LogicalSize::new(BASE_WIDTH as f32, BASE_HEIGHT as f32);
pub const RESIZE_HINT: ResizeHint = ResizeHint::resizable()
.with_min_logical_size(MIN_WINDOW_SIZE)
.with_aspect_ratio(BASE_WIDTH, BASE_HEIGHT);

pub const INITIAL_SCALE_FACTOR: f32 = 1.0;

pub fn constrained_width(width:f32) -> f32 {
    width * ASPECT_RATIO
}

// Gui component sizes
pub const BIG_PAD: f32 = 10.0;
pub const SMALL_PAD: f32 = 5.0;
pub const INNER_WIDTH: f32 = 410.0;

//Non pad components with single values are meant to be squares

pub const BIG_BUTTON: f32 = 30.0;
pub const SMALL_BUTTON: f32 = 20.0;

//Covers the Tempo, time signature, and MIDI/frequency display boxes
pub const TEXT_BOX: (f32,f32) = (70.0,30.0);

pub const TOP_MENU_FRAME: (f32, f32) = (INNER_WIDTH, 50.0);

pub const PROBE_FRAME:(f32, f32) = (INNER_WIDTH, 200.0);
pub const PROBE_DISPLAY: f32 = 200.0;

pub const PIANO_ROLL_FRAME: (f32,f32) = (INNER_WIDTH, 150.0);
pub const PIANO_ROLL_MENU: (f32,f32) = (PIANO_ROLL_FRAME.0, 30.0);
pub const PIANO_ROLL_EDITOR: (f32, f32) = (PIANO_ROLL_FRAME.0, PIANO_ROLL_FRAME.1);

pub const TRANSPORT_FRAME: (f32, f32) = (410.0, 50.0);