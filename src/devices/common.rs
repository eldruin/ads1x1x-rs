//! Common functions.

use crate::{register::Config, Ads1013, Ads1014, Ads1015, Ads1113, Ads1114, Ads1115, Error};

macro_rules! impl_common_features {
    ($Ads:ident) => {
        impl<I2C, E, MODE> $Ads<I2C, MODE>
        where
            I2C: embedded_hal::i2c::I2c<Error = E>,
        {
            /// Checks whether a measurement is currently in progress.
            pub fn is_measurement_in_progress(&mut self) -> Result<bool, Error<E>> {
                let config = self.read_reg_u16::<Config>()?;
                Ok(!config.contains(Config::OS))
            }
        }
    };
}

impl_common_features!(Ads1013);
impl_common_features!(Ads1113);
impl_common_features!(Ads1014);
impl_common_features!(Ads1114);
impl_common_features!(Ads1015);
impl_common_features!(Ads1115);
