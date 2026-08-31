/// Gui Message logic is such that every gui element get's
/// its own message, via a unique enum or nested enums
/// These messages should include all relevant functionality
/// All messages are grouped by overarching systems
/// e.g. Tranport controls, Time Signature, piano editor, etc.

#[derive(Debug, Clone, Copy)]
pub enum TransportControl{
    Rewind,
    Play,
    Arm,
    Forward,
}

#[derive(Debug, Clone, Copy)]
pub enum PianoRollMenu{
    Filters,
    Algos,
    Compile,
}

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
    Forward,
    Metronome,
    Hamburger,
    DawLock,
}

#[derive(Debug, Clone)]
pub enum Message{
    ButtonPress(ButtonId),
    TextInput(TextEntryId, String),
    TextSubmit(TextEntryId),
    PianoRoll(PianoRollMenu),
    Poll,
    WindowResized,
}