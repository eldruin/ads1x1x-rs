//! Tier 2 features.
//!
//! These are the features included only in ADS1x14, ADS1x15

use {
    conversion, ic, interface, Ads1x1x, BitFlags as BF, ComparatorLatching, ComparatorMode,
    ComparatorPolarity, ComparatorQueue, Error, FullScaleRange, Register,
};

impl<DI, IC, CONV, MODE, E> Ads1x1x<DI, IC, CONV, MODE>
where
    DI: interface::WriteData<Error = E>,
    IC: ic::Tier2Features,
    CONV: conversion::ConvertThreshold<E>,
{
    /// Set the input voltage measurable range
    ///
    /// This configures the programmable gain amplifier and determines the measurable input voltage range.
    pub fn set_full_scale_range(&mut self, range: FullScaleRange) -> Result<(), Error<E>> {
        let config;
        match range {
            FullScaleRange::Within6_144V => config = self.config.with_low( BF::PGA2).with_low( BF::PGA1).with_low( BF::PGA0),
            FullScaleRange::Within4_096V => config = self.config.with_low( BF::PGA2).with_low( BF::PGA1).with_high(BF::PGA0),
            FullScaleRange::Within2_048V => config = self.config.with_low( BF::PGA2).with_high(BF::PGA1).with_low( BF::PGA0),
            FullScaleRange::Within1_024V => config = self.config.with_low( BF::PGA2).with_high(BF::PGA1).with_high(BF::PGA0),
            FullScaleRange::Within0_512V => config = self.config.with_high(BF::PGA2).with_low( BF::PGA1).with_low( BF::PGA0),
            FullScaleRange::Within0_256V => config = self.config.with_high(BF::PGA2).with_low( BF::PGA1).with_high(BF::PGA0),
        }
        self.iface.write_register(Register::CONFIG, config.bits)?;
        self.config = config;
        Ok(())
    }

    /// Set raw comparator lower threshold
    ///
    /// The input value must be within `[2047..-2048]` for 12-bit devices (`ADS101x`)
    /// and within `[32767..-32768]` for 16-bit devices (`ADS111x`). The voltage that
    /// these values correspond to must be calculated using the full-scale range
    /// selected. See [`FullScaleRange`](enum.FullScaleRange.html).
    pub fn set_low_threshold_raw(&mut self, value: i16) -> Result<(), Error<E>> {
        let register_value = CONV::convert_threshold(value)?;
        self.iface.write_register(Register::LOW_TH, register_value)
    }

    /// Set raw comparator upper threshold
    ///
    /// The input value must be within `[2047..-2048]` for 12-bit devices (`ADS101x`)
    /// and within `[32767..-32768]` for 16-bit devices (`ADS111x`). The voltage that
    /// these values correspond to must be calculated using the full-scale range
    /// selected. See [`FullScaleRange`](enum.FullScaleRange.html).
    pub fn set_high_threshold_raw(&mut self, value: i16) -> Result<(), Error<E>> {
        let register_value = CONV::convert_threshold(value)?;
        self.iface.write_register(Register::HIGH_TH, register_value)
    }

    /// Set comparator mode
    pub fn set_comparator_mode(&mut self, mode: ComparatorMode) -> Result<(), Error<E>> {
        let config;
        match mode {
            ComparatorMode::Traditional => config = self.config.with_low( BF::COMP_MODE),
            ComparatorMode::Window      => config = self.config.with_high(BF::COMP_MODE),
        }
        self.iface.write_register(Register::CONFIG, config.bits)?;
        self.config = config;
        Ok(())
    }

    /// Set comparator polarity
    pub fn set_comparator_polarity(
        &mut self,
        polarity: ComparatorPolarity,
    ) -> Result<(), Error<E>> {
        let config;
        match polarity {
            ComparatorPolarity::ActiveLow  => config = self.config.with_low( BF::COMP_POL),
            ComparatorPolarity::ActiveHigh => config = self.config.with_high(BF::COMP_POL),
        }
        self.iface.write_register(Register::CONFIG, config.bits)?;
        self.config = config;
        Ok(())
    }

    /// Set comparator latching
    pub fn set_comparator_latching(
        &mut self,
        latching: ComparatorLatching,
    ) -> Result<(), Error<E>> {
        let config;
        match latching {
            ComparatorLatching::Nonlatching => config = self.config.with_low( BF::COMP_LAT),
            ComparatorLatching::Latching    => config = self.config.with_high(BF::COMP_LAT),
        }
        self.iface.write_register(Register::CONFIG, config.bits)?;
        self.config = config;
        Ok(())
    }

    /// Activate comparator and set the alert queue
    ///
    /// The comparator can be disabled with [`disable_comparator()`](struct.Ads1x1x.html#method.disable_comparator)
    pub fn set_comparator_queue(&mut self, queue: ComparatorQueue) -> Result<(), Error<E>> {
        let config;
        match queue {
            ComparatorQueue::One  => config = self.config.with_low( BF::COMP_QUE1).with_low( BF::COMP_QUE0),
            ComparatorQueue::Two  => config = self.config.with_low( BF::COMP_QUE1).with_high(BF::COMP_QUE0),
            ComparatorQueue::Four => config = self.config.with_high(BF::COMP_QUE1).with_low( BF::COMP_QUE0),
        }
        self.iface.write_register(Register::CONFIG, config.bits)?;
        self.config = config;
        Ok(())
    }

    /// Disable comparator (default)
    ///
    /// This will set the ALERT/RDY pin to high-impedance.
    /// The comparator can be enabled by setting the comparator queue.
    /// See [`set_comparator_queue()`](struct.Ads1x1x.html#method.set_comparator_queue)
    pub fn disable_comparator(&mut self) -> Result<(), Error<E>> {
        let config = self.config.with_high(BF::COMP_QUE1).with_high(BF::COMP_QUE0);
        self.iface.write_register(Register::CONFIG, config.bits)?;
        self.config = config;
        Ok(())
    }

    /// Use the ALERT/RDY pin as conversion-ready pin.
    ///
    /// This the ALERT/RDY pin outputs the OS bit when in OneShot mode, and
    /// provides a continuous-conversion ready pulse when in
    /// continuous-conversion mode.
    ///
    /// When calling this the comparator will be disabled and the thresholds will be cleared.
    pub fn use_alert_rdy_pin_as_ready(&mut self) -> Result<(), Error<E>> {
        if self.config != self.config.with_high(BF::COMP_QUE1).with_high(BF::COMP_QUE0) {
            self.disable_comparator()?;
        }
        self.iface.write_register(Register::HIGH_TH, 0x8000)?;
        self.iface.write_register(Register::LOW_TH, 0)
    }
}
