// This example demonstrates the use of a type alias for the `Ads1x1x` struct
// to ease usage in signatures.

use linux_embedded_hal::I2cdev;
use nb::block;

use ads1x1x::{channel, mode, Ads1115, SlaveAddr};

/// Type alias
type Adc = Ads1115<I2cdev, mode::OneShot>;

/// Read a single value from channel A.
/// Returns 0 on Error.
pub fn read(adc: &mut Adc) -> i16 {
    block!(adc.read(channel::SingleA0)).unwrap_or(0)
}

fn main() {
    let i2c = I2cdev::new("/dev/i2c-1").unwrap();
    let mut adc = Ads1115::new(i2c, SlaveAddr::default());

    let value = read(&mut adc);
    println!("Measurement: {}", value);
}
