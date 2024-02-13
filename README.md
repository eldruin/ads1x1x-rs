# Rust ADS1x1x ultra-small, low-power analog-to-digital converters (ADC) driver

[![crates.io](https://img.shields.io/crates/v/ads1x1x.svg)](https://crates.io/crates/ads1x1x)
[![Docs](https://docs.rs/ads1x1x/badge.svg)](https://docs.rs/ads1x1x)
![Minimum Supported Rust Version](https://img.shields.io/badge/rustc-1.62+-blue.svg)
[![Build Status](https://github.com/eldruin/ads1x1x-rs/workflows/Build/badge.svg)](https://github.com/eldruin/ads1x1x-rs/actions?query=workflow%3ABuild)
[![Coverage Status](https://coveralls.io/repos/github/eldruin/ads1x1x-rs/badge.svg?branch=master)](https://coveralls.io/github/eldruin/ads1x1x-rs?branch=master)

This is a platform agnostic Rust driver for the ADS1013, ADS1014, ADS1015,
ADS1113, ADS1114 and ADS1115 ultra-small, low-power
analog-to-digital converters (ADC), based on the [`embedded-hal`] traits.

[Introductory blog post]

This driver allows you to:
- Set the operating mode to one-shot or continuous. See: `into_continuous()`.
- Make a measurement in one-shot mode. See: `read()`.
- Start continuous conversion mode. See: `start()`.
- Read the last measurement made in continuous conversion mode. See: `read()`.
- Set the data rate. See: `set_data_rate()`.
- Set the full-scale range (gain amplifier). See `set_full_scale_range()`.
- Read whether a measurement is in progress. See: `is_measurement_in_progress()`.
- Set the ALERT/RDY pin to be used as conversion-ready pin. See: `use_alert_rdy_pin_as_ready()`.
- Comparator:
    - Set the low and high thresholds. See: `set_high_threshold_raw()`.
    - Set the comparator mode. See: `set_comparator_mode()`.
    - Set the comparator polarity. See: `set_comparator_polarity()`.
    - Set the comparator latching. See: `set_comparator_latching()`.
    - Set the comparator queue. See: `set_comparator_queue()`.
    - Disable the comparator. See: `disable_comparator()`.

## The devices

The devices are precision, low power, 12/16-bit analog-to-digital
converters (ADC) that provide all features necessary to measure the most
common sensor signals in an ultra-small package. Depending on the device,
these  integrate a programmable gain amplifier (PGA), voltage reference,
oscillator and high-accuracy temperature sensor.

The devices can perform conversions at data rates up to 3300 samples per
second (SPS). The PGA offers input ranges from ±256 mV to ±6.144 V,
allowing both large and small signals to be measured with high resolution.
An input multiplexer (MUX) allows to measure two differential or four
single-ended inputs. The high-accuracy temperature sensor can be used for
system-level temperature monitoring or cold-junction compensation for
thermocouples.

The devices operate either in continuous-conversion mode, or in a
single-shot mode that automatically powers down after a conversion.
Single-shot mode significantly reduces current consumption during idle
periods. Data is transferred through I2C.

Here is a comparison of the caracteristics of the devices:

| Device  | Resolution | Sample Rate  | Channels | Multi-channel | Features        |
|---------|------------|--------------|----------|---------------|-----------------|
| ADS1013 | 12-bit     | Max 3300 SPS | 1        | N/A           |                 |
| ADS1014 | 12-bit     | Max 3300 SPS | 1        | N/A           | Comparator, PGA |
| ADS1015 | 12-bit     | Max 3300 SPS | 4        | Multiplexed   | Comparator, PGA |
| ADS1113 | 16-bit     | Max 860 SPS  | 1        | N/A           |                 |
| ADS1114 | 16-bit     | Max 860 SPS  | 1        | N/A           | Comparator, PGA |
| ADS1115 | 16-bit     | Max 860 SPS  | 4        | Multiplexed   | Comparator, PGA |

Datasheets:
- [ADS101x](http://www.ti.com/lit/ds/symlink/ads1015.pdf)
- [ADS111x](http://www.ti.com/lit/ds/symlink/ads1115.pdf)

## Usage

To use this driver, import this crate and an `embedded_hal` implementation,
then instantiate the appropriate device.
In the following examples an instance of the device ADS1013 will be created
as an example. Other devices can be created with similar methods like:
`Ads1x1x::new_ads1114(...)`.

Please find additional examples using hardware in this repository: [driver-examples]

[driver-examples]: https://github.com/eldruin/driver-examples

```rust
use linux_embedded_hal::I2cdev;
use nb::block;

use ads1x1x::{channel, Ads1x1x, SlaveAddr};

fn main() {
    let dev = I2cdev::new("/dev/i2c-1").unwrap();
    let mut adc = Ads1x1x::new_ads1013(dev, SlaveAddr::default());
    let value = block!(adc.read(channel::DifferentialA0A1)).unwrap();
    println!("Measurement: {}", value);
    // get I2C device back
    let _dev = adc.destroy_ads1013();
}
```

## Support

For questions, issues, feature requests, and other changes, please file an
[issue in the github project](https://github.com/eldruin/ads1x1x-rs/issues).

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.

### Contributing

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.

[`embedded-hal`]: https://github.com/rust-embedded/embedded-hal

[Introductory blog post]: https://blog.eldruin.com/ads1x1x-analog-to-digital-converter-driver-in-rust/
