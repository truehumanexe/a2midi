use egui::{Frame, Color32};

use crate::model::{
    app::App, 
    utils::dimensions::Dimensions
};
use crate::view::editor::AppEditor;

impl AppEditor {

    pub fn transport_frame(&mut self, ui: &mut egui::Ui, dim:Dimensions, scale:f32) {
        Frame::new()
            .fill(Color32::GRAY)
            .show(ui, |ui| {

                let w = dim.transport_frame.0 * scale;
                let h = dim.transport_frame.1 * scale;
                ui.set_width(w);
                ui.set_height(h);
                
                ui.label(format!("{w} x {h}"))



            });       
    }

}