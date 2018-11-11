
use { ic, private, Error };

#[doc(hidden)]
pub trait ConvertThreshold<E> : private::Sealed {
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
pub trait ConvertMeasurement : private::Sealed {
    fn convert_measurement(register_data: u16) -> i16;
}

impl ConvertMeasurement for ic::Resolution12Bit {
    fn convert_measurement(register_data: u16) -> i16 {
        let value = register_data;
        let is_negative = (value & 0b1000_0000_0000_0000) != 0;
        if is_negative {
            let value = 0b1111_0000_0000_0000 | (value >> 4);
            value as i16
        }
        else {
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
        assert_eq!(    0, ic::Resolution12Bit::convert_measurement(0));
        assert_eq!( 2047, ic::Resolution12Bit::convert_measurement(0x7FFF));
        assert_eq!(-2048, ic::Resolution12Bit::convert_measurement(0x8000));
        assert_eq!(   -1, ic::Resolution12Bit::convert_measurement(0xFFFF));
    }

    #[test]
    fn convert_measurement_16_bits() {
        assert_eq!(     0, ic::Resolution16Bit::convert_measurement(0));
        assert_eq!( 32767, ic::Resolution16Bit::convert_measurement(0x7FFF));
        assert_eq!(-32768, ic::Resolution16Bit::convert_measurement(0x8000));
        assert_eq!(    -1, ic::Resolution16Bit::convert_measurement(0xFFFF));
    }

    fn assert_invalid_input_data<E>(result: Result<u16, Error<E>>) {
        match result {
            Err(Error::InvalidInputData) => (),
            _ => panic!("InvalidInputData error was not returned.")
        }
    }

    #[test]
    fn convert_threshold_12_bits() {
        assert_invalid_input_data::<()>(ic::Resolution12Bit::convert_threshold(2048));
        assert_invalid_input_data::<()>(ic::Resolution12Bit::convert_threshold(-2049));
        assert_eq!(     0, <ic::Resolution12Bit as ConvertThreshold<()>>::convert_threshold(0).unwrap());
        assert_eq!(0x7FF0, <ic::Resolution12Bit as ConvertThreshold<()>>::convert_threshold(2047).unwrap());
        assert_eq!(0x8000, <ic::Resolution12Bit as ConvertThreshold<()>>::convert_threshold(-2048).unwrap());
        assert_eq!(0xFFF0, <ic::Resolution12Bit as ConvertThreshold<()>>::convert_threshold(-1).unwrap());
    }

    #[test]
    fn convert_threshold_16_bits() {
        assert_eq!(0x7FFF, <ic::Resolution16Bit as ConvertThreshold<()>>::convert_threshold(32767).unwrap());
        assert_eq!(0x8000, <ic::Resolution16Bit as ConvertThreshold<()>>::convert_threshold(-32768).unwrap());
    }
}
