use std::default::Default;
use nice_plug_iced::iced::{
    Color, color, Border, border::radius, Background, Gradient, gradient::*,
    widget::{button, text_input, container, svg}
};

#[derive(Debug, Clone, Copy)]
pub struct AppColor{
    pub background: Color,
    pub top_menu: Color,
    pub piano_menu: Color,
    pub button: Color,
    pub button_pressed: Color,
    pub button_alt: Color,
    pub button_alt_pressed: Color,
    pub text_light: Color,
    pub text_dark: Color,
    pub text_panel: Color,
    pub canvas: Color,
}

pub const COLOR1:AppColor = AppColor{
    background: color!(0x646464), //dark grey
    top_menu: color!(0xcdcdcd), //light grey
    piano_menu: color!(0x9a9a9a),
    button: color!(0x575757),
    button_pressed: color!(0x141414),
    button_alt: color!(0xe3e3e3),
    button_alt_pressed: color!(0xd1d1d1),
    text_light: color!(0xcdcdcd),
    text_dark: color!(0x000000),
    text_panel: color!(0xbbbbbb),
    canvas: color!(0x303030),
};

#[derive(Debug, Clone, Copy)]
pub enum ButtonStyle{
    Button1,
    Button2,
}

impl ButtonStyle{
    pub fn get(&self, status: button::Status) -> button::Style{
        match (&self, status) {
            (ButtonStyle::Button1,button::Status::Pressed) => button::Style{
                background: Some(Background::Color(COLOR1.button_pressed)),
                text_color: COLOR1.text_light,
                ..Default::default()
            },
            (ButtonStyle::Button1,_) => button::Style{
                background: Some(Background::Color(COLOR1.button)),
                text_color: COLOR1.text_light,
                ..Default::default()
            },
            (ButtonStyle::Button2,button::Status::Pressed) => button::Style{
                background: Some(Background::Color(COLOR1.button_alt_pressed)),
                ..Default::default()
            },
            (ButtonStyle::Button2,_) => button::Style{
                background: Some(Background::Color(COLOR1.button_alt)),
                ..Default::default()
            },
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ContainerStyle{
    Background,
    TopMenu,
    Transport,
    PianoMenu,
    Message,
}

impl ContainerStyle{
    pub fn get(&self) -> container::Style{
        match &self {
            ContainerStyle::Background => container::Style{
                background: Some(Background::Color(COLOR1.background)),
                ..Default::default()
            },
            ContainerStyle::TopMenu | ContainerStyle::Transport => container::Style{
                background: Some(Background::Color(COLOR1.top_menu)),
                ..Default::default()
            },
            ContainerStyle::PianoMenu => container::Style{
                background: Some(Background::Color(COLOR1.piano_menu)),
                ..Default::default()
            },
            ContainerStyle::Message => container::Style{
                background: Some(Background::Color(COLOR1.text_panel)),
                ..Default::default()
            },
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum TextInputStyles{
    Default
}

impl TextInputStyles{
    pub fn get(&self, status: text_input::Status) -> text_input::Style{
        match (&self, status) {
            (TextInputStyles::Default,_) => text_input::Style{
                background: Background::Color(COLOR1.text_panel),
                border: Border{color: COLOR1.text_dark, width: 2., radius: radius(1)},
                icon: COLOR1.text_panel,
                placeholder: COLOR1.text_dark.scale_alpha(0.75),
                value: COLOR1.text_dark,
                selection: COLOR1.text_light,
            },
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum SvgStyles{
    Light,
    Dark,
}

impl SvgStyles{
    pub fn get(&self, status: svg::Status) -> svg::Style{
        match (&self, status) {
            (SvgStyles::Light,_) => svg::Style{
                color: Some(COLOR1.text_light),
            },
            (SvgStyles::Dark,_) => svg::Style{
                color: Some(COLOR1.text_dark),
            },
        }
    }
}
