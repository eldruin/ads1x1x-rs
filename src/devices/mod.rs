//! Functions for all devices

#[derive(Debug, Clone, Copy)]
enum OperatingMode {
    OneShot,
    Continuous
}

mod common;
mod mode;
mod features;
