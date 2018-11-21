#[derive(Debug, Clone, Copy)]
enum OperatingMode {
    OneShot,
    Continuous,
}

mod mode;
mod features;
mod common;
