//! ADC input channels
use crate::{ic, BitFlags as BF, Config};

/// ADC input channel selection
pub mod channel {
    /// Measure single-ended signal on input channel 0
    pub enum SingleA0 {}
    /// Measure single-ended signal on input channel 1
    pub enum SingleA1 {}
    /// Measure single-ended signal on input channel 2
    pub enum SingleA2 {}
    /// Measure single-ended signal on input channel 3
    pub enum SingleA3 {}
    /// Measure signal on input channel 0 differentially to signal on input channel 1
    pub enum DifferentialA0A1 {}
    /// Measure signal on input channel 0 differentially to signal on input channel 3
    pub enum DifferentialA0A3 {}
    /// Measure signal on input channel 1 differentially to signal on input channel 3
    pub enum DifferentialA1A3 {}
    /// Measure signal on input channel 3 differentially to signal on input channel 3
    pub enum DifferentialA2A3 {}
}

/// ADC input channel selection
#[derive(Debug, Clone, Copy)]
pub enum ChannelSelection {
    /// Measure single-ended signal on input channel 0
    SingleA0,
    /// Measure single-ended signal on input channel 1
    SingleA1,
    /// Measure single-ended signal on input channel 2
    SingleA2,
    /// Measure single-ended signal on input channel 3
    SingleA3,
    /// Measure signal on input channel 0 differentially to signal on input channel 1
    DifferentialA0A1,
    /// Measure signal on input channel 0 differentially to signal on input channel 3
    DifferentialA0A3,
    /// Measure signal on input channel 1 differentially to signal on input channel 3
    DifferentialA1A3,
    /// Measure signal on input channel 2 differentially to signal on input channel 3
    DifferentialA2A3,
}

/// Channel selection trait.
///
/// This trait allows restricting the channels and modes the ADCs can be started in to correct ones
/// in compile-time. If you get a channel compilation error, you may be attempting to use an
/// unsuitable mode for the chip selected.
pub trait Channel<IC> {
    /// Channel ID
    const ID: ChannelSelection;
}

macro_rules! impl_channel {
    ( $IC:ident, $CH:ident ) => {
        impl crate::Channel<ic::$IC> for channel::$CH {
            const ID: ChannelSelection = ChannelSelection::$CH;
        }
    };
}

impl_channel!(Ads1013, DifferentialA0A1);
impl_channel!(Ads1113, DifferentialA0A1);

impl_channel!(Ads1014, DifferentialA0A1);
impl_channel!(Ads1114, DifferentialA0A1);

impl_channel!(Ads1015, DifferentialA0A1);
impl_channel!(Ads1015, DifferentialA0A3);
impl_channel!(Ads1015, DifferentialA1A3);
impl_channel!(Ads1015, DifferentialA2A3);
impl_channel!(Ads1015, SingleA0);
impl_channel!(Ads1015, SingleA1);
impl_channel!(Ads1015, SingleA2);
impl_channel!(Ads1015, SingleA3);

impl_channel!(Ads1115, DifferentialA0A1);
impl_channel!(Ads1115, DifferentialA0A3);
impl_channel!(Ads1115, DifferentialA1A3);
impl_channel!(Ads1115, DifferentialA2A3);
impl_channel!(Ads1115, SingleA0);
impl_channel!(Ads1115, SingleA1);
impl_channel!(Ads1115, SingleA2);
impl_channel!(Ads1115, SingleA3);

impl Config {
    pub(crate) fn with_mux_bits(&self, ch: ChannelSelection) -> Self {
        use self::ChannelSelection as CS;
        match ch {
            CS::DifferentialA0A1 => self
                .with_low(BF::MUX2)
                .with_low(BF::MUX1)
                .with_low(BF::MUX0),
            CS::DifferentialA0A3 => self
                .with_low(BF::MUX2)
                .with_low(BF::MUX1)
                .with_high(BF::MUX0),
            CS::DifferentialA1A3 => self
                .with_low(BF::MUX2)
                .with_high(BF::MUX1)
                .with_low(BF::MUX0),
            CS::DifferentialA2A3 => self
                .with_low(BF::MUX2)
                .with_high(BF::MUX1)
                .with_high(BF::MUX0),
            CS::SingleA0 => self
                .with_high(BF::MUX2)
                .with_low(BF::MUX1)
                .with_low(BF::MUX0),
            CS::SingleA1 => self
                .with_high(BF::MUX2)
                .with_low(BF::MUX1)
                .with_high(BF::MUX0),
            CS::SingleA2 => self
                .with_high(BF::MUX2)
                .with_high(BF::MUX1)
                .with_low(BF::MUX0),
            CS::SingleA3 => self
                .with_high(BF::MUX2)
                .with_high(BF::MUX1)
                .with_high(BF::MUX0),
        }
    }
}
