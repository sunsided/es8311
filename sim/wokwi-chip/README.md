# ES8311 Wokwi custom chip

Wraps the [`es8311-sim`](../../es8311-sim) behavioral model into a
[Wokwi custom chip](https://docs.wokwi.com/chips-api/getting-started).

```sh
./build.sh   # produces es8311.chip.wasm next to es8311.chip.json
```

Reference it from a project's `wokwi.toml`:

```toml
[[chip]]
name = 'es8311'
binary = '../../sim/wokwi-chip/es8311.chip.wasm'
```

and add a `chip-es8311` part to `diagram.json`.

Projects outside this repository (on wokwi.com, in VS Code or with `wokwi-cli`) can use
the `chip.zip` attached to each GitHub Release instead of building the chip. Add a
`chip-es8311` part and a dependency to `diagram.json`; no `[[chip]]` entry is needed:

```json
"dependencies": { "chip-es8311": "github:sunsided/es8311@0.1.0" }
```

The version is given without the `v` prefix of the release tag; `@v0.1.0` fails to load.

| Pin   | Function                                                  |
|-------|-----------------------------------------------------------|
| `SDA` | I²C data                                                  |
| `SCL` | I²C clock                                                 |
| `CE`  | Address select, sampled at start-up: low or open `0x18`, high `0x19` |

The chip has no audio pins: the model only covers the registers, not the I²S signal path.
It logs `es8311: DAC output on` and `off` to the simulator console when the playback path
opens or closes.
