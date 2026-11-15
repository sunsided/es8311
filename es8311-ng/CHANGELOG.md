# Changelog

All notable changes to this project will be documented in this file.
This project uses [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Changed

- Dropped `#![deny(warnings)]`, so a warning added by a newer compiler can no longer
  fail the docs.rs build.

## [0.1.0] - 2026-11-15

[0.1.0]: https://github.com/sunsided/es8311/releases/tag/v0.1.0

### Added

- Blocking driver on `embedded-hal` 1.0 and async driver on `embedded-hal-async` 1.0
  (`async` feature): chip identification, reset and slave-mode clock configuration from
  `MCLK`, serial format and word length, DAC power-up and power-down, mute, volume with a
  configurable limit, and low-power standby.
- `init` leaves the DAC muted and the analog blocks powered down, and disables the
  loopbacks.
