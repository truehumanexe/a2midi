use egui::{Margin, Vec2, Color32, Vec2b};

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
                let h = dim.display.1 * scale;

                ui.set_width(w);
                ui.set_height(h);
                
                let (filled, empty) = self.osc_tx.pop_partial_slice(&mut self.sample_staging);

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
                    .allow_zoom(false)
                    .allow_scroll(false)
                    .allow_drag(false)
                    .auto_bounds(Vec2b::FALSE)
                    .default_x_bounds(0.0,1.0)
                    .default_y_bounds(-1.0,1.0)
                    .show_axes(false)
                    .data_aspect(0.5)
                    .show(ui, |plot_ui| plot_ui.line(line));
                });
    }


    pub fn spectrogram(&mut self, ui: &mut egui::Ui, dim:Dimensions, scale:f32) {
        egui::Frame::new()
            .fill(Color32::BLACK)
            .show(ui, |ui| {

                let w = dim.display.0 * scale;
                let h = dim.display.1 * scale;

                ui.set_width(w);
                ui.set_height(h);
                
                let (filled, empty) = self.spectrum_tx.pop_partial_slice(&mut self.sample_staging);

                let step = 1.0/w as f64;

                let s: PlotPoints = filled
                        .iter()
                        .enumerate()
                        .map(|(i, val)| {
                            [i as f64 * step, val.clone() as f64]
                        })
                        .collect();

                let line = Line::new("Magnitude", s);

                Plot::new("Spectrogram")
                    .allow_zoom(false)
                    .allow_scroll(false)
                    .allow_drag(false)
                    // .auto_bounds(Vec2b::FALSE)
                    .show_axes(false)
                    .show(ui, |plot_ui| plot_ui.line(line));
                });
    }

}