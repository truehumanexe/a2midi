use nice_plug_iced::iced::{
    widget::{column, Column, row, Row, Canvas, container, text, button, Button},
    {Length, Element, Theme, Renderer},
    core,alignment
};


use crate::view::{components::{buttons::app_button, spec::*}, messages::*};
use crate::view::gui::AppGui;


impl AppGui{
    pub fn piano_frame(&self, scale_factor:f32) -> Column<'_, Message> {
        column![
            piano_menu(scale_factor),
            container(text("Piano Canvas\nplaceholder"))
            .width(Length::Fill)
            .height(Length::Fill)
            .align_y(alignment::Alignment::Center)
            .align_x(alignment::Alignment::Center),

        ].width(Length::Fill)
        .height(PIANO_ROLL_FRAME.1*scale_factor)
    }
}


fn piano_menu<'a>(scale_factor:f32) -> Row<'a, Message, Theme, Renderer>{
    row![
        app_button(scale_factor, SMALL_BUTTON, "M", ButtonId::PianoMidiFocus),
        app_button(scale_factor, SMALL_BUTTON, "A", ButtonId::PianoAudioFocus),
        app_button(scale_factor, SMALL_BUTTON, "C", ButtonId::PianoCompile),
        app_button(scale_factor, SMALL_BUTTON, "I", ButtonId::PianoImport),
        app_button(scale_factor, SMALL_BUTTON, "E", ButtonId::PianoExport),
    ].spacing(BIG_PAD*scale_factor)
}
