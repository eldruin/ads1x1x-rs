#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
enum OperatingMode {
    OneShot,
    Continuous,
}

mod common;
mod features;
mod mode;
