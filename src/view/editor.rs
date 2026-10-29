

use std::sync::{
    Arc,
    atomic::{
        Ordering
    }
};

use nice_plug::{
    context::gui::GuiContext, 
};

use nice_plug_egui::{ 
    NiceEguiApp,  
    create_egui_editor, 
    resizable_window::ResizableWindow,
};

use egui::{Frame, Color32, Margin, Vec2, accesskit::Color};

use rtrb::{RingBuffer, PushError, PopError, Producer, Consumer};

use crate::model::{
    app::*,
};

use crate::model::utils::{
    clock::*,
    buffer::MirrorBuffer,
};

use crate::view::components::*;




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
    pub raw_samples: Consumer<f32>,
    pub sample_staging: Vec<f32>,
    pub osc_data: MirrorBuffer<f32> ,
    open_state: Option<OpenEditorState>,
}


impl AppEditor{
    pub fn new(raw_samples: Consumer<f32>) -> Self{
    
        Self{
            raw_samples,
            sample_staging: vec![0.0; App::ANALYSIS_WINDOW],
            osc_data: MirrorBuffer::new(App::ANALYSIS_WINDOW, 0.0),
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

        let scale = 1.0;

        let min_width = App::DIM.min_width() * scale;
        let min_height = App::DIM.min_heigth() * scale;
        let min_vec = Vec2::new(min_width, min_height);

        let Some(state) = self.open_state.as_mut() else {
            return;
        };

        let setter = state.nice_gui_ctx.param_setter();
        ui.set_min_size(min_vec);
        // ResizableWindow::new("res-wind")
        //     .min_size(min_vec)
        //     .show(ui, |ui| {

                Frame::new() //Main central frame
                .fill(Color32::BLACK)
                .show(ui, |ui |{
                    ui.set_width(min_width - App::DIM.pad.1);
                    ui.set_height(min_height - App::DIM.pad.1);

                    // Top frame
                    self.top_menu(ui, App::DIM, scale);
                    // Audio Displays
                        // self.oscilloscope(ui, min_width as f64);

                    // Midi Display

                    // Transport
                });
                
            // });
    }

}
