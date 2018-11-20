//! Continuous measurement mode

use core::marker::PhantomData;
use { Ads1x1x, conversion, Error, hal, interface, mode, Register };
use channels::ChannelSelection;
use super::super::OperatingMode;

impl<DI, IC, CONV, E> Ads1x1x<DI, IC, CONV, mode::Continuous>
where
    DI: interface::WriteData<Error = E>,
{
    /// Change operating mode to OneShot
    pub fn into_one_shot(mut self) -> Result<Ads1x1x<DI, IC, CONV, mode::OneShot>, Error<E>> {
        self.set_operating_mode(OperatingMode::OneShot)?;
        Ok(Ads1x1x {
            iface: self.iface,
            config: self.config,
            fsr: self.fsr,
            a_conversion_was_started: self.a_conversion_was_started,
            _conv: PhantomData,
            _ic: PhantomData,
            _mode: PhantomData
        })
    }

    /// Start continuous conversions
    pub fn start(&mut self) -> Result<(), Error<E>> {
        self.set_operating_mode(OperatingMode::Continuous)?;
        self.a_conversion_was_started = true;
        Ok(())
    }
}

impl<DI, IC, CONV, E> Ads1x1x<DI, IC, CONV, mode::Continuous>
where
    DI: interface::ReadData<Error = E> + interface::WriteData<Error = E>,
    CONV: conversion::ConvertMeasurement,
{
    /// Select the channel for measurements.
    pub fn select_channel<CH>(&mut self, _channel: &mut CH) -> Result<(), Error<E>>
    where
        CH: hal::adc::Channel<Ads1x1x<DI, IC, CONV, mode::OneShot>, ID = ChannelSelection> {
        let config = self.config.with_mux_bits(CH::channel());
        self.iface.write_register(Register::CONFIG, config.bits)?;
        self.config = config;
        Ok(())
    }
}
