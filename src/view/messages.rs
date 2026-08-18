pub enum TransportControls{
    Rewind,
    Play,
    Arm,
    Forward,
}

pub enum PianoRollMenu{
    Filters,
    Algos,
    Compile,
}

pub enum TimeEdits{
    Tempo,
    TimeSignatureNumerator,
    TimeSignatureDenominator,
}


pub enum Message{
    Transport(TransportControls),
    PianoRoll(PianoRollMenu),
    Time(TimeEdits),
    DawLock,
    Metronome,
    HamburgerMenu,
}