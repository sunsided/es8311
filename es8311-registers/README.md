# Everest Semiconductor ES8311 Audio Codec Registers

[![Crates.io][crates-image]][crates-link]
[![Docs][docs-image]][docs-link]
[![Build Status][build-image]][build-link]
[![Safety Dance][safety-image]][safety-link]
![MSRV][msrv-image]
[![EUPL 1.2 licensed][license-eupl-image]][license-eupl-link]
[![Apache 2.0 licensed][license-apache-image]][license-apache-link]
[![MIT licensed][license-mit-image]][license-mit-link]

> A typed map of the ES8311's I²C-accessible registers.

Provides functions and named fields to read and modify the codec registers `0x00`–`0x45`
and the chip ID registers `0xFD`–`0xFF`, built on
[`hardware-registers`](https://crates.io/crates/hardware-registers). Each register struct
starts at its documented power-on default.

The I²C address is `0x18` with `CE` low and `0x19` with `CE` high, in the 7-bit form used
by `embedded-hal`. Some host libraries take the 8-bit form including the R/W bit (`0x30`).

## Sources

| Fact | Source |
|------|--------|
| Register map `0x00`–`0x45`, `0xFD`–`0xFF`: names, bit layouts, defaults | [ES8311 datasheet][ds], Revision 10.0 (January 2021), section 8 |
| Chip ID `0x83` `0x11`, version `0x00` | Datasheet Rev 10.0, section 8, registers `0xFD`–`0xFF` |
| 7-bit address `0011 00x`, `x` = `CE` | Datasheet Rev 10.0, section 5; [ES8311 user guide][ug] Rev 1.11 (September 2018), section 6 |
| `ADCDAT_SEL` feeds DAC data to `ASDOUT` ("digital feedback") | User guide Rev 1.11, section 10.4 |
| `ADC2DAC_SEL` feeds the ADC to the DAC | User guide Rev 1.11, section 11.1 |
| `CSM_ON` must be set for normal operation; reset bits keep the control port | User guide Rev 1.11, section 9.1 |

Neither document has a document number. Everest's own download links were unreachable;
both PDFs were taken from the linked mirrors.

Where the documents disagree, this map follows the datasheet register default and notes
the conflict in the item docs:

- `0x01`: the bit descriptions mark the analog clocks as on by default; the register
  default is `0x00`.
- `0x14`: the user guide documents `LINSEL` as bits `[5:4]`, including a second input
  pair; the datasheet documents bit 4 only.
- `0x37`: the bit description gives the DAC equalizer as enabled by default; the
  register default `0x08` bypasses it.
- `0x21`, `0x25`, `0x29`, `0x2D`, `0x3C`, `0x40`: the datasheet lists bits `[7:0]` for the
  top byte `[29:24]` of a 30-bit coefficient; this map keeps 8 bits.
- `0x0E` bit 1 and `0x13` bit 6 are set in the register defaults but undocumented.
- `0x36` is listed without bit descriptions.

The registers `0xFA` (register reset) and `0xFC` (state flags) are documented but not
part of this map.

[ds]: https://files.waveshare.com/wiki/common/ES8311.DS.pdf
[ug]: https://files.waveshare.com/wiki/common/ES8311.user.Guide.pdf

See also the [`es8311-ng`](https://crates.io/crates/es8311-ng) driver.

## License

Copyright © 2026 Markus Mayer

Triple licensed under your choice of either of:

- European Union Public Licence, Version 1.2, ([LICENSE-EUPL](LICENSE-EUPL)
  or https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12)
- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

[crates-image]: https://img.shields.io/crates/v/es8311-registers
[crates-link]: https://crates.io/crates/es8311-registers
[docs-image]: https://docs.rs/es8311-registers/badge.svg
[docs-link]: https://docs.rs/es8311-registers/
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
