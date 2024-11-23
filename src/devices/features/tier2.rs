//! Features only supported by ADS1x14 and ADS1x15 devices.

use crate::{
    conversion, ic, Ads1x1x, BitFlags as BF, ComparatorLatching, ComparatorMode,
    ComparatorPolarity, ComparatorQueue, Error, FullScaleRange, Register,
};

impl<I2C, IC, CONV, MODE, E> Ads1x1x<I2C, IC, CONV, MODE>
where
    I2C: embedded_hal::i2c::I2c<Error = E>,
    IC: ic::Tier2Features,
    CONV: conversion::ConvertThreshold<E>,
{
    /// Sets the input voltage measurable range.
    ///
    /// This configures the programmable gain amplifier (PGA) and determines the measurable input voltage range.
    pub fn set_full_scale_range(&mut self, range: FullScaleRange) -> Result<(), Error<E>> {
        use crate::FullScaleRange as FSR;
        let cfg = self.config.clone();
        let config = match range {
            FSR::Within6_144V => cfg.with_low(BF::PGA2).with_low(BF::PGA1).with_low(BF::PGA0),
            FSR::Within4_096V => cfg
                .with_low(BF::PGA2)
                .with_low(BF::PGA1)
                .with_high(BF::PGA0),
            FSR::Within2_048V => cfg
                .with_low(BF::PGA2)
                .with_high(BF::PGA1)
                .with_low(BF::PGA0),
            FSR::Within1_024V => cfg
                .with_low(BF::PGA2)
                .with_high(BF::PGA1)
                .with_high(BF::PGA0),
            FSR::Within0_512V => cfg
                .with_high(BF::PGA2)
                .with_low(BF::PGA1)
                .with_low(BF::PGA0),
            FSR::Within0_256V => cfg
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
    /// The input value must be within `[2047..-2048]` for 12-bit devices (`ADS101x`)
    /// and within `[32767..-32768]` for 16-bit devices (`ADS111x`).
    pub fn set_low_threshold_raw(&mut self, value: i16) -> Result<(), Error<E>> {
        let register_value = CONV::convert_threshold(value)?;
        self.write_register(Register::LOW_TH, register_value)
    }

    /// Sets the raw comparator upper threshold.
    ///
    /// The voltage that these values correspond to must be calculated using the
    /// full-scale range ([`FullScaleRange`]) selected.
    ///
    /// The input value must be within `[2047..-2048]` for 12-bit devices (`ADS101x`)
    /// and within `[32767..-32768]` for 16-bit devices (`ADS111x`).
    pub fn set_high_threshold_raw(&mut self, value: i16) -> Result<(), Error<E>> {
        let register_value = CONV::convert_threshold(value)?;
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
            ComparatorQueue::One => self.config.with_low(BF::COMP_QUE1).with_low(BF::COMP_QUE0),
            ComparatorQueue::Two => self.config.with_low(BF::COMP_QUE1).with_high(BF::COMP_QUE0),
            ComparatorQueue::Four => self.config.with_high(BF::COMP_QUE1).with_low(BF::COMP_QUE0),
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

    /// Enables the ALERT/RDY pin as conversion-ready function.
    ///
    /// When in one-shot mode, this makes the ALERT/RDY pin output the OS bit,
    /// in continuous-conversion mode, provides a continuous-conversion ready pulse.
    ///
    /// When calling this the comparator will be reset to default and any thresholds will be cleared.
    pub fn use_alert_rdy_pin_as_ready(&mut self) -> Result<(), Error<E>> {
        if self.config
            != self
                .config
                .with_high(BF::COMP_QUE1)
                .with_high(BF::COMP_QUE0)
        {
            self.set_comparator_queue(ComparatorQueue::default())?;
        }
        self.write_register(Register::HIGH_TH, 0x8000)?;
        self.write_register(Register::LOW_TH, 0)
    }
}
