//! Common functions

use { Ads1x1x, DataRate, Error, Register, BitFlags, interface };

impl<DI, IC, CONV, MODE, E> Ads1x1x<DI, IC, CONV, MODE>
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
