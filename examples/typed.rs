// This example demonstrates the use of a type alias for the `Ads1x1x` struct
// to ease usage in signatures.

#[macro_use(block)]
extern crate nb;

extern crate embedded_hal;
use embedded_hal::adc::OneShot;
extern crate linux_embedded_hal;
use linux_embedded_hal::I2cdev;

extern crate ads1x1x;
use ads1x1x::{
    channel::SingleA0,
    ic::{Ads1115, Resolution16Bit},
    interface::I2cInterface,
    Ads1x1x, SlaveAddr,
};

/// Type alias
type Adc = Ads1x1x<I2cInterface<I2cdev>, Ads1115, Resolution16Bit, ads1x1x::mode::OneShot>;

/// Read a single value from channel A.
/// Returns 0 on Error.
pub fn read(adc: &mut Adc) -> i16 {
    block!(adc.read(&mut SingleA0)).unwrap_or(0)
}

fn main() {
    let dev = I2cdev::new("/dev/i2c-1").unwrap();
    let address = SlaveAddr::default();
    let mut adc = Ads1x1x::new_ads1115(dev, address);

    let value = read(&mut adc);
    println!("Measurement: {}", value);
    // get I2C device back
    let _dev = adc.destroy_ads1115();
}
