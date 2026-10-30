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

    pub fn oscilloscope(&mut self, ui: &mut egui::Ui, dim:Dimensions, scale:f32) {
        egui::Frame::new()
            .fill(Color32::BLACK)
            .show(ui, |ui| {

                let w = dim.display.0 * scale;
                
                let (filled, empty) = self.raw_samples.pop_partial_slice(&mut self.sample_staging);

                self.osc_data.push_slice(filled);

                let step = 1.0/w as f64;

                let s: PlotPoints = self.osc_data.get_slice(0, App::ANALYSIS_WINDOW)
                        .iter()
                        .enumerate()
                        .map(|(i, val)| {
                            [i as f64 * step, val.clone() as f64]
                        })
                        .collect();

                let line = Line::new("Signal", s);

                Plot::new("Oscilloscope")
                    .show(ui, |plot_ui| plot_ui.line(line));
                });
    }

}