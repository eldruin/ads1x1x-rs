//! Type definitions.

use crate::{channels::ChannelSelection, private};
use core::marker::PhantomData;

/// Errors in this crate
#[derive(Debug)]
pub enum Error<E> {
    /// I²C bus error
    I2C(E),
    /// Invalid input data provided
    InvalidInputData,
}

/// Error type for mode changes.
///
/// This allows to retrieve the unchanged device in case of an error.
pub enum ModeChangeError<E, DEV> {
    /// I²C bus error while changing mode.
    ///
    /// `E` is the error that happened.
    /// `DEV` is the device with the mode unchanged.
    I2C(E, DEV),
}

/// Mode marker types
pub mod mode {
    /// One-shot operating mode / power-down state (default)
    pub struct OneShot(());

    /// Continuous conversion mode
    pub struct Continuous(());
}

/// Data rate for ADS1013, ADS1014, ADS1015
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DataRate12Bit {
    /// 128 SPS
    Sps128,
    /// 250 SPS
    Sps250,
    /// 490 SPS
    Sps490,
    /// 920 SPS
    Sps920,
    /// 1600 SPS (default)
    Sps1600,
    /// 2400 SPS
    Sps2400,
    /// 3300 SPS
    Sps3300,
}

/// Data rate for ADS1113, ADS1114, ADS1115
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DataRate16Bit {
    /// 8 SPS
    Sps8,
    /// 16 SPS
    Sps16,
    /// 32 SPS
    Sps32,
    /// 64 SPS
    Sps64,
    /// 128 SPS (default)
    Sps128,
    /// 250 SPS
    Sps250,
    /// 475 SPS
    Sps475,
    /// 860 SPS
    Sps860,
}

/// Comparator mode (only for ADS1x14, ADS1x15)
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ComparatorMode {
    /// Traditional comparator (default)
    ///
    /// In this mode the ALERT/RDY pin asserts (according to selected active
    /// polarity) when the conversion data exceeds the limit set as *high*
    /// threshold and remains active until the conversion data falls below the
    /// value set as *low* threshold.
    Traditional,
    /// Window comparator
    ///
    /// In this mode the ALERT/RDY pin asserts (according to selected active
    /// polarity) when the conversion data exceeds the value set as *high*
    /// threshold or goes below the value set as *low* temperature threshold.
    Window,
}

/// Comparator polarity (only for ADS1x14, ADS1x15)
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ComparatorPolarity {
    /// Active low (default)
    ActiveLow,
    /// Active high
    ActiveHigh,
}

/// Comparator polarity (only for ADS1x14, ADS1x15)
///
/// Select whether the ALERT/RDY pin latches after being asserted or clears
/// after conversions are within the margin of the upper and lower
/// threshold values.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ComparatorLatching {
    /// Nonlatching (default)
    ///
    /// The ALERT/RDY pin does not latch when asserted.
    Nonlatching,
    /// Latching
    ///
    /// The asserted ALERT/RDY pin remains latched until conversion data are
    /// read by the master or an appropriate SMBus alert response is sent by
    /// the master. The device responds with its address, and it is the lowest
    /// address currently asserting the ALERT/RDY bus line.
    Latching,
}

/// Comparator alert queue (only for ADS1x14, ADS1x15)
///
/// The default state of the comparator is deactivated. It can be activated by setting
/// the comparator queue.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ComparatorQueue {
    /// Activate comparator and assert after one conversion exceeding thresholds
    One,
    /// Activate comparator and assert after two consecutive conversions exceeding thresholds
    Two,
    /// Activate comparator and assert after four consecutive conversions exceeding thresholds
    Four,
}

/// Full-scale range configuration for the programmable gain amplifier (PGA) (only for ADS1x14, ADS1x15)
///
/// This sets the input voltage measurable range.
/// The FSR is fixed at ±2.048 V in the ADS1x13.
#[derive(Clone, Copy, Debug, PartialEq)]
#[allow(non_camel_case_types)]
#[derive(Default)]
pub enum FullScaleRange {
    /// The measurable range is ±6.144V.
    Within6_144V,
    /// The measurable range is ±4.096V.
    Within4_096V,
    /// The measurable range is ±2.048V. (default)
    #[default]
    Within2_048V,
    /// The measurable range is ±1.024V.
    Within1_024V,
    /// The measurable range is ±0.512V.
    Within0_512V,
    /// The measurable range is ±0.256V.
    Within0_256V,
}

/// Possible slave addresses
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SlaveAddr {
    /// Default slave address
    Default,
    /// Alternative slave address using the provided values
    /// for the last two bits (A1, A0)
    Alternative(bool, bool),
}

impl Default for SlaveAddr {
    /// Default slave address
    fn default() -> Self {
        SlaveAddr::Default
    }
}

impl SlaveAddr {
    pub(crate) fn addr(self, default: u8) -> u8 {
        match self {
            SlaveAddr::Default => default,
            SlaveAddr::Alternative(a1, a0) => default | ((a1 as u8) << 1) | a0 as u8,
        }
    }

