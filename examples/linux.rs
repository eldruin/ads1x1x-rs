extern crate embedded_hal;
use embedded_hal::adc::OneShot;
extern crate linux_embedded_hal;
#[macro_use(block)]
extern crate nb;
extern crate ads1x1x;

use linux_embedded_hal::I2cdev;
use ads1x1x::{ Ads1x1x, SlaveAddr, channel };

fn main() {
    let dev = I2cdev::new("/dev/i2c-1").unwrap();
    let mut adc = Ads1x1x::new_ads1013(dev, SlaveAddr::default());
    let measurement = block!(adc.read(&mut channel::DifferentialA0A1)).unwrap();
    println!("Measurement: {}", measurement);
    let _dev = adc.destroy_ads1013(); // get I2C device back
}
