# Everest Semiconductor ES8311 Audio Codec

[![Build Status](https://github.com/sunsided/es8311/workflows/Rust/badge.svg)](https://github.com/sunsided/es8311/actions)
[![codecov](https://codecov.io/gh/sunsided/es8311/graph/badge.svg)](https://codecov.io/gh/sunsided/es8311)

> Platform-agnostic Rust support for the Everest Semiconductor ES8311 low-power mono audio
> codec (I²C control, I²S audio).

> **Status:** tested against a behavioral model of the chip (host tests and Wokwi), not yet
> verified on real hardware.

| Crate                                  | Description                                                    |
|----------------------------------------|----------------------------------------------------------------|
| [`es8311-registers`](es8311-registers) | A typed map of the ES8311's I²C registers.                     |
| [`es8311-ng`](es8311-ng)               | An `embedded-hal` 1.0 driver (blocking and async).             |
| [`es8311-sim`](es8311-sim)             | A register-level behavioral model for tests and emulation.     |

The crates.io name `es8311` belongs to another project; the driver is published as
`es8311-ng` and imported as `es8311_ng`.

The register map follows the ES8311 datasheet (Revision 10.0, January 2021) and the
ES8311 user guide (Rev 1.11, September 2018). Facts the Everest documents lack, such as
clock coefficients and the initialization order, come from Espressif's Apache-2.0
[`esp_codec_dev`](https://github.com/espressif/esp-adf/tree/master/components/esp_codec_dev)
ES8311 driver (v1.6.2). No code was taken from it or from other drivers. Each crate README
has a Sources table.

## Assumptions

Behavior the documents leave open, and what these crates assume:

- The reset bits in `0x00` reset the digital blocks but not the register contents
  ("except control port block"). `init` therefore writes every register it relies on.
- Starting the chip state machine (`CSM_ON`) with `0x0D` powered down keeps the analog
  blocks off; the power-up sequence does not override the power-down bits.
- The DAC needs the analog bias and references in `0x0D`, including the ADC ones.
  `dac_power_up` powers them all up and leaves VMID in "normal speed charge" (`0x0D` =
  `0x01`), as `esp_codec_dev` does, rather than switching to "normal VMID operation".
- Setting both DAC mute bits in `0x31` silences the output.
- 5 ms is enough for the reset delay ("several milliseconds").
- Leaving standby needs a full `init`.
- The user guide limits the pre-divider output to 10 MHz at DVDD = 3.3 V (4.5 MHz at
  1.8 V), but its own example and table run 12.288 MHz undivided. The clock table follows
  the tables.

The behavioral model's own assumptions (auto-increment, standby bus behavior, the output
enable condition) are listed in the `es8311-sim` crate docs.

## Board: Kode Dot

The Kode Dot has the codec at `0x18` on I²C0 (400 kHz). The ESP32-P4 is the I²S master at
48 kHz, 16-bit, MCLK = 256 × fs (`Config::new(48_000)`):

| Signal              | GPIO |
|---------------------|------|
| MCLK                | 36   |
| BCLK                | 34   |
| LRCK                | 37   |
| DSDIN (P4 to codec) | 38   |
| ASDOUT (codec to P4)| 33   |

An NS4150B class-D amplifier is always enabled by another MCU, so unmuting the DAC makes
sound. Limit the volume to `0xA9` with `set_max_volume`.

## Emulation

The behavioral model in `es8311-sim` is used both as a fake I²C bus in host tests and as a
[Wokwi](https://wokwi.com) custom chip (`sim/wokwi-chip`). The ESP32-C3 firmware in
`examples/esp32c3` configures the emulated codec over I²C; its scenario checks the
resulting register state on the serial output using `wokwi-cli`. I²S audio is not emulated.

### Use the chip in your own Wokwi project

Every [GitHub Release](https://github.com/sunsided/es8311/releases) carries the chip as
`chip.zip`. Add a `chip-es8311` part and this dependency to your `diagram.json`, with the
latest release version written without the tag's `v` (`@v0.1.0` fails to load):

```json
"dependencies": { "chip-es8311": "github:sunsided/es8311@0.1.0" }
```

This works on [wokwi.com](https://wokwi.com), in the Wokwi VS Code extension and with
`wokwi-cli`; nothing needs to be built.

### Run the example locally

Needs [`wokwi-cli`](https://docs.wokwi.com/wokwi-ci/cli-installation) and a
`WOKWI_CLI_TOKEN` from the [Wokwi CI dashboard](https://wokwi.com/dashboard/ci):

```sh
task wokwi:test
```

[`sim/wokwi-chip`](sim/wokwi-chip/README.md) describes building the chip and its pins;
[`examples/esp32c3`](examples/esp32c3/README.md) the wiring and the scenario.

## Releasing

Both crates share one version. From `main`, with the changes listed under
`## [Unreleased]` in each crate's `CHANGELOG.md`:

```sh
cargo release <patch|minor|major>            # dry run
cargo release <patch|minor|major> --execute
```

This commits `Release X.Y.Z` and pushes the tag `vX.Y.Z`. The tag runs the release workflow,
which publishes `es8311-registers` and `es8311-ng` to crates.io via
[trusted publishing](https://crates.io/docs/trusted-publishing) (no API token in the
repository) and then creates a GitHub Release named `X.Y.Z` with the changelog sections
as notes.

Each GitHub Release carries the packaged `.crate` files and the Wokwi chip as `chip.zip`,
all with [build provenance attestations](https://docs.github.com/actions/security-for-github-actions/using-artifact-attestations):

```sh
gh attestation verify chip.zip -R sunsided/es8311
```

## Code of Conduct

We abide by the [Contributor Covenant][cc] and ask that you do as well.

## License

Copyright © 2026 Markus Mayer

Triple licensed under your choice of either of:

- European Union Public Licence, Version 1.2, ([LICENSE-EUPL](LICENSE-EUPL)
  or https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12)
- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

[cc]: https://contributor-covenant.org
