use egui::{Margin, Vec2, Color32};

use crate::model::{
    app::App, 
    utils::dimensions::Dimensions
};
use crate::view::editor::AppEditor;

impl AppEditor {

    pub fn top_menu(&mut self, ui: &mut egui::Ui, dim:Dimensions, scale:f32) {
        egui::Frame::new()
            .fill(Color32::GRAY)
            .show(ui, |ui| {

                let w = dim.top_menu.0 * scale;
                let h = dim.top_menu.1 * scale;
                ui.set_width(w);
                ui.set_height(w);
                ui.label("Top Menu")


            });       
    }

}