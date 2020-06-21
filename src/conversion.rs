use crate::{ic, private, Error};

#[doc(hidden)]
pub trait ConvertThreshold<E>: private::Sealed {
    fn convert_threshold(value: i16) -> Result<u16, Error<E>>;
}

impl<E> ConvertThreshold<E> for ic::Resolution12Bit {
    fn convert_threshold(value: i16) -> Result<u16, Error<E>> {
        if value < -2048 || value > 2047 {
            return Err(Error::InvalidInputData);
        }
        Ok((value << 4) as u16)
    }
}

impl<E> ConvertThreshold<E> for ic::Resolution16Bit {
    fn convert_threshold(value: i16) -> Result<u16, Error<E>> {
        Ok(value as u16)
    }
}

#[doc(hidden)]
pub trait ConvertMeasurement: private::Sealed {
    fn convert_measurement(register_data: u16) -> i16;
}

impl ConvertMeasurement for ic::Resolution12Bit {
    fn convert_measurement(register_data: u16) -> i16 {
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

impl ConvertMeasurement for ic::Resolution16Bit {
    fn convert_measurement(register_data: u16) -> i16 {
        register_data as i16
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convert_measurement_12_bits() {
        assert_eq!(0, ic::Resolution12Bit::convert_measurement(0));
        assert_eq!(2047, ic::Resolution12Bit::convert_measurement(0x7FFF));
        assert_eq!(-2048, ic::Resolution12Bit::convert_measurement(0x8000));
        assert_eq!(-1, ic::Resolution12Bit::convert_measurement(0xFFFF));
    }

    #[test]
    fn convert_measurement_16_bits() {
        assert_eq!(0, ic::Resolution16Bit::convert_measurement(0));
        assert_eq!(32767, ic::Resolution16Bit::convert_measurement(0x7FFF));
        assert_eq!(-32768, ic::Resolution16Bit::convert_measurement(0x8000));
        assert_eq!(-1, ic::Resolution16Bit::convert_measurement(0xFFFF));
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

    fn convert_threshold<T: ConvertThreshold<()>>(value: i16) -> u16 {
        T::convert_threshold(value).unwrap()
    }

    #[test]
    fn convert_threshold_12_bits() {
        assert_invalid_input_data::<()>(ic::Resolution12Bit::convert_threshold(2048));
        assert_invalid_input_data::<()>(ic::Resolution12Bit::convert_threshold(-2049));
        assert_eq!(0, convert_threshold::<ic::Resolution12Bit>(0));
        assert_eq!(0x7FF0, convert_threshold::<ic::Resolution12Bit>(2047));
        assert_eq!(0x8000, convert_threshold::<ic::Resolution12Bit>(-2048));
        assert_eq!(0xFFF0, convert_threshold::<ic::Resolution12Bit>(-1));
    }

    #[test]
    fn convert_threshold_16_bits() {
        assert_eq!(0x7FFF, convert_threshold::<ic::Resolution16Bit>(32767));
        assert_eq!(0x8000, convert_threshold::<ic::Resolution16Bit>(-32768));
    }
}
