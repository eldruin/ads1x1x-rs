//! This is a platform-agnostic Rust driver for the ADS1013, ADS1014, ADS1015,
//! ADS1113, ADS1114, and ADS1115 ultra-small, low-power
//! analog-to-digital converters (ADC), based on the [`embedded-hal`] traits.
//!
//! [`embedded-hal`]: https://github.com/rust-embedded/embedded-hal
//!
//! This driver allows you to:
//! - Set the operating mode to one-shot or continuous. See: [`into_continuous()`].
//! - Make a measurement in one-shot mode. See: [`read()`][read_os].
//! - Start continuous conversion mode. See: [`start()`].
//! - Read the last measurement made in continuous conversion mode. See: [`read()`][read_cont].
//! - Set the data rate. See: [`set_data_rate()`].
//! - Set the full-scale range (gain amplifier). See [`set_full_scale_range()`].
//! - Set the low and high thresholds. See: [`set_high_threshold_raw()`].
//! - Read whether a measurement is in progress. See: [`is_measurement_in_progress()`].
//! - Set the comparator mode. See: [`set_comparator_mode()`].
//! - Set the comparator polarity. See: [`set_comparator_polarity()`].
//! - Set the comparator latching. See: [`set_comparator_latching()`].
//! - Set the comparator queue. See: [`set_comparator_queue()`].
//! - Disable the comparator. See: [`disable_comparator()`].
//! - Set the ALERT/RDY pin to be used as conversion-ready pin. See: [`use_alert_rdy_pin_as_ready()`].
//!
//! [`into_continuous()`]: struct.Ads1x1x.html#method.into_continuous
//! [read_os]: struct.Ads1x1x.html#method.read
//! [`start()`]: struct.Ads1x1x.html#method.read
//! [read_cont]: struct.Ads1x1x.html#impl-OneShot%3CAds1x1x%3CDI%2C%20IC%2C%20CONV%2C%20OneShot%3E%2C%20i16%2C%20CH%3E
//! [`set_data_rate()`]: struct.Ads1x1x.html#method.set_data_rate
//! [`set_full_scale_range()`]: struct.Ads1x1x.html#method.set_full_scale_range
//! [`is_measurement_in_progress()`]: struct.Ads1x1x.html#method.is_measurement_in_progress
//! [`set_high_threshold_raw()`]: struct.Ads1x1x.html#method.set_high_threshold_raw
//! [`set_comparator_mode()`]: struct.Ads1x1x.html#method.set_comparator_mode
//! [`set_comparator_polarity()`]: struct.Ads1x1x.html#method.set_comparator_polarity
//! [`set_comparator_latching()`]: struct.Ads1x1x.html#method.set_comparator_latching
//! [`set_comparator_queue()`]: struct.Ads1x1x.html#method.set_comparator_queue
//! [`disable_comparator()`]: struct.Ads1x1x.html#method.disable_comparator
//! [`use_alert_rdy_pin_as_ready()`]: struct.Ads1x1x.html#method.use_alert_rdy_pin_as_ready
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
//! periods. Data is transferred through I2C.
//!
//! Here is a comparison of the caracteristics of the devices:
//!
//! | Device  | Resolution | Sample Rate  | Channels | Multi-channel | Features                     |
//! |---------|------------|--------------|----------|---------------|------------------------------|
//! | ADS1013 | 12-bit     | Max 3300 SPS | 1        | N/A           |                              |
//! | ADS1014 | 12-bit     | Max 3300 SPS | 1        | N/A           | Comparator, PGA              |
//! | ADS1015 | 12-bit     | Max 3300 SPS | 4        | Multiplexed   | Comparator, PGA              |
//! | ADS1113 | 16-bit     | Max 860 SPS  | 1        | N/A           |                              |
//! | ADS1114 | 16-bit     | Max 860 SPS  | 1        | N/A           | Comparator, PGA              |
//! | ADS1115 | 16-bit     | Max 860 SPS  | 4        | Multiplexed   | Comparator, PGA              |
//!
//! Datasheets:
//! - [ADS101x](http://www.ti.com/lit/ds/symlink/ads1015.pdf)
//! - [ADS111x](http://www.ti.com/lit/ds/symlink/ads1115.pdf)
//!
//! ## Usage examples (see also examples folder)
//!
//! To use this driver, import this crate and an `embedded_hal` implementation,
//! then instantiate the appropriate device.
//! In the following examples an instance of the device ADS1013 will be created
//! as an example. Other devices can be created with similar methods like:
//! `Ads1x1x::new_ads1114(...)`.
//!
//! ### Create a driver instance for the ADS1013
//!
//! ```no_run
//! extern crate linux_embedded_hal as hal;
//! extern crate ads1x1x;
//! use ads1x1x::{ Ads1x1x, SlaveAddr };
//!
//! # fn main() {
//! let dev = hal::I2cdev::new("/dev/i2c-1").unwrap();
//! let address = SlaveAddr::default();
//! let adc = Ads1x1x::new_ads1013(dev, address);
//! // do something...
//!
//! // get the I2C device back
//! let dev = adc.destroy_ads1013();
//! # }
//! ```
//!
//! ### Create a driver instance for the ADS1013 with an alternative address
//!
//! ```no_run
//! extern crate linux_embedded_hal as hal;
//! extern crate ads1x1x;
//! use ads1x1x::{ Ads1x1x, SlaveAddr };
//!
//! # fn main() {
//! let dev = hal::I2cdev::new("/dev/i2c-1").unwrap();
//! let (a1, a0) = (true, false);
//! let address = SlaveAddr::Alternative(a1, a0);
//! let adc = Ads1x1x::new_ads1013(dev, address);
//! # }
//! ```
//!
//! ### Make a one-shot measurement
//! ```no_run
//! extern crate embedded_hal;
//! use embedded_hal::adc::OneShot;
//! extern crate linux_embedded_hal;
//! #[macro_use(block)]
//! extern crate nb;
//! extern crate ads1x1x;
//!
//! use linux_embedded_hal::I2cdev;
//! use ads1x1x::{ Ads1x1x, SlaveAddr, channel };
//!
//! # fn main() {
//!     let dev = I2cdev::new("/dev/i2c-1").unwrap();
//!     let mut adc = Ads1x1x::new_ads1013(dev, SlaveAddr::default());
//!     let measurement = block!(adc.read(&mut channel::DifferentialA0A1)).unwrap();
//!     println!("Measurement: {}", measurement);
//!     let _dev = adc.destroy_ads1013(); // get I2C device back
//! # }
//! ```
//!
//! ### Change into continuous conversion mode and read the last measurement
//!
//! ```no_run
//! extern crate linux_embedded_hal as hal;
//! extern crate ads1x1x;
//! use ads1x1x::{ Ads1x1x, SlaveAddr };
//!
//! # fn main() {
//! let dev = hal::I2cdev::new("/dev/i2c-1").unwrap();
//! let address = SlaveAddr::default();
//! let adc = Ads1x1x::new_ads1013(dev, address);
//! let mut adc = adc.into_continuous().unwrap();
//! adc.start().unwrap();
//! while(!adc.is_measurement_in_progress().unwrap()) {
//!     // some delay...
//! }
//! let measurement = adc.read().unwrap();
//! # }
//! ```
//!
//!
//! ### Set the data rate
//! For 12-bit devices, the available data rates are given by `DataRate12Bit`.
//! For 16-bit devices, the available data rates are given by `DataRate16Bit`.
//!
//! ```no_run
//! extern crate linux_embedded_hal as hal;
//! extern crate ads1x1x;
//! use ads1x1x::{ Ads1x1x, SlaveAddr, DataRate16Bit };
//!
//! # fn main() {
//! let dev = hal::I2cdev::new("/dev/i2c-1").unwrap();
//! let address = SlaveAddr::default();
//! let mut adc = Ads1x1x::new_ads1115(dev, address);
//! adc.set_data_rate(DataRate16Bit::Sps860).unwrap();
//! # }
//! ```
//!
//! ### Configure the comparator
//! Configure the comparator to assert when the voltage drops below -1.5V
//! or goes above 1.5V in at least two consecutive conversions. Then the
//! ALERT/RDY pin will be set high and it will be kept so until the
//! measurement is read or an appropriate SMBus alert response is sent by
//! the master.
//!
//! ```no_run
//! extern crate linux_embedded_hal as hal;
//! extern crate ads1x1x;
//! use ads1x1x::{ Ads1x1x, SlaveAddr, ComparatorQueue, ComparatorPolarity,
//!                ComparatorMode, ComparatorLatching, FullScaleRange };
//!
//! # fn main() {
//! let dev = hal::I2cdev::new("/dev/i2c-1").unwrap();
//! let address = SlaveAddr::default();
//! let mut adc = Ads1x1x::new_ads1015(dev, address);
//! adc.set_comparator_queue(ComparatorQueue::Two).unwrap();
//! adc.set_comparator_polarity(ComparatorPolarity::ActiveHigh).unwrap();
//! adc.set_comparator_mode(ComparatorMode::Window).unwrap();
//! adc.set_full_scale_range(FullScaleRange::Within2_048V).unwrap();
//! adc.set_low_threshold_raw(-1500).unwrap();
//! adc.set_high_threshold_raw(1500).unwrap();
//! adc.set_comparator_latching(ComparatorLatching::Latching).unwrap();
//! # }
//! ```

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
    /// I²C bus error
    Comm(E),
    /// Invalid input data provided
    InvalidInputData,
    /// Continuous measurement was not started
    NotStarted,
}

