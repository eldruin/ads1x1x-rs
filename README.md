# Rust ADS1013, ADS1014, ADS1015, ADS1113, ADS1114, ADS1115, ADS1018 and ADS1118 ultra-small, low-power analog-to-digital converters (ADC) Driver [![crates.io](https://img.shields.io/crates/v/ads1x1x.svg)](https://crates.io/crates/ads1x1x) [![Docs](https://docs.rs/ads1x1x/badge.svg)](https://docs.rs/ads1x1x) [![Build Status](https://travis-ci.org/eldruin/ads1x1x-rs.svg?branch=master)](https://travis-ci.org/eldruin/ads1x1x-rs)

This is a platform agnostic Rust driver for the ADS1013, ADS1014, ADS1015,
ADS1113, ADS1114, ADS1115, ADS1018 and ADS1118 ultra-small, low-power
analog-to-digital converters (ADC), based on the [`embedded-hal`] traits.

[`embedded-hal`]: https://github.com/rust-embedded/embedded-hal

This driver allows you to:
- TODO

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
periods. Data are transferred through a I2C or SPI.

Here is a comparison of the caracteristics of the devices:

| Device  | Resolution | Sample Rate  | Channels | Interface | Multi-channel | Features                     |
|---------|------------|--------------|----------|-----------|---------------|------------------------------|
| ADS1013 | 12-bit     | Max 3300 SPS | 1        | I2C       | N/A           |                              |
| ADS1014 | 12-bit     | Max 3300 SPS | 1        | I2C       | N/A           | Comparator, PGA              |
| ADS1015 | 12-bit     | Max 3300 SPS | 4        | I2C       | Multiplexed   | Comparator, PGA              |
| ADS1018 | 12-bit     | Max 3300 SPS | 4        | SPI       | Multiplexed   | Comparator, PGA, Temp sensor |
| ADS1113 | 16-bit     | Max 860 SPS  | 1        | I2C       | N/A           |                              |
| ADS1114 | 16-bit     | Max 860 SPS  | 1        | I2C       | N/A           | Comparator, PGA              |
| ADS1115 | 16-bit     | Max 860 SPS  | 4        | I2C       | Multiplexed   | Comparator, PGA              |
| ADS1118 | 16-bit     | Max 860 SPS  | 4        | SPI       | Multiplexed   | Comparator, PGA, Temp sensor |

Datasheets:
- [ADS101x](http://www.ti.com/lit/ds/symlink/ads1015.pdf)
- [ADS1018](http://www.ti.com/lit/ds/symlink/ads1018.pdf)
- [ADS111x](http://www.ti.com/lit/ds/symlink/ads1115.pdf)
- [ADS1118](http://www.ti.com/lit/ds/symlink/ads1118.pdf)

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT) at your option.

### Contributing

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.

