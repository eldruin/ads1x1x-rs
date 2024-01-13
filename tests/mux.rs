use ads1x1x::channel;
use embedded_hal_mock::eh1::i2c::Transaction as I2cTrans;
use nb::block;

mod common;
use crate::common::{
    destroy_ads1015 as destroy, new_ads1015 as new, BitFlags as BF, Config, Register,
    DEVICE_ADDRESS as DEV_ADDR,
};

macro_rules! mux_test {
    ($name:ident, $CS:ident, $config_bits:expr, $other_CS:ident, $other_config_bits:expr) => {
        mod $name {
            use super::*;

            #[test]
            fn can_read() {
                let default_config = Config::default();
                let config = Config::default().with_high(BF::OS).with_high($config_bits);
                let transactions = [
                    I2cTrans::write_read(
                        DEV_ADDR,
                        vec![Register::CONFIG],
                        vec![default_config.msb(), default_config.lsb()],
                    ),
                    I2cTrans::write(DEV_ADDR, vec![Register::CONFIG, config.msb(), config.lsb()]),
                    I2cTrans::write_read(
                        DEV_ADDR,
                        vec![Register::CONFIG],
                        vec![config.msb(), config.lsb()],
                    ),
                    I2cTrans::write_read(DEV_ADDR, vec![Register::CONVERSION], vec![0x80, 0x00]),
                ];
                let mut adc = new(&transactions);
                let measurement = block!(adc.read(channel::$CS)).unwrap();
                assert_eq!(-2048, measurement);
                destroy(adc);
            }

            #[test]
            fn read_then_read_different_triggers_new_measurement() {
                let default_config = Config::default();
                let config = Config::default().with_high(BF::OS).with_high($config_bits);
                let other_config = Config::default().with_high($other_config_bits);
                let transactions = [
                    I2cTrans::write_read(
                        DEV_ADDR,
                        vec![Register::CONFIG],
                        vec![default_config.msb(), default_config.lsb()],
                    ),
                    I2cTrans::write(DEV_ADDR, vec![Register::CONFIG, config.msb(), config.lsb()]),
                    I2cTrans::write_read(
                        DEV_ADDR,
                        vec![Register::CONFIG],
                        vec![config.msb(), config.lsb()],
                    ),
                    I2cTrans::write(
                        DEV_ADDR,
                        vec![Register::CONFIG, other_config.msb(), other_config.lsb()],
                    ),
                    I2cTrans::write_read(
                        DEV_ADDR,
                        vec![Register::CONFIG],
                        vec![other_config.msb(), other_config.lsb()],
                    ),
                    I2cTrans::write_read(DEV_ADDR, vec![Register::CONVERSION], vec![0x80, 0x00]),
                ];
                let mut adc = new(&transactions);
                assert_would_block!(adc.read(channel::$CS));
                let measurement = block!(adc.read(channel::$other_CS)).unwrap();
                assert_eq!(-2048, measurement);
                destroy(adc);
            }

            #[test]
            fn continuous_can_select_channel() {
                let config1 = Config::default().with_low(BF::OP_MODE);
                let config2 = config1.with_high($config_bits);
                let transactions = [
                    I2cTrans::write(
                        DEV_ADDR,
                        vec![Register::CONFIG, config1.msb(), config1.lsb()],
                    ),
                    I2cTrans::write(
                        DEV_ADDR,
                        vec![Register::CONFIG, config2.msb(), config2.lsb()],
                    ),
                ];
                let adc = new(&transactions);
                let mut adc = adc.into_continuous().ok().unwrap();
                adc.select_channel(channel::$CS).unwrap();
                destroy(adc);
            }
        }
    };
}

mux_test!(diffa0a1, DifferentialA0A1, 0, SingleA0, BF::MUX2);
mux_test!(diffa0a3, DifferentialA0A3, BF::MUX0, SingleA0, BF::MUX2);
mux_test!(diffa1a3, DifferentialA1A3, BF::MUX1, SingleA0, BF::MUX2);
mux_test!(
    diffa2a3,
    DifferentialA2A3,
    BF::MUX1 | BF::MUX0,
    SingleA0,
    BF::MUX2
);
mux_test!(singlea0, SingleA0, BF::MUX2, DifferentialA0A1, 0);
mux_test!(singlea1, SingleA1, BF::MUX2 | BF::MUX0, SingleA0, BF::MUX2);
mux_test!(singlea2, SingleA2, BF::MUX2 | BF::MUX1, SingleA0, BF::MUX2);
mux_test!(
    singlea3,
    SingleA3,
    BF::MUX2 | BF::MUX1 | BF::MUX0,
    SingleA0,
    BF::MUX2
);
