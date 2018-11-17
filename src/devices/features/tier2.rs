//! Tier 2 features.
//!
//! These are the features included only in ADS1x14, ADS1x15

use { Ads1x1x, Error, interface, ic, ComparatorMode, ComparatorPolarity,
      ComparatorLatching, ComparatorQueue, Register, BitFlags, conversion };

impl<DI, IC, CONV, MODE, E> Ads1x1x<DI, IC, CONV, MODE>
where
    DI: interface::WriteData<Error = E>,
    IC: ic::Tier2Features,
    CONV: conversion::ConvertThreshold<E>
{
    /// Set comparator lower threshold
    pub fn set_low_threshold(&mut self, value: i16) -> Result<(), Error<E>> {
        let register_value = CONV::convert_threshold(value)?;
        self.iface.write_register(Register::LOW_TH, register_value)
    }

    /// Set comparator upper threshold
    pub fn set_high_threshold(&mut self, value: i16) -> Result<(), Error<E>> {
        let register_value = CONV::convert_threshold(value)?;
        self.iface.write_register(Register::HIGH_TH, register_value)
    }

    /// Set comparator mode
    pub fn set_comparator_mode(&mut self, mode: ComparatorMode) -> Result<(), Error<E>> {
        let config;
        match mode {
            ComparatorMode::Traditional => config = self.config.with_low(BitFlags::COMP_MODE),
            ComparatorMode::Window      => config = self.config.with_high(BitFlags::COMP_MODE)
        }
        self.iface.write_register(Register::CONFIG, config.bits)?;
        self.config = config;
        Ok(())
    }

    /// Set comparator polarity
    pub fn set_comparator_polarity(&mut self, polarity: ComparatorPolarity) -> Result<(), Error<E>> {
        let config;
        match polarity {
            ComparatorPolarity::ActiveLow  => config = self.config.with_low( BitFlags::COMP_POL),
            ComparatorPolarity::ActiveHigh => config = self.config.with_high(BitFlags::COMP_POL)
        }
        self.iface.write_register(Register::CONFIG, config.bits)?;
        self.config = config;
        Ok(())
    }

    /// Set comparator latching
    pub fn set_comparator_latching(&mut self, latching: ComparatorLatching) -> Result<(), Error<E>> {
        let config;
        match latching {
            ComparatorLatching::Nonlatching => config = self.config.with_low( BitFlags::COMP_LAT),
            ComparatorLatching::Latching    => config = self.config.with_high(BitFlags::COMP_LAT)
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
            ComparatorQueue::One  => config = self.config.with_low( BitFlags::COMP_QUE1).with_low( BitFlags::COMP_QUE0),
            ComparatorQueue::Two  => config = self.config.with_low( BitFlags::COMP_QUE1).with_high(BitFlags::COMP_QUE0),
            ComparatorQueue::Four => config = self.config.with_high(BitFlags::COMP_QUE1).with_low( BitFlags::COMP_QUE0)
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
        let config = self.config.with_high(BitFlags::COMP_QUE1).with_high(BitFlags::COMP_QUE0);
        self.iface.write_register(Register::CONFIG, config.bits)?;
        self.config = config;
        Ok(())
    }
}
