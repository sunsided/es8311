#!/usr/bin/env sh
# Builds the chip and places it next to es8311.chip.json, as Wokwi expects.
set -eu
cd "$(dirname "$0")"
cargo build --release
cp target/wasm32-unknown-unknown/release/es8311_wokwi_chip.wasm es8311.chip.wasm
