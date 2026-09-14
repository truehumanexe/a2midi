

use nice_plug_iced::iced::{
    widget::{container, Container, column, Column, row, Row, button, Button, text, text_input, TextInput, center },
    {Length, Element, Theme, Renderer, Padding},
    core, alignment
};


use crate::view::{
    components::{
        spec::{BIG_BUTTON, BIG_PAD, TEXT_BOX, TOP_MENU_FRAME},
        buttons::*,
        colors::*,
    }, 
    messages::*
};
use crate::view::gui::AppGui;


impl AppGui{
    pub fn top_menu(&self, scale_factor:f32) -> Container<'_, Message> {

        let content = row![
            app_button(scale_factor, BIG_BUTTON, "H", ButtonId::Hamburger, ButtonStyle::Button1),
            //
            app_button(scale_factor, BIG_BUTTON, "D", ButtonId::DawLock, ButtonStyle::Button1),
            //
            app_button(scale_factor, BIG_BUTTON, "M", ButtonId::Metronome, ButtonStyle::Button1),
            //
            text_io(scale_factor, "120", &self.tempo, TextEntryId::Tempo).style(|theme, status| TextInputStyles::Default.get(status)),
            //
            text_io(scale_factor, "4/4", &self.time_signature, TextEntryId::TimeSignature).style(|theme, status| TextInputStyles::Default.get(status)),
            //
            // text(&self.note).width(TEXT_BOX.0 * scale_factor),
        ]
        .spacing(BIG_PAD * scale_factor)
        .height(Length::Fill)
        .width(Length::Shrink)
        .align_y(alignment::Alignment::Center)
        ;

        

        container(content)
        .width(TOP_MENU_FRAME.0 * scale_factor)
        .height(TOP_MENU_FRAME.1 * scale_factor)
        .align_x(alignment::Horizontal::Center)
        .style(|theme| ContainerStyle::TopMenu.get())
        
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

