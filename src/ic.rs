/// ICs
use crate::private;

#[non_exhaustive]
pub struct Resolution12Bit;
#[non_exhaustive]
pub struct Resolution16Bit;

macro_rules! ic_marker {
    ($name:ident) => {
        /// IC marker
        pub struct $name(());
    };
}

ic_marker!(Ads1013);
ic_marker!(Ads1113);
ic_marker!(Ads1014);
ic_marker!(Ads1114);
ic_marker!(Ads1015);
ic_marker!(Ads1115);

pub trait Tier2Features: private::Sealed {}

macro_rules! tier2_features {
    ($name:ident) => {
        impl Tier2Features for $name {}
    };
}

tier2_features!(Ads1014);
tier2_features!(Ads1114);
tier2_features!(Ads1015);
tier2_features!(Ads1115);
