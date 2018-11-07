//! Common functions

use core::marker::PhantomData;
use { Ads1x1x, mode, Error, interface };
use super::super::OperatingMode;

impl<DI, IC, E> Ads1x1x<DI, IC, mode::Continuous>
where
    DI: interface::WriteData<Error = E>
{
    /// Change operating mode to OneShot
    pub fn into_one_shot(mut self) -> Result<Ads1x1x<DI, IC, mode::OneShot>, Error<E>> {
        self.set_operating_mode(OperatingMode::OneShot)?;
        Ok(Ads1x1x {
            iface: self.iface,
            config: self.config,
            a_conversion_was_started: self.a_conversion_was_started,
            _ic: PhantomData,
            _mode: PhantomData
        })
    }
}
