//! I2C interface

use crate::{private, Error};

/// I2C interface
#[derive(Debug, Default)]
pub struct I2cInterface<I2C> {
    pub(crate) i2c: I2C,
    pub(crate) address: u8,
}

/// Read/write data from/to register.
pub trait ReadWriteRegister: private::Sealed {
    /// Error type.
    type Error;

    /// Reads a `u16` register.
    fn read_register(&mut self, register: u8) -> Result<u16, Error<Self::Error>>;

    /// Writes to a `u16` register.
    fn write_register(&mut self, register: u8, data: u16) -> Result<(), Error<Self::Error>>;
}

impl<I2C, E> ReadWriteRegister for I2cInterface<I2C>
where
    I2C: embedded_hal::i2c::I2c<Error = E>,
{
    type Error = E;

    fn write_register(&mut self, register: u8, data: u16) -> Result<(), Error<E>> {
        let data = data.to_be_bytes();
        let payload: [u8; 3] = [register, data[0], data[1]];
        self.i2c.write(self.address, &payload).map_err(Error::I2C)
    }

    fn read_register(&mut self, register: u8) -> Result<u16, Error<E>> {
        let mut data = [0, 0];
        self.i2c
            .write_read(self.address, &[register], &mut data)
            .map_err(Error::I2C)
            .and(Ok(u16::from_be_bytes(data)))
    }
}
