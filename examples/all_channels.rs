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
    let mut adc = Ads1x1x::new_ads1015(dev, address);
    let values = [
        block!(adc.read(&mut channel::SingleA0)).unwrap(),
        block!(adc.read(&mut channel::SingleA1)).unwrap(),
        block!(adc.read(&mut channel::SingleA2)).unwrap(),
        block!(adc.read(&mut channel::SingleA3)).unwrap(),
    ];
    for (channel, value) in values.iter().enumerate() {
        println!("Channel {}: {}", channel, value);
    }
    // get I2C device back
    let _dev = adc.destroy_ads1015();
}
