//! Configures an ES8311 over I²C and prints the resulting register state.
//!
//! Wiring: SDA = GPIO4, SCL = GPIO5, CE = GND (address `0x18`). The example does not
//! drive I²S, so the codec produces no sound.

#![no_std]
#![no_main]

use es8311::registers::{
    ClockSwitch, DacControl, DacPower, GpioControl, ReadableRegister, Reset, SystemPower,
};
use es8311::{Config, Es8311};
use esp_backtrace as _;
use esp_hal::delay::Delay;
use esp_hal::i2c::master::{Config as I2cConfig, I2c};
use esp_hal::main;
use esp_hal::time::Rate;
use esp_println::println;

esp_bootloader_esp_idf::esp_app_desc!();

/// The volume limit of the Kode Dot, whose speaker amplifier adds gain.
const MAX_VOLUME: u8 = 0xA9;

#[main]
fn main() -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default());
    let mut delay = Delay::new();

    let i2c = I2c::new(
        peripherals.I2C0,
        I2cConfig::default().with_frequency(Rate::from_khz(400)),
    )
    .expect("valid I2C config")
    .with_sda(peripherals.GPIO4)
    .with_scl(peripherals.GPIO5);

    let mut codec = Es8311::new(i2c);
    let chip_id = codec.chip_id().expect("codec responds");
    let version = codec.chip_version().unwrap();
    println!("ES8311 ok chip_id=0x{chip_id:04X} version=0x{version:02X}");

    // 48 kHz, 16-bit I²S, MCLK = 256 × fs.
    codec.init(&mut delay, &Config::new(48_000)).unwrap();
    println!(
        "init mute=0x{:02X} volume=0x{:02X} power=0x{:02X} dac=0x{:02X} routing=0x{:02X}",
        read::<DacControl, _>(&mut codec),
        codec.volume().unwrap(),
        read::<SystemPower, _>(&mut codec),
        read::<DacPower, _>(&mut codec),
        read::<GpioControl, _>(&mut codec),
    );

    codec.set_max_volume(MAX_VOLUME).unwrap();
    codec.set_volume(0xFF).unwrap();
    println!(
        "volume limit=0x{MAX_VOLUME:02X} requested=0xFF written=0x{:02X}",
        codec.volume().unwrap()
    );

    codec.dac_power_up().unwrap();
    codec.set_mute(false).unwrap();
    println!(
        "playing mute=0x{:02X} power=0x{:02X} dac=0x{:02X}",
        read::<DacControl, _>(&mut codec),
        read::<SystemPower, _>(&mut codec),
        read::<DacPower, _>(&mut codec),
    );

    codec.standby().unwrap();
    println!(
        "standby reset=0x{:02X} clocks=0x{:02X} power=0x{:02X}",
        read::<Reset, _>(&mut codec),
        read::<ClockSwitch, _>(&mut codec),
        read::<SystemPower, _>(&mut codec),
    );

    loop {
        delay.delay_millis(1_000);
    }
}

/// Reads a register as a raw byte.
fn read<R, I>(codec: &mut Es8311<I>) -> u8
where
    R: ReadableRegister + Into<u8>,
    I: embedded_hal::i2c::I2c,
{
    codec.read_register::<R>().unwrap().into()
}
