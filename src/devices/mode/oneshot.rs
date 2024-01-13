//! Common functions
use core::marker::PhantomData;

use crate::{
    devices::OperatingMode, ic, mode, Ads1013, Ads1014, Ads1015, Ads1113, Ads1114, Ads1115,
    BitFlags, ChannelId, Config, Error, Register,
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
                let config = config.with_high(BitFlags::OS);
                self.write_register(Register::CONFIG, config.bits)
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
                    let value = self
                        .read_register(Register::CONVERSION)
                        .map_err(nb::Error::Other)?;
                    self.a_conversion_was_started = false;
                    return Ok(<$conv>::convert_measurement(value));
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

impl_one_shot!(Ads1013, ic::Resolution12Bit);
impl_one_shot!(Ads1014, ic::Resolution12Bit);
impl_one_shot!(Ads1015, ic::Resolution12Bit);
impl_one_shot!(Ads1113, ic::Resolution16Bit);
impl_one_shot!(Ads1114, ic::Resolution16Bit);
impl_one_shot!(Ads1115, ic::Resolution16Bit);
