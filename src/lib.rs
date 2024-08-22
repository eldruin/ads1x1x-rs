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
//! - Read whether a measurement is in progress. See: [`is_measurement_in_progress()`].
//! - Set the ALERT/RDY pin to be used as conversion-ready pin. See: [`use_alert_rdy_pin_as_ready()`].
//! - Comparator:
//!     - Set the low and high thresholds. See: [`set_high_threshold_raw()`].
//!     - Set the comparator mode. See: [`set_comparator_mode()`].
//!     - Set the comparator polarity. See: [`set_comparator_polarity()`].
//!     - Set the comparator latching. See: [`set_comparator_latching()`].
//!     - Set the comparator queue. See: [`set_comparator_queue()`].
//!     - Disable the comparator. See: [`disable_comparator()`].
//!
//! [`into_continuous()`]: struct.Ads1x1x.html#method.into_continuous
//! [read_os]: struct.Ads1x1x.html#method.read-1
//! [`start()`]: struct.Ads1x1x.html#method.start
//! [read_cont]: struct.Ads1x1x.html#method.read
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
//! | Device  | Resolution | Sample Rate  | Channels | Multi-channel | Features        |
//! |---------|------------|--------------|----------|---------------|-----------------|
//! | ADS1013 | 12-bit     | Max 3300 SPS | 1        | N/A           |                 |
//! | ADS1014 | 12-bit     | Max 3300 SPS | 1        | N/A           | Comparator, PGA |
//! | ADS1015 | 12-bit     | Max 3300 SPS | 4        | Multiplexed   | Comparator, PGA |
//! | ADS1113 | 16-bit     | Max 860 SPS  | 1        | N/A           |                 |
//! | ADS1114 | 16-bit     | Max 860 SPS  | 1        | N/A           | Comparator, PGA |
//! | ADS1115 | 16-bit     | Max 860 SPS  | 4        | Multiplexed   | Comparator, PGA |
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
//! Please find additional examples using hardware in this repository: [driver-examples]
//!
//! [driver-examples]: https://github.com/eldruin/driver-examples
//!
//! ### Create a driver instance for an ADS1013 with the default address.
//!
//! ```no_run
//! use linux_embedded_hal::I2cdev;
//! use ads1x1x::{Ads1x1x, TargetAddr};
//!
//! let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! let adc = Ads1x1x::new_ads1013(dev, TargetAddr::default());
//! // do something...
//!
//! // get the I2C device back
//! let dev = adc.destroy_ads1013();
//! ```
//!
//! ### Create a driver instance for an ADS1013 with the ADDR pin connected to SDA.
//!
//! ```no_run
//! use linux_embedded_hal::I2cdev;
//! use ads1x1x::{Ads1x1x, TargetAddr};
//!
//! let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! let adc = Ads1x1x::new_ads1013(dev, TargetAddr::Sda);
//! ```
//!
//! ### Make a one-shot measurement
//! ```no_run
//! use ads1x1x::{channel, Ads1x1x, TargetAddr};
//! use linux_embedded_hal::I2cdev;
//! use nb::block;
//!
//! let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! let mut adc = Ads1x1x::new_ads1013(dev, TargetAddr::default());
//! let measurement = block!(adc.read(channel::DifferentialA0A1)).unwrap();
//! println!("Measurement: {}", measurement);
//! let _dev = adc.destroy_ads1013(); // get I2C device back
//! ```
//!
//! ### Change into continuous conversion mode and read the last measurement
//!
//! Changing the mode may fail in case there was a communication error.
//! In this case, you can retrieve the unchanged device from the error type.
//!
//! ```no_run
//! use linux_embedded_hal::I2cdev;
//! use ads1x1x::{Ads1x1x, ModeChangeError, TargetAddr};
//!
//! let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! let adc = Ads1x1x::new_ads1013(dev, TargetAddr::default());
//! match adc.into_continuous() {
//!     Err(ModeChangeError::I2C(e, adc)) => /* mode change failed handling */ panic!(),
//!     Ok(mut adc) => {
//!         let measurement = adc.read().unwrap();
//!         // ...
//!     }
//! }
//! ```
//!
//!
//! ### Set the data rate
//! For 12-bit devices, the available data rates are given by `DataRate12Bit`.
//! For 16-bit devices, the available data rates are given by `DataRate16Bit`.
//!
//! ```no_run
//! use linux_embedded_hal::I2cdev;
//! use ads1x1x::{Ads1x1x, DataRate16Bit, TargetAddr};
//!
//! let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! let mut adc = Ads1x1x::new_ads1115(dev, TargetAddr::default());
//! adc.set_data_rate(DataRate16Bit::Sps860).unwrap();
//! ```
//!
//! ### Configure the comparator
//! Configure the comparator to assert when the voltage drops below -1.5V
//! or goes above 1.5V in at least two consecutive conversions. Then the
//! ALERT/RDY pin will be set high and it will be kept so until the
//! measurement is read or an appropriate SMBus alert response is sent by
//! the controller.
//!
//! ```no_run
//! use linux_embedded_hal::I2cdev;
//! use ads1x1x::{
//!     Ads1x1x, TargetAddr, ComparatorQueue, ComparatorPolarity,
//!     ComparatorMode, ComparatorLatching, FullScaleRange
//! };
//!
//! let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! let address = TargetAddr::default();
//! let mut adc = Ads1x1x::new_ads1015(dev, address);
//! adc.set_comparator_queue(ComparatorQueue::Two).unwrap();
//! adc.set_comparator_polarity(ComparatorPolarity::ActiveHigh).unwrap();
//! adc.set_comparator_mode(ComparatorMode::Window).unwrap();
//! adc.set_full_scale_range(FullScaleRange::Within2_048V).unwrap();
//! adc.set_low_threshold_raw(-1500).unwrap();
//! adc.set_high_threshold_raw(1500).unwrap();
//! adc.set_comparator_latching(ComparatorLatching::Latching).unwrap();
//! ```
#![deny(unsafe_code)]
#![deny(missing_docs)]
#![no_std]

