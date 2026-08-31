use nice_plug_iced::{
    IcedEditor, IcedEditorState, IcedNiceContext, IcedNiceSettings, PersistentState,
    iced::{
        self, Center, PollSubNotifier, Subscription, Theme,
        widget::{Column, Row, ProgressBar, button, column, pick_list, row, slider, text, text_input},
    },
};

use crate::model::editor::AppEditor;
use crate::view::messages::*;


pub struct AppGui {
    persistent_state: PersistentState<AppEditor>,
    ctx: IcedNiceContext,
    tempo: String,
    time_signature: String,
}

impl AppGui{

    fn top_menu(&self) -> Row<'_, Message> {
    row![
        button("|||").on_press(Message::ButtonPress(ButtonId::Hamburger)),
        //
        text_input("120", &self.tempo)
        .on_input(|text| Message::TextInput(TextEntryId::Tempo, text))
        .on_submit(Message::TextSubmit(TextEntryId::Tempo)),
        //
        button("Met").on_press(Message::ButtonPress(ButtonId::Metronome)),
        //
        text_input("4/4", &self.time_signature)
        .on_input(|text| Message::TextInput(TextEntryId::TimeSignature, text))
        .on_submit(Message::TextSubmit(TextEntryId::TimeSignature)),
        ]
    }
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

    pub fn view(&self) -> Column<'_, Message> {
        let params = &self.persistent_state.params;

        // let scale_opts = [
        //     ScaleOption(0.5),
        //     ScaleOption(0.75),
        //     ScaleOption(1.0),
        //     ScaleOption(1.25),
        //     ScaleOption(1.5),
        //     ScaleOption(1.75),
        //     ScaleOption(2.0),
        // ];

        column![
            self.top_menu(),
        ]
        .padding(20)
        .spacing(12.0)
        .align_x(Center)
    
}
}

    

