//! Codec registers.
//!
//! Register addresses, names, bit layouts and defaults follow the ES8311 datasheet
//! (Revision 10.0, January 2021), section 8, unless noted otherwise. Bits the datasheet
//! does not describe are padding; their reset value is kept where the documented register
//! default sets them.

use crate::types::{AdcDataSelect, PreMultiplier, SerialFormat, VmidSelect, WordLength};
use bitfield_struct::bitfield;

/// Register addresses.
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
pub enum RegisterAddress {
    /// See [`Reset`].
    RESET = 0x00,
    /// See [`ClockSwitch`].
    CLK_MANAGER_01 = 0x01,
    /// See [`ClockPrescaler`].
    CLK_MANAGER_02 = 0x02,
    /// See [`AdcOversampling`].
    CLK_MANAGER_03 = 0x03,
    /// See [`DacOversampling`].
    CLK_MANAGER_04 = 0x04,
    /// See [`ClockDivider`].
    CLK_MANAGER_05 = 0x05,
    /// See [`BclkDivider`].
    CLK_MANAGER_06 = 0x06,
    /// See [`LrckDividerHigh`].
    CLK_MANAGER_07 = 0x07,
    /// See [`LrckDividerLow`].
    CLK_MANAGER_08 = 0x08,
    /// See [`SdpIn`].
    SDP_IN = 0x09,
    /// See [`SdpOut`].
    SDP_OUT = 0x0A,
    /// See [`PowerUpTiming1`].
    SYSTEM_0B = 0x0B,
    /// See [`PowerUpTiming2`].
    SYSTEM_0C = 0x0C,
    /// See [`SystemPower`].
    SYSTEM_0D = 0x0D,
    /// See [`AdcPower`].
    SYSTEM_0E = 0x0E,
    /// See [`LowPower`].
    SYSTEM_0F = 0x0F,
    /// See [`Bias`].
    SYSTEM_10 = 0x10,
    /// See [`Vsel`].
    SYSTEM_11 = 0x11,
    /// See [`DacPower`].
    SYSTEM_12 = 0x12,
    /// See [`OutputDrive`].
    SYSTEM_13 = 0x13,
    /// See [`AdcInput`].
    SYSTEM_14 = 0x14,
    /// See [`AdcRamp`].
    ADC_15 = 0x15,
    /// See [`AdcScale`].
    ADC_16 = 0x16,
    /// See [`AdcVolume`].
    ADC_17 = 0x17,
    /// See [`AlcControl`].
    ADC_18 = 0x18,
    /// See [`AlcLevel`].
    ADC_19 = 0x19,
    /// See [`AdcAutomute`].
    ADC_1A = 0x1A,
    /// See [`AdcHpf1`].
    ADC_1B = 0x1B,
    /// See [`AdcHpf2`].
    ADC_1C = 0x1C,
    /// See [`AdcEqB0Byte3`].
    ADC_EQ_1D = 0x1D,
    /// See [`AdcEqB0Byte2`].
    ADC_EQ_1E = 0x1E,
    /// See [`AdcEqB0Byte1`].
    ADC_EQ_1F = 0x1F,
    /// See [`AdcEqB0Byte0`].
    ADC_EQ_20 = 0x20,
    /// See [`AdcEqA1Byte3`].
    ADC_EQ_21 = 0x21,
    /// See [`AdcEqA1Byte2`].
    ADC_EQ_22 = 0x22,
    /// See [`AdcEqA1Byte1`].
    ADC_EQ_23 = 0x23,
    /// See [`AdcEqA1Byte0`].
    ADC_EQ_24 = 0x24,
    /// See [`AdcEqA2Byte3`].
    ADC_EQ_25 = 0x25,
    /// See [`AdcEqA2Byte2`].
    ADC_EQ_26 = 0x26,
    /// See [`AdcEqA2Byte1`].
    ADC_EQ_27 = 0x27,
    /// See [`AdcEqA2Byte0`].
    ADC_EQ_28 = 0x28,
    /// See [`AdcEqB1Byte3`].
    ADC_EQ_29 = 0x29,
    /// See [`AdcEqB1Byte2`].
    ADC_EQ_2A = 0x2A,
    /// See [`AdcEqB1Byte1`].
    ADC_EQ_2B = 0x2B,
    /// See [`AdcEqB1Byte0`].
    ADC_EQ_2C = 0x2C,
    /// See [`AdcEqB2Byte3`].
    ADC_EQ_2D = 0x2D,
    /// See [`AdcEqB2Byte2`].
    ADC_EQ_2E = 0x2E,
    /// See [`AdcEqB2Byte1`].
    ADC_EQ_2F = 0x2F,
    /// See [`AdcEqB2Byte0`].
    ADC_EQ_30 = 0x30,
    /// See [`DacControl`].
    DAC_31 = 0x31,
    /// See [`DacVolume`].
    DAC_32 = 0x32,
    /// See [`DacOffset`].
    DAC_33 = 0x33,
    /// See [`DrcControl`].
    DAC_34 = 0x34,
    /// See [`DrcLevel`].
    DAC_35 = 0x35,
    /// See [`Dac36`].
    DAC_36 = 0x36,
    /// See [`DacRamp`].
    DAC_37 = 0x37,
    /// See [`DacEqB0Byte3`].
    DAC_EQ_38 = 0x38,
    /// See [`DacEqB0Byte2`].
    DAC_EQ_39 = 0x39,
    /// See [`DacEqB0Byte1`].
    DAC_EQ_3A = 0x3A,
    /// See [`DacEqB0Byte0`].
    DAC_EQ_3B = 0x3B,
    /// See [`DacEqB1Byte3`].
    DAC_EQ_3C = 0x3C,
    /// See [`DacEqB1Byte2`].
    DAC_EQ_3D = 0x3D,
    /// See [`DacEqB1Byte1`].
    DAC_EQ_3E = 0x3E,
    /// See [`DacEqB1Byte0`].
    DAC_EQ_3F = 0x3F,
    /// See [`DacEqA1Byte3`].
    DAC_EQ_40 = 0x40,
    /// See [`DacEqA1Byte2`].
    DAC_EQ_41 = 0x41,
    /// See [`DacEqA1Byte1`].
    DAC_EQ_42 = 0x42,
    /// See [`DacEqA1Byte0`].
    DAC_EQ_43 = 0x43,
    /// See [`GpioControl`].
    GPIO_44 = 0x44,
    /// See [`GpControl`].
    GP_45 = 0x45,
    /// See [`ChipId1`]. Read-only.
    CHIP_ID1 = 0xFD,
    /// See [`ChipId2`]. Read-only.
    CHIP_ID2 = 0xFE,
    /// See [`ChipVersion`]. Read-only.
    CHIP_VER = 0xFF,
}

