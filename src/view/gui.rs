use nice_plug_iced::{
    IcedEditor, IcedEditorState, IcedNiceContext, IcedNiceSettings, PersistentState, 
    iced::{
        self, Center, PollSubNotifier, Subscription, Theme, 
        widget::{Column, Container, Row, button, column, row, text, text_input, responsive},
        Padding, Length, alignment
    },
};

use crate::view::editor::*;
use crate::view::messages::*;
use crate::view::components::spec::*;


pub struct AppGui {
    persistent_state: PersistentState<AppEditor>,
    ctx: IcedNiceContext,
    pub tempo: String,
    pub time_signature: String,
}



impl AppGui {
    pub fn new(persistent_state: PersistentState<AppEditor>, ctx: IcedNiceContext) -> Self {
        let tempo = persistent_state.tempo.clone(); 

        let time_signature = persistent_state.time_signature.clone();
        Self {
            persistent_state,
            ctx,
            tempo,
            time_signature,
        }
    }

    pub fn theme(&self) -> Option<Theme> {
        Some(Theme::Dark)
    }

    pub fn subscription(&self) -> Subscription<Message> {
        Subscription::batch(vec![
            iced::poll_events().map(|_| Message::Poll),
            iced::window_resized().map(|_| Message::WindowResized),
        ])
    }

    /// Return the user's scale (zoom) factor. This is applied on top of the system's
    /// scale factor.
    pub fn scale_factor(&self) -> f32 {
        self.ctx.user_scale_factor()
        // self.ctx.window_size().physical.height as f32 / MIN_WINDOW_SIZE.height
    }

    // pub fn view(&self) -> Column<'_, Message> {
    pub fn view(&self) -> Container<'_, Message> {
        let params = &self.persistent_state.params;


        let body = responsive(move |bound| {

            let h = bound.height.max(BASE_HEIGHT as f32);
            let w = h * ASPECT_RATIO;
            let scale_factor = h/BASE_HEIGHT as f32;

            column![
                self.top_menu(scale_factor),
                self.probe_frame(scale_factor),
                self.piano_frame(scale_factor),
                self.transport_frame(scale_factor),
            ]
            .padding(BIG_PAD * scale_factor)
            .spacing(BIG_PAD * scale_factor)
            .height(h)
            .width(w)
            .align_x(alignment::Alignment::Center)
            .into()

        });

        // Exterior container that handles arbitrary window resizing from the DAW
        Container::new(body)
        .width(Length::Fill)
        .height(Length::Fill)
        .align_x(alignment::Alignment::Center)
        .align_y(alignment::Alignment::Center)
    
    }

}

impl AppGui{

    fn force_aspect_ratio(&self){
        let w = self.ctx.window_size().physical;



    }

    pub fn update(&mut self, message: Message) {
        let setter = self.ctx.nice_context.param_setter();
        let params = &self.persistent_state.params;

        match message {
            // Message::Poll => {
            //     self.peak_meter_db = nice_plug::util::gain_to_db(
            //         self.persistent_state.peak_meter.load(Ordering::Relaxed),
            //     );
            // }

            // Message::SetScaleFactor(scale_factor) => {
            //     self.ctx.set_user_scale_factor(scale_factor);

            //     // Note, this new scale factor should generally be stored in a config
            //     // file somewhere so that it persists across sessions.
            // }
            Message::WindowResized => {
                // Sync the current window size to the iced editor's state. This is
                // needed to persist window size across editor opens.

                self.ctx.sync_window_size();
            }
            Message::TextInput(id, value) => match id {
                TextEntryId::TimeSignature => self.time_signature = value,
                TextEntryId::Tempo => self.tempo = value,
            }
            Message::TextSubmit(id) => match id {
                TextEntryId::TimeSignature => {

                    let t = &self.time_signature;
                    let e = self.persistent_state.as_mut();

                    if(e.valid_time_signature(t)){
                        e.set_time_signature(t);
                    }else{
                        self.time_signature = AppEditor::param_to_time_signature(&e.params);
                    }

                }
                TextEntryId::Tempo => {
                    let t = &self.tempo;
                    let e = self.persistent_state.as_mut();

                    if(e.valid_tempo(t)){
                        e.set_tempo(t);
                    }else{
                        self.tempo = AppEditor::param_to_tempo(&e.params);
                    }

                }
            }

            _ => ()
        };
    }
}

