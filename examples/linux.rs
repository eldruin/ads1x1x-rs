extern crate embedded_hal;
use embedded_hal::adc::OneShot;
extern crate linux_embedded_hal;
#[macro_use(block)]
extern crate nb;
extern crate ads1x1x;

use ads1x1x::{channel, Ads1x1x, SlaveAddr};
use linux_embedded_hal::I2cdev;

fn main() {
    let dev = I2cdev::new("/dev/i2c-1").unwrap();
    let address = SlaveAddr::default();
    let mut adc = Ads1x1x::new_ads1013(dev, address);
    let value = block!(adc.read(&mut channel::DifferentialA0A1)).unwrap();
    println!("Measurement: {}", value);
    // get I2C device back
    let _dev = adc.destroy_ads1013();
}
