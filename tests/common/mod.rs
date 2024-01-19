use ads1x1x::{ic, interface, mode, Ads1x1x, SlaveAddr};
use embedded_hal_mock::eh1::i2c::{Mock as I2cMock, Transaction as I2cTrans};

#[allow(unused)]
pub const DEVICE_ADDRESS: u8 = 0b100_1000;

pub struct Register;
#[allow(unused)]
impl Register {
    pub const CONVERSION: u8 = 0x00;
    pub const CONFIG: u8 = 0x01;
    pub const LOW_TH: u8 = 0x02;
    pub const HIGH_TH: u8 = 0x03;
}

pub struct BitFlags;
#[allow(unused)]
impl BitFlags {
    pub const OS: u16 = 0b1000_0000_0000_0000;
    pub const MUX2: u16 = 0b0100_0000_0000_0000;
    pub const MUX1: u16 = 0b0010_0000_0000_0000;
    pub const MUX0: u16 = 0b0001_0000_0000_0000;
    pub const PGA2: u16 = 0b0000_1000_0000_0000;
    pub const PGA1: u16 = 0b0000_0100_0000_0000;
    pub const PGA0: u16 = 0b0000_0010_0000_0000;
    pub const OP_MODE: u16 = 0b0000_0001_0000_0000;
    pub const DR2: u16 = 0b0000_0000_1000_0000;
    pub const DR1: u16 = 0b0000_0000_0100_0000;
    pub const DR0: u16 = 0b0000_0000_0010_0000;
    pub const COMP_MODE: u16 = 0b0000_0000_0001_0000;
    pub const COMP_POL: u16 = 0b0000_0000_0000_1000;
    pub const COMP_LAT: u16 = 0b0000_0000_0000_0100;
    pub const COMP_QUE1: u16 = 0b0000_0000_0000_0010;
    pub const COMP_QUE0: u16 = 0b0000_0000_0000_0001;
}

pub struct Config {
    pub bits: u16,
}

#[allow(dead_code)]
impl Config {
    pub fn with_high(&self, mask: u16) -> Self {
        Config {
            bits: self.bits | mask,
        }
    }
    pub fn with_low(&self, mask: u16) -> Self {
        Config {
            bits: self.bits & !mask,
        }
    }

    pub fn msb(&self) -> u8 {
        (self.bits >> 8) as u8
    }

    pub fn lsb(&self) -> u8 {
        self.bits as u8
    }
}

impl Default for Config {
    fn default() -> Self {
        Config { bits: 0x8583 }
    }
}

macro_rules! impl_new_destroy {
    ($ic:ident, $create:ident, $destroy:ident, $conv:ty, $trans:ty, $iface:ty) => {
        #[allow(unused)]
        pub fn $create(transactions: &[$trans]) -> Ads1x1x<$iface, ic::$ic, $conv, mode::OneShot> {
            Ads1x1x::$create(I2cMock::new(transactions), SlaveAddr::default())
        }

        #[allow(unused)]
        pub fn $destroy<MODE>(dev: Ads1x1x<$iface, ic::$ic, $conv, MODE>) {
            dev.$destroy().done();
        }
    };
}

macro_rules! impl_new_destroy_i2c {
    ($ic:ident, $create:ident, $destroy:ident, $conv:ty) => {
        impl_new_destroy!(
            $ic,
            $create,
            $destroy,
            $conv,
            I2cTrans,
            interface::I2cInterface<I2cMock>
        );
    };
}

impl_new_destroy_i2c!(Ads1013, new_ads1013, destroy_ads1013, ic::Resolution12Bit);
impl_new_destroy_i2c!(Ads1113, new_ads1113, destroy_ads1113, ic::Resolution16Bit);
impl_new_destroy_i2c!(Ads1014, new_ads1014, destroy_ads1014, ic::Resolution12Bit);
impl_new_destroy_i2c!(Ads1114, new_ads1114, destroy_ads1114, ic::Resolution16Bit);
impl_new_destroy_i2c!(Ads1015, new_ads1015, destroy_ads1015, ic::Resolution12Bit);
impl_new_destroy_i2c!(Ads1115, new_ads1115, destroy_ads1115, ic::Resolution16Bit);

#[macro_export]
macro_rules! assert_would_block {
    ($result: expr) => {
        match $result {
            Err(nb::Error::WouldBlock) => (),
            _ => panic!("Would not block."),
        }
    };
}
