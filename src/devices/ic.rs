/// ICs
#[derive(PartialEq)]
pub enum ResolutionBits {
        _12,
        _16
}

pub trait Resolution : super::private::Sealed {
    const BITS : ResolutionBits;
}

/// ADS1013 IC marker
pub struct Ads1013(());
impl Resolution for Ads1013 {
    const BITS: ResolutionBits = ResolutionBits::_12;
}

/// ADS1113 IC marker
pub struct Ads1113(());
impl Resolution for Ads1113 {
    const BITS: ResolutionBits = ResolutionBits::_16;
}