//! Continuous measurement mode

use crate::{
    devices::OperatingMode, ic, mode, Ads1013, Ads1014, Ads1015, Ads1113, Ads1114, Ads1115,
    ChannelId, Error, ModeChangeError, Register,
};
use core::marker::PhantomData;

macro_rules! impl_continuous {
    ($Ads:ident, $conv:ty) => {
        impl<I2C, E> $Ads<I2C, mode::Continuous>
        where
            I2C: embedded_hal::i2c::I2c<Error = E>,
        {
            /// Changes operating mode to `OneShot`.
            pub fn into_one_shot(
                mut self,
            ) -> Result<$Ads<I2C, mode::OneShot>, ModeChangeError<E, Self>> {
                if let Err(Error::I2C(e)) = self.set_operating_mode(OperatingMode::OneShot) {
                    return Err(ModeChangeError::I2C(e, self));
                }
                Ok($Ads {
                    i2c: self.i2c,
                    address: self.address,
                    config: self.config,
                    a_conversion_was_started: false,
                    mode: PhantomData,
                })
            }

            /// Reads the most recent measurement.
            pub fn read(&mut self) -> Result<i16, Error<E>> {
                let value = self.read_register(Register::CONVERSION)?;
                Ok(<$conv>::convert_measurement(value))
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
                self.write_register(Register::CONFIG, config.bits)?;
                self.config = config;
                Ok(())
            }
        }
    };
}

impl_continuous!(Ads1013, ic::Resolution12Bit);
impl_continuous!(Ads1014, ic::Resolution12Bit);
impl_continuous!(Ads1015, ic::Resolution12Bit);
impl_continuous!(Ads1113, ic::Resolution16Bit);
impl_continuous!(Ads1114, ic::Resolution16Bit);
impl_continuous!(Ads1115, ic::Resolution16Bit);
