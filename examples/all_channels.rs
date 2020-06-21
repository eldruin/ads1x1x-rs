use embedded_hal::adc::OneShot;
use linux_embedded_hal::I2cdev;
use nb::block;

use ads1x1x::{channel, Ads1x1x, SlaveAddr};

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
