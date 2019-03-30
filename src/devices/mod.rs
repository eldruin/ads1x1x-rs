#[derive(Debug, Clone, Copy)]
enum OperatingMode {
    OneShot,
    Continuous,
}

mod common;
mod features;
mod mode;
