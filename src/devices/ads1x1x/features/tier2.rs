//! Tier 2 features.
//!
//! These are the features included only in ADS1x14, ADS1x15

use { Ads1x1x, Error, interface, ic, ComparatorMode, ComparatorPolarity,
      ComparatorLatching, Register, BitFlags };

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

    /// Set comparator polarity
    pub fn set_comparator_polarity(&mut self, polarity: ComparatorPolarity) -> Result<(), Error<E>> {
        let config;
        match polarity {
            ComparatorPolarity::ActiveLow  => config = self.config.with_low( BitFlags::COMP_POL),
            ComparatorPolarity::ActiveHigh => config = self.config.with_high(BitFlags::COMP_POL)
        }
        self.iface.write_register(Register::CONFIG, config.bits)?;
        self.config = config;
        Ok(())
    }

    /// Set comparator latching
    pub fn set_comparator_latching(&mut self, latching: ComparatorLatching) -> Result<(), Error<E>> {
        let config;
        match latching {
            ComparatorLatching::Nonlatching => config = self.config.with_low( BitFlags::COMP_LAT),
            ComparatorLatching::Latching    => config = self.config.with_high(BitFlags::COMP_LAT)
        }
        self.iface.write_register(Register::CONFIG, config.bits)?;
        self.config = config;
        Ok(())
    }
}
