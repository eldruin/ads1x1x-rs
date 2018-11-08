//! This is a platform-agnostic Rust driver for the ADS1013, ADS1014, ADS1015,
//! ADS1113, ADS1114, ADS1115, ADS1018 and ADS1118 ultra-small, low-power
//! analog-to-digital converters (ADC), based on the [`embedded-hal`] traits.
//!
//! [`embedded-hal`]: https://github.com/rust-embedded/embedded-hal
//!
//! This driver allows you to:
//! - Set the operating mode to one-shot or continuous. See: [`into_continuous()`].
//!
//! [`into_continuous()`]: struct.Ads1x1x.html#method.into_continuous
//!
//! ## The devices
//!
//! The devices are precision, low power, 12/16-bit analog-to-digital
//! converters (ADC) that provide all features necessary to measure the most
//! common sensor signals in an ultra-small package. Depending on the device,
//! these  integrate a programmable gain amplifier (PGA), voltage reference,
//! oscillator and high-accuracy temperature sensor.
//!
//! The devices can perform conversions at data rates up to 3300 samples per
//! second (SPS). The PGA offers input ranges from ±256 mV to ±6.144 V,
//! allowing both large and small signals to be measured with high resolution.
//! An input multiplexer (MUX) allows to measure two differential or four
//! single-ended inputs. The high-accuracy temperature sensor can be used for
//! system-level temperature monitoring or cold-junction compensation for
//! thermocouples.
//!
//! The devices operate either in continuous-conversion mode, or in a
//! single-shot mode that automatically powers down after a conversion.
//! Single-shot mode significantly reduces current consumption during idle
//! periods. Data are transferred through a I2C or SPI.
//!
//! Here is a comparison of the caracteristics of the devices:
//!
//! | Device  | Resolution | Sample Rate  | Channels | Interface | Multi-channel | Features                     |
//! |---------|------------|--------------|----------|-----------|---------------|------------------------------|
//! | ADS1013 | 12-bit     | Max 3300 SPS | 1        | I2C       | N/A           |                              |
//! | ADS1014 | 12-bit     | Max 3300 SPS | 1        | I2C       | N/A           | Comparator, PGA              |
//! | ADS1015 | 12-bit     | Max 3300 SPS | 4        | I2C       | Multiplexed   | Comparator, PGA              |
//! | ADS1018 | 12-bit     | Max 3300 SPS | 4        | SPI       | Multiplexed   | Comparator, PGA, Temp sensor |
//! | ADS1113 | 16-bit     | Max 860 SPS  | 1        | I2C       | N/A           |                              |
//! | ADS1114 | 16-bit     | Max 860 SPS  | 1        | I2C       | N/A           | Comparator, PGA              |
//! | ADS1115 | 16-bit     | Max 860 SPS  | 4        | I2C       | Multiplexed   | Comparator, PGA              |
//! | ADS1118 | 16-bit     | Max 860 SPS  | 4        | SPI       | Multiplexed   | Comparator, PGA, Temp sensor |
//!
//! Datasheets:
//! - [ADS101x](http://www.ti.com/lit/ds/symlink/ads1015.pdf)
//! - [ADS1018](http://www.ti.com/lit/ds/symlink/ads1018.pdf)
//! - [ADS111x](http://www.ti.com/lit/ds/symlink/ads1115.pdf)
//! - [ADS1118](http://www.ti.com/lit/ds/symlink/ads1118.pdf)
//!

#![deny(unsafe_code)]
#![deny(missing_docs)]
#![deny(warnings)]
#![no_std]

extern crate nb;
extern crate embedded_hal as hal;
use core::marker::PhantomData;

/// All possible errors in this crate
#[derive(Debug)]
pub enum Error<E> {
    /// I²C/SPI bus error
    Comm(E),
    /// Invalid input data provided
    InvalidInputData
}

const DEVICE_BASE_ADDRESS : u8 = 0b100_1000;

/// Mode marker types
pub mod mode {
    /// One-shot operating mode / power-down state (default)
    pub struct OneShot(());

    /// Continuous conversion mode
    pub struct Continuous(());
}

/// Data rate
#[derive(Debug, Clone)]
pub enum DataRate {
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
    Sps3300
}

/// Possible slave addresses
#[derive(Debug, Clone)]
pub enum SlaveAddr {
    /// Default slave address
    Default,
    /// Alternative slave address providing bit values for A1 and A0
    Alternative(bool, bool)
}

impl Default for SlaveAddr {
    /// Default slave address
    fn default() -> Self {
        SlaveAddr::Default
    }
}

impl SlaveAddr {
    fn addr(self, default: u8) -> u8 {
        match self {
            SlaveAddr::Default => default,
            SlaveAddr::Alternative(a1, a0) => default           |
                                              ((a1 as u8) << 1) |
                                                a0 as u8
        }
    }
}

struct Register;

impl Register {
    const CONVERSION : u8 = 0x00;
    const CONFIG     : u8 = 0x01;
    const LOW_TH     : u8 = 0x02;
    const HIGH_TH    : u8 = 0x03;
}

struct BitFlags;
impl BitFlags {
    const OP_MODE      : u16 = 0b0000_0001_0000_0000;
    const OS           : u16 = 0b1000_0000_0000_0000;
    const DR2          : u16 = 0b0000_0000_1000_0000;
    const DR1          : u16 = 0b0000_0000_0100_0000;
    const DR0          : u16 = 0b0000_0000_0010_0000;
}


#[derive(Debug, Clone)]
struct Config {
    bits: u16
}

impl Config {
    fn is_high(&self, mask : u16) -> bool {
        (self.bits & mask) != 0
    }

    fn with_high(&self, mask: u16) -> Self {
        Config { bits: self.bits | mask }
    }
    fn with_low(&self, mask: u16) -> Self {
        Config { bits: self.bits & !mask }
    }
}

impl Default for Config {
    fn default() -> Self {
        Config { bits: 0x8583 }
    }
}

/// ADS1x1x ADC driver
#[derive(Debug, Default)]
pub struct Ads1x1x<DI, IC, MODE> {
    iface: DI,
    config: Config,
    a_conversion_was_started: bool,
    _ic: PhantomData<IC>,
    _mode: PhantomData<MODE>
}

#[doc(hidden)]
pub mod interface;
mod devices;
pub use devices::ic;
pub use devices::channel;

mod private {
    use super::{ devices::ic, interface };
    pub trait Sealed {}

    impl<I2C> Sealed for interface::I2cInterface<I2C> {}
    impl<SPI, CS> Sealed for interface::SpiInterface<SPI, CS> {}

    impl Sealed for ic::Ads1013 {}
    impl Sealed for ic::Ads1113 {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_get_default_address() {
        let addr = SlaveAddr::default();
        assert_eq!(DEVICE_BASE_ADDRESS, addr.addr(DEVICE_BASE_ADDRESS));
    }

    #[test]
    fn can_generate_alternative_addresses() {
        assert_eq!(0b100_1000, SlaveAddr::Alternative(false, false).addr(DEVICE_BASE_ADDRESS));
        assert_eq!(0b100_1001, SlaveAddr::Alternative(false,  true).addr(DEVICE_BASE_ADDRESS));
        assert_eq!(0b100_1010, SlaveAddr::Alternative(true, false).addr(DEVICE_BASE_ADDRESS));
        assert_eq!(0b100_1011, SlaveAddr::Alternative(true,  true).addr(DEVICE_BASE_ADDRESS));
    }
}