impl RegisterAddress {
    /// Every register in this map, in address order.
    pub const ALL: [Self; 73] = {
        let mut all = [Self::RESET; 73];
        let mut i = 0;

        while i < 70 {
            all[i] = match Self::from_addr(i as u8) {
                Some(reg) => reg,
                None => panic!("gap in the register map"),
            };
            i += 1;
        }

        all[70] = Self::CHIP_ID1;
        all[71] = Self::CHIP_ID2;
        all[72] = Self::CHIP_VER;
        all
    };

    /// Returns the address of a register.
    pub const fn addr(&self) -> u8 {
        *self as u8
    }

    /// Returns the register at an address, if it is part of this map.
    pub const fn from_addr(addr: u8) -> Option<Self> {
        use RegisterAddress::*;

        const LOW: [RegisterAddress; 70] = [
            RESET,
            CLK_MANAGER_01,
            CLK_MANAGER_02,
            CLK_MANAGER_03,
            CLK_MANAGER_04,
            CLK_MANAGER_05,
            CLK_MANAGER_06,
            CLK_MANAGER_07,
            CLK_MANAGER_08,
            SDP_IN,
            SDP_OUT,
            SYSTEM_0B,
            SYSTEM_0C,
            SYSTEM_0D,
            SYSTEM_0E,
            SYSTEM_0F,
            SYSTEM_10,
            SYSTEM_11,
            SYSTEM_12,
            SYSTEM_13,
            SYSTEM_14,
            ADC_15,
            ADC_16,
            ADC_17,
            ADC_18,
            ADC_19,
            ADC_1A,
            ADC_1B,
            ADC_1C,
            ADC_EQ_1D,
            ADC_EQ_1E,
            ADC_EQ_1F,
            ADC_EQ_20,
            ADC_EQ_21,
            ADC_EQ_22,
            ADC_EQ_23,
            ADC_EQ_24,
            ADC_EQ_25,
            ADC_EQ_26,
            ADC_EQ_27,
            ADC_EQ_28,
            ADC_EQ_29,
            ADC_EQ_2A,
            ADC_EQ_2B,
            ADC_EQ_2C,
            ADC_EQ_2D,
            ADC_EQ_2E,
            ADC_EQ_2F,
            ADC_EQ_30,
            DAC_31,
            DAC_32,
            DAC_33,
            DAC_34,
            DAC_35,
            DAC_36,
            DAC_37,
            DAC_EQ_38,
            DAC_EQ_39,
            DAC_EQ_3A,
            DAC_EQ_3B,
            DAC_EQ_3C,
            DAC_EQ_3D,
            DAC_EQ_3E,
            DAC_EQ_3F,
            DAC_EQ_40,
            DAC_EQ_41,
            DAC_EQ_42,
            DAC_EQ_43,
            GPIO_44,
            GP_45,
        ];

        match addr {
            0x00..=0x45 => Some(LOW[addr as usize]),
            0xFD => Some(CHIP_ID1),
            0xFE => Some(CHIP_ID2),
            0xFF => Some(CHIP_VER),
            _ => None,
        }
    }

    /// Whether the register is read-only (the chip ID registers).
    pub const fn is_read_only(&self) -> bool {
        matches!(self, Self::CHIP_ID1 | Self::CHIP_ID2 | Self::CHIP_VER)
    }

    /// The register value after power-on, as documented in the datasheet.
    pub const fn default_value(&self) -> u8 {
        use RegisterAddress::*;

        match self {
            RESET => 0x1F,
            CLK_MANAGER_03 | CLK_MANAGER_04 => 0x10,
            CLK_MANAGER_06 => 0x03,
            CLK_MANAGER_08 => 0xFF,
            SYSTEM_0C => 0x20,
            SYSTEM_0D => 0xFC,
            SYSTEM_0E => 0x6A,
            SYSTEM_10 => 0x13,
            SYSTEM_11 => 0x7C,
            SYSTEM_12 => 0x02,
            SYSTEM_13 => 0x40,
            SYSTEM_14 => 0x10,
            ADC_16 => 0x04,
            ADC_1B => 0x0C,
            ADC_1C => 0x4C,
            DAC_37 => 0x08,
            CHIP_ID1 => crate::ES8311_CHIP_ID1,
            CHIP_ID2 => crate::ES8311_CHIP_ID2,
            _ => 0x00,
        }
    }
}

impl From<RegisterAddress> for u8 {
    fn from(value: RegisterAddress) -> Self {
        value.addr()
    }
}

