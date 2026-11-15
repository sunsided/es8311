//! # Everest Semiconductor ES8311 Audio Codec Registers
//!
//! This crate provides a typed map of the ES8311's I²C registers `0x00`–`0x45` and the
//! chip ID registers `0xFD`–`0xFF`.
//!
//! Register addresses, names, bit layouts and defaults follow the ES8311 datasheet
//! (Revision 10.0, January 2021), section 8, unless noted otherwise.

#![deny(missing_docs)]
#![no_std]
#![forbid(unsafe_code)]
// bitfield-struct generates `with_*_checked` setters that return `Result<_, ()>`.
#![allow(clippy::result_unit_err)]
// Enables the `doc_cfg` feature when the `docsrs` configuration attribute is defined.
#![cfg_attr(docsrs, feature(doc_cfg))]

/// Exports commonly used traits.
pub mod prelude {
    pub use crate::{ReadableRegister, Register, WritableRegister};
    pub use hardware_registers::i2c::*;
    pub use hardware_registers::register_address::RegisterAddress8;
    pub use hardware_registers::sizes::R1;
    pub use hardware_registers::{FromBits, HardwareRegister, ToBits, WritableHardwareRegister};
}

/// Implements the address and size information shared by all registers.
macro_rules! register {
    ($type:ident, $addr:expr) => {
        impl $crate::Register for $type {}
        impl $crate::prelude::HardwareRegister<$crate::prelude::R1> for $type {}

        impl
            $crate::prelude::I2CRegister<
                $crate::prelude::DeviceAddress7,
                $crate::prelude::RegisterAddress8,
                $crate::prelude::R1,
            > for $type
        {
            type Backing = u8;

            const DEFAULT_DEVICE_ADDRESS: $crate::prelude::DeviceAddress7 =
                $crate::prelude::DeviceAddress7::new($crate::DEFAULT_DEVICE_ADDRESS);
            const REGISTER_ADDRESS: $crate::prelude::RegisterAddress8 =
                $crate::prelude::RegisterAddress8::new(($addr).addr());
        }
    };
}

/// Implements decoding, which makes a register readable.
macro_rules! read_access {
    ($type:ident) => {
        impl $crate::prelude::FromBits<u8> for $type {
            #[inline]
            fn from_bits(value: u8) -> Self {
                value.into()
            }

            #[inline]
            fn from_bits_ref(value: &u8) -> Self {
                (*value).into()
            }
        }

        impl $crate::ReadableRegister for $type {}
    };
}

/// Implements encoding, which makes a register writable.
macro_rules! write_access {
    ($type:ident) => {
        impl $crate::prelude::ToBits for $type {
            type Target = u8;

            #[inline]
            fn to_bits(&self) -> Self::Target {
                (*self).into()
            }
        }

        impl $crate::prelude::WritableHardwareRegister<$crate::prelude::R1> for $type {}
        impl $crate::WritableRegister for $type {}
    };
}

macro_rules! readable_register {
    ($type:ident, $addr:expr) => {
        register!($type, $addr);
        read_access!($type);
    };
}

macro_rules! writable_register {
    ($type:ident, $addr:expr) => {
        register!($type, $addr);
        read_access!($type);
        write_access!($type);
    };
}

mod registers;
mod types;

pub use registers::*;
pub use types::*;

/// The 7-bit I²C address with the `CE` pin low.
///
/// The address is `0011 00x`, where `x` is the level of `CE` (datasheet section 5):
/// `0x18` with `CE` low, `0x19` with `CE` high. Some host libraries take the 8-bit form
/// including the R/W bit (`0x30`); this crate uses the 7-bit form, like `embedded-hal`.
pub const DEFAULT_DEVICE_ADDRESS: u8 = 0x18;

/// The 7-bit I²C address with the `CE` pin high.
pub const ALTERNATE_DEVICE_ADDRESS: u8 = 0x19;

/// The value of the [`ChipId1`] register (`0xFD`) of an ES8311.
pub const ES8311_CHIP_ID1: u8 = 0x83;

/// The value of the [`ChipId2`] register (`0xFE`) of an ES8311.
pub const ES8311_CHIP_ID2: u8 = 0x11;

/// A codec register.
pub trait Register: prelude::I2CRegister8<prelude::DeviceAddress7> {}

/// A readable codec register.
pub trait ReadableRegister:
    Register + prelude::ReadableI2CRegister8<prelude::DeviceAddress7> + prelude::FromBits<u8>
{
}

/// A writable codec register.
pub trait WritableRegister:
    Register + prelude::WritableI2CRegister8<prelude::DeviceAddress7>
{
}
