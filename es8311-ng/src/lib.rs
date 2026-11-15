//! # Everest Semiconductor ES8311 audio codec driver
//!
//! > A platform-agnostic driver to configure the Everest Semiconductor ES8311 mono audio
//! > codec over I²C.
//!
//! This driver was built using [`embedded-hal`] 1.0 and [`es8311-registers`]. Enable the
//! `async` feature for an equivalent driver on [`embedded-hal-async`] in the `asynch` module.
//!
//! The driver only configures the codec. Audio data flows over I²S, which the host drives:
//! the codec runs in slave mode, with `MCLK`, `SCLK` and `LRCK` supplied by the host.
//!
//! ## Safe defaults
//!
//! After [`Es8311::init`], the DAC is muted at minimum volume and all analog blocks are
//! powered down. Nothing reaches the output until [`Es8311::dac_power_up`] and
//! `set_mute(false)`. Boards with an always-on speaker amplifier play sound as soon as
//! the DAC is unmuted.
//!
//! ## Volume limit
//!
//! [`Es8311::set_volume`] never writes a value above [`Es8311::max_volume`]. The limit
//! defaults to the chip's full scale, [`DAC_VOLUME_MAX`] (+32 dB). Lower it with
//! [`Es8311::set_max_volume`] when an amplifier after the codec adds gain, so the speaker
//! or the amplifier cannot be overdriven. On the Kode Dot, with an NS4150B class-D
//! amplifier, the limit is `0xA9` (-11 dB).
//!
//! The raw register access ([`Es8311::write_register`]) bypasses the limit.
//!
//! [`embedded-hal`]: https://crates.io/crates/embedded-hal
//! [`embedded-hal-async`]: https://crates.io/crates/embedded-hal-async
//! [`es8311-registers`]: https://crates.io/crates/es8311-registers

#![deny(missing_docs)]
#![deny(warnings)]
#![no_std]
#![forbid(unsafe_code)]
// Enables the `doc_cfg` feature when the `docsrs` configuration attribute is defined.
#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(feature = "async")]
#[cfg_attr(docsrs, doc(cfg(feature = "async")))]
pub mod asynch;
mod clock;
mod sequence;

pub use es8311_registers as registers;
pub use es8311_registers::{
    ALTERNATE_DEVICE_ADDRESS, DEFAULT_DEVICE_ADDRESS, SerialFormat, WordLength,
};

use embedded_hal::delay::DelayNs;
use embedded_hal::i2c::I2c;
use es8311_registers::prelude::*;
use es8311_registers::{ChipId1, ChipId2, ChipVersion, DacControl, DacVolume};
use sequence::Write;

/// The ID read from [`ChipId1`] and [`ChipId2`] (`0xFD`, `0xFE`) of an ES8311.
pub const ES8311_CHIP_ID: u16 =
    u16::from_be_bytes([registers::ES8311_CHIP_ID1, registers::ES8311_CHIP_ID2]);

/// The highest [`DacVolume`] value: +32 dB.
pub const DAC_VOLUME_MAX: u8 = 0xFF;

/// The [`DacVolume`] value for 0 dB. Each step is 0.5 dB.
pub const DAC_VOLUME_0DB: u8 = 0xBF;

/// How long [`Es8311::init`] holds the digital blocks in reset, in milliseconds.
///
/// The user guide (section 9.1) suggests "several milliseconds".
pub const RESET_DELAY_MS: u32 = 5;

/// The audio interface configuration.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Config {
    /// The frequency on the `MCLK` pin, in Hz.
    pub mclk_hz: u32,
    /// The sample rate (`LRCK`), in Hz.
    pub sample_rate_hz: u32,
    /// The serial data format, for both directions.
    pub format: SerialFormat,
    /// The serial data word length, for both directions.
    pub word_length: WordLength,
}

impl Config {
    /// I²S with 16-bit words at the specified sample rate and MCLK = 256 × fs.
    pub const fn new(sample_rate_hz: u32) -> Self {
        Self {
            mclk_hz: sample_rate_hz * 256,
            sample_rate_hz,
            format: SerialFormat::I2s,
            word_length: WordLength::Bits16,
        }
    }

    /// Sets the frequency on the `MCLK` pin, in Hz.
    pub const fn with_mclk(self, mclk_hz: u32) -> Self {
        Self { mclk_hz, ..self }
    }

    /// Sets the serial data format.
    pub const fn with_format(self, format: SerialFormat) -> Self {
        Self { format, ..self }
    }

    /// Sets the serial data word length.
    pub const fn with_word_length(self, word_length: WordLength) -> Self {
        Self {
            word_length,
            ..self
        }
    }
}

/// A driver error.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Error<E> {
    /// The I²C transfer failed.
    I2c(E),
    /// The chip ID is not [`ES8311_CHIP_ID`].
    UnknownChip(u16),
    /// No clock coefficients exist for this MCLK frequency and sample rate.
    UnsupportedClock,
}

/// Checks the configuration before any bus access.
fn coefficients<E>(config: &Config) -> Result<&'static clock::ClockCoefficients, Error<E>> {
    clock::lookup(config.mclk_hz, config.sample_rate_hz).ok_or(Error::UnsupportedClock)
}

/// ES8311 driver.
#[derive(Debug)]
pub struct Es8311<I2C> {
    i2c: I2C,
    address: u8,
    max_volume: u8,
}

