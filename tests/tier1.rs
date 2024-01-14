use ads1x1x::{channel, DataRate12Bit, DataRate16Bit};
use embedded_hal_mock::eh1::i2c::Transaction as I2cTrans;
use nb::block;

mod common;
use crate::common::{
    destroy_ads1013, destroy_ads1113, new_ads1013, new_ads1113, BitFlags as BF, Config, Register,
    DEVICE_ADDRESS as DEV_ADDR,
};

macro_rules! measure_tests {
    ($IC:ident, $create:ident, $destroy:ident, $expected:expr) => {
        mod $IC {
            use super::*;

            mod would_block {
                use super::*;

                #[test]
                fn read_if_measurement_in_progress() {
                    let config = Config::default().difference(BF::OS);
                    let transactions = [I2cTrans::write_read(
                        DEV_ADDR,
                        vec![Register::CONFIG],
                        vec![config.msb(), config.lsb()],
                    )];
                    let mut adc = $create(&transactions);
                    assert_would_block!(adc.read(channel::DifferentialA0A1));
                    $destroy(adc);
                }
            }

            #[test]
            fn can_measure() {
                let default_config = Config::default();
                let config_with_os = Config::default().union(BF::OS);
                let transactions = [
                    I2cTrans::write_read(
                        DEV_ADDR,
                        vec![Register::CONFIG],
                        vec![default_config.msb(), default_config.lsb()],
                    ),
                    I2cTrans::write(
                        DEV_ADDR,
                        vec![Register::CONFIG, config_with_os.msb(), config_with_os.lsb()],
                    ),
                    I2cTrans::write_read(
                        DEV_ADDR,
                        vec![Register::CONFIG],
                        vec![config_with_os.msb(), config_with_os.lsb()],
                    ),
                    I2cTrans::write_read(DEV_ADDR, vec![Register::CONVERSION], vec![0x80, 0x00]),
                ];
                let mut adc = $create(&transactions);
                let measurement = block!(adc.read(channel::DifferentialA0A1)).unwrap();
                assert_eq!($expected, measurement);
                $destroy(adc);
            }

            #[test]
            fn can_measure_continuous() {
                let config = Config::default().difference(BF::OP_MODE);
                let transactions = [
                    I2cTrans::write(DEV_ADDR, vec![Register::CONFIG, config.msb(), config.lsb()]),
                    I2cTrans::write_read(DEV_ADDR, vec![Register::CONVERSION], vec![0x80, 0x00]),
                ];
                let adc = $create(&transactions);
                let mut adc = adc.into_continuous().ok().unwrap();
                let measurement = adc.read().unwrap();
                assert_eq!($expected, measurement);
                $destroy(adc);
            }
        }
    };
}

measure_tests!(ads1013, new_ads1013, destroy_ads1013, -2048);
measure_tests!(ads1113, new_ads1113, destroy_ads1113, -32768);

mod data_rate_12bit {
    use super::*;

    macro_rules! test {
        ($name:ident, $variant:ident, $config:expr) => {
            #[test]
            fn $name() {
                let transactions = [I2cTrans::write(
                    DEV_ADDR,
                    vec![Register::CONFIG, $config.msb(), $config.lsb()],
                )];
                let mut adc = new_ads1013(&transactions);
                adc.set_data_rate(DataRate12Bit::$variant).unwrap();
                destroy_ads1013(adc);
            }
        };
    }

    test!(
        sps128,
        Sps128,
        Config::default()
            .difference(BF::DR2)
            .difference(BF::DR1)
            .difference(BF::DR0)
    );
    test!(
        sps250,
        Sps250,
        Config::default()
            .difference(BF::DR2)
            .difference(BF::DR1)
            .union(BF::DR0)
    );
    test!(
        sps490,
        Sps490,
        Config::default()
            .difference(BF::DR2)
            .union(BF::DR1)
            .difference(BF::DR0)
    );
    test!(
        sps920,
        Sps920,
        Config::default()
            .difference(BF::DR2)
            .union(BF::DR1)
            .union(BF::DR0)
    );
    test!(
        sps1600,
        Sps1600,
        Config::default()
            .union(BF::DR2)
            .difference(BF::DR1)
            .difference(BF::DR0)
    );
    test!(
        sps2400,
        Sps2400,
        Config::default()
            .union(BF::DR2)
            .difference(BF::DR1)
            .union(BF::DR0)
    );
    test!(
        sps3300,
        Sps3300,
        Config::default()
            .union(BF::DR2)
            .union(BF::DR1)
            .difference(BF::DR0)
    );
}

