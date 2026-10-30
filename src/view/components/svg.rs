
use std::{env::current_dir, fs, collections::HashMap};
use crate::view::components::{colors::*};


//implement svgs with oncelock handle

const ARROW_LEFT_RIGHT_BYTES: &[u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/svg/arrow-left-right.svg"));
const AUDIO_LINES_BYTES: &[u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/svg/audio-lines.svg"));
const BRAIN_BYTES: &[u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/svg/brain.svg"));
const CIRCLE_BYTES: &[u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/svg/circle.svg"));
const CODE_BYTES: &[u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/svg/code.svg"));
const FAST_FORWARD_BYTES: &[u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/svg/fast-forward.svg"));
const FOLDER_INPUT_BYTES: &[u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/svg/folder-input.svg"));
const FOLDER_UP_BYTES: &[u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/svg/folder-up.svg"));
const LOCK_KEYHOLE_BYTES: &[u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/svg/lock-keyhole.svg"));
const MENU_BYTES: &[u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/svg/menu.svg"));
const METRONOME_BYTES: &[u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/svg/metronome.svg"));
const MIDI_PORT_BYTES: &[u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/svg/midi-port.svg"));
const PLAY_BYTES: &[u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/svg/play.svg"));
const REFRESH_CW_BYTES: &[u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/svg/refresh-cw.svg"));
const REWIND_BYTES: &[u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/svg/rewind.svg"));
const SAVE_BYTES: &[u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/svg/save.svg"));
const SQUARE_BYTES: &[u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/svg/square.svg"));

#[derive(Debug, Clone, Copy)]
pub enum AppSvg{
    Audio,
    Analyze,
    Arm,
    Algo,
    FastForward,
    Import,
    Save,
    Lock,
    Menu,
    Metronome,
    MidiPort,
    Play,
    Refresh,
    Rewind,
    Stop,
    Switch
}