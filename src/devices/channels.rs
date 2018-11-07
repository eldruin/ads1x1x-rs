//! ADC input channels
use { Ads1x1x, ic, hal };

/// ADC input channels
#[allow(dead_code)]
pub mod channel {
    /// ADC input channel 0
    pub struct A0;
    /// ADC input channel 1
    pub struct A1;
    /// ADC input channel 2
    pub struct A2;
    /// ADC input channel 3
    pub struct A3;
}

macro_rules! impl_channel {
    ( $IC:ident, $CH:ident, $ID:expr ) => {
        impl<DI, MODE> hal::adc::Channel<Ads1x1x<DI, ic::$IC, MODE>> for channel::$CH {
            type ID = u8;

            fn channel() -> Self::ID {
                $ID
            }
        }
    }
}

impl_channel!(Ads1013, A0, 0);
impl_channel!(Ads1013, A1, 1);
impl_channel!(Ads1113, A0, 0);
impl_channel!(Ads1113, A1, 1);