const DEVICE_BASE_ADDRESS : u8 = 0b100_1000;

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
    Sps3300
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
    Sps860
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
    Window
}

/// Comparator polarity (only for ADS1x14, ADS1x15)
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ComparatorPolarity {
    /// Active low (default)
    ActiveLow,
    /// Active high
    ActiveHigh
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
    Latching
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
pub enum FullScaleRange {
    /// The measurable range is ±6.144V.
    Within6_144V,
    /// The measurable range is ±4.096V.
    Within4_096V,
    /// The measurable range is ±2.048V. (default)
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
    const OS           : u16 = 0b1000_0000_0000_0000;
    const MUX2         : u16 = 0b0100_0000_0000_0000;
    const MUX1         : u16 = 0b0010_0000_0000_0000;
    const MUX0         : u16 = 0b0001_0000_0000_0000;
    const PGA2         : u16 = 0b0000_1000_0000_0000;
    const PGA1         : u16 = 0b0000_0100_0000_0000;
    const PGA0         : u16 = 0b0000_0010_0000_0000;
    const OP_MODE      : u16 = 0b0000_0001_0000_0000;
    const DR2          : u16 = 0b0000_0000_1000_0000;
    const DR1          : u16 = 0b0000_0000_0100_0000;
    const DR0          : u16 = 0b0000_0000_0010_0000;
    const COMP_MODE    : u16 = 0b0000_0000_0001_0000;
    const COMP_POL     : u16 = 0b0000_0000_0000_1000;
    const COMP_LAT     : u16 = 0b0000_0000_0000_0100;
    const COMP_QUE1    : u16 = 0b0000_0000_0000_0010;
    const COMP_QUE0    : u16 = 0b0000_0000_0000_0001;
}


#[derive(Debug, Clone, PartialEq)]
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

impl Default for FullScaleRange {
    fn default() -> Self {
        FullScaleRange::Within2_048V
    }
}

/// ADS1x1x ADC driver
#[derive(Debug, Default)]
pub struct Ads1x1x<DI, IC, CONV, MODE> {
    iface: DI,
    config: Config,
    fsr: FullScaleRange,
    a_conversion_was_started: bool,
    _conv: PhantomData<CONV>,
    _ic: PhantomData<IC>,
    _mode: PhantomData<MODE>
}

#[doc(hidden)]
pub mod interface;
#[doc(hidden)]
pub mod ic;
mod channels;
pub use channels::channel;
mod devices;
mod construction;
mod conversion;
pub use conversion::ConvertThreshold;
pub use conversion::ConvertMeasurement;

mod private {
    use super::{ ic, interface };
    pub trait Sealed {}

    impl<I2C> Sealed for interface::I2cInterface<I2C> {}

    impl Sealed for ic::Resolution12Bit {}
    impl Sealed for ic::Resolution16Bit {}

    impl Sealed for ic::Ads1013 {}
    impl Sealed for ic::Ads1113 {}
    impl Sealed for ic::Ads1014 {}
    impl Sealed for ic::Ads1114 {}
    impl Sealed for ic::Ads1015 {}
    impl Sealed for ic::Ads1115 {}
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
