//! # Everest Semiconductor ES8311 behavioral model
//!
//! A register-level model of the ES8311 audio codec, used in two places:
//!
//! - as a fake I²C bus ([`FakeI2c`]) in host tests of the `es8311` driver, and
//! - wrapped into a [Wokwi](https://wokwi.com) custom chip (`sim/wokwi-chip`).
//!
//! The model implements the documented I²C protocol (register address write, then a data
//! write or a repeated start and a data read), the register map `0x00`–`0x45` with its
//! power-on defaults, the read-only chip ID registers `0xFD`–`0xFF`, and the `CE` address
//! selection. [`Es8311Model::dac_output_enabled`] and
//! [`Es8311Model::loopback_enabled`] derive the state of the playback path from the
//! registers.
//!
//! ## Not modelled
//!
//! The audio signal path: no I²S data is read or produced, and the clock, filter,
//! volume, ALC and DRC settings only take effect as register values. The power-up
//! sequence timings (`0x0B`, `0x0C`) are not modelled.
//!
//! ## Assumptions
//!
//! Behavior that the datasheet (Revision 10.0) and the user guide (Rev 1.11) leave open:
//!
//! - Multi-byte transfers auto-increment the register address. The documents only show
//!   single-register transfers, which is all the driver uses.
//! - The reset bits in `0x00` do not change register contents; the datasheet states that
//!   `RST_DIG` resets the digital blocks "except control port block".
//! - The control port answers in every power state, including standby.
//! - Addresses outside the map (`0x46`–`0xFC`) read as `0x00` and ignore writes.
//! - The DAC output is enabled when the state machine runs (`CSM_ON`), the DAC and
//!   digital resets are released, `MCLK` and the DAC clocks are on, the analog circuits
//!   and the DAC are powered up, and neither the DAC (`0x31`) nor the serial input
//!   (`0x09`) is muted. The documents do not state which of these the chip requires.

#![no_std]
#![forbid(unsafe_code)]
#![deny(missing_docs)]

mod bus;

pub use bus::FakeI2c;

use es8311_registers::{
    ALTERNATE_DEVICE_ADDRESS, AdcDataSelect, ClockSwitch, DEFAULT_DEVICE_ADDRESS, DacControl,
    DacPower, GpioControl, RegisterAddress as Reg, Reset, SdpIn, SystemPower,
};

/// A behavioral model of the ES8311.
#[derive(Debug, Clone)]
pub struct Es8311Model {
    address: u8,
    regs: [u8; 256],
    pointer: u8,
    /// `true` until the first byte of a write transaction set the register pointer.
    expect_pointer: bool,
}

impl Default for Es8311Model {
    fn default() -> Self {
        Self::new()
    }
}

impl Es8311Model {
    /// Creates a model with `CE` low, listening on [`DEFAULT_DEVICE_ADDRESS`], after
    /// power-on reset.
    pub fn new() -> Self {
        Self::with_ce(false)
    }

    /// Creates a model with the specified `CE` pin level, after power-on reset.
    ///
    /// `CE` high selects [`ALTERNATE_DEVICE_ADDRESS`].
    pub fn with_ce(ce_high: bool) -> Self {
        let address = if ce_high {
            ALTERNATE_DEVICE_ADDRESS
        } else {
            DEFAULT_DEVICE_ADDRESS
        };

        let mut model = Self {
            address,
            regs: [0; 256],
            pointer: 0,
            expect_pointer: false,
        };

        model.power_on_reset();
        model
    }

    /// Restores all registers to their power-on defaults, as when `DVDD` comes up.
    pub fn power_on_reset(&mut self) {
        self.regs = [0; 256];

        for reg in Reg::ALL {
            self.regs[usize::from(reg.addr())] = reg.default_value();
        }

        self.pointer = 0;
        self.expect_pointer = false;
    }

    /// The 7-bit I²C address of the model.
    pub fn address(&self) -> u8 {
        self.address
    }

    /// Reads a register directly, bypassing the bus.
    pub fn register(&self, reg: Reg) -> u8 {
        self.regs[usize::from(reg.addr())]
    }

    /// Whether the DAC output is enabled; see the crate-level assumptions.
    pub fn dac_output_enabled(&self) -> bool {
        let reset = Reset::from(self.register(Reg::RESET));
        let clocks = ClockSwitch::from(self.register(Reg::CLK_MANAGER_01));
        let power = SystemPower::from(self.register(Reg::SYSTEM_0D));
        let dac = DacControl::from(self.register(Reg::DAC_31));

        reset.csm_on()
            && !reset.rst_dig()
            && !reset.rst_dac_dig()
            && clocks.mclk_on()
            && clocks.clkdac_on()
            && clocks.anaclkdac_on()
            && !power.pdn_ana()
            && !DacPower::from(self.register(Reg::SYSTEM_12)).pdn_dac()
            && !dac.dac_dsmmute()
            && !dac.dac_demmute()
            && !SdpIn::from(self.register(Reg::SDP_IN)).sdp_in_mute()
    }

