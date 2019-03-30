//! Common functions

use {ic, interface, Ads1x1x, BitFlags as BF, DataRate12Bit, DataRate16Bit, Error, Register};

impl<DI, IC, MODE, E> Ads1x1x<DI, IC, ic::Resolution12Bit, MODE>
where
    DI: interface::WriteData<Error = E>,
{
    /// Set data rate
    pub fn set_data_rate(&mut self, rate: DataRate12Bit) -> Result<(), Error<E>> {
        use DataRate12Bit as DR;
        let cfg = self.config.clone();
        let config = match rate {
            DR::Sps128 => cfg.with_low(BF::DR2).with_low(BF::DR1).with_low(BF::DR0),
            DR::Sps250 => cfg.with_low(BF::DR2).with_low(BF::DR1).with_high(BF::DR0),
            DR::Sps490 => cfg.with_low(BF::DR2).with_high(BF::DR1).with_low(BF::DR0),
            DR::Sps920 => cfg.with_low(BF::DR2).with_high(BF::DR1).with_high(BF::DR0),
            DR::Sps1600 => cfg.with_high(BF::DR2).with_low(BF::DR1).with_low(BF::DR0),
            DR::Sps2400 => cfg.with_high(BF::DR2).with_low(BF::DR1).with_high(BF::DR0),
            DR::Sps3300 => cfg.with_high(BF::DR2).with_high(BF::DR1).with_low(BF::DR0),
        };
        self.iface.write_register(Register::CONFIG, config.bits)?;
        self.config = config;
        Ok(())
    }
}

impl<DI, IC, MODE, E> Ads1x1x<DI, IC, ic::Resolution16Bit, MODE>
where
    DI: interface::WriteData<Error = E>,
{
    /// Set data rate
    pub fn set_data_rate(&mut self, rate: DataRate16Bit) -> Result<(), Error<E>> {
        use DataRate16Bit as DR;
        let cfg = self.config.clone();
        let config = match rate {
            DR::Sps8 => cfg.with_low(BF::DR2).with_low(BF::DR1).with_low(BF::DR0),
            DR::Sps16 => cfg.with_low(BF::DR2).with_low(BF::DR1).with_high(BF::DR0),
            DR::Sps32 => cfg.with_low(BF::DR2).with_high(BF::DR1).with_low(BF::DR0),
            DR::Sps64 => cfg.with_low(BF::DR2).with_high(BF::DR1).with_high(BF::DR0),
            DR::Sps128 => cfg.with_high(BF::DR2).with_low(BF::DR1).with_low(BF::DR0),
            DR::Sps250 => cfg.with_high(BF::DR2).with_low(BF::DR1).with_high(BF::DR0),
            DR::Sps475 => cfg.with_high(BF::DR2).with_high(BF::DR1).with_low(BF::DR0),
            DR::Sps860 => cfg.with_high(BF::DR2).with_high(BF::DR1).with_high(BF::DR0),
        };
        self.iface.write_register(Register::CONFIG, config.bits)?;
        self.config = config;
        Ok(())
    }
}
