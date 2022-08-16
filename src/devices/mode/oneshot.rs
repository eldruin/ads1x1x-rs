//! Common functions
use crate::{
    conversion, devices::OperatingMode, interface, mode, types::DynamicOneShot, Ads1x1x, BitFlags,
    Channel, Config, Error, ModeChangeError, Register,
};
use core::marker::PhantomData;

impl<DI, IC, CONV, E> Ads1x1x<DI, IC, CONV, mode::OneShot>
where
    DI: interface::WriteData<Error = E> + interface::ReadData<Error = E>,
    CONV: conversion::ConvertMeasurement,
{
    /// Change operating mode to Continuous
    pub fn into_continuous(
        mut self,
    ) -> Result<Ads1x1x<DI, IC, CONV, mode::Continuous>, ModeChangeError<E, Self>> {
        if let Err(Error::I2C(e)) = self.set_operating_mode(OperatingMode::Continuous) {
            return Err(ModeChangeError::I2C(e, self));
        }
        Ok(Ads1x1x {
            iface: self.iface,
            config: self.config,
            fsr: self.fsr,
            a_conversion_was_started: true,
            _conv: PhantomData,
            _ic: PhantomData,
            _mode: PhantomData,
        })
    }

    fn trigger_measurement(&mut self, config: &Config) -> Result<(), Error<E>> {
        let config = config.with_high(BitFlags::OS);
        self.iface.write_register(Register::CONFIG, config.bits)
    }
}

impl<DI, IC, CONV, E> DynamicOneShot<IC> for Ads1x1x<DI, IC, CONV, mode::OneShot>
where
    DI: interface::ReadData<Error = E> + interface::WriteData<Error = E>,
    CONV: conversion::ConvertMeasurement,
{
    type Error = Error<E>;

    /// Request that the ADC begin a conversion on the specified channel.
    ///
    /// The output value will be within `[2047..-2048]` for 12-bit devices
    /// (`ADS101x`) and within `[32767..-32768]` for 16-bit devices (`ADS111x`).
    /// The voltage that these values correspond to must be calculated using
    /// the full-scale range selected.
    /// See [`FullScaleRange`](enum.FullScaleRange.html).
    ///
    /// Returns `nb::Error::WouldBlock` while a measurement is in progress.
    ///
    /// In case a measurement was requested and after is it is finished a
    /// measurement on a different channel is requested, a new measurement on
    /// using the new channel selection is triggered.
    fn read<CH>(&mut self, _channel: &mut CH) -> nb::Result<i16, Self::Error>
    where
        CH: Channel<IC>,
    {
        if self
            .is_measurement_in_progress()
            .map_err(nb::Error::Other)?
        {
            return Err(nb::Error::WouldBlock);
        }
        let same_channel = self.config == self.config.with_mux_bits(CH::ID);
        if self.a_conversion_was_started && same_channel {
            // result is ready
            let value = self
                .iface
                .read_register(Register::CONVERSION)
                .map_err(nb::Error::Other)?;
            self.a_conversion_was_started = false;
            return Ok(CONV::convert_measurement(value));
        }
        let config = self.config.with_mux_bits(CH::ID);
        self.trigger_measurement(&config)
            .map_err(nb::Error::Other)?;
        self.config = config;
        self.a_conversion_was_started = true;
        Err(nb::Error::WouldBlock)
    }
}
