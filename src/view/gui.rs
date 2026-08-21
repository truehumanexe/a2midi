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
}

impl AppGui{
    fn top_menu(&self) -> Row<'_, Message> {
        let tempo = &self.persistent_state.tempo_text;
        let num = &self.persistent_state.numerator_text;
        let denom =  &self.persistent_state.denominator_text;
    row![
        button("|||").on_press(Message::HamburgerMenu),
        text_input("120", tempo).on_submit(Message::Time(TimeEdits::Tempo)),
        button("Met").on_press(Message::Metronome),
        text_input("4", num).on_submit(Message::Time(TimeEdits::TimeSignature)),
        text_input("4",denom).on_submit(Message::Time(TimeEdits::TimeSignature)),
        ]
    }
}

impl AppGui {
    pub fn new(persistent_state: PersistentState<AppEditor>, ctx: IcedNiceContext) -> Self {
        Self {
            persistent_state,
            ctx,
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

    

