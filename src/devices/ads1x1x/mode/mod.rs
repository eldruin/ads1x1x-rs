//! Functions for all devices specific to each operating mode

use ic;

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

mod oneshot;
mod continuous;
