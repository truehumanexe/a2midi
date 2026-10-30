use egui::{Frame, Color32};

use crate::model::{
    app::App, 
    utils::dimensions::Dimensions
};
use crate::view::{
    editor::AppEditor,
    components::{analysis_scope::*, audio_scope::*,}
};


impl AppEditor {

    pub fn analysis_frame(&mut self, ui: &mut egui::Ui, dim:Dimensions, scale:f32) {
        Frame::new()
            .fill(Color32::GRAY)
            .show(ui, |ui| {

                let w = dim.analysis_frame.0 * scale;
                let h = dim.analysis_frame.1 * scale;
                ui.set_width(w);
                ui.set_height(h);

                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = dim.pad.1 * scale;
                    // self.analysis_scope(ui, dim, scale);
                    self.spectrogram(ui, dim, scale);
                    self.oscilloscope(ui, dim, scale);
                });

            });       
    }

}