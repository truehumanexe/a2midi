use nice_plug_iced::iced::{widget::svg, Length};
use std::{env::current_dir, fs, collections::HashMap};
use crate::view::components::{colors::*};


//implement svgs with oncelock handle

const AUDIO_LINES_BYTES: &[u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/svg/audio-lines.svg"));
const CIRCLE_BYTES: &[u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/svg/circle.svg"));
const FAST_FORWARD_BYTES: &[u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/svg/fast-forward.svg"));
const FOLDER_INPUT_BYTES: &[u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/svg/folder-input.svg"));
const LOCK_KEYHOLE_BYTES: &[u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/svg/lock-keyhole.svg"));
const MENU_BYTES: &[u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/svg/menu.svg"));
const METRONOME_BYTES: &[u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/svg/metronome.svg"));
const MIDI_PORT_BYTES: &[u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/svg/midi-port.svg"));
const PLAY_BYTES: &[u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/svg/play.svg"));
const REFRESH_CW_BYTES: &[u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/svg/refresh-cw.svg"));
const REWIND_BYTES: &[u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/svg/rewind.svg"));
const SQUARE_BYTES: &[u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/svg/square.svg"));


pub enum AppSvg{
    AudioLines,
    Circle,
    FastForward,
    FolderInput,
    LockKeyhole,
    Menu,
    Metronome,
    MidiPort,
    Play,
    Refresh,
    Rewind,
    Square,
}

pub fn get_svg<'a>(name: AppSvg) -> svg::Svg<'a> {
    let bytes = match name {
        AppSvg::AudioLines => AUDIO_LINES_BYTES,
        AppSvg::Circle => CIRCLE_BYTES,
        AppSvg::FastForward => FAST_FORWARD_BYTES,
        AppSvg::FolderInput => FOLDER_INPUT_BYTES,
        AppSvg::LockKeyhole => LOCK_KEYHOLE_BYTES,
        AppSvg::Menu => MENU_BYTES,
        AppSvg::Metronome => METRONOME_BYTES,
        AppSvg::MidiPort => MIDI_PORT_BYTES,
        AppSvg::Play => PLAY_BYTES,
        AppSvg::Refresh => REFRESH_CW_BYTES,
        AppSvg::Rewind => REWIND_BYTES,
        AppSvg::Square => SQUARE_BYTES,
    };

    svg(svg::Handle::from_memory(bytes))
    .width(Length::Fill)
    .height(Length::Fill)
    .style(move |theme, status| SvgStyles::Light.get(status))
}