struct Register;
impl Register {
    const CONVERSION: u8 = 0x00;
    const CONFIG: u8 = 0x01;
    const LOW_TH: u8 = 0x02;
    const HIGH_TH: u8 = 0x03;
}

struct BitFlags;
impl BitFlags {
    const OS: u16 = 0b1000_0000_0000_0000;
    const MUX2: u16 = 0b0100_0000_0000_0000;
    const MUX1: u16 = 0b0010_0000_0000_0000;
    const MUX0: u16 = 0b0001_0000_0000_0000;
    const PGA2: u16 = 0b0000_1000_0000_0000;
    const PGA1: u16 = 0b0000_0100_0000_0000;
    const PGA0: u16 = 0b0000_0010_0000_0000;
    const OP_MODE: u16 = 0b0000_0001_0000_0000;
    const DR2: u16 = 0b0000_0000_1000_0000;
    const DR1: u16 = 0b0000_0000_0100_0000;
    const DR0: u16 = 0b0000_0000_0010_0000;
    const COMP_MODE: u16 = 0b0000_0000_0001_0000;
    const COMP_POL: u16 = 0b0000_0000_0000_1000;
    const COMP_LAT: u16 = 0b0000_0000_0000_0100;
    const COMP_QUE1: u16 = 0b0000_0000_0000_0010;
    const COMP_QUE0: u16 = 0b0000_0000_0000_0001;
}

pub mod channel;
pub use channel::ChannelId;
mod construction;
mod conversion;
pub use crate::conversion::{ConvertMeasurement, ConvertThreshold};
mod devices;
#[doc(hidden)]
pub mod ic;
mod types;
use crate::types::Config;
pub use crate::types::{
    mode, Ads1x1x, ComparatorLatching, ComparatorMode, ComparatorPolarity, ComparatorQueue,
    DataRate12Bit, DataRate16Bit, Error, FullScaleRange, ModeChangeError, TargetAddr,
};

mod private {
    use super::{ic, Ads1x1x};
    pub trait Sealed {}

    impl<I2C, IC, CONV, MODE> Sealed for Ads1x1x<I2C, IC, CONV, MODE> {}

    impl Sealed for ic::Resolution12Bit {}
    impl Sealed for ic::Resolution16Bit {}

    impl Sealed for ic::Ads1013 {}
    impl Sealed for ic::Ads1113 {}
    impl Sealed for ic::Ads1014 {}
    impl Sealed for ic::Ads1114 {}
    impl Sealed for ic::Ads1015 {}
    impl Sealed for ic::Ads1115 {}
}
