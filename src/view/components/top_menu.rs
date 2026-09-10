use nice_plug_iced::iced::{
    widget::{column, Column, row, Row, button, Button, text, text_input, TextInput, },
    {Length, Element, Theme, Renderer},
    core,alignment
};


use crate::view::{
    components::{
        spec::{BIG_BUTTON, BIG_PAD, TEXT_BOX},
        buttons::*
    }, 
    messages::*
};
use crate::view::gui::AppGui;


impl AppGui{
    pub fn top_menu(&self, scale_factor:f32) -> Row<'_, Message> {
        row![
            app_button(scale_factor, BIG_BUTTON, "H", ButtonId::Hamburger),
            //
            app_button(scale_factor, BIG_BUTTON, "D", ButtonId::DawLock),
            //
            app_button(scale_factor, BIG_BUTTON, "M", ButtonId::Metronome),
            //
            text_io(scale_factor, "120", &self.tempo, TextEntryId::Tempo),
            //
            text_io(scale_factor, "4/4", &self.time_signature, TextEntryId::TimeSignature),
        ]
        .spacing(BIG_PAD * scale_factor)
        .align_y(alignment::Alignment::Center)
    }
}

fn text_io<'a>(scale_factor:f32, placeholder: &str, value: &str, text_entry_id: TextEntryId) -> TextInput<'a, Message, Theme, Renderer>
where
    Message: Clone,
    Theme: text_input::Catalog + 'a,
    Renderer: core::text::Renderer{
        text_input(placeholder, value)
            .on_input(move |text| Message::TextInput(text_entry_id, text))
            .on_submit(Message::TextSubmit(text_entry_id.clone()))
            .width(TEXT_BOX.0*scale_factor)
            
}

