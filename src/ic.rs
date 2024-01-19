pub(crate) struct Resolution12Bit;

impl Resolution12Bit {
    pub fn convert_threshold(value: i16) -> u16 {
        if !(-2048..=2047).contains(&value) {
            panic!("Threshold must be between -2048 and 2047, but is {value}.")
        }

        (value << 4) as u16
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
    pub fn convert_threshold(value: i16) -> u16 {
        value as u16
    }

    pub fn convert_measurement(register_data: u16) -> i16 {
        register_data as i16
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convert_measurement_12bit() {
        assert_eq!(0, Resolution12Bit::convert_measurement(0));
        assert_eq!(2047, Resolution12Bit::convert_measurement(0x7FFF));
        assert_eq!(-2048, Resolution12Bit::convert_measurement(0x8000));
        assert_eq!(-1, Resolution12Bit::convert_measurement(0xFFFF));
    }

    #[test]
    fn convert_measurement_16bit() {
        assert_eq!(0, Resolution16Bit::convert_measurement(0));
        assert_eq!(32767, Resolution16Bit::convert_measurement(0x7FFF));
        assert_eq!(-32768, Resolution16Bit::convert_measurement(0x8000));
        assert_eq!(-1, Resolution16Bit::convert_measurement(0xFFFF));
    }

    #[test]
    fn convert_threshold_12bit() {
        assert_eq!(0, Resolution12Bit::convert_threshold(0));
        assert_eq!(0x7FF0, Resolution12Bit::convert_threshold(2047));
        assert_eq!(0x8000, Resolution12Bit::convert_threshold(-2048));
        assert_eq!(0xFFF0, Resolution12Bit::convert_threshold(-1));
    }

    #[test]
    #[should_panic]
    fn convert_threshold_12bit_invalid_low() {
        Resolution12Bit::convert_threshold(-2049);
    }

    #[test]
    #[should_panic]
    fn convert_threshold_12bit_invalid_high() {
        Resolution12Bit::convert_threshold(2048);
    }

    #[test]
    fn convert_threshold_16bit() {
        assert_eq!(0x7FFF, Resolution16Bit::convert_threshold(32767));
        assert_eq!(0x8000, Resolution16Bit::convert_threshold(-32768));
    }
}
