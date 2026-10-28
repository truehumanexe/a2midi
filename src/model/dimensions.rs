use nice_plug::{editor::dpi::LogicalSize, editor::ResizeHint};


/// This file define constants that represent ideal sizes
/// for various gui components based on a desktop
/// size of 1280 x 720
/// It also contains helper functions

// Main window specs



// Gui component sizes
pub const BIG_PAD: f32 = 10.0;
pub const SMALL_PAD: f32 = 5.0;
pub const INNER_WIDTH: f32 = 410.0;

//Non pad components with single values are meant to be squares

pub const BIG_BUTTON: f32 = 30.0;
pub const SMALL_BUTTON: f32 = 20.0;

//Covers the Tempo, time signature, and MIDI/frequency display boxes
pub const TEXT_BOX_WIDTH: f32 = 70.0;
pub const TEXT_BOX_HEIGTH: f32 = 30.0;

pub const TOP_MENU_FRAME_HEIGTH: f32 = 50.0;

pub const PROBE_FRAME_HEIGTH: f32 = 200.0;
pub const PROBE_DISPLAY: f32 = 200.0;

pub const PIANO_ROLL_FRAME_HEIGTH: f32 = 150.0;
pub const PIANO_ROLL_MENU: f32 = 30.0;

pub const TRANSPORT_FRAME_HEIGTH: f32 = 50.0;


#[derive(Debug, Clone, Copy)]
pub struct Dimensions{
    min_heigth:f32,
    min_width:f32,
    pub scale:f32,
}

impl Default for Dimensions{
    fn default() -> Self{
        let h = 500.0;
        let w = 430.0;
        Self{
            min_heigth: h,
            min_width: w,
            scale: 1.0,
        }
    }
}

impl Dimensions{

    pub fn new(min_width:f32, min_heigth:f32) -> Self{
        Self { 
            min_heigth, 
            min_width, 
            ..Dimensions::default()
        }
    }

    pub fn aspect_ratio(&self) -> f32 {
        self.min_width as f32 / self.min_heigth as f32
    }

    pub fn height(&self) -> f32 {
        self.min_heigth as f32 * self.scale
    }

    pub fn min_height(&self) -> f32 {
        self.min_heigth
    }

    pub fn width(&self) -> f32 {
        self.min_width as f32 * self.scale
    }

    pub fn min_width(&self) -> f32 {
        self.min_width
    }

    pub fn constrained_width(&self) -> f32 {
        self.width() * self.aspect_ratio()
    }

    pub fn min_window_size(&self) -> LogicalSize<f32> {
        LogicalSize::new(self.min_width, self.min_heigth)
    }

    pub fn resize_hint(&self) -> ResizeHint {
        ResizeHint::resizable()
        .with_min_logical_size(self.min_window_size())
        .with_aspect_ratio(self.min_width as u32, self.min_heigth as u32)
    }

}