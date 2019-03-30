//! ADC input channels
use {hal, ic, Ads1x1x, BitFlags as BF, Config};

/// ADC input channel selection
#[allow(dead_code)]
pub mod channel {
    /// Measure single-ended signal on input channel 0
    pub struct SingleA0;
    /// Measure single-ended signal on input channel 1
    pub struct SingleA1;
    /// Measure single-ended signal on input channel 2
    pub struct SingleA2;
    /// Measure single-ended signal on input channel 3
    pub struct SingleA3;
    /// Measure signal on input channel 0 differentially to signal on input channel 1
    pub struct DifferentialA0A1;
    /// Measure signal on input channel 0 differentially to signal on input channel 3
    pub struct DifferentialA0A3;
    /// Measure signal on input channel 1 differentially to signal on input channel 3
    pub struct DifferentialA1A3;
    /// Measure signal on input channel 3 differentially to signal on input channel 3
    pub struct DifferentialA2A3;
}

#[derive(Debug, Clone, Copy)]
pub enum ChannelSelection {
    SingleA0,
    SingleA1,
    SingleA2,
    SingleA3,
    DifferentialA0A1,
    DifferentialA0A3,
    DifferentialA1A3,
    DifferentialA2A3,
}

macro_rules! impl_channel {
    ( $IC:ident, $CH:ident ) => {
        impl<DI, CONV, MODE> hal::adc::Channel<Ads1x1x<DI, ic::$IC, CONV, MODE>> for channel::$CH {
            type ID = ChannelSelection;

            fn channel() -> Self::ID {
                ChannelSelection::$CH
            }
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
