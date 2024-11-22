use linux_embedded_hal::I2cdev;
use nb::block;

use ads1x1x::{channel, Ads1015, TargetAddr};

fn main() {
    let i2c = I2cdev::new("/dev/i2c-1").unwrap();
    let mut adc = Ads1015::new(i2c, TargetAddr::default());

    let values = [
        block!(adc.read(channel::SingleA0)).unwrap(),
        block!(adc.read(channel::SingleA1)).unwrap(),
        block!(adc.read(channel::SingleA2)).unwrap(),
        block!(adc.read(channel::SingleA3)).unwrap(),
    ];
    for (channel, value) in values.iter().enumerate() {
        println!("Channel {}: {}", channel, value);
    }
}
