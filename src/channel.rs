//! ADC input channels
use crate::{ic, Ads1x1x, BitFlags as BF, Config};

mod private {
    pub trait Sealed {}
}

/// Marker type for an ADC input channel.
pub trait ChannelId<T>: private::Sealed {
    /// Get the channel.
    fn channel_id() -> ChannelSelection;
}

macro_rules! impl_channels {
    ($(#[doc = $doc:expr] $CH:ident => [$($IC:ident),+]),+ $(,)?) => {
        #[derive(Debug, Clone, Copy)]
        /// ADC input channel selection.
        pub enum ChannelSelection {
            $(
                #[doc = $doc]
                $CH,
            )+
        }

        $(
            #[doc = $doc]
            pub struct $CH;

            impl private::Sealed for $CH {}

            $(
                impl<DI, CONV, MODE> ChannelId<Ads1x1x<DI, ic::$IC, CONV, MODE>> for $CH {
                    fn channel_id() -> ChannelSelection {
                        ChannelSelection::$CH
                    }
                }
            )+
        )+
    };
}

impl_channels!(
    /// Measure signal on input channel 0 differentially to signal on input channel 1.
    DifferentialA0A1 => [Ads1013, Ads1014, Ads1015, Ads1113, Ads1114,  Ads1115],
    /// Measure signal on input channel 0 differentially to signal on input channel 3.
    DifferentialA0A3 => [Ads1015, Ads1115],
    /// Measure signal on input channel 1 differentially to signal on input channel 3.
    DifferentialA1A3 => [Ads1015, Ads1115],
    /// Measure signal on input channel 3 differentially to signal on input channel 3.
    DifferentialA2A3 => [Ads1015, Ads1115],
    /// Measure single-ended signal on input channel 0.
    SingleA0 => [Ads1015, Ads1115],
    /// Measure single-ended signal on input channel 1.
    SingleA1 => [Ads1015, Ads1115],
    /// Measure single-ended signal on input channel 2.
    SingleA2 => [Ads1015, Ads1115],
    /// Measure single-ended signal on input channel 3.
    SingleA3 => [Ads1015, Ads1115]
);

impl Config {
    pub(crate) fn with_mux_bits(&self, ch: ChannelSelection) -> Self {
        match ch {
            ChannelSelection::DifferentialA0A1 => self
                .with_low(BF::MUX2)
                .with_low(BF::MUX1)
                .with_low(BF::MUX0),
            ChannelSelection::DifferentialA0A3 => self
                .with_low(BF::MUX2)
                .with_low(BF::MUX1)
                .with_high(BF::MUX0),
            ChannelSelection::DifferentialA1A3 => self
                .with_low(BF::MUX2)
                .with_high(BF::MUX1)
                .with_low(BF::MUX0),
            ChannelSelection::DifferentialA2A3 => self
                .with_low(BF::MUX2)
                .with_high(BF::MUX1)
                .with_high(BF::MUX0),
            ChannelSelection::SingleA0 => self
                .with_high(BF::MUX2)
                .with_low(BF::MUX1)
                .with_low(BF::MUX0),
            ChannelSelection::SingleA1 => self
                .with_high(BF::MUX2)
                .with_low(BF::MUX1)
                .with_high(BF::MUX0),
            ChannelSelection::SingleA2 => self
                .with_high(BF::MUX2)
                .with_high(BF::MUX1)
                .with_low(BF::MUX0),
            ChannelSelection::SingleA3 => self
                .with_high(BF::MUX2)
                .with_high(BF::MUX1)
                .with_high(BF::MUX0),
        }
    }
}
