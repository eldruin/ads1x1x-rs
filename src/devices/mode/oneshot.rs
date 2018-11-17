//! Common functions

use core::marker::PhantomData;
use { Ads1x1x, mode, Error, Register, BitFlags, Config };
use { interface, conversion, hal, nb };
use devices::OperatingMode;
use channels::ChannelSelection;

impl<DI, IC, CONV, E> Ads1x1x<DI, IC, CONV, mode::OneShot>
where
    DI: interface::WriteData<Error = E> + interface::ReadData<Error = E>,
    CONV: conversion::ConvertMeasurement
{
    /// Change operating mode to Continuous
    pub fn into_continuous(mut self) -> Result<Ads1x1x<DI, IC, CONV, mode::Continuous>, Error<E>> {
        self.set_operating_mode(OperatingMode::Continuous)?;
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

    fn is_measurement_in_progress(&mut self) -> Result<bool, Error<E>> {
        let config = Config {
            bits: self.iface.read_register(Register::CONFIG)?
        };
        Ok(!config.is_high(BitFlags::OS))
    }

    fn trigger_measurement(&mut self, config: &Config) -> Result<(), Error<E>> {
        let config = config.with_high(BitFlags::OS);
        self.iface.write_register(Register::CONFIG, config.bits)
    }
}

impl<DI, IC, CONV, E, CH> hal::adc::OneShot<Ads1x1x<DI, IC, CONV, mode::OneShot>, i16, CH>
    for Ads1x1x<DI, IC, CONV, mode::OneShot>
where
    DI: interface::ReadData<Error = E> + interface::WriteData<Error = E>,
    CONV: conversion::ConvertMeasurement,
    CH: hal::adc::Channel<Ads1x1x<DI, IC, CONV, mode::OneShot>, ID = ChannelSelection>
{
    type Error = Error<E>;

    /// Request that the ADC begin a conversion on the specified channel.
    ///
    /// Returns `nb::Error::WouldBlock` while a measurement is in progress.
    ///
    /// In case a measurement was requested and after is it is finished a
    /// measurement on a different channel is requested, a new measurement on
    /// using the new channel selection is triggered.
    fn read(&mut self, _channel: &mut CH) -> nb::Result<i16, Self::Error> {
        if self.is_measurement_in_progress().map_err(nb::Error::Other)? {
            return Err(nb::Error::WouldBlock);
        }
        let same_channel = self.config == self.config.with_mux_bits(CH::channel());
        if self.a_conversion_was_started && same_channel {
            // result is ready
            let value = self.iface.read_register(Register::CONVERSION).map_err(nb::Error::Other)?;
            self.a_conversion_was_started = false;
            return Ok(CONV::convert_measurement(value));
        }
        let config = self.config.with_mux_bits(CH::channel());
        self.trigger_measurement(&config).map_err(nb::Error::Other)?;
        self.config = config;
        self.a_conversion_was_started = true;
        Err(nb::Error::WouldBlock)
    }
}
