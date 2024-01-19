//! Tier 2 features.
//!
//! These are the features included only in ADS1x14, ADS1x15

use crate::{
    ic, Ads1014, Ads1015, Ads1114, Ads1115, BitFlags as BF, ComparatorLatching, ComparatorMode,
    ComparatorPolarity, ComparatorQueue, Error, FullScaleRange, Register,
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
                let config = match range {
                    FullScaleRange::Within6_144V => self
                        .config
                        .with_low(BF::PGA2)
                        .with_low(BF::PGA1)
                        .with_low(BF::PGA0),
                    FullScaleRange::Within4_096V => self
                        .config
                        .with_low(BF::PGA2)
                        .with_low(BF::PGA1)
                        .with_high(BF::PGA0),
                    FullScaleRange::Within2_048V => self
                        .config
                        .with_low(BF::PGA2)
                        .with_high(BF::PGA1)
                        .with_low(BF::PGA0),
                    FullScaleRange::Within1_024V => self
                        .config
                        .with_low(BF::PGA2)
                        .with_high(BF::PGA1)
                        .with_high(BF::PGA0),
                    FullScaleRange::Within0_512V => self
                        .config
                        .with_high(BF::PGA2)
                        .with_low(BF::PGA1)
                        .with_low(BF::PGA0),
                    FullScaleRange::Within0_256V => self
                        .config
                        .with_high(BF::PGA2)
                        .with_low(BF::PGA1)
                        .with_high(BF::PGA0),
                };
                self.write_register(Register::CONFIG, config.bits)?;
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
                self.write_register(Register::LOW_TH, register_value)
            }

            /// Sets the raw comparator upper threshold.
            ///
            /// The voltage that these values correspond to must be calculated using the
            /// full-scale range ([`FullScaleRange`]) selected.
            ///
            #[doc = doc_threshold!($($th_range)+)]
            pub fn set_high_threshold_raw(&mut self, value: i16) -> Result<(), Error<E>> {
                let register_value = <$conv>::convert_threshold(value);
                self.write_register(Register::HIGH_TH, register_value)
            }

            /// Sets the comparator mode.
            pub fn set_comparator_mode(&mut self, mode: ComparatorMode) -> Result<(), Error<E>> {
                let config = match mode {
                    ComparatorMode::Traditional => self.config.with_low(BF::COMP_MODE),
                    ComparatorMode::Window => self.config.with_high(BF::COMP_MODE),
                };
                self.write_register(Register::CONFIG, config.bits)?;
                self.config = config;
                Ok(())
            }

            /// Sets the comparator polarity.
            pub fn set_comparator_polarity(
                &mut self,
                polarity: ComparatorPolarity,
            ) -> Result<(), Error<E>> {
                let config = match polarity {
                    ComparatorPolarity::ActiveLow => self.config.with_low(BF::COMP_POL),
                    ComparatorPolarity::ActiveHigh => self.config.with_high(BF::COMP_POL),
                };
                self.write_register(Register::CONFIG, config.bits)?;
                self.config = config;
                Ok(())
            }

            /// Sets the comparator latching.
            pub fn set_comparator_latching(
                &mut self,
                latching: ComparatorLatching,
            ) -> Result<(), Error<E>> {
                let config = match latching {
                    ComparatorLatching::Nonlatching => self.config.with_low(BF::COMP_LAT),
                    ComparatorLatching::Latching => self.config.with_high(BF::COMP_LAT),
                };
                self.write_register(Register::CONFIG, config.bits)?;
                self.config = config;
                Ok(())
            }

            /// Activates the comparator and sets the alert queue.
            ///
            /// The comparator can be disabled with [`disable_comparator`](Self::disable_comparator).
            pub fn set_comparator_queue(&mut self, queue: ComparatorQueue) -> Result<(), Error<E>> {
                let config = match queue {
                    ComparatorQueue::One => {
                        self.config.with_low(BF::COMP_QUE1).with_low(BF::COMP_QUE0)
                    }
                    ComparatorQueue::Two => {
                        self.config.with_low(BF::COMP_QUE1).with_high(BF::COMP_QUE0)
                    }
                    ComparatorQueue::Four => {
                        self.config.with_high(BF::COMP_QUE1).with_low(BF::COMP_QUE0)
                    }
                };
                self.write_register(Register::CONFIG, config.bits)?;
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
                    .with_high(BF::COMP_QUE1)
                    .with_high(BF::COMP_QUE0);
                self.write_register(Register::CONFIG, config.bits)?;
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
                if self.config
                    != self
                        .config
                        .with_high(BF::COMP_QUE1)
                        .with_high(BF::COMP_QUE0)
                {
                    self.disable_comparator()?;
                }
                self.write_register(Register::HIGH_TH, 0x8000)?;
                self.write_register(Register::LOW_TH, 0)
            }
        }
    };
}

impl_tier2_features!(Ads1014, ic::Resolution12Bit, [-2048, 2047]);
impl_tier2_features!(Ads1015, ic::Resolution12Bit, [-2048, 2047]);
impl_tier2_features!(Ads1114, ic::Resolution16Bit, [-32768, 32767]);
impl_tier2_features!(Ads1115, ic::Resolution16Bit, [-32768, 32767]);
