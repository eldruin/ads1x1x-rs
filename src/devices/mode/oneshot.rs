//! Common functions
use core::marker::PhantomData;

use crate::{
    devices::OperatingMode,
    mode,
    register::{Conversion12, Conversion16},
    Ads1013, Ads1014, Ads1015, Ads1113, Ads1114, Ads1115, ChannelId, Config, Error,
};

macro_rules! impl_one_shot {
    ($Ads:ident, $conv:ty) => {
        impl<I2C, E> $Ads<I2C, mode::OneShot>
        where
            I2C: embedded_hal::i2c::I2c<Error = E>,
        {
            /// Change operating mode to Continuous
            ///
            /// On error, returns a pair of the error and the current instance.
            pub fn into_continuous(
                mut self,
            ) -> Result<$Ads<I2C, mode::Continuous>, (Error<E>, Self)> {
                if let Err(e) = self.set_operating_mode(OperatingMode::Continuous) {
                    return Err((e, self));
                }
                Ok($Ads {
                    i2c: self.i2c,
                    address: self.address,
                    config: self.config,
                    a_conversion_was_started: true,
                    mode: PhantomData,
                })
            }

            fn trigger_measurement(&mut self, config: &Config) -> Result<(), Error<E>> {
                let config = config.union(Config::OS);
                self.write_reg_u16(config)
            }

            /// Requests that the ADC begins a conversion on the specified channel.
            ///
            /// The output value will be within `[2047..-2048]` for 12-bit devices
            /// (`ADS101x`) and within `[32767..-32768]` for 16-bit devices (`ADS111x`).
            /// The voltage that these values correspond to must be calculated using
            /// the full-scale range ([`FullScaleRange`](crate::FullScaleRange)) selected.
            ///
            /// Returns `nb::Error::WouldBlock` while a measurement is in progress.
            ///
            /// In case a measurement was requested and after is it is finished a
            /// measurement on a different channel is requested, a new measurement on
            /// using the new channel selection is triggered.
            #[allow(unused_variables)]
            pub fn read<CH: ChannelId<Self>>(&mut self, channel: CH) -> nb::Result<i16, Error<E>> {
                if self
                    .is_measurement_in_progress()
                    .map_err(nb::Error::Other)?
                {
                    return Err(nb::Error::WouldBlock);
                }
                let config = self.config.with_mux_bits(CH::channel_id());
                let same_channel = self.config == config;
                if self.a_conversion_was_started && same_channel {
                    // result is ready
                    let value = self.read_reg_u16::<$conv>().map_err(nb::Error::Other)?;
                    self.a_conversion_was_started = false;
                    return Ok(<$conv>::convert_measurement(value.0));
                }
                self.trigger_measurement(&config)
                    .map_err(nb::Error::Other)?;
                self.config = config;
                self.a_conversion_was_started = true;
                Err(nb::Error::WouldBlock)
            }
        }
    };
}

impl_one_shot!(Ads1013, Conversion12);
impl_one_shot!(Ads1014, Conversion12);
impl_one_shot!(Ads1015, Conversion12);
impl_one_shot!(Ads1113, Conversion16);
impl_one_shot!(Ads1114, Conversion16);
impl_one_shot!(Ads1115, Conversion16);
