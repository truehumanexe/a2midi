use nice_plug_iced::iced::{
    widget::{column, Column, row, Row, button, Button, text, text_input, TextInput, },
    {Length, Element, Theme, Renderer},
    core,
};


use crate::view::{components::spec::{BIG_BUTTON, BIG_PAD, TEXT_BOX}, messages::*};
use crate::view::gui::AppGui;

impl AppGui{
    pub fn top_menu(&self) -> Row<'_, Message> {
        row![
            top_menu_button("|||", ButtonId::Hamburger),
            //
            top_menu_button("D", ButtonId::DawLock),
            //
            top_menu_button("M", ButtonId::Metronome),
            //
            text_io("120", &self.tempo, TextEntryId::Tempo),
            //
            text_io("4/4", &self.time_signature, TextEntryId::TimeSignature),
        ].spacing(BIG_PAD)
    }
}

fn top_menu_button<'a>(content: impl Into<Element<'a, Message, Theme, Renderer>>, button_id: ButtonId) -> Button<'a, Message, Theme, Renderer>{
    button(content)
        .on_press(Message::ButtonPress(button_id))
        .width(BIG_BUTTON)
        .height(BIG_BUTTON)
}

fn text_io<'a>(placeholder: &str, value: &str, text_entry_id: TextEntryId) -> TextInput<'a, Message, Theme, Renderer>
where
    Message: Clone,
    Theme: text_input::Catalog + 'a,
    Renderer: core::text::Renderer{
        text_input(placeholder, value)
            .on_input(move |text| Message::TextInput(text_entry_id, text))
            .on_submit(Message::TextSubmit(text_entry_id.clone()))
            .width(TEXT_BOX.0)
}

