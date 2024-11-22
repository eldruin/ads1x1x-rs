//! Type definitions.

use crate::Config;

/// Data rate for ADS101x.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub enum DataRate12Bit {
    /// 128 SPS
    Sps128,
    /// 250 SPS
    Sps250,
    /// 490 SPS
    Sps490,
    /// 920 SPS
    Sps920,
    #[default]
    /// 1600 SPS (default)
    Sps1600,
    /// 2400 SPS
    Sps2400,
    /// 3300 SPS
    Sps3300,
}

impl DataRate12Bit {
    pub(crate) fn configure(self, cfg: Config) -> Config {
        match self {
            Self::Sps128 => cfg
                .difference(Config::DR2)
                .difference(Config::DR1)
                .difference(Config::DR0),
            Self::Sps250 => cfg
                .difference(Config::DR2)
                .difference(Config::DR1)
                .union(Config::DR0),
            Self::Sps490 => cfg
                .difference(Config::DR2)
                .union(Config::DR1)
                .difference(Config::DR0),
            Self::Sps920 => cfg
                .difference(Config::DR2)
                .union(Config::DR1)
                .union(Config::DR0),
            Self::Sps1600 => cfg
                .union(Config::DR2)
                .difference(Config::DR1)
                .difference(Config::DR0),
            Self::Sps2400 => cfg
                .union(Config::DR2)
                .difference(Config::DR1)
                .union(Config::DR0),
            Self::Sps3300 => cfg
                .union(Config::DR2)
                .union(Config::DR1)
                .difference(Config::DR0),
        }
    }
}

/// Data rate for ADS111x.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub enum DataRate16Bit {
    /// 8 SPS
    Sps8,
    /// 16 SPS
    Sps16,
    /// 32 SPS
    Sps32,
    /// 64 SPS
    Sps64,
    #[default]
    /// 128 SPS (default)
    Sps128,
    /// 250 SPS
    Sps250,
    /// 475 SPS
    Sps475,
    /// 860 SPS
    Sps860,
}

impl DataRate16Bit {
    pub(crate) fn configure(self, cfg: Config) -> Config {
        match self {
            Self::Sps8 => cfg
                .difference(Config::DR2)
                .difference(Config::DR1)
                .difference(Config::DR0),
            Self::Sps16 => cfg
                .difference(Config::DR2)
                .difference(Config::DR1)
                .union(Config::DR0),
            Self::Sps32 => cfg
                .difference(Config::DR2)
                .union(Config::DR1)
                .difference(Config::DR0),
            Self::Sps64 => cfg
                .difference(Config::DR2)
                .union(Config::DR1)
                .union(Config::DR0),
            Self::Sps128 => cfg
                .union(Config::DR2)
                .difference(Config::DR1)
                .difference(Config::DR0),
            Self::Sps250 => cfg
                .union(Config::DR2)
                .difference(Config::DR1)
                .union(Config::DR0),
            Self::Sps475 => cfg
                .union(Config::DR2)
                .union(Config::DR1)
                .difference(Config::DR0),
            Self::Sps860 => cfg.union(Config::DR2).union(Config::DR1).union(Config::DR0),
        }
    }
}

/// Comparator mode (only for ADS1x14, ADS1x15).
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub enum ComparatorMode {
    #[default]
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

/// Comparator polarity (only for ADS1x14, ADS1x15).
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub enum ComparatorPolarity {
    #[default]
    /// Active low (default)
    ActiveLow,
    /// Active high
    ActiveHigh,
}

/// Comparator latching (only for ADS1x14, ADS1x15).
///
/// Select whether the ALERT/RDY pin latches after being asserted or clears
/// after conversions are within the margin of the upper and lower
/// threshold values.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub enum ComparatorLatching {
    #[default]
    /// Nonlatching (default)
    ///
    /// The ALERT/RDY pin does not latch when asserted.
    Nonlatching,
    /// Latching
    ///
    /// The asserted ALERT/RDY pin remains latched until conversion data are
    /// read by the controller or an appropriate SMBus alert response is sent by
    /// the controller. The device responds with its address, and it is the
    /// lowest address currently asserting the ALERT/RDY bus line.
    Latching,
}

