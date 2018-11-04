//! I2C/SPI interfaces

#![deny(missing_docs)]

extern crate embedded_hal as hal;
use hal::blocking;
use Error;

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
pub trait WriteData {
    /// Error type
    type Error;
    /// Write to an u8 register
    fn write_register(&mut self, register: u8, data: u8) -> Result<(), Error<Self::Error>>;
    /// Write data. The first element corresponds to the starting address.
    fn write_data(&mut self, payload: &mut [u8]) -> Result<(), Error<Self::Error>>;
}

impl<I2C, E> WriteData for I2cInterface<I2C>
where
    I2C: blocking::i2c::Write<Error = E>
{
    type Error = E;
    fn write_register(&mut self, register: u8, data: u8) -> Result<(), Error<E>> {
        let payload: [u8; 2] = [register, data];
        self.i2c
            .write(self.address, &payload)
            .map_err(Error::Comm)
    }

    fn write_data(&mut self, payload: &mut [u8]) -> Result<(), Error<Self::Error>> {
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
    fn write_register(&mut self, register: u8, data: u8) -> Result<(), Error<E>> {
        self.cs.set_low();

        let payload: [u8; 2] = [register + 0x80, data];
        let result = self.spi
                         .write(&payload)
                         .map_err(Error::Comm);

        self.cs.set_high();
        result
    }

    fn write_data(&mut self, payload: &mut [u8]) -> Result<(), Error<Self::Error>> {
        self.cs.set_low();
        payload[0] += 0x80;
        let result = self.spi
                         .write(&payload)
                         .map_err(Error::Comm);

        self.cs.set_high();
        result
    }
}


/// Read data
pub trait ReadData {
    /// Error type
    type Error;
    /// Read an u8 register
    fn read_register(&mut self, register: u8) -> Result<u8, Error<Self::Error>>;
    /// Read some data. The first element corresponds to the starting address.
    fn read_data(&mut self, payload: &mut [u8]) -> Result<(), Error<Self::Error>>;
}

impl<I2C, E> ReadData for I2cInterface<I2C>
where
    I2C: blocking::i2c::WriteRead<Error = E>
{
    type Error = E;
    fn read_register(&mut self, register: u8) -> Result<u8, Error<E>> {
        let mut data = [0];
        self.i2c
            .write_read(self.address, &[register], &mut data)
            .map_err(Error::Comm)
            .and(Ok(data[0]))
    }

    fn read_data(&mut self, payload: &mut [u8]) -> Result<(), Error<Self::Error>> {
        let len = payload.len();
        self.i2c
            .write_read(self.address, &[payload[0]], &mut payload[1..=(len-1)])
            .map_err(Error::Comm)
    }
}

impl<SPI, CS, E> ReadData for SpiInterface<SPI, CS>
where
    SPI: blocking::spi::Transfer<u8, Error = E>,
    CS:  hal::digital::OutputPin
{
    type Error = E;
    fn read_register(&mut self, register: u8) -> Result<u8, Error<E>> {
        self.cs.set_low();
        let mut data = [register, 0];
        let result = self.spi
                         .transfer(&mut data)
                         .map_err(Error::Comm);
        self.cs.set_high();
        match result {
            Ok(result) => Ok(result[1]),
            Err(e) => Err(e)
        }
    }

    fn read_data(&mut self, mut payload: &mut [u8]) -> Result<(), Error<Self::Error>> {
        self.cs.set_low();
        let result = self.spi
                         .transfer(&mut payload)
                         .map_err(Error::Comm);
        self.cs.set_high();
        result?;
        Ok(())
    }
}
