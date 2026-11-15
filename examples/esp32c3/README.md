# ES8311 on ESP32-C3

Configures an ES8311 with the blocking driver and prints the resulting register state
over the serial console: identification, the muted state after `init`, the volume limit
capping a full-scale request, the open playback path, and standby. The example does not
drive I²S, so it makes no sound.

| ESP32-C3 | ES8311 |
|----------|--------|
| GPIO4    | CDATA (SDA) |
| GPIO5    | CCLK (SCL)  |
| GND      | CE (address `0x18`) |

The volume limit `0xA9` is the one used on the Kode Dot (ESP32-P4, NS4150B amplifier);
see the [workspace README](../../README.md#board-kode-dot) for its pins.

## Run in Wokwi

Requires `rustup target add riscv32imc-unknown-none-elf wasm32-unknown-unknown`,
[`wokwi-cli`](https://docs.wokwi.com/wokwi-ci/cli-installation) and `WOKWI_CLI_TOKEN`.

```sh
../../sim/wokwi-chip/build.sh
cargo build --release
wokwi-cli . --scenario scenarios/codec.yaml --timeout 30000
```

## Run on hardware

```sh
cargo run --release   # uses espflash
```

With an amplifier and speaker attached, the codec is unmuted between the `playing` and
`standby` lines; without I²S clocks it stays silent.

## Adding another board

Create `examples/<board>/` with its own firmware, `wokwi.toml` and `diagram.json`,
and reference the same chip binary (`../../sim/wokwi-chip/es8311.chip.wasm`).
