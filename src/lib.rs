#![allow(unused)] //removed this before merge with main

use nice_plug::{editor::dpi::LogicalSize, prelude::*};

use nice_plug_iced::{
    application, create_iced_editor,
    IcedEditor, IcedEditorState, IcedNiceContext, IcedNiceSettings, PersistentState,
    iced::{
        self, Center, PollSubNotifier, Subscription, Theme,
        widget::{Column, ProgressBar, button, column, pick_list, row, slider, text},
    },
};

use std::sync::Arc;
use rtrb::{RingBuffer, PushError, PopError, Producer, Consumer};
use triple_buffer::{TripleBuffer, Input, Output};

mod model; 
use model::params::*;


const MIN_WINDOW_SIZE: LogicalSize<f32> = LogicalSize::new(640.0, 360.0);
const RESIZE_HINT: ResizeHint = ResizeHint::resizable().with_min_logical_size(MIN_WINDOW_SIZE);
const INITIAL_SCALE_FACTOR: f32 = 1.0;


//Audio 2 MIDI
//build with: cargo nice-plug bundle a2midi
pub struct A2M {
    params: Arc<A2MParams>,

    editor_state: Arc<IcedEditorState>,

    //Single Arc Struct with all of the data that needs to be passed to the gui

    //notifier for update frames

    audio_block: Option<Producer<f32>>,
    oscilloscope_tx: Option<Input<Box<[f32]>>>,
    spectrum_tx: Option<Input<Box<[f32]>>>,

    
}


impl Default for A2M{
    fn default() -> Self {
        Self {
            params: Arc::new(A2MParams::default()),
            editor_state: IcedEditorState::from_size(MIN_WINDOW_SIZE, INITIAL_SCALE_FACTOR),
            audio_block: None,
            oscilloscope_tx: None,
            spectrum_tx: None,
        }
    }
}

impl Plugin for A2M{
    const NAME: &'static str = "Audio2MIDI";
    const VENDOR: &'static str = "Melodious Plugins";
    // You can use `env!("CARGO_PKG_HOMEPAGE")` to reference the homepage field from the
    // `Cargo.toml` file here
    const URL: &'static str = "";
    const EMAIL: &'static str = "truehuman.exe@gmail.com";

    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[
        AudioIOLayout {
            main_input_channels: NonZeroU32::new(1),
            main_output_channels: NonZeroU32::new(1),
            ..AudioIOLayout::const_default()
        },
    ];

    const MIDI_OUTPUT: MidiConfig = MidiConfig::Basic;
    const SAMPLE_ACCURATE_AUTOMATION: bool = true;
    
    type Editor = IcedEditor;
    type SysExMessage = ();
    type BackgroundTask = ();

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }

    // startup behavior, should be lightweight calls
    // define reset if needed; see docs for details
    fn activate(
        &mut self,
        _audio_io_layout: &AudioIOLayout,
        _buffer_config: &BufferConfig,
        _context: &mut impl ActivateContext<Self>,
    ) -> bool  {



        true
    }

    // This can be used for cleaning up special resources like socket connections whenever the
    // plugin is deactivated. Most plugins won't need to do anything here.
    // fn deactivate(&mut self) {}




    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        _context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        for channel_samples in buffer.iter_samples() {

            for sample in channel_samples {
                //do stuff
            }
        }

        ProcessStatus::Normal
    }

    

}

impl ClapPlugin for A2M {
    const CLAP_ID: &'static str = "com.melodious-plugins.a2midi";
    const CLAP_DESCRIPTION: Option<&'static str> = Some("An audio to MIDI conversion plugin");
    const CLAP_MANUAL_URL: Option<&'static str> = None;
    const CLAP_SUPPORT_URL: Option<&'static str> = None;
    const CLAP_FEATURES: &'static [ClapFeature] = &[
        ClapFeature::Stereo,
        ClapFeature::Mono,
        ClapFeature::Utility,
    ];
}

nice_export_clap!(A2M);