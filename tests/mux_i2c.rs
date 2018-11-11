#[macro_use(block)]
extern crate nb;
extern crate embedded_hal;
extern crate embedded_hal_mock as hal;
use hal::i2c::Transaction as I2cTrans;
extern crate ads1x1x;
use ads1x1x::channel;

#[macro_use]
mod common;
use common::{ new_ads1015 as new, destroy_ads1015 as destroy,
              DEVICE_ADDRESS as DEV_ADDR, Register, BitFlags as BF, Config };

use embedded_hal::adc::OneShot;

macro_rules! mux_test {
    ($name:ident, $CS:ident, $config_bits:expr) => {
        #[test]
        fn $name() {
            let default_config = Config::default();
            let config = Config::default().with_high(BF::OS).with_high($config_bits);
            let transactions = [ I2cTrans::write_read(DEV_ADDR, vec![Register::CONFIG], vec![default_config.msb(), default_config.lsb()]),
                                 I2cTrans::write(DEV_ADDR, vec![Register::CONFIG, config.msb(), config.lsb()]),
                                 I2cTrans::write_read(DEV_ADDR, vec![Register::CONFIG], vec![config.msb(), config.lsb()]),
                                 I2cTrans::write_read(DEV_ADDR, vec![Register::CONVERSION], vec![0x80, 0x00] ) ];
            let mut dev = new(&transactions);
            let measurement = block!(dev.read(&mut channel::$CS)).unwrap();
            assert_eq!(-2048, measurement);
            destroy(dev);
        }
    };
}

mux_test!(diffa0a1, DifferentialA0A1, 0);
mux_test!(diffa0a3, DifferentialA0A3, BF::MUX0);
mux_test!(diffa1a3, DifferentialA1A3, BF::MUX1);
mux_test!(diffa2a3, DifferentialA2A3, BF::MUX1 | BF::MUX0);
mux_test!(singlea0, SingleA0, BF::MUX2);
mux_test!(singlea1, SingleA1, BF::MUX2 | BF::MUX0);
mux_test!(singlea2, SingleA2, BF::MUX2 | BF::MUX1);
mux_test!(singlea3, SingleA3, BF::MUX2 | BF::MUX1 | BF::MUX0);
