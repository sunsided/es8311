//! Clock coefficients for slave mode with the main clock on the `MCLK` pin.
//!
//! The datasheet does not list clock coefficients. The user guide (Rev 1.11, section 8.5)
//! gives the rules and a short table; the entries here follow the coefficient table of
//! Espressif's `esp_codec_dev` ES8311 driver (v1.6.2), restricted to the entries that meet
//! the user guide's rule for the DAC: the internal DAC clock must be at least 256 times
//! `LRCK` and a multiple of 16 times `LRCK`. Every entry runs the ADC in single speed mode
//! with the default `ADC_OSR` of 16, and uses the same divider for the ADC and the DAC.

use es8311_registers::PreMultiplier::{self, X1, X2, X4, X8};

/// Clock settings for one MCLK frequency and sample rate.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(crate) struct ClockCoefficients {
    /// The frequency on the `MCLK` pin, in Hz.
    pub mclk_hz: u32,
    /// The sample rate (`LRCK`), in Hz.
    pub sample_rate_hz: u32,
    /// `DIV_PRE`: the pre-divider minus one.
    pub div_pre: u8,
    /// `MULT_PRE`: the pre-multiplier.
    pub mult_pre: PreMultiplier,
    /// `DIV_CLKADC` and `DIV_CLKDAC`: the ADC and DAC clock divider minus one.
    pub div_clk: u8,
    /// `DAC_OSR`.
    pub dac_osr: u8,
}

const fn c(
    mclk_hz: u32,
    sample_rate_hz: u32,
    div_pre: u8,
    mult_pre: PreMultiplier,
    div_clk: u8,
    dac_osr: u8,
) -> ClockCoefficients {
    ClockCoefficients {
        mclk_hz,
        sample_rate_hz,
        div_pre,
        mult_pre,
        div_clk,
        dac_osr,
    }
}

/// The supported clock configurations.
const TABLE: [ClockCoefficients; 68] = [
    c(12_288_000, 8_000, 5, X1, 0, 32),
    c(18_432_000, 8_000, 2, X2, 2, 32),
    c(16_384_000, 8_000, 7, X1, 0, 32),
    c(8_192_000, 8_000, 3, X1, 0, 32),
    c(6_144_000, 8_000, 2, X1, 0, 32),
    c(4_096_000, 8_000, 1, X1, 0, 32),
    c(3_072_000, 8_000, 0, X1, 0, 32),
    c(2_048_000, 8_000, 0, X1, 0, 32),
    c(1_536_000, 8_000, 2, X4, 0, 32),
    c(1_024_000, 8_000, 0, X2, 0, 32),
    c(11_289_600, 11_025, 3, X1, 0, 32),
    c(5_644_800, 11_025, 1, X1, 0, 32),
    c(2_822_400, 11_025, 0, X1, 0, 32),
    c(1_411_200, 11_025, 0, X2, 0, 32),
    c(12_288_000, 12_000, 3, X1, 0, 32),
    c(6_144_000, 12_000, 1, X1, 0, 32),
    c(3_072_000, 12_000, 0, X1, 0, 32),
    c(1_536_000, 12_000, 0, X2, 0, 32),
    c(12_288_000, 16_000, 2, X1, 0, 32),
    c(18_432_000, 16_000, 2, X2, 2, 32),
    c(16_384_000, 16_000, 3, X1, 0, 32),
    c(8_192_000, 16_000, 1, X1, 0, 32),
    c(6_144_000, 16_000, 2, X2, 0, 32),
    c(4_096_000, 16_000, 0, X1, 0, 32),
    c(3_072_000, 16_000, 2, X4, 0, 32),
    c(2_048_000, 16_000, 0, X2, 0, 32),
    c(1_536_000, 16_000, 2, X8, 0, 32),
    c(1_024_000, 16_000, 0, X4, 0, 32),
    c(11_289_600, 22_050, 1, X1, 0, 16),
    c(5_644_800, 22_050, 0, X1, 0, 16),
    c(2_822_400, 22_050, 0, X2, 0, 16),
    c(1_411_200, 22_050, 0, X4, 0, 16),
    c(12_288_000, 24_000, 1, X1, 0, 16),
    c(18_432_000, 24_000, 2, X1, 0, 16),
    c(6_144_000, 24_000, 0, X1, 0, 16),
    c(3_072_000, 24_000, 0, X2, 0, 16),
    c(1_536_000, 24_000, 0, X4, 0, 16),
    c(12_288_000, 32_000, 2, X2, 0, 16),
    c(18_432_000, 32_000, 2, X4, 2, 16),
    c(16_384_000, 32_000, 1, X1, 0, 16),
    c(8_192_000, 32_000, 0, X1, 0, 16),
    c(6_144_000, 32_000, 2, X4, 0, 16),
    c(4_096_000, 32_000, 0, X2, 0, 16),
    c(3_072_000, 32_000, 2, X8, 0, 16),
    c(2_048_000, 32_000, 0, X4, 0, 16),
    c(1_024_000, 32_000, 0, X8, 0, 16),
    c(11_289_600, 44_100, 0, X1, 0, 16),
    c(5_644_800, 44_100, 0, X2, 0, 16),
    c(2_822_400, 44_100, 0, X4, 0, 16),
    c(1_411_200, 44_100, 0, X8, 0, 16),
    c(12_288_000, 48_000, 0, X1, 0, 16),
    c(18_432_000, 48_000, 2, X2, 0, 16),
    c(6_144_000, 48_000, 0, X2, 0, 16),
    c(3_072_000, 48_000, 0, X4, 0, 16),
    c(1_536_000, 48_000, 0, X8, 0, 16),
    c(12_288_000, 64_000, 2, X4, 0, 16),
    c(16_384_000, 64_000, 0, X1, 0, 16),
    c(8_192_000, 64_000, 0, X2, 0, 16),
    c(4_096_000, 64_000, 0, X4, 0, 16),
    c(2_048_000, 64_000, 0, X8, 0, 16),
    c(11_289_600, 88_200, 0, X2, 0, 16),
    c(5_644_800, 88_200, 0, X4, 0, 16),
    c(2_822_400, 88_200, 0, X8, 0, 16),
    c(24_576_000, 96_000, 1, X2, 0, 16),
    c(12_288_000, 96_000, 0, X2, 0, 16),
    c(18_432_000, 96_000, 2, X4, 0, 16),
    c(6_144_000, 96_000, 0, X4, 0, 16),
    c(3_072_000, 96_000, 0, X8, 0, 16),
];

