#[derive(Debug, Clone, Copy)]
pub enum TransportControls{
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
pub enum TimeEdits{
    TimeSignature,
    Tempo,
}

#[derive(Debug, Clone, Copy)]
pub enum Message{
    Transport(TransportControls),
    PianoRoll(PianoRollMenu),
    Time(TimeEdits),
    DawLock,
    Metronome,
    HamburgerMenu,
    Poll,
    WindowResized,
}