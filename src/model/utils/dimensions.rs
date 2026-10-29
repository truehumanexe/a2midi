use nice_plug::{editor::dpi::LogicalSize, editor::ResizeHint};


// This file defines app dimensions that represent ideal sizes
// for various gui components  with defaults based on a desktop
// size of 1280 x 720


#[derive(Debug, Clone, Copy)]
pub struct Dimensions {
    pub min_heigth: f32,
    pub min_width: f32,
    pub inner_width: f32,
    pub pad: (f32, f32),
    pub button: (f32, f32),
    pub text_box: (f32, f32),
    pub top_menu: (f32, f32),
    pub analysis_frame: (f32, f32),
    pub display: (f32, f32),
    pub piano_roll: (f32, f32),
    pub piano_roll_menu: (f32, f32),
    pub transport: (f32, f32),
}

impl Dimensions {

    pub const fn new(min_width: f32, min_heigth: f32) -> Self {
        let pad = (5.0, 10.0);
        let inner_width = min_width - pad.1 * 2.0;

        Self {
            min_heigth,
            min_width,
            inner_width,
            pad,
            button: (20.0, 30.0),
            text_box: (70.0, 30.0),
            top_menu: (inner_width, 50.0),
            analysis_frame: (inner_width, 200.0),
            display: (200.0, 200.0),
            piano_roll: (inner_width, 150.0),
            piano_roll_menu: (inner_width, 30.0),
            transport: (inner_width, 50.0),
        }
    }

    pub fn aspect_ratio(&self) -> f32 {
        self.min_width / self.min_heigth
    }

    pub fn min_heigth(&self) -> f32 {
        self.min_heigth
    }

    pub fn height(&self, scale: f32) -> f32 {
        self.min_heigth * scale
    }

    pub fn min_width(&self) -> f32 {
        self.min_width
    }

    pub fn width(&self, scale: f32) -> f32 {
        self.min_width * scale
    }

    pub fn constrained_width(&self, scale: f32) -> f32 {
        self.width(scale) * self.aspect_ratio()
    }

    pub fn min_window_size(&self) -> LogicalSize<f32> {
        let w = self.min_width();
        let h = self.min_heigth();
        LogicalSize::new(w, h)
    }

    pub fn resize_hint(&self) -> ResizeHint {
        let w = self.min_width();
        let h = self.min_heigth();
        ResizeHint::resizable()
        .with_min_logical_size(self.min_window_size())
        .with_aspect_ratio(w as u32 , h as u32 )
    }
}