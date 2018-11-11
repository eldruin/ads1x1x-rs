//! Functions for all devices specific to each operating mode

use ic;
mod oneshot;
mod continuous;

fn convert_measurement<IC>(register_data: u16) -> i16
where
    IC: ic::Resolution
{
    let value = register_data;
    if IC::BITS == ic::ResolutionBits::_12 {
        let is_negative = (value & 0b1000_0000_0000_0000) != 0;
        if is_negative {
            let value = 0b1111_0000_0000_0000 | (value >> 4);
            value as i16
        }
        else {
            (value >> 4) as i16
        }
    }
    else {
        value as i16
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convert_12_bits() {
        assert_eq!(    0, convert_measurement::<ic::Ads1013>(0));
        assert_eq!( 2047, convert_measurement::<ic::Ads1013>(0x7FFF));
        assert_eq!(-2048, convert_measurement::<ic::Ads1013>(0x8000));
        assert_eq!(   -1, convert_measurement::<ic::Ads1013>(0xFFFF));
    }

    #[test]
    fn convert_16_bits() {
        assert_eq!(     0, convert_measurement::<ic::Ads1113>(0));
        assert_eq!( 32767, convert_measurement::<ic::Ads1113>(0x7FFF));
        assert_eq!(-32768, convert_measurement::<ic::Ads1113>(0x8000));
        assert_eq!(    -1, convert_measurement::<ic::Ads1113>(0xFFFF));
    }
}
