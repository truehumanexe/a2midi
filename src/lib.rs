// Audio 2 MIDI Converter
// build with: cargo nice-plug bundle a2midi

#![allow(unused)] //removed this before merge with main

use std::sync::{Arc, atomic::AtomicUsize};

use nice_plug::{editor::dpi::LogicalSize, prelude::*};

use nice_plug_iced::{
    application, create_iced_editor,
    IcedEditor, IcedEditorState, IcedNiceContext, IcedNiceSettings, PersistentState,
    iced::{
        self, Center, PollSubNotifier, Subscription, Theme,
        widget::{Column, ProgressBar, button, column, pick_list, row, slider, text},
    },
};


use rtrb::{RingBuffer, PushError, PopError, Producer, Consumer};
use triple_buffer::{TripleBuffer, Input, Output};

mod model; 
use model::{params::*, probe::*, editor::*};





pub struct App {
    params: Arc<AppParams>,

    editor_state: Arc<IcedEditorState>,

    notifer: PollSubNotifier,

    audio_write_block: Option<Producer<f32>>,
    audio_analysis_block: Option<ProbeData>,
    
    
}


impl Default for App{
    fn default() -> Self {
        Self {
            params: Arc::new(AppParams::default()),
            editor_state: IcedEditorState::from_size(MIN_WINDOW_SIZE, INITIAL_SCALE_FACTOR),
            notifer: PollSubNotifier::new(),
            audio_write_block: None,
            audio_analysis_block: None,
        }
    }
}

impl Plugin for App{
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

        let probe_size = self.params.probe_size.value() as usize;
        let v = vec![0.0f32;probe_size];
        let (mut buf_producer, mut buf_consumer) = triple_buffer::triple_buffer(&v);

        self.audio_analysis_block = Some(
            ProbeData::new(
                probe_size,
                buf_producer,
                buf_consumer,
                vec![0.0f32;probe_size],
            )
        );


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

impl ClapPlugin for App {
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

nice_export_clap!(App);