/// Declares a register whose eight bits form a single value.
macro_rules! value_register {
    ($(#[$meta:meta])* $type:ident, $addr:ident, $access:ident, $writable:ident) => {
        value_register!($(#[$meta])* $type, $addr, $access, $writable, 0);
    };

    ($(#[$meta:meta])* $type:ident, $addr:ident, $access:ident, $writable:ident, $default:expr) => {
        $(#[$meta])*
        #[bitfield(u8, order = Msb)]
        #[derive(PartialEq, Eq)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub struct $type {
            /// The register value.
            #[bits(8, access = $access, default = $default)]
            pub value: u8,
        }

        $writable!($type, RegisterAddress::$addr);
    };
}

/// Declares a register holding the 6 most significant bits `[29:24]` of an equalizer
/// coefficient.
macro_rules! eq_top_register {
    ($(#[$meta:meta])* $type:ident, $addr:ident) => {
        $(#[$meta])*
        #[bitfield(u8, order = Msb)]
        #[derive(PartialEq, Eq)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub struct $type {
            #[bits(2)]
            __: u8,

            /// Coefficient bits `[29:24]`.
            #[bits(6, access = RW)]
            pub value: u8,
        }

        writable_register!($type, RegisterAddress::$addr);
    };
}

/// [`RESET`](RegisterAddress::RESET) (00h)
#[bitfield(u8, order = Msb)]
#[derive(PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Reset {
    /// Chip state machine: `false` powers it down (default), `true` powers it on.
    #[bits(1, access = RW)]
    pub csm_on: bool,

    /// Serial port mode: `false` for slave (default), `true` for master.
    #[bits(1, access = RW)]
    pub msc: bool,

    /// Disables the power-up sequence when `true`.
    #[bits(1, access = RW)]
    pub seq_dis: bool,

    /// Resets the digital blocks except the control port (default).
    #[bits(1, access = RW, default = true)]
    pub rst_dig: bool,

    /// Resets the clock manager (default).
    #[bits(1, access = RW, default = true)]
    pub rst_cmg: bool,

    /// Resets the master block (default).
    #[bits(1, access = RW, default = true)]
    pub rst_mst: bool,

    /// Resets the ADC digital block (default).
    #[bits(1, access = RW, default = true)]
    pub rst_adc_dig: bool,

    /// Resets the DAC digital block (default).
    #[bits(1, access = RW, default = true)]
    pub rst_dac_dig: bool,
}

writable_register!(Reset, RegisterAddress::RESET);

/// [`CLK_MANAGER_01`](RegisterAddress::CLK_MANAGER_01) (01h)
///
/// The datasheet marks `anaclkadc_on` and `anaclkdac_on` as on by default, but gives the
/// register default as `0000 0000`. This map follows the register default.
#[bitfield(u8, order = Msb)]
#[derive(PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ClockSwitch {
    /// Main clock source: `false` for the `MCLK` pin (default), `true` for `BCLK`.
    #[bits(1, access = RW)]
    pub mclk_sel: bool,

    /// Inverts the main clock.
    #[bits(1, access = RW)]
    pub mclk_inv: bool,

    /// Enables the `MCLK` input.
    #[bits(1, access = RW)]
    pub mclk_on: bool,

    /// Enables the serial port bit clock.
    #[bits(1, access = RW)]
    pub bclk_on: bool,

    /// Enables the ADC digital clock.
    #[bits(1, access = RW)]
    pub clkadc_on: bool,

    /// Enables the DAC digital clock.
    #[bits(1, access = RW)]
    pub clkdac_on: bool,

    /// Enables the ADC analog clock.
    #[bits(1, access = RW)]
    pub anaclkadc_on: bool,

    /// Enables the DAC analog clock.
    #[bits(1, access = RW)]
    pub anaclkdac_on: bool,
}

writable_register!(ClockSwitch, RegisterAddress::CLK_MANAGER_01);

/// [`CLK_MANAGER_02`](RegisterAddress::CLK_MANAGER_02) (02h)
#[bitfield(u8, order = Msb)]
#[derive(PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ClockPrescaler {
    /// Pre-divider: `mclk_prediv = mclkin / (div_pre + 1)`.
    #[bits(3, access = RW)]
    pub div_pre: u8,

    /// Pre-multiplier: `dig_mclk = mclk_prediv × mult_pre`.
    #[bits(2, access = RW)]
    pub mult_pre: PreMultiplier,

    /// Clock doubler path: `false` for no DFF path (default), `true` for the DFF path.
    #[bits(1, access = RW)]
    pub pathsel: bool,

    /// Clock doubler delay cell: 0 for 5 ns (default), 1 for 10 ns, 2 and 3 for 15 ns.
    #[bits(2, access = RW)]
    pub delysel: u8,
}

writable_register!(ClockPrescaler, RegisterAddress::CLK_MANAGER_02);

/// [`CLK_MANAGER_03`](RegisterAddress::CLK_MANAGER_03) (03h)
#[bitfield(u8, order = Msb)]
#[derive(PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct AdcOversampling {
    #[bits(1)]
    __: u8,

    /// ADC speed mode: `false` for single speed (default), `true` for double speed.
    #[bits(1, access = RW)]
    pub adc_fsmode: bool,

    /// ADC delta-sigma oversampling rate: `4 × adc_osr × fs` in single speed,
    /// `2 × adc_osr × fs` in double speed. Values 0 to 14 are not used. Default: 16.
    #[bits(6, access = RW, default = 16)]
    pub adc_osr: u8,
}

writable_register!(AdcOversampling, RegisterAddress::CLK_MANAGER_03);

/// [`CLK_MANAGER_04`](RegisterAddress::CLK_MANAGER_04) (04h)
#[bitfield(u8, order = Msb)]
#[derive(PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct DacOversampling {
    #[bits(1)]
    __: u8,

    /// DAC oversampling rate: `4 × dac_osr × fs`. Values 0 to 14 are not used; 15 disables
    /// the equalizer. Default: 16.
    #[bits(7, access = RW, default = 16)]
    pub dac_osr: u8,
}

writable_register!(DacOversampling, RegisterAddress::CLK_MANAGER_04);

/// [`CLK_MANAGER_05`](RegisterAddress::CLK_MANAGER_05) (05h)
#[bitfield(u8, order = Msb)]
#[derive(PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ClockDivider {
    /// ADC clock divider: `adc_mclk = dig_mclk / (div_clkadc + 1)`.
    #[bits(4, access = RW)]
    pub div_clkadc: u8,

    /// DAC clock divider: `dac_mclk = dig_mclk / (div_clkdac + 1)`.
    #[bits(4, access = RW)]
    pub div_clkdac: u8,
}

writable_register!(ClockDivider, RegisterAddress::CLK_MANAGER_05);

/// [`CLK_MANAGER_06`](RegisterAddress::CLK_MANAGER_06) (06h)
#[bitfield(u8, order = Msb)]
#[derive(PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct BclkDivider {
    #[bits(1)]
    __: u8,

    /// Master mode: `false` outputs `BCLK` continuously (default), `true` stops it when
    /// the data transfer has finished.
    #[bits(1, access = RW)]
    pub bclk_con: bool,

    /// Inverts `BCLK`.
    #[bits(1, access = RW)]
    pub bclk_inv: bool,

    /// Master mode `BCLK` divider. 0 to 19 divide by `div_bclk + 1`; 20 to 31 divide by
    /// 22, 24, 25, 30, 32, 33, 34, 36, 44, 48, 66 and 72. Default: 3.
    #[bits(5, access = RW, default = 3)]
    pub div_bclk: u8,
}

writable_register!(BclkDivider, RegisterAddress::CLK_MANAGER_06);

/// [`CLK_MANAGER_07`](RegisterAddress::CLK_MANAGER_07) (07h)
#[bitfield(u8, order = Msb)]
#[derive(PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct LrckDividerHigh {
    #[bits(2)]
    __: u8,

    /// Tri-states the `BCLK` and `LRCK` outputs.
    #[bits(1, access = RW)]
    pub tri_blrck: bool,

    /// Tri-states the `ASDOUT` output.
    #[bits(1, access = RW)]
    pub tri_adcdat: bool,

    /// Master mode `LRCK` divider bits `[11:8]`: `LRCK = MCLK / (div_lrck + 1)`.
    #[bits(4, access = RW)]
    pub div_lrck_high: u8,
}

writable_register!(LrckDividerHigh, RegisterAddress::CLK_MANAGER_07);

value_register!(
    /// [`CLK_MANAGER_08`](RegisterAddress::CLK_MANAGER_08) (08h)
    ///
    /// Master mode `LRCK` divider bits `[7:0]`. Default: `0xFF`.
    LrckDividerLow, CLK_MANAGER_08, RW, writable_register, 0xFF
);