/// Comparator alert queue (only for ADS1x14, ADS1x15).
///
/// The default state of the comparator is deactivated. It can be activated by setting
/// the comparator queue.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub enum ComparatorQueue {
    /// Activate comparator and assert after one conversion exceeding thresholds
    One,
    /// Activate comparator and assert after two consecutive conversions exceeding thresholds
    Two,
    #[default]
    /// Activate comparator and assert after four consecutive conversions exceeding thresholds (default)
    Four,
}

/// Full-scale range configuration for the programmable gain amplifier (PGA) (only for ADS1x14, ADS1x15).
///
/// This sets the input voltage measurable range.
/// The FSR is fixed at ±2.048 V in the ADS1x13.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[allow(non_camel_case_types)]
pub enum FullScaleRange {
    /// ±6.144V
    Within6_144V,
    /// ±4.096V
    Within4_096V,
    /// ±2.048V (default)
    #[default]
    Within2_048V,
    /// ±1.024V
    Within1_024V,
    /// ±0.512V
    Within0_512V,
    /// ±0.256V
    Within0_256V,
}

impl FullScaleRange {
    pub(crate) fn configure(self, config: Config) -> Config {
        match self {
            Self::Within6_144V => config
                .difference(Config::PGA2)
                .difference(Config::PGA1)
                .difference(Config::PGA0),
            Self::Within4_096V => config
                .difference(Config::PGA2)
                .difference(Config::PGA1)
                .union(Config::PGA0),
            Self::Within2_048V => config
                .difference(Config::PGA2)
                .union(Config::PGA1)
                .difference(Config::PGA0),
            Self::Within1_024V => config
                .difference(Config::PGA2)
                .union(Config::PGA1)
                .union(Config::PGA0),
            Self::Within0_512V => config
                .union(Config::PGA2)
                .difference(Config::PGA1)
                .difference(Config::PGA0),
            Self::Within0_256V => config
                .union(Config::PGA2)
                .difference(Config::PGA1)
                .union(Config::PGA0),
        }
    }
}

/// A slave address.
///
/// See [Table 4 in the datasheet](https://www.ti.com/lit/ds/symlink/ads1115.pdf#%5B%7B%22num%22%3A716%2C%22gen%22%3A0%7D%2C%7B%22name%22%3A%22XYZ%22%7D%2C0%2C602.2%2C0%5D).
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub enum TargetAddr {
    /// Address when the ADDR pin is connected to GND. (default)
    #[default]
    Gnd,
    /// Address when the ADDR pin is connected to VDD.
    Vdd,
    /// Address when the ADDR pin is connected to SDA.
    ///
    /// If SDA is used as the device address, hold the SDA line low for at
    /// least 100 ns after the SCL line goes low to make sure the device
    /// decodes the address correctly during I²C communication.
    Sda,
    /// Address when the ADDR pin is connected to SCL.
    Scl,
}

impl TargetAddr {
    pub(crate) const fn bits(self) -> u8 {
        match self {
            Self::Gnd => 0b1001000,
            Self::Vdd => 0b1001001,
            Self::Sda => 0b1001010,
            Self::Scl => 0b1001011,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{FullScaleRange, TargetAddr};

    #[test]
    fn target_addr_default() {
        assert_eq!(0b100_1000, TargetAddr::default().bits());
    }

    #[test]
    fn target_addr_bits() {
        assert_eq!(0b100_1000, TargetAddr::Gnd.bits());
        assert_eq!(0b100_1001, TargetAddr::Vdd.bits());
        assert_eq!(0b100_1010, TargetAddr::Sda.bits());
        assert_eq!(0b100_1011, TargetAddr::Scl.bits());
    }

    #[test]
    fn default_full_scale_range() {
        assert_eq!(FullScaleRange::Within2_048V, FullScaleRange::default());
    }
}
