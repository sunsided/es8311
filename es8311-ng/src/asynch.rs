//! Async driver on `embedded-hal-async`.
//!
//! Mirrors the blocking [`Es8311`](crate::Es8311).

use crate::sequence::{self, Write};
use crate::{
    Config, DAC_VOLUME_MAX, DEFAULT_DEVICE_ADDRESS, ES8311_CHIP_ID, Error, RESET_DELAY_MS,
    coefficients,
};
use embedded_hal_async::delay::DelayNs;
use embedded_hal_async::i2c::I2c;
use es8311_registers::prelude::*;
use es8311_registers::{ChipId1, ChipId2, ChipVersion, DacControl, DacVolume};

/// Async ES8311 driver.
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
    pub async fn chip_id(&mut self) -> Result<u16, I2C::Error> {
        let high = self.read_register::<ChipId1>().await?.value();
        let low = self.read_register::<ChipId2>().await?.value();
        Ok(u16::from_be_bytes([high, low]))
    }

    /// Reads the chip version from `0xFF`.
    pub async fn chip_version(&mut self) -> Result<u8, I2C::Error> {
        Ok(self.read_register::<ChipVersion>().await?.value())
    }

    /// Checks whether the chip ID matches [`ES8311_CHIP_ID`].
    pub async fn identify(&mut self) -> Result<bool, I2C::Error> {
        Ok(self.chip_id().await? == ES8311_CHIP_ID)
    }

    /// Resets and configures the codec as an I²S slave clocked from `MCLK`.
    ///
    /// See [`Es8311::init`](crate::Es8311::init).
    pub async fn init<D: DelayNs>(
        &mut self,
        delay: &mut D,
        config: &Config,
    ) -> Result<(), Error<I2C::Error>> {
        let clock = coefficients(config)?;
        let id = self.chip_id().await.map_err(Error::I2c)?;

        if id != ES8311_CHIP_ID {
            return Err(Error::UnknownChip(id));
        }

        self.write_all(&sequence::RESET).await.map_err(Error::I2c)?;
        delay.delay_ms(RESET_DELAY_MS).await;
        self.write_all(&sequence::configure(config, clock))
            .await
            .map_err(Error::I2c)
    }

    /// Powers up the analog circuits and the DAC. The mute state is unchanged.
    pub async fn dac_power_up(&mut self) -> Result<(), I2C::Error> {
        self.write_all(&sequence::DAC_POWER_UP).await
    }

    /// Mutes the DAC, then powers down the DAC and the analog circuits.
    pub async fn dac_power_down(&mut self) -> Result<(), I2C::Error> {
        self.set_mute(true).await?;
        self.write_all(&sequence::DAC_POWER_DOWN).await
    }

    /// Mutes or unmutes the DAC.
    pub async fn set_mute(&mut self, mute: bool) -> Result<(), I2C::Error> {
        self.modify_register(|r: DacControl| r.with_dac_dsmmute(mute).with_dac_demmute(mute))
            .await
    }

    /// The highest value [`set_volume`](Self::set_volume) writes.
    pub fn max_volume(&self) -> u8 {
        self.max_volume
    }

    /// Sets the highest value [`set_volume`](Self::set_volume) writes.
    ///
    /// Lowers the current volume to the new limit if it is above it.
    pub async fn set_max_volume(&mut self, max: u8) -> Result<(), I2C::Error> {
        self.max_volume = max;

        if self.volume().await? > max {
            self.write_register(DacVolume::new().with_value(max))
                .await?;
        }

        Ok(())
    }

    /// Sets the DAC volume, clamped to [`max_volume`](Self::max_volume).
    pub async fn set_volume(&mut self, volume: u8) -> Result<(), I2C::Error> {
        self.write_register(DacVolume::new().with_value(volume.min(self.max_volume)))
            .await
    }

    /// Reads the DAC volume.
    pub async fn volume(&mut self) -> Result<u8, I2C::Error> {
        Ok(self.read_register::<DacVolume>().await?.value())
    }

    /// Mutes the DAC and enters low-power standby. Call [`init`](Self::init) to leave it.
    pub async fn standby(&mut self) -> Result<(), I2C::Error> {
        self.set_mute(true).await?;
        self.write_all(&sequence::STANDBY).await
    }

    /// Reads a single register.
    pub async fn read_register<R>(&mut self) -> Result<R, I2C::Error>
    where
        R: ReadableRegister,
    {
        let mut buffer = [0];
        self.i2c
            .write_read(self.address, &[*R::REGISTER_ADDRESS], &mut buffer)
            .await?;
        Ok(R::from_bits(buffer[0]))
    }

    /// Writes a single register.
    pub async fn write_register<R>(&mut self, register: R) -> Result<(), I2C::Error>
    where
        R: WritableRegister,
    {
        self.i2c
            .write(self.address, &[*R::REGISTER_ADDRESS, register.to_bits()])
            .await
    }

    /// Reads, modifies and writes back a single register.
    pub async fn modify_register<R, F>(&mut self, f: F) -> Result<(), I2C::Error>
    where
        R: ReadableRegister + WritableRegister,
        F: FnOnce(R) -> R,
    {
        let register = self.read_register::<R>().await?;
        self.write_register(f(register)).await
    }

    async fn write_all(&mut self, writes: &[Write]) -> Result<(), I2C::Error> {
        for &(reg, value) in writes {
            self.i2c.write(self.address, &[reg.addr(), value]).await?;
        }

        Ok(())
    }
}
