// Audio 2 MIDI Converter
// build with: cargo nice-plug bundle a2midi

#![allow(unused)] //removed this before merge with main

use std::panic::{self, AssertUnwindSafe};
use std::fs::OpenOptions;
use std::io::Write;

use std::sync::{Arc, atomic::AtomicUsize, atomic::Ordering};
use tracing::level_filters::LevelFilter;


use nice_plug::{editor::dpi::LogicalSize, prelude::*};

use nice_plug_egui::{
    EguiEditor, EguiEditorState, EguiNiceSettings, NiceEguiApp, RepaintNotifier,
    create_egui_editor, resizable_window::ResizableWindow, widgets,
};

use rtrb::{RingBuffer, PushError, PopError, Producer, Consumer};

mod model; 
use model::{
    params::*,
    app::*,
    utils::{ clock::*,},
    dsp::fft::Fft,
};

mod view;
use view::{
    editor::*,
};



pub struct NicePlug {
    params: Arc<AppParams>,
    osc_tx: Producer<f32>,
    spectrum_tx: Producer<f32>,
    repaint_notifier: RepaintNotifier,
    initial_editor: Option<AppEditor>,
    editor_state: Arc<EguiEditorState>,
    pub value: Arc<AtomicF32>,
    pub gui_clock: Clock,
    pub fft: Fft<f32>,

    
    
}


impl Default for NicePlug{
    fn default() -> Self {
        let params = Arc::new(AppParams::default());
        let v = Arc::new(AtomicF32::new(0.0));
        
        
        let min_width = App::DIM.min_width();
        let min_heigth = App::DIM.min_height();
        let min_window_size = App::DIM.min_window_size();

        let (osc_tx, osc_rx) = RingBuffer::<f32>::new(App::ANALYSIS_WINDOW);
        let (spectrum_tx, spectrum_rx) = RingBuffer::<f32>::new(App::SPECTRUM_WINDOW);
        

        Self {
            params: params.clone(),
            osc_tx,
            spectrum_tx,
            repaint_notifier: RepaintNotifier::new(),
            initial_editor: Some(AppEditor::new(osc_rx, spectrum_rx)),
            editor_state: EguiEditorState::from_size(min_window_size, 1.0),
            value: v.clone(),
            gui_clock: Clock::new(16),
            fft: Fft::new(App::ANALYSIS_WINDOW, App::ANALYSIS_WINDOW/2)
        }
    }
}

impl NicePlug{
    fn probe_samples(&self) -> usize {
        2_i32.pow(self.params.probe_exponent.value() as u32) as usize
    }
}

impl Plugin for NicePlug{
    const NAME: &'static str = "Audio2MIDI";
    const VENDOR: &'static str = "Melodious Plugins";
    // You can use `env!("CARGO_PKG_HOMEPAGE")` to reference the homepage field from the
    // `Cargo.toml` file here
    const URL: &'static str = "";
    const EMAIL: &'static str = "";

    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[
        AudioIOLayout {
            main_input_channels: NonZeroU32::new(1),
            main_output_channels: NonZeroU32::new(1),
            ..AudioIOLayout::const_default()
        },
    ];

    const MIDI_INPUT: MidiConfig = MidiConfig::None;

    // const MIDI_OUTPUT: MidiConfig = MidiConfig::Basic;
    const SAMPLE_ACCURATE_AUTOMATION: bool = true;

    // type Editor = ();
    type Editor = EguiEditor<AppEditor>;
    type SysExMessage = ();
    type BackgroundTask = ();

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }

    fn editor(&mut self, _async_executor: AsyncExecutor<Self>) -> Option<Self::Editor> {
        create_egui_editor(
            self.editor_state.clone(),
            self.repaint_notifier.clone(),
            EguiNiceSettings::new().with_resize_hint(App::DIM.resize_hint()),
            self.initial_editor.take().unwrap(),
        )
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

    fn setup_logger() -> Option<bool> {
    // 1. Set up a panic hook that logs panics via tracing so they go into debug.log
        std::panic::set_hook(Box::new(|info| {
            let location = info.location().map_or("unknown location".to_string(), |loc| {
                format!("{}:{}:{}", loc.file(), loc.line(), loc.column())
            });
            let message = if let Some(s) = info.payload().downcast_ref::<&str>() {
                s
            } else if let Some(s) = info.payload().downcast_ref::<String>() {
                s.as_str()
            } else {
                "Box<dyn Any>"
            };

            tracing::error!(target: "panic", "PANIC at {}: {}", location, message);
        }));

        // 2. Standard tracing setup
        Some(
            tracing::subscriber::set_global_default(
                tracing_subscriber::FmtSubscriber::builder()
                    .with_max_level(LevelFilter::DEBUG) // Force DEBUG level to see everything
                    .with_writer(nice_plug::log::writer_from_env())
                    .with_ansi(false)
                    .finish(),
            )
            .is_ok(),
        )
    }
    

    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        _context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {

        for channel_samples in buffer.iter_samples() {

            for sample in channel_samples {
                if !self.osc_tx.is_full(){
                    self.osc_tx.push(*sample);
                }
                self.fft.push(*sample);
                
                
            }
        }

        self.fft.try_spectrum_creation();
        self.spectrum_tx.push_entire_slice(self.fft.spectrum());


        if self.gui_clock.ready() {
                    self.repaint_notifier.request_repaint();
        }

        ProcessStatus::Normal
    }

    fn deactivate(&mut self) {}

    

}

impl ClapPlugin for NicePlug {
    const CLAP_ID: &'static str = "com.melodious.a2midi";
    const CLAP_DESCRIPTION: Option<&'static str> = Some("An audio to MIDI conversion plugin");
    const CLAP_MANUAL_URL: Option<&'static str> = None;
    const CLAP_SUPPORT_URL: Option<&'static str> = None;
    const CLAP_FEATURES: &'static [ClapFeature] = &[
        // ClapFeature::Stereo,
        ClapFeature::Mono,
        ClapFeature::Utility,
        ClapFeature::Analyzer,
    ];
}

impl Vst3Plugin for NicePlug {
    const VST3_CLASS_ID: [u8; 16] = *b"melodious-plugin";
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] =
        &[Vst3SubCategory::Analyzer, Vst3SubCategory::Tools];
}


// nice_export_clap!(NicePlug);
nice_export_vst3!(NicePlug);
