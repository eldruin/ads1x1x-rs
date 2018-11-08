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
