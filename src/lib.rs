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
use triple_buffer::{TripleBuffer, Input, Output};

mod model; 
use model::{
    params::*,
    probe::*,
    clock::*,
    editor::*,
    app::*,
};

const MIN_WINDOW_SIZE: LogicalSize<f32> = LogicalSize::new(500.0, 500.0);
const RESIZE_HINT: ResizeHint = ResizeHint::resizable().with_min_logical_size(MIN_WINDOW_SIZE);



pub struct NicePlug {
    params: Arc<AppParams>,
    osc_data: Producer<f32>,

    repaint_notifier: RepaintNotifier,
    initial_editor: Option<AppEditor>,
    editor_state: Arc<EguiEditorState>,
    pub value: Arc<AtomicF32>,
    pub clock: Clock,

    
    
}


impl Default for NicePlug{
    fn default() -> Self {
        let p = Arc::new(AppParams::default());
        let exponent = match p.probe_exponent.range() {
            IntRange::Linear {min, max} => max,
            _ => 11,
        } as u32;
        let samples = 2_i32.pow(exponent) as usize;
        let v = Arc::new(AtomicF32::new(0.0));
        let a = Arc::new(App::default());
        let (producer, consumer) = RingBuffer::<f32>::new(1024);
        

        Self {
            params: p.clone(),
            osc_data: producer,
            repaint_notifier: RepaintNotifier::new(),
            initial_editor: Some(AppEditor::new( v.clone(),consumer)),
            editor_state: EguiEditorState::from_size(MIN_WINDOW_SIZE, 1.0),
            value: v.clone(),
            clock: Clock::new(16),
            // probe_data: ProbeData::new(samples),
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
            EguiNiceSettings::new().with_resize_hint(RESIZE_HINT),
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
        Some(
            tracing::subscriber::set_global_default(
                tracing_subscriber::FmtSubscriber::builder()
                    .with_max_level(if cfg!(debug_assertions) {
                        LevelFilter::DEBUG
                    } else {
                        LevelFilter::INFO
                    })
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
                // self.probe_data.push(sample.float_value())
                // println!("{}", *sample);
                // self.value.store(*sample,Ordering::Relaxed);
                if !self.osc_data.is_full(){
                    self.osc_data.push(*sample);
                }
                
                
            }
        }


        if self.clock.ready() {
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

// nice_export_clap!(App);
nice_export_vst3!(NicePlug);
