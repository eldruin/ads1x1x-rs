extern crate embedded_hal;
extern crate embedded_hal_mock as hal;
use hal::i2c::Transaction as I2cTrans;
extern crate ads1x1x;
use ads1x1x::DataRate;

#[macro_use]
mod common;
use common::{ new_ads1013, destroy_ads1013,
              DEVICE_ADDRESS as DEV_ADDR, Register, BitFlags, Config };


macro_rules! test_set_data_rate {
    ($name:ident, $variant:ident, $config:expr) => {
        #[test]
        fn $name() {
            let transactions = [ I2cTrans::write(DEV_ADDR, vec![Register::CONFIG, $config.msb(), $config.lsb()]) ];
            let mut dev = new_ads1013(&transactions);
            dev.set_data_rate(DataRate::$variant).unwrap();
            destroy_ads1013(dev);
        }
    }
}

mod data_rate {
    use super::*;
    test_set_data_rate!(sps128,  Sps128,  Config::default().with_low( BitFlags::DR2).with_low( BitFlags::DR1).with_low( BitFlags::DR0));
    test_set_data_rate!(sps250,  Sps250,  Config::default().with_low( BitFlags::DR2).with_low( BitFlags::DR1).with_high(BitFlags::DR0));
    test_set_data_rate!(sps490,  Sps490,  Config::default().with_low( BitFlags::DR2).with_high(BitFlags::DR1).with_low( BitFlags::DR0));
    test_set_data_rate!(sps920,  Sps920,  Config::default().with_low( BitFlags::DR2).with_high(BitFlags::DR1).with_high(BitFlags::DR0));
    test_set_data_rate!(sps1600, Sps1600, Config::default().with_high(BitFlags::DR2).with_low( BitFlags::DR1).with_low( BitFlags::DR0));
    test_set_data_rate!(sps2400, Sps2400, Config::default().with_high(BitFlags::DR2).with_low( BitFlags::DR1).with_high(BitFlags::DR0));
    test_set_data_rate!(sps3300, Sps3300, Config::default().with_high(BitFlags::DR2).with_high(BitFlags::DR1).with_low( BitFlags::DR0));
}



