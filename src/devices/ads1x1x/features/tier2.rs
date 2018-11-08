//! Tier 2 features.
//!
//! These are the features included only in ADS1x14, ADS1x15

use { Ads1x1x, Error, interface, ic, ComparatorMode, Register, BitFlags };

impl<DI, IC, MODE, E> Ads1x1x<DI, IC, MODE>
where
    DI: interface::WriteData<Error = E>,
    IC: ic::Resolution + ic::Tier2Features
{
    /// Set comparator mode
    pub fn set_comparator_mode(&mut self, mode: ComparatorMode) -> Result<(), Error<E>> {
        let config;
        match mode {
            ComparatorMode::Traditional => config = self.config.with_low(BitFlags::COMP_MODE),
            ComparatorMode::Window      => config = self.config.with_high(BitFlags::COMP_MODE)
        }
        self.iface.write_register(Register::CONFIG, config.bits)?;
        self.config = config;
        Ok(())
    }
}
