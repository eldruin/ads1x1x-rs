//! Constructor/destructor functions.

use crate::{ic, mode, Ads1x1x, Config, FullScaleRange, SlaveAddr};
use core::marker::PhantomData;

macro_rules! impl_new_destroy {
    ( $IC:ident, $create:ident, $destroy:ident, $conv:ty ) => {
        impl<I2C, E> Ads1x1x<I2C, ic::$IC, $conv, mode::OneShot>
        where
            I2C: embedded_hal::i2c::I2c<Error = E>,
        {
            /// Create a new instance of the device in OneShot mode.
            pub fn $create(i2c: I2C, address: SlaveAddr) -> Self {
                Ads1x1x {
                    i2c,
                    address: address.bits(),
                    config: Config::default(),
                    fsr: FullScaleRange::default(),
                    a_conversion_was_started: false,
                    _conv: PhantomData,
                    _ic: PhantomData,
                    _mode: PhantomData,
                }
            }
        }
        impl<I2C, CONV, MODE> Ads1x1x<I2C, ic::$IC, CONV, MODE> {
            /// Destroy driver instance, return I²C bus instance.
            pub fn $destroy(self) -> I2C {
                self.i2c
            }
        }
    };
}

impl_new_destroy!(Ads1013, new_ads1013, destroy_ads1013, ic::Resolution12Bit);
impl_new_destroy!(Ads1113, new_ads1113, destroy_ads1113, ic::Resolution16Bit);
impl_new_destroy!(Ads1014, new_ads1014, destroy_ads1014, ic::Resolution12Bit);
impl_new_destroy!(Ads1114, new_ads1114, destroy_ads1114, ic::Resolution16Bit);
impl_new_destroy!(Ads1015, new_ads1015, destroy_ads1015, ic::Resolution12Bit);
impl_new_destroy!(Ads1115, new_ads1115, destroy_ads1115, ic::Resolution16Bit);
