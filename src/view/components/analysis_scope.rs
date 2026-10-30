use egui::{Margin, Vec2, Color32};

use egui_plot::Line;
use egui_plot::Plot;
use egui_plot::PlotPoints;

use crate::model::{
    app::App, 
    utils::dimensions::Dimensions
};
use crate::view::editor::AppEditor;

impl AppEditor {

    pub fn analysis_scope(&mut self, ui: &mut egui::Ui, dim:Dimensions, scale:f32) {
        egui::Frame::new()
            .fill(Color32::BLACK)
            .show(ui, |ui| {

                let w = dim.display.0 * scale;
                let h = dim.display.1 * scale;

                ui.set_width(w);
                ui.set_height(h);
                
            });
    }

}