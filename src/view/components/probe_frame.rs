
use nice_plug_iced::iced::{
    widget::{row, Row, Canvas, container, Container, text},
    {Length, Element, Theme, Renderer},
    core,alignment
};


use crate::view::{components::spec::*, messages::*};
use crate::view::gui::AppGui;


impl AppGui{
    pub fn probe_frame(&self, scale_factor:f32) -> Container<'_, Message> {
        let content = row![
            container(text("Spectrum\nplaceholder")) // placeholder
            .width(PROBE_DISPLAY * scale_factor)
            .height(PROBE_DISPLAY * scale_factor)
            .align_y(alignment::Alignment::Center)
            .align_x(alignment::Alignment::Center),
            //
            container(text("Oscilloscope\nplaceholder")) // placeholder
            .width(PROBE_DISPLAY * scale_factor)
            .height(PROBE_DISPLAY * scale_factor)
            .align_y(alignment::Alignment::Center)
            .align_x(alignment::Alignment::Center)
            
        ].spacing(BIG_PAD * scale_factor)
        .width(Length::Shrink)
        .height(Length::Fill)
        .align_y(alignment::Alignment::Center);

        container(content)
        .width(PROBE_FRAME.0 * scale_factor)
        .height(PROBE_FRAME.1 * scale_factor)
        .center_x(Length::Fill)
    }
}