/// [`SDP_IN`](RegisterAddress::SDP_IN) (09h): the serial data input to the DAC.
#[bitfield(u8, order = Msb)]
#[derive(PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct SdpIn {
    /// DAC input channel: `false` for left (default), `true` for right.
    #[bits(1, access = RW)]
    pub sdp_in_sel: bool,

    /// Mutes the serial data input.
    #[bits(1, access = RW)]
    pub sdp_in_mute: bool,

    /// I²S and left justified: inverts the left/right polarity. DSP/PCM: `false` puts the
    /// MSB on the 2nd `BCLK` rising edge after `LRCK` rises (default), `true` on the 1st.
    #[bits(1, access = RW)]
    pub sdp_in_lrp: bool,

    /// Word length.
    #[bits(3, access = RW)]
    pub sdp_in_wl: WordLength,

    /// Data format.
    #[bits(2, access = RW)]
    pub sdp_in_fmt: SerialFormat,
}

writable_register!(SdpIn, RegisterAddress::SDP_IN);

/// [`SDP_OUT`](RegisterAddress::SDP_OUT) (0Ah): the serial data output from the ADC.
#[bitfield(u8, order = Msb)]
#[derive(PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct SdpOut {
    #[bits(1)]
    __: u8,

    /// Mutes the serial data output.
    #[bits(1, access = RW)]
    pub sdp_out_mute: bool,

    /// Polarity, as for [`SdpIn::sdp_in_lrp`].
    #[bits(1, access = RW)]
    pub sdp_out_lrp: bool,

    /// Word length.
    #[bits(3, access = RW)]
    pub sdp_out_wl: WordLength,

    /// Data format.
    #[bits(2, access = RW)]
    pub sdp_out_fmt: SerialFormat,
}

writable_register!(SdpOut, RegisterAddress::SDP_OUT);

/// [`SYSTEM_0B`](RegisterAddress::SYSTEM_0B) (0Bh)
#[bitfield(u8, order = Msb)]
#[derive(PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct PowerUpTiming1 {
    /// Power-up stage A duration: 0 to 31 span 21 µs to 232 ms at `LRCK` = 48 kHz.
    #[bits(5, access = RW)]
    pub pwrup_a: u8,

    /// Power-up stage B duration bits `[3:1]`.
    #[bits(3, access = RW)]
    pub pwrup_b_high: u8,
}

writable_register!(PowerUpTiming1, RegisterAddress::SYSTEM_0B);

/// [`SYSTEM_0C`](RegisterAddress::SYSTEM_0C) (0Ch)
#[bitfield(u8, order = Msb)]
#[derive(PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct PowerUpTiming2 {
    /// Power-up stage B duration bit `[0]`.
    #[bits(1, access = RW)]
    pub pwrup_b_low: bool,

    /// Power-up stage C duration: 0 to 31 span 21 µs to 234 ms at `LRCK` = 48 kHz.
    /// Default: `0x20`.
    #[bits(7, access = RW, default = 0x20)]
    pub pwrup_c: u8,
}

writable_register!(PowerUpTiming2, RegisterAddress::SYSTEM_0C);

/// [`SYSTEM_0D`](RegisterAddress::SYSTEM_0D) (0Dh): analog power management.
#[bitfield(u8, order = Msb)]
#[derive(PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct SystemPower {
    /// Powers down the analog circuits (default).
    #[bits(1, access = RW, default = true)]
    pub pdn_ana: bool,

    /// Powers down the analog bias circuits (default).
    #[bits(1, access = RW, default = true)]
    pub pdn_ibiasgen: bool,

    /// Powers down the analog ADC bias circuits (default).
    #[bits(1, access = RW, default = true)]
    pub pdn_adcbiasgen: bool,

    /// Powers down the analog ADC reference circuits (default).
    #[bits(1, access = RW, default = true)]
    pub pdn_adcvrefgen: bool,

    /// Powers down the analog DAC reference circuits (default).
    #[bits(1, access = RW, default = true)]
    pub pdn_dacvrefgen: bool,

    /// The datasheet describes `0` as "disable internal reference circuits" and `1` as
    /// "enable reference circuits" (default), despite the name.
    #[bits(1, access = RW, default = true)]
    pub pdn_vref: bool,

    /// VMID mode.
    #[bits(2, access = RW)]
    pub vmidsel: VmidSelect,
}

writable_register!(SystemPower, RegisterAddress::SYSTEM_0D);

/// [`SYSTEM_0E`](RegisterAddress::SYSTEM_0E) (0Eh): ADC analog power.
///
/// The default `0110 1010` sets the undocumented bit 1.
#[bitfield(u8, order = Msb)]
#[derive(PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct AdcPower {
    #[bits(1)]
    __: u8,

    /// Powers down the analog PGA (default).
    #[bits(1, access = RW, default = true)]
    pub pdn_pga: bool,

    /// Powers down the analog ADC modulator (default).
    #[bits(1, access = RW, default = true)]
    pub pdn_mod: bool,

    /// Resets the ADC modulator.
    #[bits(1, access = RW)]
    pub rst_mod: bool,

    /// Reference impedance: `false` for normal, `true` for low (default).
    #[bits(1, access = RW, default = true)]
    pub vroi: bool,

    /// Puts the internal reference voltage in low power mode.
    #[bits(1, access = RW)]
    pub lpvrefbuf: bool,

    /// Undocumented; set in the register default.
    #[bits(2, access = RW, default = 0b10)]
    pub reserved: u8,
}

writable_register!(AdcPower, RegisterAddress::SYSTEM_0E);

/// [`SYSTEM_0F`](RegisterAddress::SYSTEM_0F) (0Fh): low power modes.
///
/// Each bit selects a low power mode instead of the normal mode (default).
#[bitfield(u8, order = Msb)]
#[derive(PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct LowPower {
    /// Low power mode for the DAC.
    #[bits(1, access = RW)]
    pub lpdac: bool,

    /// Low power mode for the PGA.
    #[bits(1, access = RW)]
    pub lppga: bool,

    /// Low power mode for the PGA output.
    #[bits(1, access = RW)]
    pub lppgaout: bool,

    /// Low power mode for the ADC.
    #[bits(1, access = RW)]
    pub lpvcmmod: bool,

    /// Low power mode for the ADC reference.
    #[bits(1, access = RW)]
    pub lpadcvrp: bool,

    /// Low power mode for the DAC reference.
    #[bits(1, access = RW)]
    pub lpdacvrp: bool,

    /// Low power mode for the ADC.
    #[bits(1, access = RW)]
    pub lpflash: bool,

    /// Low power mode for the ADC.
    #[bits(1, access = RW)]
    pub lpint1: bool,
}

writable_register!(LowPower, RegisterAddress::SYSTEM_0F);

