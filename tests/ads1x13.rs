#[macro_use(block)]
extern crate nb;
extern crate embedded_hal;
extern crate embedded_hal_mock as hal;
use hal::i2c::Transaction as I2cTrans;
extern crate ads1x1x;
use ads1x1x::channel;

#[macro_use]
mod common;
use common::{ new_ads1013, destroy_ads1013, new_ads1113, destroy_ads1113,
              DEVICE_ADDRESS as DEV_ADDR, Register, BitFlags, Config };

macro_rules! impl_tests {
    ($IC:ident, $create:ident, $destroy:ident, $expected:expr) => {
        mod $IC {
            use embedded_hal::adc::OneShot;
            use super::*;
            #[test]
            fn can_create() {
                let dev = $create(&[]);
                $destroy(dev);
            }

            mod would_block {
                use super::*;

                #[test]
                fn read_if_measurement_in_progress() {
                    let config = Config::default().with_low(BitFlags::OS);
                    let transactions = [ I2cTrans::write_read(DEV_ADDR, vec![Register::CONFIG], vec![config.msb(), config.lsb()] ) ];
                    let mut dev = $create(&transactions);
                    assert_would_block!(dev.read(&mut channel::A0));
                    $destroy(dev);
                }
            }

            #[test]
            fn can_measure() {
                let default_config = Config::default();
                let config_with_os = Config::default().with_high(BitFlags::OS);
                let transactions = [ I2cTrans::write_read(DEV_ADDR, vec![Register::CONFIG], vec![default_config.msb(), default_config.lsb()]),
                                    I2cTrans::write(DEV_ADDR, vec![Register::CONFIG, config_with_os.msb(), config_with_os.lsb()]),
                                    I2cTrans::write_read(DEV_ADDR, vec![Register::CONFIG], vec![config_with_os.msb(), config_with_os.lsb()]),
                                    I2cTrans::write_read(DEV_ADDR, vec![Register::CONVERSION], vec![0x80, 0x00] ) ];
                let mut dev = $create(&transactions);
                let measurement = block!(dev.read(&mut channel::A0{})).unwrap();
                assert_eq!($expected, measurement);
                $destroy(dev);
            }
        }
    }
}

impl_tests!(ads1013, new_ads1013, destroy_ads1013,  -2048);
impl_tests!(ads1113, new_ads1113, destroy_ads1113, -32768);