mod data_rate_16bit {
    use super::*;

    macro_rules! test {
        ($name:ident, $variant:ident, $config:expr) => {
            #[test]
            fn $name() {
                let transactions = [I2cTrans::write(
                    DEV_ADDR,
                    vec![Register::CONFIG, $config.msb(), $config.lsb()],
                )];
                let mut adc = new_ads1113(&transactions);
                adc.set_data_rate(DataRate16Bit::$variant).unwrap();
                destroy_ads1113(adc);
            }
        };
    }

    test!(
        sps8,
        Sps8,
        Config::default()
            .difference(BF::DR2)
            .difference(BF::DR1)
            .difference(BF::DR0)
    );
    test!(
        sps16,
        Sps16,
        Config::default()
            .difference(BF::DR2)
            .difference(BF::DR1)
            .union(BF::DR0)
    );
    test!(
        sps32,
        Sps32,
        Config::default()
            .difference(BF::DR2)
            .union(BF::DR1)
            .difference(BF::DR0)
    );
    test!(
        sps64,
        Sps64,
        Config::default()
            .difference(BF::DR2)
            .union(BF::DR1)
            .union(BF::DR0)
    );
    test!(
        sps128,
        Sps128,
        Config::default()
            .union(BF::DR2)
            .difference(BF::DR1)
            .difference(BF::DR0)
    );
    test!(
        sps250,
        Sps250,
        Config::default()
            .union(BF::DR2)
            .difference(BF::DR1)
            .union(BF::DR0)
    );
    test!(
        sps475,
        Sps475,
        Config::default()
            .union(BF::DR2)
            .union(BF::DR1)
            .difference(BF::DR0)
    );
    test!(
        sps860,
        Sps860,
        Config::default()
            .union(BF::DR2)
            .union(BF::DR1)
            .union(BF::DR0)
    );
}

#[test]
fn can_read_measurement_in_progress() {
    let config_os = Config::default().difference(BF::OS);
    let transactions = [I2cTrans::write_read(
        DEV_ADDR,
        vec![Register::CONFIG],
        vec![config_os.msb(), config_os.lsb()],
    )];
    let mut adc = new_ads1013(&transactions);
    assert!(adc.is_measurement_in_progress().unwrap());
    destroy_ads1013(adc);
}

#[test]
fn can_read_measurement_not_in_progress() {
    let config_os = Config::default().union(BF::OS);
    let transactions = [I2cTrans::write_read(
        DEV_ADDR,
        vec![Register::CONFIG],
        vec![config_os.msb(), config_os.lsb()],
    )];
    let mut adc = new_ads1013(&transactions);
    assert!(!adc.is_measurement_in_progress().unwrap());
    destroy_ads1013(adc);
}

#[test]
fn can_convert_to_continuous() {
    let config = Config::default().difference(BF::OP_MODE);
    let transactions = [I2cTrans::write(
        DEV_ADDR,
        vec![Register::CONFIG, config.msb(), config.lsb()],
    )];
    let adc = new_ads1013(&transactions);
    let adc = adc.into_continuous().ok().unwrap();
    destroy_ads1013(adc);
}

#[test]
fn can_convert_to_one_shot() {
    let config_cont = Config::default().difference(BF::OP_MODE);
    let config_os = Config::default();
    let transactions = [
        I2cTrans::write(
            DEV_ADDR,
            vec![Register::CONFIG, config_cont.msb(), config_cont.lsb()],
        ),
        I2cTrans::write(
            DEV_ADDR,
            vec![Register::CONFIG, config_os.msb(), config_os.lsb()],
        ),
    ];
    let adc = new_ads1013(&transactions);
    let adc = adc.into_continuous().ok().unwrap();
    let adc = adc.into_one_shot().ok().unwrap();
    destroy_ads1013(adc);
}
