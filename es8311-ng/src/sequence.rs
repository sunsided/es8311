//! Register write sequences shared by the blocking and the async driver.
//!
//! Each sequence is a list of `(register, value)` writes, in order. Where the Everest
//! documents give no order or value, the sequences follow Espressif's `esp_codec_dev`
//! ES8311 driver (v1.6.2); see the crate README for the sources of each step.

use crate::Config;
use crate::clock::ClockCoefficients;
use es8311_registers::RegisterAddress as Reg;
use es8311_registers::{
    AdcOversampling, AdcPower, Bias, ClockDivider, ClockPrescaler, ClockSwitch, DacControl,
    DacOversampling, DacPower, DacRamp, GpControl, GpioControl, OutputDrive, PowerUpTiming1,
    PowerUpTiming2, Reset, SdpIn, SdpOut, SystemPower, VmidSelect, Vsel,
};

/// A single register write.
pub(crate) type Write = (Reg, u8);

/// Asserts all block resets and stops the chip state machine (user guide 9.1).
pub(crate) const RESET: [Write; 1] = [(Reg::RESET, Reset::new().into_bits())];

/// Main clock on, with the serial port bit clock.
const MCLK_AND_BCLK: u8 = ClockSwitch::new()
    .with_mclk_on(true)
    .with_bclk_on(true)
    .into_bits();

/// All clocks on, with the main clock taken from the `MCLK` pin.
const ALL_CLOCKS: u8 = ClockSwitch::new()
    .with_mclk_on(true)
    .with_bclk_on(true)
    .with_clkadc_on(true)
    .with_clkdac_on(true)
    .with_anaclkadc_on(true)
    .with_anaclkdac_on(true)
    .into_bits();

/// DAC muted at the delta-sigma modulator and the DEM stage.
pub(crate) const DAC_MUTED: DacControl = DacControl::new()
    .with_dac_dsmmute(true)
    .with_dac_demmute(true);

/// Serial data routing without loopback: the DAC plays the serial input, and `ASDOUT`
/// carries only ADC data. `I2C_WL` is set as in `esp_codec_dev`.
const NO_LOOPBACK: u8 = GpioControl::new().with_i2c_wl(true).into_bits();

/// Analog circuits, references and DAC powered down (the register defaults).
const ANALOG_DOWN: u8 = SystemPower::new().into_bits();
const DAC_DOWN: u8 = DacPower::new().into_bits();

/// Takes the chip out of reset with the clocks, the serial port and the output driver
/// configured, the DAC muted at minimum volume, and all analog blocks powered down.
///
/// Releasing the resets and starting the state machine comes last (user guide 9.1); the
/// clocks are switched fully on after that, as in `esp_codec_dev`.
pub(crate) fn configure(config: &Config, clock: &ClockCoefficients) -> [Write; 23] {
    let prescaler = ClockPrescaler::new()
        .with_div_pre(clock.div_pre)
        .with_mult_pre(clock.mult_pre);
    let divider = ClockDivider::new()
        .with_div_clkadc(clock.div_clk)
        .with_div_clkdac(clock.div_clk);
    let dac_osr = DacOversampling::new().with_dac_osr(clock.dac_osr);
    let sdp_in = SdpIn::new()
        .with_sdp_in_wl(config.word_length)
        .with_sdp_in_fmt(config.format);
    // The ADC stays off; mute its serial output.
    let sdp_out = SdpOut::new()
        .with_sdp_out_mute(true)
        .with_sdp_out_wl(config.word_length)
        .with_sdp_out_fmt(config.format);
    let bias = Bias::new().with_ibias_sw(3);
    let vsel = Vsel::new().with_vsel(0x7F);
    let output = OutputDrive::new().with_reserved(0).with_hpsw(true);
    let running = Reset::new()
        .with_csm_on(true)
        .with_rst_dig(false)
        .with_rst_cmg(false)
        .with_rst_mst(false)
        .with_rst_adc_dig(false)
        .with_rst_dac_dig(false);

    [
        // `esp_codec_dev` writes this register twice: the first write after power-up
        // occasionally fails.
        (Reg::GPIO_44, NO_LOOPBACK),
        (Reg::GPIO_44, NO_LOOPBACK),
        (Reg::CLK_MANAGER_01, MCLK_AND_BCLK),
        (Reg::CLK_MANAGER_02, prescaler.into_bits()),
        (Reg::CLK_MANAGER_05, divider.into_bits()),
        (Reg::CLK_MANAGER_03, AdcOversampling::new().into_bits()),
        (Reg::CLK_MANAGER_04, dac_osr.into_bits()),
        (Reg::SYSTEM_0B, PowerUpTiming1::new().into_bits()),
        (
            Reg::SYSTEM_0C,
            PowerUpTiming2::new().with_pwrup_c(0).into_bits(),
        ),
        (Reg::SYSTEM_10, bias.into_bits()),
        (Reg::SYSTEM_11, vsel.into_bits()),
        (Reg::SDP_IN, sdp_in.into_bits()),
        (Reg::SDP_OUT, sdp_out.into_bits()),
        (Reg::SYSTEM_13, output.into_bits()),
        (Reg::DAC_31, DAC_MUTED.into_bits()),
        (Reg::DAC_32, 0x00),
        (Reg::DAC_37, DacRamp::new().into_bits()),
        (Reg::SYSTEM_12, DAC_DOWN),
        (Reg::SYSTEM_0E, AdcPower::new().into_bits()),
        (Reg::SYSTEM_0D, ANALOG_DOWN),
        (Reg::GP_45, GpControl::new().into_bits()),
        (Reg::RESET, running.into_bits()),
        (Reg::CLK_MANAGER_01, ALL_CLOCKS),
    ]
}

