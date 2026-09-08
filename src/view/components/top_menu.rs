use nice_plug_iced::iced::widget::{Column, Row, button, column, row, text, text_input};
use crate::view::messages::*;
use crate::view::gui::AppGui;

impl AppGui{
    pub fn top_menu(&self) -> Row<'_, Message> {
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

