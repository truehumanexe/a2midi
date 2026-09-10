use nice_plug_iced::iced::{
    widget::{column, Column, row, Row, button, Button },
    {Length, Element, Theme, Renderer},
    core,alignment
};


use crate::view::{
    components::{
        spec::{BIG_BUTTON, BIG_PAD, TRANSPORT_FRAME},
        buttons::*
    }, 
    messages::*
};
use crate::view::gui::AppGui;


impl AppGui{
    pub fn transport_frame(&self, scale_factor:f32) -> Row<'_, Message> {
        row![
            app_button(scale_factor, BIG_BUTTON, "<", ButtonId::Rewind),
            //
            app_button(scale_factor, BIG_BUTTON, "P", ButtonId::Play),
            //
            app_button(scale_factor, BIG_BUTTON, "A", ButtonId::RecordArm),
            //
            app_button(scale_factor, BIG_BUTTON, "S", ButtonId::Stop),
            //
            app_button(scale_factor, BIG_BUTTON, ">", ButtonId::Forward),
        ]
        .spacing(BIG_PAD * scale_factor)
        .align_y(alignment::Alignment::Center)
        .height(TRANSPORT_FRAME.1 * scale_factor)
    }
}