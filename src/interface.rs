//! I2C/SPI interfaces

extern crate embedded_hal as hal;
use hal::blocking;
use { private, Error };


/// I2C interface
#[derive(Debug, Default)]
pub struct I2cInterface<I2C> {
    pub(crate) i2c: I2C,
    pub(crate) address : u8,
}

/// Write data
pub trait WriteData : private::Sealed {
    /// Error type
    type Error;
    /// Write to an u16 register
    fn write_register(&mut self, register: u8, data: u16) -> Result<(), Error<Self::Error>>;
}

impl<I2C, E> WriteData for I2cInterface<I2C>
where
    I2C: blocking::i2c::Write<Error = E>
{
    type Error = E;
    fn write_register(&mut self, register: u8, data: u16) -> Result<(), Error<E>> {
        let payload: [u8; 3] = [register, (data >> 8) as u8, data as u8];
        self.i2c
            .write(self.address, &payload)
            .map_err(Error::I2C)
    }
}

/// Read data
pub trait ReadData : private::Sealed {
    /// Error type
    type Error;
    /// Read an u16 register
    fn read_register(&mut self, register: u8) -> Result<u16, Error<Self::Error>>;
}

impl<I2C, E> ReadData for I2cInterface<I2C>
where
    I2C: blocking::i2c::WriteRead<Error = E>
{
    type Error = E;
    fn read_register(&mut self, register: u8) -> Result<u16, Error<E>> {
        let mut data = [0, 0];
        self.i2c
            .write_read(self.address, &[register], &mut data)
            .map_err(Error::I2C)
            .and(Ok((u16::from(data[0]) << 8) | u16::from(data[1])))
    }
}
