//! This is a platform-agnostic Rust driver for the ADS1013, ADS1014, ADS1015,
//! ADS1113, ADS1114, and ADS1115 ultra-small, low-power
//! analog-to-digital converters (ADC), based on the [`embedded-hal`] traits.
//!
//! [`embedded-hal`]: https://github.com/rust-embedded/embedded-hal
//!
//! This driver allows you to:
//! - Set the operating mode to one-shot or continuous. See [`Ads1115::into_one_shot`] and [`Ads1115::into_continuous`].
//! - Take a measurement in one-shot mode. See [`Ads1115<_, OneShot>::read`][read_one_shot].
//! - Read the last measurement made in continuous conversion mode. See [`Ads1115<_, Continuous>::read`][read_continous].
//! - Set the data rate. See [`Ads1115::set_data_rate`].
//! - Set the full-scale range (gain amplifier). See [`Ads1115::set_full_scale_range`].
//! - Read whether a measurement is in progress. See [`Ads1115::is_measurement_in_progress`].
//! - Set the ALERT/RDY pin to be used as conversion-ready pin. See [`Ads1115::use_alert_rdy_pin_as_ready`].
//! - Comparator:
//!     - Set the low and high thresholds. See [`Ads1115::set_high_threshold_raw`].
//!     - Set the comparator mode. See [`Ads1115::set_comparator_mode`].
//!     - Set the comparator polarity. See [`Ads1115::set_comparator_polarity`].
//!     - Set the comparator latching. See [`Ads1115::set_comparator_latching`].
//!     - Set the comparator queue. See [`Ads1115::set_comparator_queue`].
//!     - Disable the comparator. See [`Ads1115::disable_comparator`].
//!
//! [read_one_shot]: struct.Ads1115.html#method.read-1
//! [read_continous]: struct.Ads1115.html#method.read
//!
//! # The devices
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
//! periods. Data is transferred through I²C.
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
//! # Examples
//!
//! To use this driver, import this crate and an `embedded_hal` implementation,
//! then instantiate the appropriate device.
//! In the following examples an instance of the device ADS1013 will be created
//! as an example.
//!
//! Please find additional examples using hardware in this repository: [driver-examples]
//!
//! [driver-examples]: https://github.com/eldruin/driver-examples
//!
//! ## Creating a Driver Instance for an ADS1013
//!
//! ```no_run
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! use ads1x1x::{Ads1013, SlaveAddr};
//! use linux_embedded_hal::I2cdev;
//!
//! let i2c = I2cdev::new("/dev/i2c-1")?;
//! let adc = Ads1013::new(i2c, SlaveAddr::default());
//!
//! // Do something.
//!
//! // Get the I²C device back.
//! let i2c = adc.release();
//! # drop(i2c);
//! # Ok(())
//! # }
//! ```
//!
//! ### Creating a Driver Instance for an ADS1013 with an Alternative Address (Method 1)
//!
//! Using an alternative [`SlaveAddr`].
//!
//! ```no_run
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! # use ads1x1x::{Ads1013, SlaveAddr};
//! # use linux_embedded_hal::I2cdev;
//! #
//! # let i2c = I2cdev::new("/dev/i2c-1")?;
//! let adc = Ads1013::new(i2c, SlaveAddr::Alt2);
//! # Ok(())
//! # }
//! ```