impl<I2C> Es8311<I2C>
where
    I2C: I2c,
{
    /// Creates a driver for a chip at [`DEFAULT_DEVICE_ADDRESS`]. Performs no bus access.
    pub fn new(i2c: I2C) -> Self {
        Self::with_address(i2c, DEFAULT_DEVICE_ADDRESS)
    }

    /// Creates a driver for a chip at the specified 7-bit address. Performs no bus access.
    pub fn with_address(i2c: I2C, address: u8) -> Self {
        Self {
            i2c,
            address,
            max_volume: DAC_VOLUME_MAX,
        }
    }

    /// Consumes the driver and returns the I²C bus.
    pub fn release(self) -> I2C {
        self.i2c
    }

    /// Reads the chip ID from `0xFD` and `0xFE`.
    pub fn chip_id(&mut self) -> Result<u16, I2C::Error> {
        let high = self.read_register::<ChipId1>()?.value();
        let low = self.read_register::<ChipId2>()?.value();
        Ok(u16::from_be_bytes([high, low]))
    }

    /// Reads the chip version from `0xFF`.
    pub fn chip_version(&mut self) -> Result<u8, I2C::Error> {
        Ok(self.read_register::<ChipVersion>()?.value())
    }

    /// Checks whether the chip ID matches [`ES8311_CHIP_ID`].
    pub fn identify(&mut self) -> Result<bool, I2C::Error> {
        Ok(self.chip_id()? == ES8311_CHIP_ID)
    }

    /// Resets and configures the codec as an I²S slave clocked from `MCLK`.
    ///
    /// Checks the chip ID, resets the digital blocks for [`RESET_DELAY_MS`], then sets
    /// the clock dividers, the serial format and the output driver. Disables the
    /// ADC-to-DAC and DAC-to-`ASDOUT` loopbacks. Leaves the DAC muted at minimum volume
    /// and all analog blocks powered down.
    ///
    /// Fails with [`Error::UnsupportedClock`] before any bus access if the clock
    /// configuration is not supported. Call again to leave [`standby`](Self::standby).
    pub fn init<D: DelayNs>(
        &mut self,
        delay: &mut D,
        config: &Config,
    ) -> Result<(), Error<I2C::Error>> {
        let clock = coefficients(config)?;
        let id = self.chip_id().map_err(Error::I2c)?;

        if id != ES8311_CHIP_ID {
            return Err(Error::UnknownChip(id));
        }

        self.write_all(&sequence::RESET).map_err(Error::I2c)?;
        delay.delay_ms(RESET_DELAY_MS);
        self.write_all(&sequence::configure(config, clock))
            .map_err(Error::I2c)
    }

    /// Powers up the analog circuits and the DAC. The mute state is unchanged.
    pub fn dac_power_up(&mut self) -> Result<(), I2C::Error> {
        self.write_all(&sequence::DAC_POWER_UP)
    }

    /// Mutes the DAC, then powers down the DAC and the analog circuits.
    pub fn dac_power_down(&mut self) -> Result<(), I2C::Error> {
        self.set_mute(true)?;
        self.write_all(&sequence::DAC_POWER_DOWN)
    }

    /// Mutes or unmutes the DAC.
    pub fn set_mute(&mut self, mute: bool) -> Result<(), I2C::Error> {
        self.modify_register(|r: DacControl| r.with_dac_dsmmute(mute).with_dac_demmute(mute))
    }

    /// The highest value [`set_volume`](Self::set_volume) writes.
    pub fn max_volume(&self) -> u8 {
        self.max_volume
    }

    /// Sets the highest value [`set_volume`](Self::set_volume) writes.
    ///
    /// Lowers the current volume to the new limit if it is above it.
    pub fn set_max_volume(&mut self, max: u8) -> Result<(), I2C::Error> {
        self.max_volume = max;

        if self.volume()? > max {
            self.write_register(DacVolume::new().with_value(max))?;
        }

        Ok(())
    }

    /// Sets the DAC volume, clamped to [`max_volume`](Self::max_volume).
    ///
    /// `0x00` is -95.5 dB, [`DAC_VOLUME_0DB`] is 0 dB and each step is 0.5 dB.
    pub fn set_volume(&mut self, volume: u8) -> Result<(), I2C::Error> {
        self.write_register(DacVolume::new().with_value(volume.min(self.max_volume)))
    }

    /// Reads the DAC volume.
    pub fn volume(&mut self) -> Result<u8, I2C::Error> {
        Ok(self.read_register::<DacVolume>()?.value())
    }

    /// Mutes the DAC and enters low-power standby: powers down all analog blocks, stops
    /// the clocks and holds the digital blocks in reset. Call [`init`](Self::init) to
    /// leave standby.
    pub fn standby(&mut self) -> Result<(), I2C::Error> {
        self.set_mute(true)?;
        self.write_all(&sequence::STANDBY)
    }

    /// Reads a single register.
    pub fn read_register<R>(&mut self) -> Result<R, I2C::Error>
    where
        R: ReadableRegister,
    {
        let mut buffer = [0];
        self.i2c
            .write_read(self.address, &[*R::REGISTER_ADDRESS], &mut buffer)?;
        Ok(R::from_bits(buffer[0]))
    }

    /// Writes a single register.
    pub fn write_register<R>(&mut self, register: R) -> Result<(), I2C::Error>
    where
        R: WritableRegister,
    {
        self.i2c
            .write(self.address, &[*R::REGISTER_ADDRESS, register.to_bits()])
    }

    /// Reads, modifies and writes back a single register.
    pub fn modify_register<R, F>(&mut self, f: F) -> Result<(), I2C::Error>
    where
        R: ReadableRegister + WritableRegister,
        F: FnOnce(R) -> R,
    {
        let register = self.read_register::<R>()?;
        self.write_register(f(register))
    }

    fn write_all(&mut self, writes: &[Write]) -> Result<(), I2C::Error> {
        for &(reg, value) in writes {
            self.i2c.write(self.address, &[reg.addr(), value])?;
        }

        Ok(())
    }
}