/// Looks up the coefficients for an MCLK frequency and a sample rate.
pub(crate) fn lookup(mclk_hz: u32, sample_rate_hz: u32) -> Option<&'static ClockCoefficients> {
    TABLE
        .iter()
        .find(|c| c.mclk_hz == mclk_hz && c.sample_rate_hz == sample_rate_hz)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The internal DAC clock divided by `LRCK`.
    fn dac_ratio(c: &ClockCoefficients) -> u32 {
        let dig_mclk = c.mclk_hz / (u32::from(c.div_pre) + 1) * c.mult_pre.factor();
        dig_mclk / (u32::from(c.div_clk) + 1) / c.sample_rate_hz
    }

    #[test]
    fn entries_meet_the_dac_clock_rule() {
        for c in &TABLE {
            let ratio = dac_ratio(c);
            assert!(ratio >= 256 && ratio % 16 == 0, "{c:?}: {ratio}");
            assert!(c.div_pre < 8 && c.div_clk < 16 && c.dac_osr < 128, "{c:?}");
        }
    }

    #[test]
    fn entries_are_unique() {
        for (i, a) in TABLE.iter().enumerate() {
            assert!(
                TABLE[i + 1..]
                    .iter()
                    .all(|b| (a.mclk_hz, a.sample_rate_hz) != (b.mclk_hz, b.sample_rate_hz))
            );
        }
    }

    #[test]
    fn mclk_256_fs() {
        for rate in [
            8_000, 11_025, 12_000, 16_000, 22_050, 24_000, 32_000, 44_100, 48_000, 64_000, 96_000,
        ] {
            assert!(lookup(256 * rate, rate).is_some(), "{rate}");
        }

        // shortcut: no table entry for 88.2 kHz at 256 × fs (22.5792 MHz).
        assert!(lookup(256 * 88_200, 88_200).is_none());
    }

    /// Register values from the user guide's clock table (section 8.5) for the entries
    /// that are also in this table: `(MCLK, LRCK, reg 0x02, reg 0x05, reg 0x04)`.
    #[test]
    fn matches_user_guide_table() {
        for (mclk, rate, reg02, reg05, reg04) in [
            (12_288_000, 32_000, 0x48, 0x00, 0x10),
            (12_288_000, 48_000, 0x00, 0x00, 0x10),
            (18_432_000, 48_000, 0x48, 0x00, 0x10),
            (11_289_600, 44_100, 0x00, 0x00, 0x10),
        ] {
            let c = lookup(mclk, rate).unwrap();
            assert_eq!(c.div_pre << 5 | c.mult_pre.into_bits() << 3, reg02);
            assert_eq!(c.div_clk << 4 | c.div_clk, reg05);
            assert_eq!(c.dac_osr, reg04);
        }
    }
}
