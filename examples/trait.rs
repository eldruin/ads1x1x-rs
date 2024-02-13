// This example demonstrates the use of the `DynamicOneSot` trait to ease the usage
// of the `Ads1x1x` struct in functions.

use linux_embedded_hal::I2cdev;
use nb::block;

use ads1x1x::{Ads1x1x, ChannelSelection, DynamicOneShot, SlaveAddr};

/// Read a single value from channel A.
/// Returns 0 on Error.
pub fn read<E, A: DynamicOneShot<Error = E>>(adc: &mut A) -> i16 {
    block!(adc.read(ChannelSelection::SingleA0)).unwrap_or(0)
}

fn main() {
    let dev = I2cdev::new("/dev/i2c-1").unwrap();
    let mut adc = Ads1x1x::new_ads1115(dev, SlaveAddr::default());

    let value = read(&mut adc);
    println!("Measurement: {}", value);
    // get I2C device back
    let _dev = adc.destroy_ads1115();
}
