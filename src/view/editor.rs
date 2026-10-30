use std::sync::{ Arc, atomic::{ Ordering }};

use nice_plug::context::gui::GuiContext;

use nice_plug_egui::{ NiceEguiApp, create_egui_editor, resizable_window::ResizableWindow,};

use egui::{ Color32, Frame, Margin, Pos2, Separator, Vec2};

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
    pub osc_rx: Consumer<f32>,
    pub spectrum_rx: Consumer<f32>,
    pub sample_staging: Vec<f32>,
    pub osc_data: MirrorBuffer<f32> ,
    open_state: Option<OpenEditorState>,
}


impl AppEditor{
    pub fn new(osc_rx: Consumer<f32>, spectrum_rx: Consumer<f32>) -> Self{
    
        Self{
            osc_rx,
            spectrum_rx,
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
        let min_height = App::DIM.min_height() * scale;
        let min_vec = Vec2::new(min_width, min_height);

        let Some(state) = self.open_state.as_mut() else {
            return;
        };

        let setter = state.nice_gui_ctx.param_setter();
        
        ResizableWindow::new("res-wind")
            .min_size(min_vec)
            .show(ui, |ui| {
                ui.set_min_size(min_vec);

                Frame::new() //Main central frame
                .outer_margin(App::DIM.pad.1)
                .show(ui, |ui |{

                    ui.vertical(|ui| {

                        ui.spacing_mut().item_spacing.y = App::DIM.pad.1 * scale;

                        self.top_menu(ui, App::DIM, scale);
                        
                        self.analysis_frame(ui, App::DIM, scale);

                        self.midi_frame(ui, App::DIM, scale);

                        self.transport_frame(ui, App::DIM, scale);

                    });

                });
                
            });
    }

}
