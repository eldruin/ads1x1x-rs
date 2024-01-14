//! Tier 2 features.
//!
//! These are the features included only in ADS1x14, ADS1x15

use crate::{
    register::{Config, Conversion12, Conversion16, HiThresh, LoThresh},
    Ads1014, Ads1015, Ads1114, Ads1115, ComparatorLatching, ComparatorMode, ComparatorPolarity,
    ComparatorQueue, Error, FullScaleRange,
};

macro_rules! doc_threshold {
    (-32768, 32767) => {
        ""
    };
    ($th_low:literal, $th_high:literal) => {
        concat!(
            "The given value must be within \\[",
            stringify!($th_low),
            ", ",
            stringify!($th_high),
            "\\].\n\n# Panics\n\nPanics if the threshold is outside the \\[",
            stringify!($th_low),
            ", ",
            stringify!($th_high),
            "\\] range."
        )
    };
}

macro_rules! impl_tier2_features {
    ($Ads:ident, $conv:ty, [$($th_range:tt)+]) => {
        impl<I2C, E, MODE> $Ads<I2C, MODE>
        where
            I2C: embedded_hal::i2c::I2c<Error = E>,
        {
            /// Sets the input voltage measurable range.
            ///
            /// This configures the programmable gain amplifier (PGA) and determines the measurable input voltage range.
            pub fn set_full_scale_range(&mut self, range: FullScaleRange) -> Result<(), Error<E>> {
                let config = range.configure(self.config);
                self.write_reg_u16(config)?;
                self.config = config;
                Ok(())
            }

            /// Sets the raw comparator lower threshold.
            ///
            /// The voltage that these values correspond to must be calculated using the
            /// full-scale range ([`FullScaleRange`]) selected.
            ///
            #[doc = doc_threshold!($($th_range)+)]
            pub fn set_low_threshold_raw(&mut self, value: i16) -> Result<(), Error<E>> {
                let register_value = <$conv>::convert_threshold(value);
                self.write_reg_u16(LoThresh(register_value))
            }

            /// Sets the raw comparator upper threshold.
            ///
            /// The voltage that these values correspond to must be calculated using the
            /// full-scale range ([`FullScaleRange`]) selected.
            ///
            #[doc = doc_threshold!($($th_range)+)]
            pub fn set_high_threshold_raw(&mut self, value: i16) -> Result<(), Error<E>> {
                let register_value = <$conv>::convert_threshold(value);
                self.write_reg_u16(HiThresh(register_value))
            }

            /// Sets the comparator mode.
            pub fn set_comparator_mode(&mut self, mode: ComparatorMode) -> Result<(), Error<E>> {
                let config = match mode {
                    ComparatorMode::Traditional => self.config.difference(Config::COMP_MODE),
                    ComparatorMode::Window => self.config.union(Config::COMP_MODE),
                };
                self.write_reg_u16(config)?;
                self.config = config;
                Ok(())
            }

            /// Sets the comparator polarity.
            pub fn set_comparator_polarity(
                &mut self,
                polarity: ComparatorPolarity,
            ) -> Result<(), Error<E>> {
                let config = match polarity {
                    ComparatorPolarity::ActiveLow => self.config.difference(Config::COMP_POL),
                    ComparatorPolarity::ActiveHigh => self.config.union(Config::COMP_POL),
                };
                self.write_reg_u16(config)?;
                self.config = config;
                Ok(())
            }

            /// Sets the comparator latching.
            pub fn set_comparator_latching(
                &mut self,
                latching: ComparatorLatching,
            ) -> Result<(), Error<E>> {
                let config = match latching {
                    ComparatorLatching::Nonlatching => self.config.difference(Config::COMP_LAT),
                    ComparatorLatching::Latching => self.config.union(Config::COMP_LAT),
                };
                self.write_reg_u16(config)?;
                self.config = config;
                Ok(())
            }

            /// Activates the comparator and sets the alert queue.
            ///
            /// The comparator can be disabled with [`disable_comparator`](Self::disable_comparator).
            pub fn set_comparator_queue(&mut self, queue: ComparatorQueue) -> Result<(), Error<E>> {
                let config = match queue {
                    ComparatorQueue::One => {
                        self.config.difference(Config::COMP_QUE1).difference(Config::COMP_QUE0)
                    }
                    ComparatorQueue::Two => {
                        self.config.difference(Config::COMP_QUE1).union(Config::COMP_QUE0)
                    }
                    ComparatorQueue::Four => {
                        self.config.union(Config::COMP_QUE1).difference(Config::COMP_QUE0)
                    }
                };
                self.write_reg_u16(config)?;
                self.config = config;
                Ok(())
            }

            /// Disables the comparator. (default)
            ///
            /// This sets the ALERT/RDY pin to high-impedance.
            ///
            /// The comparator can be enabled by setting the comparator queue using
            /// the [`set_comparator_queue`](Self::set_comparator_queue) method.
            pub fn disable_comparator(&mut self) -> Result<(), Error<E>> {
                let config = self
                    .config
                    .union(Config::COMP_QUE1)
                    .union(Config::COMP_QUE0);
                self.write_reg_u16(config)?;
                self.config = config;
                Ok(())
            }

            /// Sets the ALERT/RDY pin as conversion-ready pin.
            ///
            /// This the ALERT/RDY pin outputs the OS bit when in OneShot mode, and
            /// provides a continuous-conversion ready pulse when in
            /// continuous-conversion mode.
            ///
            /// When calling this the comparator will be disabled and the thresholds will be cleared.
            pub fn use_alert_rdy_pin_as_ready(&mut self) -> Result<(), Error<E>> {
                if !self.config.contains(Config::COMP_QUE)
                {
                    self.disable_comparator()?;
                }
                self.write_reg_u16(HiThresh(0b1000000000000000))?;
                self.write_reg_u16(LoThresh(0))
            }
        }
    };
}

impl_tier2_features!(Ads1014, Conversion12, [-2048, 2047]);
impl_tier2_features!(Ads1015, Conversion12, [-2048, 2047]);
impl_tier2_features!(Ads1114, Conversion16, [-32768, 32767]);
impl_tier2_features!(Ads1115, Conversion16, [-32768, 32767]);
