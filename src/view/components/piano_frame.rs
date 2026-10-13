
use nice_plug_iced::iced::{
    widget::{column, Column, row, Row, Canvas, container, Container, text, button, Button, svg},
    {Length, Element, Theme, Renderer, Padding},
    core,alignment
};


use crate::view::{
    components::{buttons::app_button, spec::*, colors::*, svg::*,}, 
    messages::*,
    
};
use crate::view::gui::AppGui;


impl AppGui{
    pub fn piano_frame(&self, scale_factor:f32) -> Container<'_, Message> {
        let content = column![
            piano_menu(scale_factor),
            //
            container(text("Piano Canvas\nplaceholder"))
            .width(Length::Fill)
            .height(Length::Fill)
            .align_y(alignment::Alignment::Center)
            .align_x(alignment::Alignment::Center),

        ]
        .width(Length::Fill)
        .height(Length::Fill)
        ;

        container(content)
        .width(PIANO_ROLL_FRAME.0 * scale_factor)
        .height(PIANO_ROLL_FRAME.1*scale_factor)
        .style(|theme| ContainerStyle::TopMenu.get())
    }
}


fn piano_menu<'a>(scale_factor:f32) -> Container<'a, Message>{
    let c = &SvgStyles::Dark;
    let content = row![
        app_button(scale_factor, SMALL_BUTTON,  get_svg(AppSvg::Switch, c.clone()), ButtonId::PianoMidiFocus, ButtonStyle::Button2),
        app_button(scale_factor, SMALL_BUTTON,  get_svg(AppSvg::Algo, c.clone()), ButtonId::PianoAudioFocus, ButtonStyle::Button2),
        app_button(scale_factor, SMALL_BUTTON,  get_svg(AppSvg::Analyze, c.clone() ), ButtonId::PianoCompile, ButtonStyle::Button2),
        app_button(scale_factor, SMALL_BUTTON,  get_svg(AppSvg::Import, c.clone() ), ButtonId::PianoImport, ButtonStyle::Button2),
        app_button(scale_factor, SMALL_BUTTON,  get_svg(AppSvg::Save, c.clone()), ButtonId::PianoExport, ButtonStyle::Button2),
    ]
    .spacing(BIG_PAD*scale_factor)
    .width(Length::Fill);

    container(content)
    .align_y(alignment::Alignment::Center)
    .height(PIANO_ROLL_MENU.1 * scale_factor)
    .width(Length::Fill)
    .padding(Padding::new(0.0).left(BIG_PAD*scale_factor))
    .style(|theme| ContainerStyle::PianoMenu.get())
}
