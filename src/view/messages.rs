/// Gui Message logic is such that every gui element get's
/// its own message, via a unique enum or nested enums
/// These messages should include all relevant functionality
/// All messages are grouped by overarching systems
/// e.g. Tranport controls, Time Signature, piano editor, etc.


#[derive(Debug, Clone, Copy)]
pub enum TextEntryId{
    Tempo,
    TimeSignature,
}

#[derive(Debug, Clone)]
pub enum ButtonId{
    Rewind,
    Play,
    RecordArm,
    Stop,
    Forward,
    Metronome,
    Hamburger,
    DawLock,
    PianoMidiFocus,
    PianoAudioFocus,
    PianoCompile,
    PianoImport,
    PianoExport,
}

#[derive(Debug, Clone)]
pub enum Message{
    ButtonPress(ButtonId),
    TextInput(TextEntryId, String),
    TextSubmit(TextEntryId),
    Poll,
    WindowResized,
    Tick,
    None,
}