/// [`SYSTEM_10`](RegisterAddress::SYSTEM_10) (10h): bias and reference settings.
#[bitfield(u8, order = Msb)]
#[derive(PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Bias {
    /// Sync mode.
    #[bits(1, access = RW)]
    pub syncmode: bool,

    /// VMID level: 0 for `VDDA/2` (default), 1 to 3 for 75, 145 and 175 mV below.
    #[bits(2, access = RW)]
    pub vmidlow: u8,

    /// Higher DAC bias setting (default).
    #[bits(1, access = RW, default = true)]
    pub dac_ibias_sw: bool,

    /// Bias level 0 (default) to 3 (highest).
    #[bits(2, access = RW)]
    pub ibias_sw: u8,

    /// Turns off the internal reference voltage doubler (default).
    #[bits(1, access = RW, default = true)]
    pub vx2off: bool,

    /// `vx1`: `false` for 1.45 V, `true` for 1.65 V (default).
    #[bits(1, access = RW, default = true)]
    pub vx1sel: bool,
}

writable_register!(Bias, RegisterAddress::SYSTEM_10);

/// [`SYSTEM_11`](RegisterAddress::SYSTEM_11) (11h)
#[bitfield(u8, order = Msb)]
#[derive(PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Vsel {
    #[bits(1)]
    __: u8,

    /// Internal use. Default: `0x7C`.
    #[bits(7, access = RW, default = 0x7C)]
    pub vsel: u8,
}

writable_register!(Vsel, RegisterAddress::SYSTEM_11);

/// [`SYSTEM_12`](RegisterAddress::SYSTEM_12) (12h): DAC power.
#[bitfield(u8, order = Msb)]
#[derive(PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct DacPower {
    #[bits(6)]
    __: u8,

    /// Powers down the DAC (default).
    #[bits(1, access = RW, default = true)]
    pub pdn_dac: bool,

    /// Enables the internal reference circuits for the DAC output.
    #[bits(1, access = RW)]
    pub enrefr: bool,
}

writable_register!(DacPower, RegisterAddress::SYSTEM_12);

/// [`SYSTEM_13`](RegisterAddress::SYSTEM_13) (13h): output driver.
///
/// The default `0100 0000` sets the undocumented bit 6.
#[bitfield(u8, order = Msb)]
#[derive(PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct OutputDrive {
    /// Undocumented bits `[7:5]`; bit 6 is set in the register default.
    #[bits(3, access = RW, default = 0b010)]
    pub reserved: u8,

    /// Output driver: `false` for line out (default), `true` for the headphone driver.
    #[bits(1, access = RW)]
    pub hpsw: bool,

    #[bits(4)]
    __: u8,
}

writable_register!(OutputDrive, RegisterAddress::SYSTEM_13);

/// [`SYSTEM_14`](RegisterAddress::SYSTEM_14) (14h): ADC input selection and PGA gain.
///
/// The datasheet documents `linsel` as bit 4; the user guide (Rev 1.11) documents
/// `LINSEL` as bits `[5:4]` with a second input pair the ES8311 does not have. This map
/// follows the datasheet.
#[bitfield(u8, order = Msb)]
#[derive(PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct AdcInput {
    #[bits(1)]
    __: u8,

    /// Selects the digital microphone, with data from `MIC1P`.
    #[bits(1, access = RW)]
    pub dmic_on: bool,

    #[bits(1)]
    __: u8,

    /// Selects the `MIC1P`–`MIC1N` input (default).
    #[bits(1, access = RW, default = true)]
    pub linsel: bool,

    /// PGA gain in 3 dB steps: 0 for 0 dB to 10 for 30 dB.
    #[bits(4, access = RW)]
    pub pga_gain: u8,
}

writable_register!(AdcInput, RegisterAddress::SYSTEM_14);

/// [`ADC_15`](RegisterAddress::ADC_15) (15h)
#[bitfield(u8, order = Msb)]
#[derive(PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct AdcRamp {
    /// ADC volume ramp rate: 0 disables the soft ramp, `n` ramps 0.25 dB per `2^(n+1)`
    /// `LRCK` cycles.
    #[bits(4, access = RW)]
    pub adc_ramprate: u8,

    #[bits(3)]
    __: u8,

    /// Digital microphone data latch: `false` at the positive clock edge, `true` at the
    /// negative clock edge.
    #[bits(1, access = RW)]
    pub dmic_sense: bool,
}

writable_register!(AdcRamp, RegisterAddress::ADC_15);

/// [`ADC_16`](RegisterAddress::ADC_16) (16h)
#[bitfield(u8, order = Msb)]
#[derive(PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct AdcScale {
    #[bits(2)]
    __: u8,

    /// Synchronizes the filter counter with `LRCK`: `false` for non-standard audio clocks,
    /// `true` for standard audio clocks.
    #[bits(1, access = RW)]
    pub adc_sync: bool,

    /// Inverts the ADC polarity.
    #[bits(1, access = RW)]
    pub adc_inv: bool,

    /// Clears the ADC RAM while `LRCK`/`adc_mclk` is active.
    #[bits(1, access = RW)]
    pub adc_ramclr: bool,

    /// ADC gain scale-up in 6 dB steps: 0 for 0 dB to 7 for 42 dB. Default: 4 (24 dB).
    #[bits(3, access = RW, default = 4)]
    pub adc_scale: u8,
}

writable_register!(AdcScale, RegisterAddress::ADC_16);

value_register!(
    /// [`ADC_17`](RegisterAddress::ADC_17) (17h)
    ///
    /// ADC volume in 0.5 dB steps: `0x00` for -95.5 dB (default), `0xBF` for 0 dB, `0xFF`
    /// for +32 dB. With the ALC enabled, this is the maximum gain.
    AdcVolume, ADC_17, RW, writable_register
);

/// [`ADC_18`](RegisterAddress::ADC_18) (18h)
#[bitfield(u8, order = Msb)]
#[derive(PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct AlcControl {
    /// Enables the automatic level control.
    #[bits(1, access = RW)]
    pub alc_en: bool,

    /// Enables the ADC automute.
    #[bits(1, access = RW)]
    pub adc_automute_en: bool,

    #[bits(2)]
    __: u8,

    /// ALC window size: `n` for 0.25 dB per `2^(n+1)` `LRCK` cycles.
    #[bits(4, access = RW)]
    pub alc_winsize: u8,
}

writable_register!(AlcControl, RegisterAddress::ADC_18);

/// [`ADC_19`](RegisterAddress::ADC_19) (19h)
///
/// Levels 0 to 15 map to -30.1, -24.1, -20.6, -18.1, -16.1, -14.5, -13.2, -12.0, -11.0,
/// -10.1, -9.3, -8.5, -7.8, -7.2, -6.6 and -6.0 dB.
#[bitfield(u8, order = Msb)]
#[derive(PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct AlcLevel {
    /// ALC target maximum level.
    #[bits(4, access = RW)]
    pub alc_maxlevel: u8,

    /// ALC target minimum level.
    #[bits(4, access = RW)]
    pub alc_minlevel: u8,
}

writable_register!(AlcLevel, RegisterAddress::ADC_19);

