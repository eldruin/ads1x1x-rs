//! Common functions

use { Ads1x1x, DataRate, Error, Register, BitFlags, interface, ic };

impl<DI, IC, MODE, E> Ads1x1x<DI, IC, MODE>
where
    DI: interface::WriteData<Error = E>,
    IC: ic::Resolution
{
    /// Set data rate
    pub fn set_data_rate(&mut self, rate: DataRate) -> Result<(), Error<E>> {
        let config;
        match rate {
            DataRate::Sps128  => config = self.config.with_low( BitFlags::DR2).with_low( BitFlags::DR1).with_low( BitFlags::DR0),
            DataRate::Sps250  => config = self.config.with_low( BitFlags::DR2).with_low( BitFlags::DR1).with_high(BitFlags::DR0),
            DataRate::Sps490  => config = self.config.with_low( BitFlags::DR2).with_high(BitFlags::DR1).with_low( BitFlags::DR0),
            DataRate::Sps920  => config = self.config.with_low( BitFlags::DR2).with_high(BitFlags::DR1).with_high(BitFlags::DR0),
            DataRate::Sps1600 => config = self.config.with_high(BitFlags::DR2).with_low( BitFlags::DR1).with_low( BitFlags::DR0),
            DataRate::Sps2400 => config = self.config.with_high(BitFlags::DR2).with_low( BitFlags::DR1).with_high(BitFlags::DR0),
            DataRate::Sps3300 => config = self.config.with_high(BitFlags::DR2).with_high(BitFlags::DR1).with_low( BitFlags::DR0),
        }
        self.iface.write_register(Register::CONFIG, config.bits)?;
        self.config = config;
        Ok(())
    }

    /// Set comparator lower threshold
    pub fn set_low_threshold(&mut self, value: i16) -> Result<(), Error<E>> {
        let register_value = convert_threshold::<IC, E>(value)?;
        self.iface.write_register(Register::LOW_TH, register_value)
    }

    /// Set comparator upper threshold
    pub fn set_high_threshold(&mut self, value: i16) -> Result<(), Error<E>> {
        let register_value = convert_threshold::<IC, E>(value)?;
        self.iface.write_register(Register::HIGH_TH, register_value)
    }
}

fn convert_threshold<IC, E>(value: i16) -> Result<u16, Error<E>>
where
    IC: ic::Resolution
{
    if IC::BITS == ic::ResolutionBits::_12 {
        if value < -2048 || value > 2047 {
            return Err(Error::InvalidInputData);
        }
        Ok((value << 4) as u16)
    }
    else {
        Ok(value as u16)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_invalid_input_data<E>(result: Result<u16, Error<E>>) {
        match result {
            Err(Error::InvalidInputData) => (),
            _ => panic!("InvalidInputData error was not returned.")
        }
    }

    #[test]
    fn convert_12_bits() {
        assert_invalid_input_data(convert_threshold::<ic::Ads1013, ()>(2048));
        assert_invalid_input_data(convert_threshold::<ic::Ads1013, ()>(-2049));

        assert_eq!(     0, convert_threshold::<ic::Ads1013, ()>(0).unwrap());
        assert_eq!(0x7FF0, convert_threshold::<ic::Ads1013, ()>(2047).unwrap());
        assert_eq!(0x8000, convert_threshold::<ic::Ads1013, ()>(-2048).unwrap());
        assert_eq!(0xFFF0, convert_threshold::<ic::Ads1013, ()>(-1).unwrap());
    }

    #[test]
    fn convert_16_bits() {
        assert_eq!(0x7FFF, convert_threshold::<ic::Ads1113, ()>(32767).unwrap());
        assert_eq!(0x8000, convert_threshold::<ic::Ads1113,()>(-32768).unwrap());
    }
}
