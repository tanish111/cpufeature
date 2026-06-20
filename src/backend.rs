// SPDX-License-Identifier: MIT
//

use crate::leaves::CpuidFeature;
use core::arch::x86_64::{__cpuid_count, CpuidResult};

/// Backend for executing CPUID and reading feature values.
///
/// Implement this trait on a type to query CPUID using the descriptors in
/// [`crate::leaves`]. The default implementation issues a real CPUID via
/// [`core::arch::x86_64::__cpuid_count`]; override [`Self::cpuid`] to provide a different backend
/// (for example, mocked results in tests).
pub trait CpuidBackend: Sized {
    /// Execute CPUID for the leaf and subleaf described by `feature`.
    ///
    /// Returns the raw EAX/EBX/ECX/EDX values on success, or `None` if the
    /// backend cannot execute CPUID for this descriptor.
    fn cpuid(feature: &CpuidFeature) -> Option<CpuidResult> {
        // SAFETY: CPUID only reads processor feature information and does not
        // affect memory safety.
        Some(unsafe { __cpuid_count(feature.leaf, feature.subleaf) })
    }

    /// Read the value described by `feature` from the CPUID result.
    ///
    /// Selects the register, shift, and width defined in `feature`, then
    /// returns the extracted field. Returns `None` if [`Self::cpuid`] fails.
    fn cpuid_value(feature: &CpuidFeature) -> Option<u32> {
        Self::cpuid(feature).map(|regs| {
            let raw = feature
                .register
                .select(regs.eax, regs.ebx, regs.ecx, regs.edx);
            let mask = ((1u64 << feature.width) as u32).wrapping_sub(1);
            (raw >> feature.shift) & mask
        })
    }
}
