// SPDX-License-Identifier: MIT
// Generator: x86-cpuid-db v3.1-3-gf5fd0fd-dirty

// Auto-generated file.
// Please submit all updates and bugfixes to https://x86-cpuid.org

#![allow(dead_code)]

use super::common::{CpuidFeature, CpuidRegister};

// Leaf 0x0: Maximum standard leaf + CPU vendor string

/// Highest standard CPUID leaf
pub const MAX_STD_LEAF: CpuidFeature = CpuidFeature {
    leaf: 0x0,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 0,
    width: 32,
};

/// CPU vendor ID string bytes 0 - 3
pub const CPU_VENDORID_0: CpuidFeature = CpuidFeature {
    leaf: 0x0,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 0,
    width: 32,
};

/// CPU vendor ID string bytes 8 - 11
pub const CPU_VENDORID_2: CpuidFeature = CpuidFeature {
    leaf: 0x0,
    subleaf: 0,
    register: CpuidRegister::Ecx,
    shift: 0,
    width: 32,
};

/// CPU vendor ID string bytes 4 - 7
pub const CPU_VENDORID_1: CpuidFeature = CpuidFeature {
    leaf: 0x0,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 0,
    width: 32,
};
