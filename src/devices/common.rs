//! Common functions

use crate::{devices::OperatingMode, Ads1x1x, BitFlags, Config, Error, Register};

impl<I2C, IC, CONV, MODE, E> Ads1x1x<I2C, IC, CONV, MODE>
where
    I2C: embedded_hal::i2c::I2c<Error = E>,
{
    pub(super) fn write_register(&mut self, register: u8, data: u16) -> Result<(), Error<E>> {
        let data = data.to_be_bytes();
        let payload: [u8; 3] = [register, data[0], data[1]];
        self.i2c.write(self.address, &payload).map_err(Error::I2C)
    }

    pub(super) fn read_register(&mut self, register: u8) -> Result<u16, Error<E>> {
        let mut data = [0, 0];
        self.i2c
            .write_read(self.address, &[register], &mut data)
            .map_err(Error::I2C)
            .and(Ok(u16::from_be_bytes(data)))
    }

    pub(super) fn set_operating_mode(&mut self, mode: OperatingMode) -> Result<(), Error<E>> {
        let config = match mode {
            OperatingMode::OneShot => self.config.with_high(BitFlags::OP_MODE),
            OperatingMode::Continuous => self.config.with_low(BitFlags::OP_MODE),
        };
        self.write_register(Register::CONFIG, config.bits)?;
        self.config = config;
        Ok(())
    }

    /// Read whether a measurement is currently in progress.
    pub fn is_measurement_in_progress(&mut self) -> Result<bool, Error<E>> {
        let config = Config {
            bits: self.read_register(Register::CONFIG)?,
        };
        Ok(!config.is_high(BitFlags::OS))
    }
}