/// Powers up the analog circuits and the DAC. Leaves the mute state untouched.
pub(crate) const DAC_POWER_UP: [Write; 2] = [
    (
        Reg::SYSTEM_0D,
        SystemPower::new()
            .with_pdn_ana(false)
            .with_pdn_ibiasgen(false)
            .with_pdn_adcbiasgen(false)
            .with_pdn_adcvrefgen(false)
            .with_pdn_dacvrefgen(false)
            .with_pdn_vref(false)
            .with_vmidsel(VmidSelect::NormalCharge)
            .into_bits(),
    ),
    (
        Reg::SYSTEM_12,
        DacPower::new().with_pdn_dac(false).into_bits(),
    ),
];

/// Powers down the DAC and the analog circuits. Run after muting.
pub(crate) const DAC_POWER_DOWN: [Write; 2] =
    [(Reg::SYSTEM_12, DAC_DOWN), (Reg::SYSTEM_0D, ANALOG_DOWN)];

/// Low-power standby. Run after muting.
///
/// Powers down the ADC (with the modulator in reset, user guide 10.2), the DAC and the
/// analog circuits (9.2), asserts all resets with the state machine off (9.1), and stops
/// the clocks.
pub(crate) const STANDBY: [Write; 6] = [
    (Reg::DAC_32, 0x00),
    (
        Reg::SYSTEM_0E,
        AdcPower::new().with_rst_mod(true).into_bits(),
    ),
    (Reg::SYSTEM_12, DAC_DOWN),
    (Reg::SYSTEM_0D, ANALOG_DOWN),
    (Reg::RESET, Reset::new().into_bits()),
    (Reg::CLK_MANAGER_01, ClockSwitch::new().into_bits()),
];

#[cfg(test)]
mod tests {
    use super::*;
    use es8311_registers::{SerialFormat, WordLength};

    #[test]
    fn values() {
        assert_eq!(RESET[0].1, 0x1F);
        assert_eq!(MCLK_AND_BCLK, 0x30);
        assert_eq!(ALL_CLOCKS, 0x3F);
        assert_eq!(NO_LOOPBACK, 0x08);
        assert_eq!(DAC_MUTED.into_bits(), 0x60);
        assert_eq!(
            DAC_POWER_UP,
            [(Reg::SYSTEM_0D, 0x01), (Reg::SYSTEM_12, 0x00)]
        );
        assert_eq!(
            DAC_POWER_DOWN,
            [(Reg::SYSTEM_12, 0x02), (Reg::SYSTEM_0D, 0xFC)]
        );
        assert_eq!(STANDBY[1].1, 0x7A);
    }

    #[test]
    fn configure_48k_16bit() {
        let config = Config::new(48_000)
            .with_format(SerialFormat::I2s)
            .with_word_length(WordLength::Bits16);
        let clock = crate::clock::lookup(config.mclk_hz, config.sample_rate_hz).unwrap();
        let writes = configure(&config, clock);
        let value = |reg| writes.iter().rev().find(|w| w.0 == reg).unwrap().1;

        assert_eq!(value(Reg::CLK_MANAGER_02), 0x00);
        assert_eq!(value(Reg::CLK_MANAGER_05), 0x00);
        assert_eq!(value(Reg::CLK_MANAGER_03), 0x10);
        assert_eq!(value(Reg::CLK_MANAGER_04), 0x10);
        assert_eq!(value(Reg::SYSTEM_0C), 0x00);
        assert_eq!(value(Reg::SYSTEM_10), 0x1F);
        assert_eq!(value(Reg::SYSTEM_11), 0x7F);
        assert_eq!(value(Reg::SDP_IN), 0x0C);
        assert_eq!(value(Reg::SDP_OUT), 0x4C);
        assert_eq!(value(Reg::SYSTEM_13), 0x10);
        assert_eq!(value(Reg::RESET), 0x80);
        assert_eq!(writes[21], (Reg::RESET, 0x80));
        assert_eq!(writes[22], (Reg::CLK_MANAGER_01, 0x3F));
    }
}
