use ads1x1x::{channel, Ads1x1x, SlaveAddr};
use linux_embedded_hal::I2cdev;
use nb::block;

fn main() {
    let i2c = I2cdev::new("/dev/i2c-1").unwrap();
    let mut adc = Ads1013::new(i2c, SlaveAddr::default());

    let value = block!(adc.read(channel::DifferentialA0A1)).unwrap();
    println!("Measurement: {}", value);
}
