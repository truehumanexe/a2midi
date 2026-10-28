

use std::sync::{
    Arc,
    atomic::{AtomicUsize, AtomicBool, AtomicI32, Ordering}
};


use egui::{Margin, Vec2, Color32};
use nice_plug::{context::gui::GuiContext, editor::dpi::LogicalSize, prelude::*};
use nice_plug_egui::{
    EguiEditor, EguiEditorState, EguiNiceSettings, NiceEguiApp, RepaintNotifier,
    create_egui_editor, resizable_window::ResizableWindow, widgets,
};

use egui_plot::Line;
use egui_plot::Plot;
use egui_plot::PlotPoints;

use rtrb::{RingBuffer, PushError, PopError, Producer, Consumer};
use triple_buffer::{TripleBuffer, Input, Output};

use crate::model::{
    dimensions::*,
    app::*,
    clock::*,
};




struct OpenEditorState {
    _egui_ctx: egui::Context,
    nice_gui_ctx: GuiContext,
}

impl OpenEditorState {
    fn new(egui_ctx: egui::Context, nice_gui_ctx: GuiContext) -> Self {
        Self {
            _egui_ctx: egui_ctx,
            nice_gui_ctx,
        }
    }
}



/// AppEditor holds persistent editor and gui data 
/// that should remain even when the gui is closed in the DAW
pub struct AppEditor{
    pub value: Arc<AtomicF32>,
    // pub osc_data: Arc<Output<Vec<f32>>>,
    pub osc_data: Consumer<f32>,
    pub osc_data_temp: [f32;1024],
    pub osc_data_storage: [f32;1024],
    pub app: App,
    open_state: Option<OpenEditorState>,
}


impl AppEditor{
    pub fn new(value: Arc<AtomicF32>, osc_data: Consumer<f32>) -> Self{
    
        Self{
            value,
            osc_data,
            osc_data_temp: [0.0;1024],
            osc_data_storage: [0.0;1024],
            app: App::default(),
            open_state: None,
        }
    }


}


impl NiceEguiApp for AppEditor {
    fn build(
        &mut self,
        egui_ctx: egui::Context,
        nice_gui_ctx: GuiContext,
        _frame: &mut nice_plug_egui::Frame,
    ) -> Result<(), nice_plug_egui::baseview::HandlerError> {
        self.open_state = Some(OpenEditorState::new(egui_ctx, nice_gui_ctx));
        Ok(())
    }

    /// Called when the editor is closed. This is needed because the plugin editor window
    /// can be opened and closed multiple times.
    ///
    /// If your app holds onto an `egui::Context` context, then all copies of that context
    /// should be dropped so that egui can properly clean up.
    fn editor_closed(&mut self) {
        self.open_state = None;
    }

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut nice_plug_egui::Frame) {

        let app = &mut self.app;
        let min_width = app.dimensions.min_width();
        let min_height = app.dimensions.min_height();

        let Some(state) = self.open_state.as_mut() else {
            return;
        };

        let setter = state.nice_gui_ctx.param_setter();

        ResizableWindow::new("res-wind")
            .min_size(Vec2::new(min_width, min_height))
            .show(ui, |ui| {
                egui::Frame::new()
                    .inner_margin(Margin::same(5))
                    .fill(Color32::BLACK)
                    .show(ui, |ui| {
                        self.osc_data.pop_partial_slice(&mut self.osc_data_storage);
                        
                        let step = 1.0/min_width as f64;
                        let s: PlotPoints= self.osc_data_storage
                        .iter()
                        .enumerate()
                        .map(|(i, val)| {
                            [i as f64 * step, val.clone() as f64]
                        })
                        .collect();
                        let line = Line::new("sin", s);
                        Plot::new("my_plot")
                            // .view_aspect(2.0)
                            .show(ui, |plot_ui| plot_ui.line(line));
                        // let v = self.value.load(Ordering::Relaxed).to_string();
                        // ui.label(v);
                    });
            });
    }

}
