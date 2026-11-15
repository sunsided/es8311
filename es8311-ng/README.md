# Everest Semiconductor ES8311 Audio Codec Driver

[![Crates.io][crates-image]][crates-link]
[![Docs][docs-image]][docs-link]
[![Build Status][build-image]][build-link]
[![Safety Dance][safety-image]][safety-link]
![MSRV][msrv-image]
[![EUPL 1.2 licensed][license-eupl-image]][license-eupl-link]
[![Apache 2.0 licensed][license-apache-image]][license-apache-link]
[![MIT licensed][license-mit-image]][license-mit-link]

> A platform-agnostic driver to configure the Everest Semiconductor ES8311 mono audio codec.

Built on [`embedded-hal`] 1.0 and the [`es8311-registers`] map. Enable the `async`
feature for a driver on [`embedded-hal-async`]. The package is `es8311-ng`; the library
is imported as `es8311`.

```rust,ignore
use es8311::{Config, Es8311};

let mut codec = Es8311::new(i2c); // 7-bit address 0x18 (CE low)
codec.set_max_volume(0xA9)?;       // board-specific limit, see below
codec.init(&mut delay, &Config::new(48_000))?; // I²S, 16-bit, MCLK = 256 × fs

// Start the I²S clocks, then:
codec.dac_power_up()?;
codec.set_volume(0xBF)?;           // 0 dB, clamped to the limit
codec.set_mute(false)?;            // audible from here on
```

## Implemented so far

- Chip identification (`0xFD`–`0xFF`)
- Reset and configuration as an I²S slave clocked from `MCLK`, for the MCLK frequencies
  and sample rates in the clock table (every rate from 8 to 96 kHz at MCLK = 256 × fs,
  except 88.2 kHz)
- Serial format (I²S, left justified, DSP/PCM) and word length (16 to 32 bits)
- DAC power-up and power-down, mute and volume with a configurable limit
- Low-power standby
- Direct access to registers via [`es8311-registers`] structs and [`hardware-registers`] traits

The ADC, ALC, DRC and equalizers are not configured beyond their defaults; `init` powers
the ADC down and mutes the serial output. Master mode is not supported.

## Safe defaults and the volume limit

After `init`, the DAC is muted at minimum volume, all analog blocks are powered down, and
the ADC-to-DAC and DAC-to-`ASDOUT` loopbacks are off. On boards whose speaker amplifier is
always on, such as the Kode Dot, unmuting makes sound.

`set_volume` never writes above `max_volume`, which defaults to the chip's full scale
(`0xFF`, +32 dB). An amplifier after the codec adds its own gain; lower the limit so that
full volume cannot overdrive the amplifier or the speaker. The Kode Dot, with an NS4150B
class-D amplifier, uses `0xA9` (-11 dB). Raw register writes bypass the limit.

## Sources

| Fact | Source |
|------|--------|
| Slave mode: `SCLK`/`LRCK` supplied by the host; the chip detects MCLK/LRCK | [ES8311 user guide][ug] Rev 1.11, sections 8.2 and 8.7 |
| Clock path `MCLK` → `DIV_PRE` → `MULT_PRE` → `DIV_CLKDAC`; DAC clock ≥ 256 × `LRCK` and a multiple of 16 | User guide Rev 1.11, sections 8.3–8.5 |
| Clock coefficients per MCLK and sample rate | [`esp_codec_dev`][esp] v1.6.2, `device/es8311/es8311.c`, coefficient table; 4 entries cross-checked with the user guide table in section 8.5 |
| Reset procedure: assert resets, stop `CSM`, wait several ms, release, start `CSM` last | User guide Rev 1.11, section 9.1 |
| Standby: resets asserted and `CSM` off (9.1), `0x0D` powered down (9.2), ADC modulator powered down and in reset (10.2) | User guide Rev 1.11 |
| Serial format and word length (`0x09`, `0x0A`) | [ES8311 datasheet][ds] Rev 10.0, section 8 |
| DAC mute bits (`0x31`), volume -95.5 to +32 dB in 0.5 dB steps (`0x32`) | Datasheet Rev 10.0, section 8; user guide Rev 1.11, section 11.3 |
| Loopback controls `ADC2DAC_SEL`, `ADCDAT_SEL` (`0x44`) | User guide Rev 1.11, sections 10.4 and 11.1 |
| Write order of `init`; `0x01` = `0x30` before and `0x3F` after starting `CSM` | `esp_codec_dev` v1.6.2, `es8311_open` |
| `0x44` = `0x08` written twice (I²C noise immunity; the first write may fail) | `esp_codec_dev` v1.6.2, `es8311_open` |
| `0x0B` = `0x00`, `0x0C` = `0x00`, `0x10` = `0x1F`, `0x11` = `0x7F`, `0x13` = `0x10`, `0x45` = `0x00` | `esp_codec_dev` v1.6.2, `es8311_open` and `es8311_start` |
| DAC power-up: `0x0D` = `0x01`, `0x12` = `0x00` | `esp_codec_dev` v1.6.2, `es8311_start` |
| Standby also sets volume `0x00` and stops all clocks (`0x01` = `0x00`) | `esp_codec_dev` v1.6.2, `es8311_suspend` |

No code was taken from `esp_codec_dev`; the facts above were restated from it. The
coefficient table drops the 8 `esp_codec_dev` entries that break the user guide's DAC clock
rule, and keeps only the fields slave mode uses.

See the [workspace README](https://github.com/sunsided/es8311#assumptions) for the
behavior this driver assumes where the documents are silent.

[ds]: https://files.waveshare.com/wiki/common/ES8311.DS.pdf
[ug]: https://files.waveshare.com/wiki/common/ES8311.user.Guide.pdf
[esp]: https://github.com/espressif/esp-adf/blob/master/components/esp_codec_dev/device/es8311/es8311.c
[`embedded-hal`]: https://crates.io/crates/embedded-hal
[`embedded-hal-async`]: https://crates.io/crates/embedded-hal-async
[`es8311-registers`]: https://crates.io/crates/es8311-registers
[`hardware-registers`]: https://crates.io/crates/hardware-registers

## License

Copyright © 2026 Markus Mayer

Triple licensed under your choice of either of:

- European Union Public Licence, Version 1.2, ([LICENSE-EUPL](LICENSE-EUPL)
  or https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12)
- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

[crates-image]: https://img.shields.io/crates/v/es8311-ng
[crates-link]: https://crates.io/crates/es8311-ng
[docs-image]: https://docs.rs/es8311-ng/badge.svg
[docs-link]: https://docs.rs/es8311-ng/
[build-image]: https://github.com/sunsided/es8311/workflows/Rust/badge.svg
[build-link]: https://github.com/sunsided/es8311/actions
[safety-image]: https://img.shields.io/badge/unsafe-forbidden-success.svg
[safety-link]: https://github.com/rust-secure-code/safety-dance/
[msrv-image]: https://img.shields.io/badge/rustc-1.85+-blue.svg
[license-eupl-image]: https://img.shields.io/badge/license-EUPL_1.2-blue.svg
[license-eupl-link]: https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12
[license-apache-image]: https://img.shields.io/badge/license-Apache_2.0-blue.svg
[license-apache-link]: https://www.apache.org/licenses/LICENSE-2.0
[license-mit-image]: https://img.shields.io/badge/license-MIT-blue.svg
[license-mit-link]: https://opensource.org/licenses/MIT
