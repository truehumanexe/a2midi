use nice_plug::prelude::*;

#[derive(Debug, Enum, PartialEq, Copy, Clone)]
pub enum ChannelConfig{
    Mono,
    Stereo,
}

#[derive(Params)]
pub struct A2MParams {

    #[id = "volume"]
    pub volume: FloatParam,

    #[id = "channel"]
    pub channel: EnumParam<ChannelConfig>
}

impl Default for A2MParams{
    fn default() -> Self {
        Self {
            volume: FloatParam::new("Volume", 1.0, FloatRange::Linear{min: 0.0, max: 1.0}),
            channel: EnumParam::new("Channel Config", ChannelConfig::Mono),
        }
    }
}