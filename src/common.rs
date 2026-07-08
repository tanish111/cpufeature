// SPDX-License-Identifier: MIT
// Generator: x86-cpuid-db v3.1-3-gf5fd0fd-dirty

// Auto-generated file.
// Please submit all updates and bugfixes to https://x86-cpuid.org

//! Shared types for generated CPUID leaf modules.
//!
//! [`CpuidFeature`] describes the location of a bitfield within a CPUID
//! leaf/subleaf result.  [`CpuidRegister`] identifies which result register
//! contains the field.

/// CPUID result register (`eax`, `ebx`, `ecx`, or `edx`).
#[derive(Clone, Copy, Debug)]
#[repr(u8)]
pub enum CpuidRegister {
    /// Result register `eax`.
    Eax = 0,
    /// Result register `ebx`.
    Ebx = 1,
    /// Result register `ecx`.
    Ecx = 2,
    /// Result register `edx`.
    Edx = 3,
}

impl CpuidRegister {
    /// Return the value of the register selected by `self`.
    #[must_use]
    pub const fn select(self, eax: u32, ebx: u32, ecx: u32, edx: u32) -> u32 {
        match self {
            Self::Eax => eax,
            Self::Ebx => ebx,
            Self::Ecx => ecx,
            Self::Edx => edx,
        }
    }
}

/// Location of a CPUID bitfield within a leaf/subleaf result.
#[derive(Clone, Copy, Debug)]
pub struct CpuidFeature {
    /// CPUID leaf number (input `eax`, including `0x80000000` leaves).
    pub leaf: u32,
    /// CPUID subleaf (input `ecx` when applicable, otherwise `0`).
    pub subleaf: u32,
    /// Register containing the bitfield.
    pub register: CpuidRegister,
    /// Least significant bit index of the field within the register.
    pub shift: u8,
    /// Width of the field in bits.
    pub width: u8,
}