//! ### Creating a Driver Instance for an ADS1013 with an Alternative Address (Method 2)
//!
//! Using [`SlaveAddr`] constants, depending on the connection of the ADDR pin.
//!
//! ```no_run
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! # use ads1x1x::{Ads1013, SlaveAddr};
//! # use linux_embedded_hal::I2cdev;
//! #
//! # let i2c = I2cdev::new("/dev/i2c-1")?;
//! // ADDR pin connected to SDA results in the `0x4A` effective address.
//! let adc = Ads1013::new(i2c, SlaveAddr::SDA);
//! # Ok(())
//! # }
//! ```
//!
//! ## Taking a One-Shot Measurement
//!
//! ```no_run
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! use ads1x1x::{channel, Ads1013, SlaveAddr};
//! use linux_embedded_hal::I2cdev;
//! use nb::block;
//!
//! let i2c = I2cdev::new("/dev/i2c-1")?;
//! let mut adc = Ads1013::new(i2c, SlaveAddr::default());
//!
//! let measurement = block!(adc.read(channel::DifferentialA0A1))?;
//! println!("Measurement: {}", measurement);
//! # Ok(())
//! # }
//! ```
//!
//! ## Changing to Continuous Conversion Mode and Reading the Last Measurement
//!
//! Changing the mode may fail in case there was a communication error.
//! In this case, you can retrieve the unchanged device from the error type.
//!
//! ```no_run
//! use ads1x1x::{Ads1013, SlaveAddr};
//! use linux_embedded_hal::I2cdev;
//!
//! let i2c = I2cdev::new("/dev/i2c-1").unwrap();
//! let adc = Ads1013::new(i2c, SlaveAddr::default());
//! match adc.into_continuous() {
//!     Err((e, adc)) => {
//!         panic!("Mode change failed: {e}")
//!     },
//!     Ok(mut adc) => {
//!         let measurement = adc.read().unwrap();
//!         // ...
//!     }
//! }
//! ```
//!
//!
//! ## Setting the Data Rate
//! For 12-bit devices, the available data rates are given by `DataRate12Bit`.
//! For 16-bit devices, the available data rates are given by `DataRate16Bit`.
//!
//! ```no_run
//! use ads1x1x::{Ads1115, DataRate16Bit, SlaveAddr};
//! use linux_embedded_hal::I2cdev;
//!
//! let i2c = I2cdev::new("/dev/i2c-1").unwrap();
//! let mut adc = Ads1115::new(i2c, SlaveAddr::default());
//!
//! adc.set_data_rate(DataRate16Bit::Sps860).unwrap();
//! ```
//!
//! ## Configuring the Comparator
//!
//! Configure the comparator to assert when the voltage drops below -1.5V
//! or goes above 1.5V in at least two consecutive conversions. Then the
//! ALERT/RDY pin will be set high and it will be kept so until the
//! measurement is read or an appropriate SMBus alert response is sent by
//! the master.
//!
//! ```no_run
//! use ads1x1x::{
//!     Ads1015, SlaveAddr, ComparatorQueue, ComparatorPolarity,
//!     ComparatorMode, ComparatorLatching, FullScaleRange
//! };
//! use linux_embedded_hal::I2cdev;
//!
//! let i2c = I2cdev::new("/dev/i2c-1").unwrap();
//! let mut adc = Ads1015::new(i2c, SlaveAddr::default());
//!
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

use core::marker::PhantomData;

pub mod channel;
pub use channel::ChannelId;
mod devices;
mod types;
pub use types::{
    ComparatorLatching, ComparatorMode, ComparatorPolarity, ComparatorQueue, DataRate12Bit,
    DataRate16Bit, Error, FullScaleRange, SlaveAddr,
};
pub mod mode;
pub use mode::*;
pub(crate) mod register;
use register::{Config, Reg};

macro_rules! impl_ads1x1x {
    ($name:expr, $Ads:ident) => {
        #[doc = concat!("An ", $name, " ADC.")]
        #[derive(Debug)]
        pub struct $Ads<I2C, MODE> {
            pub(crate) i2c: I2C,
            pub(crate) address: SlaveAddr,
            pub(crate) config: Config,
            pub(crate) a_conversion_was_started: bool,
            pub(crate) mode: PhantomData<MODE>,
        }

        impl<I2C> $Ads<I2C, mode::OneShot> {
            #[doc = concat!("Creates a new ", $name, " instance in one-shot mode.")]
            pub fn new(i2c: I2C, address: SlaveAddr) -> Self {
                $Ads {
                    i2c,
                    address,
                    config: Config::default(),
                    a_conversion_was_started: false,
                    mode: PhantomData,
                }
            }
        }

        impl<I2C, E, MODE> $Ads<I2C, MODE>
        where
            I2C: embedded_hal::i2c::I2c<Error = E>,
        {
            pub(crate) fn read_reg_u16<R: Reg<u16>>(&mut self) -> Result<R, Error<E>> {
                let mut buf = [0, 0];
                self.i2c
                    .write_read(self.address.bits(), &[R::ADDR], &mut buf)
                    .map_err(Error::I2C)?;
                Ok(R::from_reg(u16::from_be_bytes(buf)))
            }

            pub(crate) fn write_reg_u16<R: Reg<u16>>(&mut self, reg: R) -> Result<(), Error<E>> {
                let buf = reg.to_reg().to_be_bytes();
                let payload: [u8; 3] = [R::ADDR, buf[0], buf[1]];
                self.i2c
                    .write(self.address.bits(), &payload)
                    .map_err(Error::I2C)
            }
        }

        impl<I2C, MODE> $Ads<I2C, MODE> {
            /// Releases the contained I²C peripheral.
            pub fn release(self) -> I2C {
                self.i2c
            }
        }
    };
}

impl_ads1x1x!("ADS1013", Ads1013);
impl_ads1x1x!("ADS1113", Ads1113);
impl_ads1x1x!("ADS1014", Ads1014);
impl_ads1x1x!("ADS1114", Ads1114);
impl_ads1x1x!("ADS1015", Ads1015);
impl_ads1x1x!("ADS1115", Ads1115);