/// [`ADC_1A`](RegisterAddress::ADC_1A) (1Ah)
#[bitfield(u8, order = Msb)]
#[derive(PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct AdcAutomute {
    /// Automute window: `2048 × (n + 1)` samples.
    #[bits(4, access = RW)]
    pub adc_automute_ws: u8,

    /// Automute noise gate: 0 to 7 for -96 to -54 dB in 6 dB steps, 8 to 15 for -51 to
    /// -30 dB in 3 dB steps.
    #[bits(4, access = RW)]
    pub adc_automute_ng: u8,
}

writable_register!(AdcAutomute, RegisterAddress::ADC_1A);

/// [`ADC_1B`](RegisterAddress::ADC_1B) (1Bh)
#[bitfield(u8, order = Msb)]
#[derive(PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct AdcHpf1 {
    /// Automute output gain: `n × -4` dB.
    #[bits(3, access = RW)]
    pub adc_automute_vol: u8,

    /// ADC high-pass filter stage 1 coefficient. Default: `0x0C`.
    #[bits(5, access = RW, default = 0x0C)]
    pub adc_hpfs1: u8,
}

writable_register!(AdcHpf1, RegisterAddress::ADC_1B);

/// [`ADC_1C`](RegisterAddress::ADC_1C) (1Ch)
#[bitfield(u8, order = Msb)]
#[derive(PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct AdcHpf2 {
    #[bits(1)]
    __: u8,

    /// Bypasses the ADC equalizer (default).
    #[bits(1, access = RW, default = true)]
    pub adc_eqbypass: bool,

    /// High-pass filter: `false` freezes the offset, `true` for a dynamic HPF.
    #[bits(1, access = RW)]
    pub adc_hpf: bool,

    /// ADC high-pass filter stage 2 coefficient. Default: `0x0C`.
    #[bits(5, access = RW, default = 0x0C)]
    pub adc_hpfs2: u8,
}

writable_register!(AdcHpf2, RegisterAddress::ADC_1C);

eq_top_register!(
    /// [`ADC_EQ_1D`](RegisterAddress::ADC_EQ_1D) (1Dh): ADC equalizer `B0[29:24]`.
    AdcEqB0Byte3, ADC_EQ_1D
);
value_register!(
    /// [`ADC_EQ_1E`](RegisterAddress::ADC_EQ_1E) (1Eh): ADC equalizer `B0[23:16]`.
    AdcEqB0Byte2, ADC_EQ_1E, RW, writable_register
);
value_register!(
    /// [`ADC_EQ_1F`](RegisterAddress::ADC_EQ_1F) (1Fh): ADC equalizer `B0[15:8]`.
    AdcEqB0Byte1, ADC_EQ_1F, RW, writable_register
);
value_register!(
    /// [`ADC_EQ_20`](RegisterAddress::ADC_EQ_20) (20h): ADC equalizer `B0[7:0]`.
    AdcEqB0Byte0, ADC_EQ_20, RW, writable_register
);
value_register!(
    /// [`ADC_EQ_21`](RegisterAddress::ADC_EQ_21) (21h): ADC equalizer `A1[29:24]`.
    ///
    /// The datasheet lists bits `[7:0]` for this register.
    AdcEqA1Byte3, ADC_EQ_21, RW, writable_register
);
value_register!(
    /// [`ADC_EQ_22`](RegisterAddress::ADC_EQ_22) (22h): ADC equalizer `A1[23:16]`.
    AdcEqA1Byte2, ADC_EQ_22, RW, writable_register
);
value_register!(
    /// [`ADC_EQ_23`](RegisterAddress::ADC_EQ_23) (23h): ADC equalizer `A1[15:8]`.
    AdcEqA1Byte1, ADC_EQ_23, RW, writable_register
);
value_register!(
    /// [`ADC_EQ_24`](RegisterAddress::ADC_EQ_24) (24h): ADC equalizer `A1[7:0]`.
    AdcEqA1Byte0, ADC_EQ_24, RW, writable_register
);
value_register!(
    /// [`ADC_EQ_25`](RegisterAddress::ADC_EQ_25) (25h): ADC equalizer `A2[29:24]`.
    ///
    /// The datasheet lists bits `[7:0]` for this register.
    AdcEqA2Byte3, ADC_EQ_25, RW, writable_register
);
value_register!(
    /// [`ADC_EQ_26`](RegisterAddress::ADC_EQ_26) (26h): ADC equalizer `A2[23:16]`.
    AdcEqA2Byte2, ADC_EQ_26, RW, writable_register
);
value_register!(
    /// [`ADC_EQ_27`](RegisterAddress::ADC_EQ_27) (27h): ADC equalizer `A2[15:8]`.
    AdcEqA2Byte1, ADC_EQ_27, RW, writable_register
);
value_register!(
    /// [`ADC_EQ_28`](RegisterAddress::ADC_EQ_28) (28h): ADC equalizer `A2[7:0]`.
    AdcEqA2Byte0, ADC_EQ_28, RW, writable_register
);
value_register!(
    /// [`ADC_EQ_29`](RegisterAddress::ADC_EQ_29) (29h): ADC equalizer `B1[29:24]`.
    ///
    /// The datasheet lists bits `[7:0]` for this register.
    AdcEqB1Byte3, ADC_EQ_29, RW, writable_register
);
value_register!(
    /// [`ADC_EQ_2A`](RegisterAddress::ADC_EQ_2A) (2Ah): ADC equalizer `B1[23:16]`.
    AdcEqB1Byte2, ADC_EQ_2A, RW, writable_register
);
value_register!(
    /// [`ADC_EQ_2B`](RegisterAddress::ADC_EQ_2B) (2Bh): ADC equalizer `B1[15:8]`.
    AdcEqB1Byte1, ADC_EQ_2B, RW, writable_register
);
value_register!(
    /// [`ADC_EQ_2C`](RegisterAddress::ADC_EQ_2C) (2Ch): ADC equalizer `B1[7:0]`.
    AdcEqB1Byte0, ADC_EQ_2C, RW, writable_register
);
value_register!(
    /// [`ADC_EQ_2D`](RegisterAddress::ADC_EQ_2D) (2Dh): ADC equalizer `B2[29:24]`.
    ///
    /// The datasheet lists bits `[7:0]` for this register.
    AdcEqB2Byte3, ADC_EQ_2D, RW, writable_register
);
value_register!(
    /// [`ADC_EQ_2E`](RegisterAddress::ADC_EQ_2E) (2Eh): ADC equalizer `B2[23:16]`.
    AdcEqB2Byte2, ADC_EQ_2E, RW, writable_register
);
value_register!(
    /// [`ADC_EQ_2F`](RegisterAddress::ADC_EQ_2F) (2Fh): ADC equalizer `B2[15:8]`.
    AdcEqB2Byte1, ADC_EQ_2F, RW, writable_register
);
value_register!(
    /// [`ADC_EQ_30`](RegisterAddress::ADC_EQ_30) (30h): ADC equalizer `B2[7:0]`.
    AdcEqB2Byte0, ADC_EQ_30, RW, writable_register
);

