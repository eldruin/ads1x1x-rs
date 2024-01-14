//! Continuous measurement mode

use crate::{
    mode,
    register::{Config, Conversion12, Conversion16},
    Ads1013, Ads1014, Ads1015, Ads1113, Ads1114, Ads1115, ChannelId, Error,
};
use core::marker::PhantomData;

macro_rules! impl_continuous {
    ($Ads:ident, $conv:ty) => {
        impl<I2C, E> $Ads<I2C, mode::Continuous>
        where
            I2C: embedded_hal::i2c::I2c<Error = E>,
        {
            /// Changes operating mode to `OneShot`.
            ///
            /// On error, returns a pair of the error and the current instance.
            pub fn into_one_shot(mut self) -> Result<$Ads<I2C, mode::OneShot>, (Error<E>, Self)> {
                let config = self.config.union(Config::MODE);
                if let Err(e) = self.write_reg_u16(config) {
                    return Err((e, self));
                }
                Ok($Ads {
                    i2c: self.i2c,
                    address: self.address,
                    config,
                    a_conversion_was_started: false,
                    mode: PhantomData,
                })
            }

            /// Reads the most recent measurement.
            pub fn read(&mut self) -> Result<i16, Error<E>> {
                let value = self.read_reg_u16::<$conv>()?;
                Ok(<$conv>::convert_measurement(value.0))
            }

            /// Selects the channel for measurements.
            ///
            /// Note that when changing the channel in continuous conversion mode, the
            /// ongoing conversion will be completed.
            /// The following conversions will use the new channel configuration.
            #[allow(unused_variables)]
            pub fn select_channel<CH: ChannelId<Self>>(
                &mut self,
                channel: CH,
            ) -> Result<(), Error<E>> {
                let config = self.config.with_mux_bits(CH::channel_id());
                self.write_reg_u16(config)?;
                self.config = config;
                Ok(())
            }
        }
    };
}

impl_continuous!(Ads1013, Conversion12);
impl_continuous!(Ads1014, Conversion12);
impl_continuous!(Ads1015, Conversion12);
impl_continuous!(Ads1113, Conversion16);
impl_continuous!(Ads1114, Conversion16);
impl_continuous!(Ads1115, Conversion16);
