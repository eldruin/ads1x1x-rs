//! Common functions

use { Ads1x1x, Error, Register, BitFlags, interface, Config };
use super::OperatingMode;

impl<DI, IC, CONV, MODE, E> Ads1x1x<DI, IC, CONV, MODE>
where
    DI: interface::WriteData<Error = E>
{
    pub(super) fn set_operating_mode(&mut self, mode: OperatingMode) -> Result<(), Error<E>> {
        let config;
        match mode {
            OperatingMode::OneShot => config = self.config.with_high(BitFlags::OP_MODE),
            OperatingMode::Continuous => config = self.config.with_low(BitFlags::OP_MODE),
        }
        self.iface.write_register(Register::CONFIG, config.bits)?;
        self.config = config;
        Ok(())
    }

    /// Reset the internal state of this driver to the default values.
    ///
    /// *Note:* This does not alter the state or configuration of the device.
    ///
    /// This resets the cached configuration register value in this driver to
    /// the power-up (reset) configuration of the device.
    ///
    /// This needs to be called after performing a reset on the device, for
    /// example through an I2C general-call Reset command, which was not done
    /// through this driver to ensure that the configurations in the device
    /// and in the driver match.
    pub fn reset_internal_driver_state(&mut self) {
        self.config = Config::default();
    }
}
