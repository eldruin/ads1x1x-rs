// This example demonstrates how the `Ads1x1x` struct may 
// look in a type signature.

#[macro_use(block)]
use nb;

use embedded_hal;
use embedded_hal::adc::OneShot;
use linux_embedded_hal;

use ads1x1x::{
    channel::SingleA0,
    ic::{Ads1115, Resolution16Bit},
    interface::I2cInterface,
    Ads1x1x, SlaveAddr,
};
use linux_embedded_hal::I2cdev;

type Adc = Ads1x1x<I2cInterface<I2cdev>, Ads1115, Resolution16Bit, ads1x1x::mode::OneShot>;

/// Read a single value from channel A
pub fn read(adc: &mut Adc) -> f32 {
    block!(adc.read(&mut SingleA0));
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
