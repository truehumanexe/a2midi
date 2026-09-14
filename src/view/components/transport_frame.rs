
use nice_plug_iced::iced::{
    widget::{container, Container, column, Column, row, Row, button, Button },
    {Length, Element, Theme, Renderer, },
    core,alignment
};


use crate::view::{
    components::{
        spec::{BIG_BUTTON, BIG_PAD, TRANSPORT_FRAME},
        buttons::*,
        colors::*
    }, 
    messages::*
};
use crate::view::gui::AppGui;


impl AppGui{
    pub fn transport_frame(&self, scale_factor:f32) -> Container<'_, Message> {
        let content = row![
            app_button(scale_factor, BIG_BUTTON, "<", ButtonId::Rewind, ButtonStyle::Button1),
            //
            app_button(scale_factor, BIG_BUTTON, "P", ButtonId::Play, ButtonStyle::Button1),
            //
            app_button(scale_factor, BIG_BUTTON, "A", ButtonId::RecordArm, ButtonStyle::Button1),
            //
            app_button(scale_factor, BIG_BUTTON, "S", ButtonId::Stop, ButtonStyle::Button1),
            //
            app_button(scale_factor, BIG_BUTTON, ">", ButtonId::Forward, ButtonStyle::Button1),
        ]
        .spacing(BIG_PAD * scale_factor)
        .width(Length::Shrink)
        .height(Length::Fill)
        .align_y(alignment::Vertical::Center)
        ;

        container(content)
        .width(TRANSPORT_FRAME.0 * scale_factor)
        .height(TRANSPORT_FRAME.1 * scale_factor)
        .align_x(alignment::Horizontal::Center)
        
        .style(|theme| ContainerStyle::TopMenu.get())
        
    }
}