# ES8311 demo for the Wokwi web editor

wokwi.com no longer compiles Rust online, so this demo uses an Arduino sketch. It speaks
raw I²C to the same ES8311 chip, runs the same configuration sequence and prints the
register state in the same format as the [Rust example](../esp32c3). As there, nothing
drives I²S, so the codec produces no sound. The chip is loaded from this repository's
GitHub Release via the `dependencies` entry in `diagram.json`.

Live project: <https://wokwi.com/projects/480780808722372609>

To recreate it on wokwi.com:

1. Start a new **ESP32-C3** Arduino project.
2. Replace `sketch.ino` and `diagram.json` with the files from this directory.
3. Start the simulation and watch the serial monitor. The ES8311 chip has no controls.
