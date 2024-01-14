//! ADC input channels.
use crate::{Ads1013, Ads1014, Ads1015, Ads1113, Ads1114, Ads1115, Config};

use private::ChannelSelection;

/// Marker type for an ADC input channel.
pub trait ChannelId<T>: private::Sealed {
    /// Get the channel.
    fn channel_id() -> ChannelSelection;
}

macro_rules! impl_channels {
    ($(#[doc = $doc:expr] $CH:ident => [$($Ads:ident),+]),+ $(,)?) => {
        mod private {
            pub trait Sealed {}

            #[derive(Debug, Clone, Copy)]
            /// ADC input channel selection.
            pub enum ChannelSelection {
                $(
                    #[doc = $doc]
                    $CH,
                )+
            }
        }

        $(
            #[doc = $doc]
            pub struct $CH;

            impl private::Sealed for $CH {}

            $(
                impl<I2C, MODE> ChannelId<$Ads<I2C, MODE>> for $CH {
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
                .difference(Self::MUX2)
                .difference(Self::MUX1)
                .difference(Self::MUX0),
            ChannelSelection::DifferentialA0A3 => self
                .difference(Self::MUX2)
                .difference(Self::MUX1)
                .union(Self::MUX0),
            ChannelSelection::DifferentialA1A3 => self
                .difference(Self::MUX2)
                .union(Self::MUX1)
                .difference(Self::MUX0),
            ChannelSelection::DifferentialA2A3 => self
                .difference(Self::MUX2)
                .union(Self::MUX1)
                .union(Self::MUX0),
            ChannelSelection::SingleA0 => self
                .union(Self::MUX2)
                .difference(Self::MUX1)
                .difference(Self::MUX0),
            ChannelSelection::SingleA1 => self
                .union(Self::MUX2)
                .difference(Self::MUX1)
                .union(Self::MUX0),
            ChannelSelection::SingleA2 => self
                .union(Self::MUX2)
                .union(Self::MUX1)
                .difference(Self::MUX0),
            ChannelSelection::SingleA3 => {
                self.union(Self::MUX2).union(Self::MUX1).union(Self::MUX0)
            }
        }
    }
}
