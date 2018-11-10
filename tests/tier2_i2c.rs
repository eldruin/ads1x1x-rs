extern crate embedded_hal;
extern crate embedded_hal_mock as hal;
use hal::i2c::Transaction as I2cTrans;
extern crate ads1x1x;
use ads1x1x::{ ComparatorMode, ComparatorPolarity };

#[macro_use]
mod common;
use common::{ new_ads1014, destroy_ads1014,
              DEVICE_ADDRESS as DEV_ADDR, Register, BitFlags, Config };

macro_rules! set_value_test {
    ($name:ident, $method:ident, $value:expr, $config:expr) => {
        #[test]
        fn $name() {
            let transactions = [ I2cTrans::write(DEV_ADDR, vec![Register::CONFIG, $config.msb(), $config.lsb()]) ];
            let mut dev = new_ads1014(&transactions);
            dev.$method($value).unwrap();
            destroy_ads1014(dev);
        }
    }
}

mod can_set_comparator_mode {
    use super::*;
    set_value_test!(traditional, set_comparator_mode, ComparatorMode::Traditional, Config::default().with_low( BitFlags::COMP_MODE));
    set_value_test!(window,      set_comparator_mode, ComparatorMode::Window,      Config::default().with_high(BitFlags::COMP_MODE));
}

mod can_set_comparator_polarity {
    use super::*;
    set_value_test!(low,  set_comparator_polarity, ComparatorPolarity::ActiveLow,  Config::default().with_low( BitFlags::COMP_POL));
    set_value_test!(high, set_comparator_polarity, ComparatorPolarity::ActiveHigh, Config::default().with_high(BitFlags::COMP_POL));
}


