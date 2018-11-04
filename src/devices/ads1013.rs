//! Functions exclusive of ADS1013

extern crate embedded_hal as hal;
use hal::blocking;
use core::marker::PhantomData;
use { Ads1x1x, DEVICE_BASE_ADDRESS, SlaveAddr, ic };
use interface::I2cInterface;

impl<I2C, E> Ads1x1x<I2cInterface<I2C>, ic::ADS1013>
where
    I2C: blocking::i2c::Write<Error = E> + blocking::i2c::WriteRead<Error = E>
{
    /// Create a new instance of the ADS1013 device.
    pub fn new_ads1013(i2c: I2C, address: SlaveAddr) -> Self {
        Ads1x1x {
            iface: I2cInterface {
                i2c,
                address: address.addr(DEVICE_BASE_ADDRESS)
            },
            _ic: PhantomData
        }
    }

    /// Destroy driver instance, return I²C bus instance.
    pub fn destroy_ads1013(self) -> I2C {
        self.iface.i2c
    }
}
