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

/// SPI interface
#[derive(Debug, Default)]
pub struct SpiInterface<SPI, CS> {
    pub(crate) spi: SPI,
    pub(crate) cs: CS
}

/// Write data
pub trait WriteData : private::Sealed {
    /// Error type
    type Error;
    /// Write to an u8 register
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
            .map_err(Error::Comm)
    }
}

impl<SPI, CS, E> WriteData for SpiInterface<SPI, CS>
where
    SPI: blocking::spi::Write<u8, Error = E>,
    CS:  hal::digital::OutputPin
{
    type Error = E;
    fn write_register(&mut self, register: u8, data: u16) -> Result<(), Error<E>> {
        self.cs.set_low();

        let payload: [u8; 3] = [register + 0x80, (data >> 8) as u8, data as u8];
        let result = self.spi
                         .write(&payload)
                         .map_err(Error::Comm);

        self.cs.set_high();
        result
    }
}


/// Read data
pub trait ReadData : private::Sealed {
    /// Error type
    type Error;
    /// Read an u8 register
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
            .map_err(Error::Comm)
            .and(Ok(((data[0] as u16) << 8) | data[1] as u16))
    }
}

impl<SPI, CS, E> ReadData for SpiInterface<SPI, CS>
where
    SPI: blocking::spi::Transfer<u8, Error = E>,
    CS:  hal::digital::OutputPin
{
    type Error = E;
    fn read_register(&mut self, register: u8) -> Result<u16, Error<E>> {
        self.cs.set_low();
        let mut data = [register, 0, 0];
        let result = self.spi
                         .transfer(&mut data)
                         .map_err(Error::Comm);
        self.cs.set_high();
        let result = result?;
        Ok(((result[0] as u16) << 8) | result[1] as u16)
    }
}
