//! Constructor/destructor functions for devices using I2C interface.

extern crate embedded_hal as hal;
use hal::blocking;
use core::marker::PhantomData;
use { Ads1x1x, DEVICE_BASE_ADDRESS, SlaveAddr, ic, Config, mode };
use interface::I2cInterface;


macro_rules! impl_new_destroy {
    ( $IC:ident, $create:ident, $destroy:ident ) => {
        impl<I2C, E> Ads1x1x<I2cInterface<I2C>, ic::$IC, mode::OneShot>
        where
            I2C: blocking::i2c::Write<Error = E> + blocking::i2c::WriteRead<Error = E>
        {
            /// Create a new instance of the device in OneShot mode.
            pub fn $create(i2c: I2C, address: SlaveAddr) -> Self {
                Ads1x1x {
                    iface: I2cInterface {
                        i2c,
                        address: address.addr(DEVICE_BASE_ADDRESS)
                    },
                    config: Config::default(),
                    a_conversion_was_started: false,
                    _ic: PhantomData,
                    _mode: PhantomData
                }
            }
        }
        impl<I2C, MODE> Ads1x1x<I2cInterface<I2C>, ic::$IC, MODE>
        {
            /// Destroy driver instance, return I²C bus instance.
            pub fn $destroy(self) -> I2C {
                self.iface.i2c
            }
        }
    }
}

impl_new_destroy!(Ads1013, new_ads1013, destroy_ads1013);
impl_new_destroy!(Ads1113, new_ads1113, destroy_ads1113);
impl_new_destroy!(Ads1014, new_ads1014, destroy_ads1014);
impl_new_destroy!(Ads1114, new_ads1114, destroy_ads1114);
impl_new_destroy!(Ads1015, new_ads1015, destroy_ads1015);
impl_new_destroy!(Ads1115, new_ads1115, destroy_ads1115);
