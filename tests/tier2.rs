extern crate embedded_hal_mock;
use embedded_hal_mock::i2c::Transaction as I2cTrans;
extern crate ads1x1x;
use ads1x1x::{ FullScaleRange, ComparatorMode, ComparatorPolarity,
               ComparatorLatching, ComparatorQueue };

#[macro_use]
mod common;
use common::{ new_ads1014, destroy_ads1014,
              DEVICE_ADDRESS as DEV_ADDR, Register, BitFlags, Config };

macro_rules! set_value_test {
    ($name:ident, $method:ident, $value:expr, $reg:ident, $msb:expr, $lsb:expr) => {
        #[test]
        fn $name() {
            let transactions = [ I2cTrans::write(DEV_ADDR, vec![Register::$reg, $msb, $lsb]) ];
            let mut dev = new_ads1014(&transactions);
            dev.$method($value).unwrap();
            destroy_ads1014(dev);
        }
    }
}

macro_rules! config_test {
    ($name:ident, $method:ident, $value:expr, $config:expr) => {
        set_value_test!($name, $method, $value, CONFIG, $config.msb(), $config.lsb());
    }
}

mod can_set_comparator_thresholds {
    use super::*;
    set_value_test!(low,  set_low_threshold_raw,  2047, LOW_TH,  0x7F, 0xF0);
    set_value_test!(high, set_high_threshold_raw, 2047, HIGH_TH, 0x7F, 0xF0);
}

mod can_set_comparator_mode {
    use super::*;
    config_test!(traditional, set_comparator_mode, ComparatorMode::Traditional, Config::default().with_low( BitFlags::COMP_MODE));
    config_test!(window,      set_comparator_mode, ComparatorMode::Window,      Config::default().with_high(BitFlags::COMP_MODE));
}

mod can_set_comparator_polarity {
    use super::*;
    config_test!(low,  set_comparator_polarity, ComparatorPolarity::ActiveLow,  Config::default().with_low( BitFlags::COMP_POL));
    config_test!(high, set_comparator_polarity, ComparatorPolarity::ActiveHigh, Config::default().with_high(BitFlags::COMP_POL));
}

mod can_set_comparator_latching {
    use super::*;
    config_test!(non, set_comparator_latching, ComparatorLatching::Nonlatching, Config::default().with_low( BitFlags::COMP_LAT));
    config_test!(lat, set_comparator_latching, ComparatorLatching::Latching,    Config::default().with_high(BitFlags::COMP_LAT));
}

#[test]
fn can_disable_comparator() {
    let config = Config::default().with_high(BitFlags::COMP_QUE1).with_high(BitFlags::COMP_QUE0);
    let transactions = [ I2cTrans::write(DEV_ADDR, vec![Register::CONFIG, config.msb(), config.lsb()]) ];
    let mut dev = new_ads1014(&transactions);
    dev.disable_comparator().unwrap();
    destroy_ads1014(dev);
}

mod can_set_comparator_queue {
    use super::*;
    config_test!(one,  set_comparator_queue, ComparatorQueue::One,  Config::default().with_low( BitFlags::COMP_QUE1).with_low( BitFlags::COMP_QUE0));
    config_test!(two,  set_comparator_queue, ComparatorQueue::Two,  Config::default().with_low( BitFlags::COMP_QUE1).with_high(BitFlags::COMP_QUE0));
    config_test!(four, set_comparator_queue, ComparatorQueue::Four, Config::default().with_high(BitFlags::COMP_QUE1).with_low( BitFlags::COMP_QUE0));
}

#[test]
fn can_use_alert_rdy_pin_as_rdy_does_not_disable_comparator_if_already_disabled() {
    let transactions = [ I2cTrans::write(DEV_ADDR, vec![Register::HIGH_TH, 0b1000_0000, 0]),
                         I2cTrans::write(DEV_ADDR, vec![Register::LOW_TH,  0,    0]), ];
    let mut dev = new_ads1014(&transactions);
    dev.use_alert_rdy_pin_as_ready().unwrap();
    destroy_ads1014(dev);
}

#[test]
fn can_use_alert_rdy_pin_as_rdy_disabled_comparator() {
    let config = Config::default().with_low(BitFlags::COMP_QUE1).with_low(BitFlags::COMP_QUE0);
    let config_disabled_comp = Config::default().with_high(BitFlags::COMP_QUE1).with_high(BitFlags::COMP_QUE0);
    let transactions = [ I2cTrans::write(DEV_ADDR, vec![Register::CONFIG, config.msb(), config.lsb()]),
                         I2cTrans::write(DEV_ADDR, vec![Register::CONFIG, config_disabled_comp.msb(), config_disabled_comp.lsb()]),
                         I2cTrans::write(DEV_ADDR, vec![Register::HIGH_TH, 0b1000_0000, 0]),
                         I2cTrans::write(DEV_ADDR, vec![Register::LOW_TH,  0,    0]), ];
    let mut dev = new_ads1014(&transactions);
    dev.set_comparator_queue(ComparatorQueue::One).unwrap();
    dev.use_alert_rdy_pin_as_ready().unwrap();
    destroy_ads1014(dev);
}

mod can_set_full_scale_range {
    use super::*;
    config_test!(fsr6,   set_full_scale_range, FullScaleRange::Within6_144V, Config::default().with_low( BitFlags::PGA2).with_low( BitFlags::PGA1).with_low( BitFlags::PGA0));
    config_test!(fsr4,   set_full_scale_range, FullScaleRange::Within4_096V, Config::default().with_low( BitFlags::PGA2).with_low( BitFlags::PGA1).with_high(BitFlags::PGA0));
    config_test!(fsr2,   set_full_scale_range, FullScaleRange::Within2_048V, Config::default().with_low( BitFlags::PGA2).with_high(BitFlags::PGA1).with_low( BitFlags::PGA0));
    config_test!(fsr1,   set_full_scale_range, FullScaleRange::Within1_024V, Config::default().with_low( BitFlags::PGA2).with_high(BitFlags::PGA1).with_high(BitFlags::PGA0));
    config_test!(fsr0_5, set_full_scale_range, FullScaleRange::Within0_512V, Config::default().with_high(BitFlags::PGA2).with_low( BitFlags::PGA1).with_low( BitFlags::PGA0));
    config_test!(fsr0_2, set_full_scale_range, FullScaleRange::Within0_256V, Config::default().with_high(BitFlags::PGA2).with_low( BitFlags::PGA1).with_high(BitFlags::PGA0));
}
