#[cfg_attr(test, macro_use(block))]
extern crate nb;
extern crate embedded_hal;
extern crate embedded_hal_mock as hal;
use hal::i2c::Transaction as I2cTrans;
extern crate ads1x1x;
use ads1x1x::{ channel, DataRate12Bit, DataRate16Bit };

#[macro_use]
mod common;
use common::{ new_ads1013, destroy_ads1013, new_ads1113, destroy_ads1113,
              DEVICE_ADDRESS as DEV_ADDR, Register, BitFlags, Config };

macro_rules! impl_tests {
    ($IC:ident, $create:ident, $destroy:ident, $expected:expr) => {
        mod $IC {
            use embedded_hal::adc::OneShot;
            use super::*;

            mod would_block {
                use super::*;

                #[test]
                fn read_if_measurement_in_progress() {
                    let config = Config::default().with_low(BitFlags::OS);
                    let transactions = [ I2cTrans::write_read(DEV_ADDR, vec![Register::CONFIG], vec![config.msb(), config.lsb()] ) ];
                    let mut dev = $create(&transactions);
                    assert_would_block!(dev.read(&mut channel::DifferentialA0A1));
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
                let measurement = block!(dev.read(&mut channel::DifferentialA0A1)).unwrap();
                assert_eq!($expected, measurement);
                $destroy(dev);
            }
        }
    }
}

impl_tests!(ads1013, new_ads1013, destroy_ads1013,  -2048);
impl_tests!(ads1113, new_ads1113, destroy_ads1113, -32768);


mod data_rate_12bit {
    macro_rules! test {
        ($name:ident, $variant:ident, $config:expr) => {
            #[test]
            fn $name() {
                let transactions = [ I2cTrans::write(DEV_ADDR, vec![Register::CONFIG, $config.msb(), $config.lsb()]) ];
                let mut dev = new_ads1013(&transactions);
                dev.set_data_rate(DataRate12Bit::$variant).unwrap();
                destroy_ads1013(dev);
            }
        }
    }

    use super::*;
    test!(sps128,  Sps128,  Config::default().with_low( BitFlags::DR2).with_low( BitFlags::DR1).with_low( BitFlags::DR0));
    test!(sps250,  Sps250,  Config::default().with_low( BitFlags::DR2).with_low( BitFlags::DR1).with_high(BitFlags::DR0));
    test!(sps490,  Sps490,  Config::default().with_low( BitFlags::DR2).with_high(BitFlags::DR1).with_low( BitFlags::DR0));
    test!(sps920,  Sps920,  Config::default().with_low( BitFlags::DR2).with_high(BitFlags::DR1).with_high(BitFlags::DR0));
    test!(sps1600, Sps1600, Config::default().with_high(BitFlags::DR2).with_low( BitFlags::DR1).with_low( BitFlags::DR0));
    test!(sps2400, Sps2400, Config::default().with_high(BitFlags::DR2).with_low( BitFlags::DR1).with_high(BitFlags::DR0));
    test!(sps3300, Sps3300, Config::default().with_high(BitFlags::DR2).with_high(BitFlags::DR1).with_low( BitFlags::DR0));
}

mod data_rate_16bit {
    macro_rules! test {
        ($name:ident, $variant:ident, $config:expr) => {
            #[test]
            fn $name() {
                let transactions = [ I2cTrans::write(DEV_ADDR, vec![Register::CONFIG, $config.msb(), $config.lsb()]) ];
                let mut dev = new_ads1113(&transactions);
                dev.set_data_rate(DataRate16Bit::$variant).unwrap();
                destroy_ads1113(dev);
            }
        }
    }

    use super::*;
    test!(sps8,   Sps8,   Config::default().with_low( BitFlags::DR2).with_low( BitFlags::DR1).with_low( BitFlags::DR0));
    test!(sps16,  Sps16,  Config::default().with_low( BitFlags::DR2).with_low( BitFlags::DR1).with_high(BitFlags::DR0));
    test!(sps32,  Sps32,  Config::default().with_low( BitFlags::DR2).with_high(BitFlags::DR1).with_low( BitFlags::DR0));
    test!(sps64,  Sps64,  Config::default().with_low( BitFlags::DR2).with_high(BitFlags::DR1).with_high(BitFlags::DR0));
    test!(sps128, Sps128, Config::default().with_high(BitFlags::DR2).with_low( BitFlags::DR1).with_low( BitFlags::DR0));
    test!(sps250, Sps250, Config::default().with_high(BitFlags::DR2).with_low( BitFlags::DR1).with_high(BitFlags::DR0));
    test!(sps475, Sps475, Config::default().with_high(BitFlags::DR2).with_high(BitFlags::DR1).with_low( BitFlags::DR0));
    test!(sps860, Sps860, Config::default().with_high(BitFlags::DR2).with_high(BitFlags::DR1).with_high(BitFlags::DR0));
}
