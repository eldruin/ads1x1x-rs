mod common;
use crate::common::{
    destroy_ads1013, destroy_ads1014, destroy_ads1015, destroy_ads1113, destroy_ads1114,
    destroy_ads1115, new_ads1013, new_ads1014, new_ads1015, new_ads1113, new_ads1114, new_ads1115,
};

macro_rules! impl_tests {
    ($IC:ident, $create:ident, $destroy:ident) => {
        mod $IC {
            use super::*;
            #[test]
            fn can_create() {
                let adc = $create(&[]);
                $destroy(adc);
            }
        }
    };
}

impl_tests!(ads1013, new_ads1013, destroy_ads1013);
impl_tests!(ads1113, new_ads1113, destroy_ads1113);
impl_tests!(ads1014, new_ads1014, destroy_ads1014);
impl_tests!(ads1114, new_ads1114, destroy_ads1114);
impl_tests!(ads1015, new_ads1015, destroy_ads1015);
impl_tests!(ads1115, new_ads1115, destroy_ads1115);
