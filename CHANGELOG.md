# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Derived common traits for types.
- Add default for `ComparatorQueue`.

### Changed
- Updated `embedded-hal` to version `1`, `read` in one-shot mode is therefore only an inherent method.
- Raised MSRV to 1.62.0.

### Removed
- Removed `I2cInterface`.
- Removed `DynamicOneShot` for simplicity.
- Rewrite to use separate struct for each variant.

## [0.2.2] - 2021-07-29

### Added
- `DynamicOneShot` trait to ease usage of driver in functions. See `trait` example.

### Changed
- Updated `nb` dependency to version `1`.

## [0.2.1] - 2020-06-22

### Added
- Added helper construction methods for `SlaveAddr`.

### Changed
- Use Rust edition 2018 in code an examples.

## [0.2.0] - 2019-03-31

### Changed
- [breaking-change] Mode change has been integrated into `into_continuous` and
  `into_one_shot` methods. This removes the need for a `start` method in continuous
  mode and the `Error::NotStarted`. Everything is now encoded into the modes.
  When changing into continuous mode the measurements are started and to stop one
  can simply change into one-shot mode. (This is how the hardware does it anyway).
  The one-shot mode is not affected.
  When changing the mode an I²C communication error can occur but the unchanged device
  can now be retrieved.

## [0.1.0] - 2018-11-21

This is the initial release to crates.io of the feature-complete driver. There
may be some API changes in the future, in case I decide that something can be
further improved. All changes will be documented in this CHANGELOG.

[Unreleased]: https://github.com/eldruin/ads1x1x-rs/compare/v0.2.2...HEAD
[0.2.2]: https://github.com/eldruin/ads1x1x-rs/compare/v0.2.1...v0.2.2
[0.2.1]: https://github.com/eldruin/ads1x1x-rs/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/eldruin/ads1x1x-rs/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/eldruin/ads1x1x-rs/releases/tag/v0.1.0
