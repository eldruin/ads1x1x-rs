//! Common functions

use crate::{
    Ads1013, Ads1014, Ads1015, Ads1113, Ads1114, Ads1115, DataRate12Bit, DataRate16Bit, Error,
    Register,
};

macro_rules! impl_tier1_features {
    ($Ads:ident, $DataRate:ty) => {
        impl<I2C, E, MODE> $Ads<I2C, MODE>
        where
            I2C: embedded_hal::i2c::I2c<Error = E>,
        {
            /// Sets the data rate.
            pub fn set_data_rate(&mut self, rate: $DataRate) -> Result<(), Error<E>> {
                let mut config = self.config.clone();
                rate.configure(&mut config);
                self.write_register(Register::CONFIG, config.bits)?;
                self.config = config;
                Ok(())
            }
        }
    };
}

impl_tier1_features!(Ads1013, DataRate12Bit);
impl_tier1_features!(Ads1014, DataRate12Bit);
impl_tier1_features!(Ads1015, DataRate12Bit);
impl_tier1_features!(Ads1113, DataRate16Bit);
impl_tier1_features!(Ads1114, DataRate16Bit);
impl_tier1_features!(Ads1115, DataRate16Bit);
