

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

use rtrb::{RingBuffer, PushError, PopError, Producer, Consumer};
use triple_buffer::{TripleBuffer, Input, Output};

use crate::model::params::*;
use crate::model::regex::*;




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
    pub params: Arc<AppParams>,
    // pub probe_rx: Arc<Output<Vec<f32>>>,
    pub value: Arc<AtomicF32>,
    open_state: Option<OpenEditorState>,
}


impl AppEditor{

    // pub fn param_to_time_signature(params: &Arc<AppParams>) -> String {
    //     let numerator = params.time_sig_numerator.load(Ordering::Relaxed).to_string();
    //     let denominator = params.time_sig_denominator.load(Ordering::Relaxed).to_string();
    //     format!("{}/{}",numerator, denominator)
    // }

    // pub fn param_to_tempo(params: &Arc<AppParams>) -> String {
    //     format!("{:2}",params.tempo.load(Ordering::Relaxed))
    // }

    // pub fn valid_time_signature(&self, text: &str) -> bool {
    //     AppRegex::validate_all(&self.re.time_signature, text)
    // }

    // pub fn valid_tempo(&self, text: &str) -> bool {
    //     // only integer portion needs to be validated
    //     // if decimal portion passes regex, it is automatically valid
    //     AppRegex::validate_n(&self.re.tempo, text, 1)
    // }

    // pub fn set_time_signature(&mut self, text: &str){
    //     if self.valid_time_signature(text) {
    //         let caps = self.re.time_signature.captures(text).expect("Invalid time signature");
            
    //         let num = caps[1].parse::<i32>().expect("Invalid time signature numerator").min(99);
    //         let denom = caps[3].parse::<i32>().expect("Invalid time signature denominator").min(99);

    //         self.params.time_sig_numerator.store(num, Ordering::Relaxed);
    //         self.params.time_sig_denominator.store(denom, Ordering::Relaxed);
    //     }
    // }

    // pub fn set_tempo(&mut self, text: &str){
    //     if self.valid_tempo(text) {

    //         let caps = self.re.tempo.captures(text).expect("Invalid tempo");
    //         let integer = caps[1].parse::<f64>().expect("Tempo not valid");
    //         let decimal = caps.get(2) //option match to protect against none group
    //         .map(|m| m.as_str().parse::<f64>().unwrap_or(0.0)) //match to string for parse functionality
    //         .unwrap_or(0.0)
    //         .min(0.99);

    //         let f = integer + decimal;

    //         self.params.tempo.store(f, Ordering::Relaxed);
            
    //     }
    // }



    pub fn new(params: Arc<AppParams>, value: Arc<AtomicF32>) -> Self{
    
        Self{
            params,
            value,
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
        let Some(state) = self.open_state.as_mut() else {
            return;
        };

        let setter = state.nice_gui_ctx.param_setter();

        ResizableWindow::new("res-wind")
            .min_size(Vec2::new(500.0, 500.0))
            .show(ui, |ui| {
                egui::Frame::new()
                    .inner_margin(Margin::same(5))
                    .fill(Color32::BLACK)
                    .show(ui, |ui| {
                        let v = self.value.load(Ordering::Relaxed).to_string();
                        ui.label(v);
                    });
            });
    }

}
