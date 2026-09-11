use nice_plug_iced::iced::{
    widget::{button, Button},
    {Length, Element, Theme, Renderer},
    core,alignment
};


use crate::view::messages::*;

use crate::view::components::colors::*;

pub fn app_button<'a>(
    scale_factor:f32, 
    size:f32, 
    content: impl Into<Element<'a, Message, Theme, Renderer>>, 
    button_id: ButtonId,
    style: ButtonStyle
) -> Button<'a, Message, Theme, Renderer>{
    button(content)
        .on_press(Message::ButtonPress(button_id))
        .width(size * scale_factor)
        .height(size * scale_factor)
        .style(move |theme, status| style.get(status))
}