    /// Whether the ADC feeds the DAC, or the DAC data is sent on `ASDOUT`.
    pub fn loopback_enabled(&self) -> bool {
        let routing = GpioControl::from(self.register(Reg::GPIO_44));

        routing.adc2dac_sel()
            || matches!(
                routing.adcdat_sel(),
                AdcDataSelect::DacLeftAdc
                    | AdcDataSelect::AdcDacRight
                    | AdcDataSelect::DacLeftDacRight
            )
    }

    /// Starts a transaction addressed to the model. Returns `true` to acknowledge.
    ///
    /// `is_read` selects a read (`true`) or write (`false`) transfer.
    pub fn i2c_start(&mut self, is_read: bool) -> bool {
        self.expect_pointer = !is_read;
        true
    }

    /// Writes a byte within a write transfer. Returns `true` to acknowledge.
    ///
    /// The first byte sets the register address; subsequent bytes are written to
    /// consecutive registers. Writes to read-only or unmapped registers are ignored.
    pub fn i2c_write(&mut self, byte: u8) -> bool {
        if self.expect_pointer {
            self.pointer = byte;
            self.expect_pointer = false;

            return true;
        }

        let addr = self.pointer;
        self.pointer = self.pointer.wrapping_add(1);

        if Reg::from_addr(addr).is_some_and(|reg| !reg.is_read_only()) {
            self.regs[usize::from(addr)] = byte;
        }

        true
    }

    /// Reads the byte at the register address within a read transfer and advances it.
    pub fn i2c_read(&mut self) -> u8 {
        let value = self.regs[usize::from(self.pointer)];
        self.pointer = self.pointer.wrapping_add(1);
        value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn write(model: &mut Es8311Model, bytes: &[u8]) -> bool {
        model.i2c_start(false) && bytes.iter().all(|&b| model.i2c_write(b))
    }

    fn read<const N: usize>(model: &mut Es8311Model, reg: u8) -> [u8; N] {
        assert!(write(model, &[reg]));
        assert!(model.i2c_start(true));
        core::array::from_fn(|_| model.i2c_read())
    }

    #[test]
    fn reports_chip_id() {
        let mut model = Es8311Model::new();
        assert_eq!(read::<3>(&mut model, 0xFD), [0x83, 0x11, 0x00]);
    }

    #[test]
    fn ce_selects_address() {
        assert_eq!(Es8311Model::new().address(), 0x18);
        assert_eq!(Es8311Model::with_ce(true).address(), 0x19);
    }

    #[test]
    fn power_on_defaults() {
        let mut model = Es8311Model::new();
        assert_eq!(read::<1>(&mut model, 0x00), [0x1F]);
        assert_eq!(read::<1>(&mut model, 0x0D), [0xFC]);
        assert_eq!(read::<1>(&mut model, 0x08), [0xFF]);
        assert!(!model.dac_output_enabled());
        assert!(!model.loopback_enabled());
    }

    #[test]
    fn read_only_and_unmapped_registers_ignore_writes() {
        let mut model = Es8311Model::new();
        assert!(write(&mut model, &[0xFD, 0x00]));
        assert!(write(&mut model, &[0x50, 0xAA]));
        assert_eq!(model.register(Reg::CHIP_ID1), 0x83);
        assert_eq!(read::<1>(&mut model, 0x50), [0x00]);
    }

    #[test]
    fn multi_byte_write_auto_increments() {
        let mut model = Es8311Model::new();
        assert!(write(&mut model, &[0x31, 0x60, 0xA9]));
        assert_eq!(model.register(Reg::DAC_31), 0x60);
        assert_eq!(model.register(Reg::DAC_32), 0xA9);
    }

    #[test]
    fn output_path() {
        let mut model = Es8311Model::new();
        for (reg, value) in [(0x00, 0x80), (0x01, 0x3F), (0x0D, 0x01), (0x12, 0x00)] {
            assert!(write(&mut model, &[reg, value]));
        }
        assert!(model.dac_output_enabled());

        assert!(write(&mut model, &[0x31, 0x60]));
        assert!(!model.dac_output_enabled());
    }

    #[test]
    fn loopback() {
        let mut model = Es8311Model::new();
        assert!(write(&mut model, &[0x44, 0x58]));
        assert!(model.loopback_enabled());
        assert!(write(&mut model, &[0x44, 0x80]));
        assert!(model.loopback_enabled());
        assert!(write(&mut model, &[0x44, 0x08]));
        assert!(!model.loopback_enabled());
    }
}