/// [`DAC_31`](RegisterAddress::DAC_31) (31h): DAC mute and data control.
#[bitfield(u8, order = Msb)]
#[derive(PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct DacControl {
    /// DSM mute target: `false` mutes to 8 (default), `true` to 7/9.
    #[bits(1, access = RW)]
    pub dac_dsmmute_to: bool,

    /// Mutes the DAC delta-sigma modulator.
    #[bits(1, access = RW)]
    pub dac_dsmmute: bool,

    /// Mutes the DAC dynamic element matching.
    #[bits(1, access = RW)]
    pub dac_demmute: bool,

    /// Inverts the DAC data phase by 180 degrees.
    #[bits(1, access = RW)]
    pub dac_inv: bool,

    /// Clears the DAC RAM while `LRCK`/`dac_mclk` is active.
    #[bits(1, access = RW)]
    pub dac_ramclr: bool,

    /// Turns the DAC DSM dither off.
    #[bits(1, access = RW)]
    pub dac_dsmdith_off: bool,

    #[bits(2)]
    __: u8,
}

writable_register!(DacControl, RegisterAddress::DAC_31);

value_register!(
    /// [`DAC_32`](RegisterAddress::DAC_32) (32h)
    ///
    /// DAC volume in 0.5 dB steps: `0x00` for -95.5 dB (default), `0xBF` for 0 dB, `0xFF`
    /// for +32 dB. With the DRC enabled, this is the maximum gain.
    DacVolume, DAC_32, RW, writable_register
);

value_register!(
    /// [`DAC_33`](RegisterAddress::DAC_33) (33h): DAC offset.
    DacOffset, DAC_33, RW, writable_register
);

/// [`DAC_34`](RegisterAddress::DAC_34) (34h)
#[bitfield(u8, order = Msb)]
#[derive(PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct DrcControl {
    /// Enables the dynamic range control.
    #[bits(1, access = RW)]
    pub drc_en: bool,

    #[bits(3)]
    __: u8,

    /// DRC window size: `n` for 0.25 dB per `2^(n+1)` `LRCK` cycles.
    #[bits(4, access = RW)]
    pub drc_winsize: u8,
}

writable_register!(DrcControl, RegisterAddress::DAC_34);

/// [`DAC_35`](RegisterAddress::DAC_35) (35h)
///
/// Levels map to dB as in [`AlcLevel`].
#[bitfield(u8, order = Msb)]
#[derive(PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct DrcLevel {
    /// DRC target maximum level.
    #[bits(4, access = RW)]
    pub drc_maxlevel: u8,

    /// DRC target minimum level.
    #[bits(4, access = RW)]
    pub drc_minlevel: u8,
}

writable_register!(DrcLevel, RegisterAddress::DAC_35);

value_register!(
    /// [`DAC_36`](RegisterAddress::DAC_36) (36h)
    ///
    /// Listed in the datasheet with default `0x00`, but without bit descriptions.
    Dac36, DAC_36, RW, writable_register
);

/// [`DAC_37`](RegisterAddress::DAC_37) (37h)
#[bitfield(u8, order = Msb)]
#[derive(PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct DacRamp {
    /// DAC volume/DRC ramp rate: 0 disables the soft ramp (default), `n` ramps 0.25 dB
    /// per `2^(n+1)` `LRCK` cycles.
    #[bits(4, access = RW)]
    pub dac_ramprate: u8,

    /// Bypasses the DAC equalizer. The register default `0000 1000` sets this bit,
    /// although the bit description gives `0` (enabled) as default.
    #[bits(1, access = RW, default = true)]
    pub dac_eqbypass: bool,

    #[bits(3)]
    __: u8,
}

writable_register!(DacRamp, RegisterAddress::DAC_37);

eq_top_register!(
    /// [`DAC_EQ_38`](RegisterAddress::DAC_EQ_38) (38h): DAC equalizer `B0[29:24]`.
    DacEqB0Byte3, DAC_EQ_38
);
value_register!(
    /// [`DAC_EQ_39`](RegisterAddress::DAC_EQ_39) (39h): DAC equalizer `B0[23:16]`.
    DacEqB0Byte2, DAC_EQ_39, RW, writable_register
);
value_register!(
    /// [`DAC_EQ_3A`](RegisterAddress::DAC_EQ_3A) (3Ah): DAC equalizer `B0[15:8]`.
    DacEqB0Byte1, DAC_EQ_3A, RW, writable_register
);
value_register!(
    /// [`DAC_EQ_3B`](RegisterAddress::DAC_EQ_3B) (3Bh): DAC equalizer `B0[7:0]`.
    DacEqB0Byte0, DAC_EQ_3B, RW, writable_register
);
value_register!(
    /// [`DAC_EQ_3C`](RegisterAddress::DAC_EQ_3C) (3Ch): DAC equalizer `B1[29:24]`.
    ///
    /// The datasheet lists bits `[7:0]` for this register.
    DacEqB1Byte3, DAC_EQ_3C, RW, writable_register
);
value_register!(
    /// [`DAC_EQ_3D`](RegisterAddress::DAC_EQ_3D) (3Dh): DAC equalizer `B1[23:16]`.
    DacEqB1Byte2, DAC_EQ_3D, RW, writable_register
);
value_register!(
    /// [`DAC_EQ_3E`](RegisterAddress::DAC_EQ_3E) (3Eh): DAC equalizer `B1[15:8]`.
    DacEqB1Byte1, DAC_EQ_3E, RW, writable_register
);
value_register!(
    /// [`DAC_EQ_3F`](RegisterAddress::DAC_EQ_3F) (3Fh): DAC equalizer `B1[7:0]`.
    DacEqB1Byte0, DAC_EQ_3F, RW, writable_register
);
value_register!(
    /// [`DAC_EQ_40`](RegisterAddress::DAC_EQ_40) (40h): DAC equalizer `A1[29:24]`.
    ///
    /// The datasheet lists bits `[7:0]` for this register.
    DacEqA1Byte3, DAC_EQ_40, RW, writable_register
);
value_register!(
    /// [`DAC_EQ_41`](RegisterAddress::DAC_EQ_41) (41h): DAC equalizer `A1[23:16]`.
    DacEqA1Byte2, DAC_EQ_41, RW, writable_register
);
value_register!(
    /// [`DAC_EQ_42`](RegisterAddress::DAC_EQ_42) (42h): DAC equalizer `A1[15:8]`.
    DacEqA1Byte1, DAC_EQ_42, RW, writable_register
);
value_register!(
    /// [`DAC_EQ_43`](RegisterAddress::DAC_EQ_43) (43h): DAC equalizer `A1[7:0]`.
    DacEqA1Byte0, DAC_EQ_43, RW, writable_register
);

