# Changelog

All notable changes to this project will be documented in this file.
This project uses [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Blocking driver on `embedded-hal` 1.0 and async driver on `embedded-hal-async` 1.0
  (`async` feature): chip identification, reset and slave-mode clock configuration from
  `MCLK`, serial format and word length, DAC power-up and power-down, mute, volume with a
  configurable limit, and low-power standby.
- `init` leaves the DAC muted and the analog blocks powered down, and disables the
  loopbacks.
