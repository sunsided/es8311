//! Value types used in register fields.

/// The serial audio data format, as set in [`SdpIn`](crate::SdpIn) and
/// [`SdpOut`](crate::SdpOut).
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
pub enum SerialFormat {
    /// I²S (default).
    I2s = 0b00,
    /// Left justified.
    LeftJustified = 0b01,
    /// Reserved.
    Reserved = 0b10,
    /// DSP/PCM mode. [`SdpIn::sdp_in_lrp`](crate::SdpIn) selects mode A or B.
    DspPcm = 0b11,
}

impl SerialFormat {
    /// Converts the value into its register representation.
    pub const fn into_bits(self) -> u8 {
        self as u8
    }

    /// Converts the two lowest bits of a register value into a format.
    pub const fn from_bits(value: u8) -> Self {
        match value & 0b11 {
            0b00 => Self::I2s,
            0b01 => Self::LeftJustified,
            0b10 => Self::Reserved,
            _ => Self::DspPcm,
        }
    }
}

/// The serial audio data word length, as set in [`SdpIn`](crate::SdpIn) and
/// [`SdpOut`](crate::SdpOut).
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
pub enum WordLength {
    /// 24 bits (default).
    Bits24 = 0,
    /// 20 bits.
    Bits20 = 1,
    /// 18 bits.
    Bits18 = 2,
    /// 16 bits.
    Bits16 = 3,
    /// 32 bits.
    Bits32 = 4,
    /// Undocumented value 5.
    Reserved5 = 5,
    /// Undocumented value 6.
    Reserved6 = 6,
    /// Undocumented value 7.
    Reserved7 = 7,
}

impl WordLength {
    /// Converts the value into its register representation.
    pub const fn into_bits(self) -> u8 {
        self as u8
    }

    /// Converts the three lowest bits of a register value into a word length.
    pub const fn from_bits(value: u8) -> Self {
        match value & 0b111 {
            0 => Self::Bits24,
            1 => Self::Bits20,
            2 => Self::Bits18,
            3 => Self::Bits16,
            4 => Self::Bits32,
            5 => Self::Reserved5,
            6 => Self::Reserved6,
            _ => Self::Reserved7,
        }
    }
}

/// The clock pre-multiplier, as set in [`ClockPrescaler`](crate::ClockPrescaler).
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
pub enum PreMultiplier {
    /// `dig_mclk = mclk_prediv × 1` (default).
    X1 = 0,
    /// `dig_mclk = mclk_prediv × 2`.
    X2 = 1,
    /// `dig_mclk = mclk_prediv × 4`.
    X4 = 2,
    /// `dig_mclk = mclk_prediv × 8`.
    X8 = 3,
}

impl PreMultiplier {
    /// Converts the value into its register representation.
    pub const fn into_bits(self) -> u8 {
        self as u8
    }

    /// Converts the two lowest bits of a register value into a multiplier.
    pub const fn from_bits(value: u8) -> Self {
        match value & 0b11 {
            0 => Self::X1,
            1 => Self::X2,
            2 => Self::X4,
            _ => Self::X8,
        }
    }

    /// The multiplication factor.
    pub const fn factor(self) -> u32 {
        1 << self as u32
    }
}

/// The VMID mode, as set in [`SystemPower`](crate::SystemPower).
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
pub enum VmidSelect {
    /// VMID powered down (default).
    PowerDown = 0,
    /// Start up VMID with normal speed charge.
    NormalCharge = 1,
    /// Normal VMID operation.
    Normal = 2,
    /// Start up VMID with fast speed charge.
    FastCharge = 3,
}

impl VmidSelect {
    /// Converts the value into its register representation.
    pub const fn into_bits(self) -> u8 {
        self as u8
    }

    /// Converts the two lowest bits of a register value into a VMID mode.
    pub const fn from_bits(value: u8) -> Self {
        match value & 0b11 {
            0 => Self::PowerDown,
            1 => Self::NormalCharge,
            2 => Self::Normal,
            _ => Self::FastCharge,
        }
    }
}

/// The data sent on `ASDOUT` (left + right slot), as set in
/// [`GpioControl`](crate::GpioControl).
///
/// The values with `DACL` or `DACR` feed the DAC input back to the serial output
/// ("digital feedback", user guide section 10.4).
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
pub enum AdcDataSelect {
    /// ADC + ADC (default).
    AdcAdc = 0,
    /// ADC + 0.
    AdcZero = 1,
    /// 0 + ADC.
    ZeroAdc = 2,
    /// 0 + 0.
    ZeroZero = 3,
    /// DAC left + ADC.
    DacLeftAdc = 4,
    /// ADC + DAC right.
    AdcDacRight = 5,
    /// DAC left + DAC right.
    DacLeftDacRight = 6,
    /// Not available.
    Reserved = 7,
}

impl AdcDataSelect {
    /// Converts the value into its register representation.
    pub const fn into_bits(self) -> u8 {
        self as u8
    }

    /// Converts the three lowest bits of a register value into a selection.
    pub const fn from_bits(value: u8) -> Self {
        match value & 0b111 {
            0 => Self::AdcAdc,
            1 => Self::AdcZero,
            2 => Self::ZeroAdc,
            3 => Self::ZeroZero,
            4 => Self::DacLeftAdc,
            5 => Self::AdcDacRight,
            6 => Self::DacLeftDacRight,
            _ => Self::Reserved,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrips() {
        for v in 0..4 {
            assert_eq!(SerialFormat::from_bits(v).into_bits(), v);
            assert_eq!(PreMultiplier::from_bits(v).into_bits(), v);
            assert_eq!(VmidSelect::from_bits(v).into_bits(), v);
        }

        for v in 0..8 {
            assert_eq!(WordLength::from_bits(v).into_bits(), v);
            assert_eq!(AdcDataSelect::from_bits(v).into_bits(), v);
        }
    }

    #[test]
    fn multiplier_factor() {
        assert_eq!(PreMultiplier::X1.factor(), 1);
        assert_eq!(PreMultiplier::X8.factor(), 8);
    }
}