/// [`GPIO_44`](RegisterAddress::GPIO_44) (44h): data routing.
#[bitfield(u8, order = Msb)]
#[derive(PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct GpioControl {
    /// DAC data source: `false` for the serial port (default), `true` for the ADC
    /// (ADC-to-DAC loopback).
    #[bits(1, access = RW)]
    pub adc2dac_sel: bool,

    /// The data sent on `ASDOUT`.
    #[bits(3, access = RW)]
    pub adcdat_sel: AdcDataSelect,

    /// Internal use.
    #[bits(1, access = RW)]
    pub i2c_wl: bool,

    /// Internal use.
    #[bits(3, access = RW)]
    pub gpio_sel: u8,
}

writable_register!(GpioControl, RegisterAddress::GPIO_44);

/// [`GP_45`](RegisterAddress::GP_45) (45h)
#[bitfield(u8, order = Msb)]
#[derive(PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct GpControl {
    /// Internal use.
    #[bits(4, access = RW)]
    pub forcecsm: u8,

    /// Internal use.
    #[bits(1, access = RW)]
    pub adc_dly_sel: bool,

    /// Internal use.
    #[bits(1, access = RW)]
    pub dac_dly_sel: bool,

    /// Internal use.
    #[bits(1, access = RW)]
    pub dac_autochn: bool,

    /// Turns the `BCLK`/`LRCK` pull-ups off. `false` leaves them on (default).
    #[bits(1, access = RW)]
    pub pullup_se: bool,
}

writable_register!(GpControl, RegisterAddress::GP_45);

value_register!(
    /// [`CHIP_ID1`](RegisterAddress::CHIP_ID1) (FDh)
    ///
    /// Chip ID, `0x83`. See [`ES8311_CHIP_ID1`](crate::ES8311_CHIP_ID1).
    ChipId1, CHIP_ID1, RO, readable_register, crate::ES8311_CHIP_ID1
);

value_register!(
    /// [`CHIP_ID2`](RegisterAddress::CHIP_ID2) (FEh)
    ///
    /// Chip ID, `0x11`. See [`ES8311_CHIP_ID2`](crate::ES8311_CHIP_ID2).
    ChipId2, CHIP_ID2, RO, readable_register, crate::ES8311_CHIP_ID2
);

value_register!(
    /// [`CHIP_VER`](RegisterAddress::CHIP_VER) (FFh)
    ///
    /// Chip version, `0x00`.
    ChipVersion, CHIP_VER, RO, readable_register
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ReadableRegister;

    fn default_of<R: ReadableRegister + Default + Into<u8>>() -> (u8, u8) {
        (*R::REGISTER_ADDRESS, R::default().into())
    }

    #[test]
    fn struct_defaults_match_datasheet() {
        for (addr, value) in [
            default_of::<Reset>(),
            default_of::<ClockSwitch>(),
            default_of::<ClockPrescaler>(),
            default_of::<AdcOversampling>(),
            default_of::<DacOversampling>(),
            default_of::<ClockDivider>(),
            default_of::<BclkDivider>(),
            default_of::<LrckDividerHigh>(),
            default_of::<LrckDividerLow>(),
            default_of::<SdpIn>(),
            default_of::<SdpOut>(),
            default_of::<PowerUpTiming1>(),
            default_of::<PowerUpTiming2>(),
            default_of::<SystemPower>(),
            default_of::<AdcPower>(),
            default_of::<LowPower>(),
            default_of::<Bias>(),
            default_of::<Vsel>(),
            default_of::<DacPower>(),
            default_of::<OutputDrive>(),
            default_of::<AdcInput>(),
            default_of::<AdcRamp>(),
            default_of::<AdcScale>(),
            default_of::<AdcVolume>(),
            default_of::<AlcControl>(),
            default_of::<AlcLevel>(),
            default_of::<AdcAutomute>(),
            default_of::<AdcHpf1>(),
            default_of::<AdcHpf2>(),
            default_of::<AdcEqB0Byte3>(),
            default_of::<DacControl>(),
            default_of::<DacVolume>(),
            default_of::<DrcControl>(),
            default_of::<DrcLevel>(),
            default_of::<DacRamp>(),
            default_of::<DacEqB0Byte3>(),
            default_of::<GpioControl>(),
            default_of::<GpControl>(),
            default_of::<ChipId1>(),
            default_of::<ChipId2>(),
            default_of::<ChipVersion>(),
        ] {
            let reg = RegisterAddress::from_addr(addr).unwrap();
            assert_eq!(value, reg.default_value(), "{reg:?}");
        }
    }

    #[test]
    fn address_map() {
        for (i, reg) in RegisterAddress::ALL.iter().enumerate() {
            assert_eq!(RegisterAddress::from_addr(reg.addr()), Some(*reg));

            if i > 0 {
                assert!(RegisterAddress::ALL[i - 1].addr() < reg.addr());
            }
        }

        assert_eq!(RegisterAddress::from_addr(0x46), None);
        assert_eq!(RegisterAddress::from_addr(0xFC), None);
        assert!(RegisterAddress::CHIP_ID1.is_read_only());
        assert!(!RegisterAddress::DAC_32.is_read_only());
    }

    #[test]
    fn sdp_layout() {
        let reg = SdpIn::new()
            .with_sdp_in_wl(WordLength::Bits16)
            .with_sdp_in_fmt(SerialFormat::DspPcm);
        assert_eq!(u8::from(reg), 0b0000_1111);
        assert_eq!(u8::from(SdpOut::new().with_sdp_out_mute(true)), 0x40);
    }

    #[test]
    fn clock_layout() {
        let reg = ClockPrescaler::new()
            .with_div_pre(2)
            .with_mult_pre(PreMultiplier::X2);
        assert_eq!(u8::from(reg), 0x48);

        let reg = ClockDivider::new().with_div_clkadc(1).with_div_clkdac(2);
        assert_eq!(u8::from(reg), 0x12);
        assert_eq!(u8::from(AdcOversampling::new().with_adc_fsmode(true)), 0x50);
    }

    #[test]
    fn routing_layout() {
        let reg = GpioControl::new()
            .with_adcdat_sel(AdcDataSelect::AdcDacRight)
            .with_i2c_wl(true);
        assert_eq!(u8::from(reg), 0x58);
        assert_eq!(u8::from(GpioControl::new().with_adc2dac_sel(true)), 0x80);
    }

    #[test]
    fn mute_layout() {
        let reg = DacControl::new()
            .with_dac_dsmmute(true)
            .with_dac_demmute(true);
        assert_eq!(u8::from(reg), 0x60);
    }
}
