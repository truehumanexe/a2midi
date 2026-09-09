use nice_plug::{editor::dpi::LogicalSize, editor::ResizeHint};
use nice_plug_iced::iced::Length;


/// This file define constants that represent ideal sizes
/// for various gui components based on a desktop
/// size of 1280 x 720
/// It also contains helper functions

// Main window specs
pub const HEIGHT:u32 = 500;
pub const WIDTH:u32  = 430;
pub const ASPECT_RATIO:f32 = WIDTH as f32 / HEIGHT as f32;

pub const MIN_WINDOW_SIZE: LogicalSize<f32> = LogicalSize::new(WIDTH as f32, HEIGHT as f32);
pub const RESIZE_HINT: ResizeHint = ResizeHint::resizable()
.with_min_logical_size(MIN_WINDOW_SIZE)
.with_aspect_ratio(WIDTH, HEIGHT);

pub const INITIAL_SCALE_FACTOR: f32 = 1.0;

pub fn constrained_width(width:f32) -> f32 {
    width * ASPECT_RATIO
}

// Gui component sizes
pub const BIG_PAD: f32 = 10.0;
pub const SMALL_PAD: f32 = 5.0;

//Non pad components with single values are meant to be squares

pub const BIG_BUTTON: Length = Length::Fixed(30.0);
pub const SMALL_BUTTON: Length = Length::Fixed(20.0);

//Covers the Tempo, time signature, and MIDI/frequency display boxes
pub const TEXT_BOX: (Length,Length) = (Length::Fixed(70.0),Length::Fixed(30.0));

pub const TOP_MENU_FRAME: (Length, Length) = (Length::Fixed(410.0), Length::Fixed(50.0));

pub const PROBE_FRAME:(Length, Length) = (Length::Fixed(410.0), Length::Fixed(200.0));
pub const PROBE_DISPLAY: Length = Length::Fixed(200.0);

pub const PIANO_ROLL_FRAME: (Length,Length) = (Length::Fixed(410.0), Length::Fixed(150.0));
pub const PIANO_ROLL_MENU: (Length,Length) = (Length::Fill, Length::Fixed(30.0));
pub const PIANO_ROLL_EDITOR: (Length, Length) = (Length::Fill, Length::Fill);

pub const TRANSPORT_FRAME: (Length, Length) = (Length::Fixed(410.0), Length::Fixed(50.0));