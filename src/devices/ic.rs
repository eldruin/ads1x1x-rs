/// ICs
#[derive(PartialEq)]
pub enum ResolutionBits {
        _12,
        _16
}

pub trait Resolution : super::private::Sealed {
    const BITS : ResolutionBits;
}

macro_rules! ic_marker {
    ($name:ident, $resolution:ident) => {
        /// IC marker
        pub struct $name(());
        impl Resolution for $name {
            const BITS: ResolutionBits = ResolutionBits::$resolution;
        }
    };
}

ic_marker!(Ads1013, _12);
ic_marker!(Ads1113, _16);
ic_marker!(Ads1014, _12);
ic_marker!(Ads1114, _16);
ic_marker!(Ads1015, _12);
ic_marker!(Ads1115, _16);

pub trait Tier2Features : super::private::Sealed { }

macro_rules! tier2_features {
    ($name:ident) => {
        impl Tier2Features for $name {}
    }
}

tier2_features!(Ads1014);
tier2_features!(Ads1114);
tier2_features!(Ads1015);
tier2_features!(Ads1115);
