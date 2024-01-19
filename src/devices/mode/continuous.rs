//! Continuous measurement mode

use crate::{
    conversion, devices::OperatingMode, interface, mode, Ads1x1x, ChannelId, Error,
    ModeChangeError, Register,
};
use core::marker::PhantomData;

impl<DI, IC, CONV, E> Ads1x1x<DI, IC, CONV, mode::Continuous>
where
    DI: interface::ReadData<Error = E> + interface::WriteData<Error = E>,
    CONV: conversion::ConvertMeasurement,
{
    /// Change operating mode to OneShot
    pub fn into_one_shot(
        mut self,
    ) -> Result<Ads1x1x<DI, IC, CONV, mode::OneShot>, ModeChangeError<E, Self>> {
        if let Err(Error::I2C(e)) = self.set_operating_mode(OperatingMode::OneShot) {
            return Err(ModeChangeError::I2C(e, self));
        }
        Ok(Ads1x1x {
            iface: self.iface,
            config: self.config,
            fsr: self.fsr,
            a_conversion_was_started: false,
            _conv: PhantomData,
            _ic: PhantomData,
            _mode: PhantomData,
        })
    }

    /// Read the most recent measurement
    pub fn read(&mut self) -> Result<i16, Error<E>> {
        let value = self.iface.read_register(Register::CONVERSION)?;
        Ok(CONV::convert_measurement(value))
    }

    /// Select the channel for measurements.
    ///
    /// Note that when changing the channel in continuous conversion mode, the
    /// ongoing conversion will be completed.
    /// The following conversions will use the new channel configuration.
    #[allow(unused_variables)]
    pub fn select_channel<CH: ChannelId<Self>>(&mut self, channel: CH) -> Result<(), Error<E>> {
        let config = self.config.with_mux_bits(CH::channel_id());
        self.iface.write_register(Register::CONFIG, config.bits)?;
        self.config = config;
        Ok(())
    }
}
