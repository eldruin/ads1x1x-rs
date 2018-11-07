//! Common functions

use core::marker::PhantomData;
use { Ads1x1x, mode, Error, Register, BitFlags, Config, ic };
use { interface, hal };
use super::super::OperatingMode;
use super::convert_measurement;

impl<DI, IC, E> Ads1x1x<DI, IC, mode::OneShot>
where
    DI: interface::WriteData<Error = E> + interface::ReadData<Error = E>
{
    /// Change operating mode to Continuous
    pub fn into_continuous(mut self) -> Result<Ads1x1x<DI, IC, mode::Continuous>, Error<E>> {
        self.set_operating_mode(OperatingMode::Continuous)?;
        Ok(Ads1x1x {
            iface: self.iface,
            config: self.config,
            a_conversion_was_started: self.a_conversion_was_started,
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

    fn trigger_measurement(&mut self) -> Result<(), Error<E>> {
        let config = self.config.with_high(BitFlags::OS);
        self.iface.write_register(Register::CONFIG, config.bits)
    }
}

impl<DI, IC, E, CH> hal::adc::OneShot<Ads1x1x<DI, IC, mode::OneShot>, i16, CH> for Ads1x1x<DI, IC, mode::OneShot>
where
    DI: interface::ReadData<Error = E> + interface::WriteData<Error = E>,
    IC: ic::Resolution,
    CH: hal::adc::Channel<Ads1x1x<DI, IC, mode::OneShot>>
{
    type Error = Error<E>;

    fn read(&mut self, _channel: &mut CH) -> nb::Result<i16, Self::Error> {
        //TODO for devices with MUX select channel, if it is the wrong one, return AlreadyInProgress or WrongChannel error
        if self.is_measurement_in_progress().map_err(nb::Error::Other)? {
            return Err(nb::Error::WouldBlock);
        }
        if self.a_conversion_was_started {
            // result is ready
            let value = self.iface.read_register(Register::CONVERSION).map_err(nb::Error::Other)?;
            self.a_conversion_was_started = false;
            return Ok(convert_measurement::<IC>(value));
        }
        self.trigger_measurement().map_err(nb::Error::Other)?;
        self.a_conversion_was_started = true;
        Err(nb::Error::WouldBlock)
    }
}
