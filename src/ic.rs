use crate::Error;

pub(crate) struct Resolution12Bit;

impl Resolution12Bit {
    pub fn convert_threshold<E>(value: i16) -> Result<u16, Error<E>> {
        if !(-2048..=2047).contains(&value) {
            return Err(Error::InvalidInputData);
        }
        Ok((value << 4) as u16)
    }

    pub fn convert_measurement(register_data: u16) -> i16 {
        let value = register_data;
        let is_negative = (value & 0b1000_0000_0000_0000) != 0;
        if is_negative {
            let value = 0b1111_0000_0000_0000 | (value >> 4);
            value as i16
        } else {
            (value >> 4) as i16
        }
    }
}

pub(crate) struct Resolution16Bit;

impl Resolution16Bit {
    pub fn convert_threshold<E>(value: i16) -> Result<u16, Error<E>> {
        Ok(value as u16)
    }

    pub fn convert_measurement(register_data: u16) -> i16 {
        register_data as i16
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convert_measurement_12_bits() {
        assert_eq!(0, Resolution12Bit::convert_measurement(0));
        assert_eq!(2047, Resolution12Bit::convert_measurement(0x7FFF));
        assert_eq!(-2048, Resolution12Bit::convert_measurement(0x8000));
        assert_eq!(-1, Resolution12Bit::convert_measurement(0xFFFF));
    }

    #[test]
    fn convert_measurement_16_bits() {
        assert_eq!(0, Resolution16Bit::convert_measurement(0));
        assert_eq!(32767, Resolution16Bit::convert_measurement(0x7FFF));
        assert_eq!(-32768, Resolution16Bit::convert_measurement(0x8000));
        assert_eq!(-1, Resolution16Bit::convert_measurement(0xFFFF));
    }

    fn assert_invalid_input_data<E>(result: Result<u16, Error<E>>) {
        match result {
            Err(Error::InvalidInputData) => (),
            _ => panic!("InvalidInputData error was not returned."),
        }
    }

    #[test]
    fn check_assert_matches() {
        assert_invalid_input_data::<()>(Err(Error::InvalidInputData));
    }

    #[test]
    #[should_panic]
    fn check_assert_fails() {
        assert_invalid_input_data::<()>(Ok(0));
    }

    #[test]
    fn convert_threshold_12_bits() {
        assert_invalid_input_data::<()>(Resolution12Bit::convert_threshold::<()>(2048));
        assert_invalid_input_data::<()>(Resolution12Bit::convert_threshold::<()>(-2049));
        assert_eq!(Ok(0), Resolution12Bit::convert_threshold::<()>(0));
        assert_eq!(Ok(0x7FF0), Resolution12Bit::convert_threshold::<()>(2047));
        assert_eq!(Ok(0x8000), Resolution12Bit::convert_threshold::<()>(-2048));
        assert_eq!(Ok(0xFFF0), Resolution12Bit::convert_threshold::<()>(-1));
    }

    #[test]
    fn convert_threshold_16_bits() {
        assert_eq!(Ok(0x7FFF), Resolution16Bit::convert_threshold::<()>(32767));
        assert_eq!(Ok(0x8000), Resolution16Bit::convert_threshold::<()>(-32768));
    }
}
