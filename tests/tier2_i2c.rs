extern crate embedded_hal;
extern crate embedded_hal_mock as hal;
use hal::i2c::Transaction as I2cTrans;
extern crate ads1x1x;
use ads1x1x::ComparatorMode;

#[macro_use]
mod common;
use common::{ new_ads1014, destroy_ads1014,
              DEVICE_ADDRESS as DEV_ADDR, Register, BitFlags, Config };

macro_rules! test_set_comparator_mode {
    ($name:ident, $variant:ident, $config:expr) => {
        #[test]
        fn $name() {
            let transactions = [ I2cTrans::write(DEV_ADDR, vec![Register::CONFIG, $config.msb(), $config.lsb()]) ];
            let mut dev = new_ads1014(&transactions);
            dev.set_comparator_mode(ComparatorMode::$variant).unwrap();
            destroy_ads1014(dev);
        }
    }
}


mod can_set_comparator_mode {
    use super::*;
    test_set_comparator_mode!(traditional, Traditional, Config::default().with_low( BitFlags::COMP_MODE));
    test_set_comparator_mode!(window,      Window,      Config::default().with_high(BitFlags::COMP_MODE));
}



