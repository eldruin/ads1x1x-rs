//! Common functions

use { Ads1x1x, DataRate12Bit, Error, Register, BitFlags, interface, ic };

impl<DI, IC, MODE, E> Ads1x1x<DI, IC, ic::Resolution12Bit, MODE>
where
    DI: interface::WriteData<Error = E>,
{
    /// Set data rate
    pub fn set_data_rate(&mut self, rate: DataRate12Bit) -> Result<(), Error<E>> {
        let config;
        match rate {
            DataRate12Bit::Sps128  => config = self.config.with_low( BitFlags::DR2).with_low( BitFlags::DR1).with_low( BitFlags::DR0),
            DataRate12Bit::Sps250  => config = self.config.with_low( BitFlags::DR2).with_low( BitFlags::DR1).with_high(BitFlags::DR0),
            DataRate12Bit::Sps490  => config = self.config.with_low( BitFlags::DR2).with_high(BitFlags::DR1).with_low( BitFlags::DR0),
            DataRate12Bit::Sps920  => config = self.config.with_low( BitFlags::DR2).with_high(BitFlags::DR1).with_high(BitFlags::DR0),
            DataRate12Bit::Sps1600 => config = self.config.with_high(BitFlags::DR2).with_low( BitFlags::DR1).with_low( BitFlags::DR0),
            DataRate12Bit::Sps2400 => config = self.config.with_high(BitFlags::DR2).with_low( BitFlags::DR1).with_high(BitFlags::DR0),
            DataRate12Bit::Sps3300 => config = self.config.with_high(BitFlags::DR2).with_high(BitFlags::DR1).with_low( BitFlags::DR0),
        }
        self.iface.write_register(Register::CONFIG, config.bits)?;
        self.config = config;
        Ok(())
    }
}

where
    DI: interface::WriteData<Error = E>
{
    /// Set data rate
    pub fn set_data_rate(&mut self, rate: DataRate) -> Result<(), Error<E>> {
        let config;
        match rate {
            DataRate::Sps128  => config = self.config.with_low( BitFlags::DR2).with_low( BitFlags::DR1).with_low( BitFlags::DR0),
            DataRate::Sps250  => config = self.config.with_low( BitFlags::DR2).with_low( BitFlags::DR1).with_high(BitFlags::DR0),
            DataRate::Sps490  => config = self.config.with_low( BitFlags::DR2).with_high(BitFlags::DR1).with_low( BitFlags::DR0),
            DataRate::Sps920  => config = self.config.with_low( BitFlags::DR2).with_high(BitFlags::DR1).with_high(BitFlags::DR0),
            DataRate::Sps1600 => config = self.config.with_high(BitFlags::DR2).with_low( BitFlags::DR1).with_low( BitFlags::DR0),
            DataRate::Sps2400 => config = self.config.with_high(BitFlags::DR2).with_low( BitFlags::DR1).with_high(BitFlags::DR0),
            DataRate::Sps3300 => config = self.config.with_high(BitFlags::DR2).with_high(BitFlags::DR1).with_low( BitFlags::DR0),
        }
        self.iface.write_register(Register::CONFIG, config.bits)?;
        self.config = config;
        Ok(())
    }
}