    /// Create `SlaveAddr` instance corresponding to the address
    /// effective when connecting the pin `ADDR` to GND (0x48).
    ///
    /// See [Table 4 in the datasheet](https://www.ti.com/lit/ds/symlink/ads1115.pdf#%5B%7B%22num%22%3A716%2C%22gen%22%3A0%7D%2C%7B%22name%22%3A%22XYZ%22%7D%2C0%2C602.2%2C0%5D).
    pub fn new_gnd() -> Self {
        SlaveAddr::default()
    }

    /// Create `SlaveAddr` instance corresponding to the address
    /// effective when connecting the pin `ADDR` to VDD (0x49).
    ///
    /// See [Table 4 in the datasheet](https://www.ti.com/lit/ds/symlink/ads1115.pdf#%5B%7B%22num%22%3A716%2C%22gen%22%3A0%7D%2C%7B%22name%22%3A%22XYZ%22%7D%2C0%2C602.2%2C0%5D).
    pub fn new_vdd() -> Self {
        SlaveAddr::Alternative(false, true)
    }

    /// Create `SlaveAddr` instance corresponding to the address
    /// effective when connecting the pin `ADDR` to SDA (0x4A).
    ///
    /// See [Table 4 in the datasheet](https://www.ti.com/lit/ds/symlink/ads1115.pdf#%5B%7B%22num%22%3A716%2C%22gen%22%3A0%7D%2C%7B%22name%22%3A%22XYZ%22%7D%2C0%2C602.2%2C0%5D).
    pub fn new_sda() -> Self {
        SlaveAddr::Alternative(true, false)
    }

    /// Create `SlaveAddr` instance corresponding to the address
    /// effective when connecting the pin `ADDR` to SCL (0x4B).
    ///
    /// See [Table 4 in the datasheet](https://www.ti.com/lit/ds/symlink/ads1115.pdf#%5B%7B%22num%22%3A716%2C%22gen%22%3A0%7D%2C%7B%22name%22%3A%22XYZ%22%7D%2C0%2C602.2%2C0%5D).
    pub fn new_scl() -> Self {
        SlaveAddr::Alternative(true, true)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Config {
    pub(crate) bits: u16,
}

impl Config {
    pub(crate) fn is_high(&self, mask: u16) -> bool {
        (self.bits & mask) != 0
    }

    pub(crate) fn with_high(&self, mask: u16) -> Self {
        Config {
            bits: self.bits | mask,
        }
    }

    pub(crate) fn with_low(&self, mask: u16) -> Self {
        Config {
            bits: self.bits & !mask,
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Config { bits: 0x8583 }
    }
}



/// ADS1x1x ADC driver
#[derive(Debug, Default)]
pub struct Ads1x1x<DI, IC, CONV, MODE> {
    pub(crate) iface: DI,
    pub(crate) config: Config,
    pub(crate) fsr: FullScaleRange,
    pub(crate) a_conversion_was_started: bool,
    pub(crate) _conv: PhantomData<CONV>,
    pub(crate) _ic: PhantomData<IC>,
    pub(crate) _mode: PhantomData<MODE>,
}

/// Multi channel One-shot ADC
pub trait DynamicOneShot: private::Sealed {
    /// Error type
    type Error;

    /// Read a measurement
    fn read(&mut self, channel: ChannelSelection) -> nb::Result<i16, Self::Error>;
}

#[cfg(test)]
mod tests {
    use crate::DEVICE_BASE_ADDRESS as ADDR;
    use crate::{FullScaleRange, SlaveAddr};

    #[test]
    fn can_get_default_address() {
        let addr = SlaveAddr::default();
        assert_eq!(ADDR, addr.addr(ADDR));
    }

    #[test]
    fn can_generate_alternative_addresses() {
        assert_eq!(0b100_1000, SlaveAddr::Alternative(false, false).addr(ADDR));
        assert_eq!(0b100_1001, SlaveAddr::Alternative(false, true).addr(ADDR));
        assert_eq!(0b100_1010, SlaveAddr::Alternative(true, false).addr(ADDR));
        assert_eq!(0b100_1011, SlaveAddr::Alternative(true, true).addr(ADDR));
    }

    #[test]
    fn can_generate_alternative_addresses_using_helper_constructors() {
        assert_eq!(0b100_1000, SlaveAddr::new_gnd().addr(ADDR));
        assert_eq!(0b100_1001, SlaveAddr::new_vdd().addr(ADDR));
        assert_eq!(0b100_1010, SlaveAddr::new_sda().addr(ADDR));
        assert_eq!(0b100_1011, SlaveAddr::new_scl().addr(ADDR));
    }

    #[test]
    fn default_full_scale_range() {
        assert_eq!(FullScaleRange::Within2_048V, FullScaleRange::default());
    }
}
