// SPDX-License-Identifier: MIT
// Generator: x86-cpuid-db v3.1-3-gf5fd0fd-dirty

// Auto-generated file.
// Please submit all updates and bugfixes to https://x86-cpuid.org

use super::common::{CpuidFeature, CpuidRegister};

/// Performance monitoring v2
pub const X86_FEATURE_PERFMON_V2: CpuidFeature = CpuidFeature {
    leaf: 0x80000022,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 0,
    width: 1,
};

/// Last Branch Record v2 extensions (LBR Stack)
pub const X86_FEATURE_LBR_V2: CpuidFeature = CpuidFeature {
    leaf: 0x80000022,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 1,
    width: 1,
};

/// Freezing core performance counters / LBR Stack
pub const LBR_PMC_FREEZE: CpuidFeature = CpuidFeature {
    leaf: 0x80000022,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 2,
    width: 1,
};

/// Number of core performance counters
pub const N_PMC_CORE: CpuidFeature = CpuidFeature {
    leaf: 0x80000022,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 0,
    width: 4,
};

/// Number of LBR stack entries
pub const LBR_V2_STACK_SIZE: CpuidFeature = CpuidFeature {
    leaf: 0x80000022,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 4,
    width: 6,
};

/// Number of northbridge performance counters
pub const N_PMC_NORTHBRIDGE: CpuidFeature = CpuidFeature {
    leaf: 0x80000022,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 10,
    width: 6,
};

/// Number of UMC performance counters
pub const N_PMC_UMC: CpuidFeature = CpuidFeature {
    leaf: 0x80000022,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 16,
    width: 6,
};

/// Active UMCs bitmask
pub const ACTIVE_UMC_BITMASK: CpuidFeature = CpuidFeature {
    leaf: 0x80000022,
    subleaf: 0,
    register: CpuidRegister::Ecx,
    shift: 0,
    width: 32,
};

/// MEM-HMK encryption mode
pub const MEM_HMK_MODE: CpuidFeature = CpuidFeature {
    leaf: 0x80000023,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 0,
    width: 1,
};

/// Total number of available encryption keys
pub const MEM_HMK_AVAIL_KEYS: CpuidFeature = CpuidFeature {
    leaf: 0x80000023,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 0,
    width: 16,
};

/// Current CPU frequency, in MHz
pub const CPU_CUR_MHZ: CpuidFeature = CpuidFeature {
    leaf: 0x80860007,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 0,
    width: 32,
};

/// Current CPU voltage, in millivolts
pub const CPU_CUR_VOLTAGE: CpuidFeature = CpuidFeature {
    leaf: 0x80860007,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 0,
    width: 32,
};

/// Current CPU performance percentage, 0 - 100
pub const CPU_CUR_PERF_PCTG: CpuidFeature = CpuidFeature {
    leaf: 0x80860007,
    subleaf: 0,
    register: CpuidRegister::Ecx,
    shift: 0,
    width: 32,
};

/// Current CPU gate delay, in femtoseconds
pub const CPU_CUR_GATE_DELAY: CpuidFeature = CpuidFeature {
    leaf: 0x80860007,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 0,
    width: 32,
};

/// L1 iTLB #entries, 1G pages
pub const L1_ITLB_1G_NENTRIES: CpuidFeature = CpuidFeature {
    leaf: 0x80000019,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 0,
    width: 12,
};

/// L1 iTLB associativity, 1G pages
pub const L1_ITLB_1G_ASSOC: CpuidFeature = CpuidFeature {
    leaf: 0x80000019,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 12,
    width: 4,
};

/// L1 dTLB #entries, 1G pages
pub const L1_DTLB_1G_NENTRIES: CpuidFeature = CpuidFeature {
    leaf: 0x80000019,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 16,
    width: 12,
};

/// L1 dTLB associativity, 1G pages
pub const L1_DTLB_1G_ASSOC: CpuidFeature = CpuidFeature {
    leaf: 0x80000019,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 28,
    width: 4,
};

/// L2 iTLB #entries, 1G pages
pub const L2_ITLB_1G_NENTRIES: CpuidFeature = CpuidFeature {
    leaf: 0x80000019,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 0,
    width: 12,
};

/// L2 iTLB associativity, 1G pages
pub const L2_ITLB_1G_ASSOC: CpuidFeature = CpuidFeature {
    leaf: 0x80000019,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 12,
    width: 4,
};

/// L2 dTLB #entries, 1G pages
pub const L2_DTLB_1G_NENTRIES: CpuidFeature = CpuidFeature {
    leaf: 0x80000019,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 16,
    width: 12,
};

/// L2 dTLB associativity, 1G pages
pub const L2_DTLB_1G_ASSOC: CpuidFeature = CpuidFeature {
    leaf: 0x80000019,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 28,
    width: 4,
};

/// CPU brand ID string bytes, 16 - 19
pub const CPU_BRANDID_4: CpuidFeature = CpuidFeature {
    leaf: 0x80000003,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 0,
    width: 32,
};

/// CPU brand ID string bytes, 20 - 23
pub const CPU_BRANDID_5: CpuidFeature = CpuidFeature {
    leaf: 0x80000003,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 0,
    width: 32,
};

/// CPU brand ID string bytes, 24 - 27
pub const CPU_BRANDID_6: CpuidFeature = CpuidFeature {
    leaf: 0x80000003,
    subleaf: 0,
    register: CpuidRegister::Ecx,
    shift: 0,
    width: 32,
};

/// CPU brand ID string bytes, 28 - 31
pub const CPU_BRANDID_7: CpuidFeature = CpuidFeature {
    leaf: 0x80000003,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 0,
    width: 32,
};

/// Number of times this leaf must be queried
pub const ITERATION_COUNT: CpuidFeature = CpuidFeature {
    leaf: 0x2,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 0,
    width: 8,
};

/// Descriptor #1
pub const DESC1: CpuidFeature = CpuidFeature {
    leaf: 0x2,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 8,
    width: 8,
};

/// Descriptor #2
pub const DESC2: CpuidFeature = CpuidFeature {
    leaf: 0x2,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 16,
    width: 8,
};

/// Descriptor #3
pub const DESC3: CpuidFeature = CpuidFeature {
    leaf: 0x2,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 24,
    width: 7,
};

/// Descriptors 1-3 are invalid if set
pub const EAX_INVALID: CpuidFeature = CpuidFeature {
    leaf: 0x2,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 31,
    width: 1,
};

/// Descriptor #4
pub const DESC4: CpuidFeature = CpuidFeature {
    leaf: 0x2,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 0,
    width: 8,
};

/// Descriptor #5
pub const DESC5: CpuidFeature = CpuidFeature {
    leaf: 0x2,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 8,
    width: 8,
};

/// Descriptor #6
pub const DESC6: CpuidFeature = CpuidFeature {
    leaf: 0x2,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 16,
    width: 8,
};

/// Descriptor #7
pub const DESC7: CpuidFeature = CpuidFeature {
    leaf: 0x2,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 24,
    width: 7,
};

/// Descriptors 4-7 are invalid if set
pub const EBX_INVALID: CpuidFeature = CpuidFeature {
    leaf: 0x2,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 31,
    width: 1,
};

/// Descriptor #8
pub const DESC8: CpuidFeature = CpuidFeature {
    leaf: 0x2,
    subleaf: 0,
    register: CpuidRegister::Ecx,
    shift: 0,
    width: 8,
};

/// Descriptor #9
pub const DESC9: CpuidFeature = CpuidFeature {
    leaf: 0x2,
    subleaf: 0,
    register: CpuidRegister::Ecx,
    shift: 8,
    width: 8,
};

/// Descriptor #10
pub const DESC10: CpuidFeature = CpuidFeature {
    leaf: 0x2,
    subleaf: 0,
    register: CpuidRegister::Ecx,
    shift: 16,
    width: 8,
};

/// Descriptor #11
pub const DESC11: CpuidFeature = CpuidFeature {
    leaf: 0x2,
    subleaf: 0,
    register: CpuidRegister::Ecx,
    shift: 24,
    width: 7,
};

/// Descriptors 8-11 are invalid if set
pub const ECX_INVALID: CpuidFeature = CpuidFeature {
    leaf: 0x2,
    subleaf: 0,
    register: CpuidRegister::Ecx,
    shift: 31,
    width: 1,
};

/// Descriptor #12
pub const DESC12: CpuidFeature = CpuidFeature {
    leaf: 0x2,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 0,
    width: 8,
};

/// Descriptor #13
pub const DESC13: CpuidFeature = CpuidFeature {
    leaf: 0x2,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 8,
    width: 8,
};

/// Descriptor #14
pub const DESC14: CpuidFeature = CpuidFeature {
    leaf: 0x2,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 16,
    width: 8,
};

/// Descriptor #15
pub const DESC15: CpuidFeature = CpuidFeature {
    leaf: 0x2,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 24,
    width: 7,
};

/// Descriptors 12-15 are invalid if set
pub const EDX_INVALID: CpuidFeature = CpuidFeature {
    leaf: 0x2,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 31,
    width: 1,
};

/// Maximum hypervisor leaf
pub const MAX_HYP_LEAF: CpuidFeature = CpuidFeature {
    leaf: 0x40000000,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 0,
    width: 32,
};

/// Hypervisor ID string bytes 0 - 3
pub const HYPERVISOR_ID_0: CpuidFeature = CpuidFeature {
    leaf: 0x40000000,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 0,
    width: 32,
};

/// Hypervisor ID string bytes 4 - 7
pub const HYPERVISOR_ID_1: CpuidFeature = CpuidFeature {
    leaf: 0x40000000,
    subleaf: 0,
    register: CpuidRegister::Ecx,
    shift: 0,
    width: 32,
};

/// Hypervisor ID string bytes 8 - 11
pub const HYPERVISOR_ID_2: CpuidFeature = CpuidFeature {
    leaf: 0x40000000,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 0,
    width: 32,
};

/// Max physical address bits
pub const PHYS_ADDR_BITS: CpuidFeature = CpuidFeature {
    leaf: 0x80000008,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 0,
    width: 8,
};

/// Max virtual address bits
pub const VIRT_ADDR_BITS: CpuidFeature = CpuidFeature {
    leaf: 0x80000008,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 8,
    width: 8,
};

/// Max nested-paging guest physical address bits
pub const GUEST_PHYS_ADDR_BITS: CpuidFeature = CpuidFeature {
    leaf: 0x80000008,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 16,
    width: 8,
};

/// CLZERO instruction
pub const X86_FEATURE_CLZERO: CpuidFeature = CpuidFeature {
    leaf: 0x80000008,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 0,
    width: 1,
};

/// Instruction retired counter MSR
pub const X86_FEATURE_IRPERF: CpuidFeature = CpuidFeature {
    leaf: 0x80000008,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 1,
    width: 1,
};

/// XSAVE/XRSTOR always saves/restores FPU error pointers
pub const X86_FEATURE_XSAVEERPTR: CpuidFeature = CpuidFeature {
    leaf: 0x80000008,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 2,
    width: 1,
};

/// INVLPGB broadcasts a TLB invalidate
pub const INVLPGB: CpuidFeature = CpuidFeature {
    leaf: 0x80000008,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 3,
    width: 1,
};

/// RDPRU (Read Processor Register at User level)
pub const X86_FEATURE_RDPRU: CpuidFeature = CpuidFeature {
    leaf: 0x80000008,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 4,
    width: 1,
};

/// Memory Bandwidth Allocation (AMD bit)
pub const X86_FEATURE_MBA: CpuidFeature = CpuidFeature {
    leaf: 0x80000008,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 6,
    width: 1,
};

/// MCOMMIT instruction
pub const MCOMMIT: CpuidFeature = CpuidFeature {
    leaf: 0x80000008,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 8,
    width: 1,
};

/// WBNOINVD instruction
pub const X86_FEATURE_WBNOINVD: CpuidFeature = CpuidFeature {
    leaf: 0x80000008,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 9,
    width: 1,
};

/// Indirect Branch Prediction Barrier
pub const X86_FEATURE_AMD_IBPB: CpuidFeature = CpuidFeature {
    leaf: 0x80000008,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 12,
    width: 1,
};

/// Interruptible WBINVD/WBNOINVD
pub const WBINVD_INT: CpuidFeature = CpuidFeature {
    leaf: 0x80000008,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 13,
    width: 1,
};

/// Indirect Branch Restricted Speculation
pub const X86_FEATURE_AMD_IBRS: CpuidFeature = CpuidFeature {
    leaf: 0x80000008,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 14,
    width: 1,
};

/// Single Thread Indirect Branch Prediction mode
pub const X86_FEATURE_AMD_STIBP: CpuidFeature = CpuidFeature {
    leaf: 0x80000008,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 15,
    width: 1,
};

/// IBRS always-on preferred
pub const IBRS_ALWAYS_ON: CpuidFeature = CpuidFeature {
    leaf: 0x80000008,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 16,
    width: 1,
};

/// STIBP always-on preferred
pub const X86_FEATURE_AMD_STIBP_ALWAYS_ON: CpuidFeature = CpuidFeature {
    leaf: 0x80000008,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 17,
    width: 1,
};

/// IBRS is preferred over software solution
pub const IBRS_FAST: CpuidFeature = CpuidFeature {
    leaf: 0x80000008,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 18,
    width: 1,
};

/// IBRS provides same mode protection
pub const IBRS_SAME_MODE: CpuidFeature = CpuidFeature {
    leaf: 0x80000008,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 19,
    width: 1,
};

/// Long-Mode Segment Limit Enable unsupported
pub const NO_EFER_LMSLE: CpuidFeature = CpuidFeature {
    leaf: 0x80000008,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 20,
    width: 1,
};

/// INVLPGB RAX[5] bit can be set
pub const TLB_FLUSH_NESTED: CpuidFeature = CpuidFeature {
    leaf: 0x80000008,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 21,
    width: 1,
};

/// Protected Processor Inventory Number
pub const X86_FEATURE_AMD_PPIN: CpuidFeature = CpuidFeature {
    leaf: 0x80000008,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 23,
    width: 1,
};

/// Speculative Store Bypass Disable
pub const X86_FEATURE_AMD_SSBD: CpuidFeature = CpuidFeature {
    leaf: 0x80000008,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 24,
    width: 1,
};

/// virtualized SSBD (Speculative Store Bypass Disable)
pub const X86_FEATURE_VIRT_SSBD: CpuidFeature = CpuidFeature {
    leaf: 0x80000008,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 25,
    width: 1,
};

/// SSBD is not needed (fixed in hardware)
pub const X86_FEATURE_AMD_SSB_NO: CpuidFeature = CpuidFeature {
    leaf: 0x80000008,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 26,
    width: 1,
};

/// Collaborative Processor Performance Control
pub const X86_FEATURE_CPPC: CpuidFeature = CpuidFeature {
    leaf: 0x80000008,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 27,
    width: 1,
};

/// Predictive Store Forward Disable
pub const X86_FEATURE_AMD_PSFD: CpuidFeature = CpuidFeature {
    leaf: 0x80000008,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 28,
    width: 1,
};

/// CPU not affected by Branch Type Confusion
pub const X86_FEATURE_BTC_NO: CpuidFeature = CpuidFeature {
    leaf: 0x80000008,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 29,
    width: 1,
};

/// IBPB clears RSB/RAS too
pub const IBPB_RET: CpuidFeature = CpuidFeature {
    leaf: 0x80000008,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 30,
    width: 1,
};

/// Branch Sampling
pub const X86_FEATURE_BRS: CpuidFeature = CpuidFeature {
    leaf: 0x80000008,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 31,
    width: 1,
};

/// Number of physical threads - 1
pub const CPU_NTHREADS: CpuidFeature = CpuidFeature {
    leaf: 0x80000008,
    subleaf: 0,
    register: CpuidRegister::Ecx,
    shift: 0,
    width: 8,
};

/// Number of thread core ID bits (shift) in APIC ID
pub const APICID_COREID_LEN: CpuidFeature = CpuidFeature {
    leaf: 0x80000008,
    subleaf: 0,
    register: CpuidRegister::Ecx,
    shift: 12,
    width: 4,
};

/// Performance time-stamp counter size
pub const PERF_TSC_LEN: CpuidFeature = CpuidFeature {
    leaf: 0x80000008,
    subleaf: 0,
    register: CpuidRegister::Ecx,
    shift: 16,
    width: 2,
};

/// INVLPGB maximum page count
pub const INVLPGB_MAX_PAGES: CpuidFeature = CpuidFeature {
    leaf: 0x80000008,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 0,
    width: 16,
};

/// RDPRU max register ID (ECX input)
pub const RDPRU_MAX_REG_ID: CpuidFeature = CpuidFeature {
    leaf: 0x80000008,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 16,
    width: 16,
};

/// L1 ITLB #entries, 2M and 4M pages
pub const L1_ITLB_2M_4M_NENTRIES: CpuidFeature = CpuidFeature {
    leaf: 0x80000005,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 0,
    width: 8,
};

/// L1 ITLB associativity, 2M and 4M pages
pub const L1_ITLB_2M_4M_ASSOC: CpuidFeature = CpuidFeature {
    leaf: 0x80000005,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 8,
    width: 8,
};

/// L1 DTLB #entries, 2M and 4M pages
pub const L1_DTLB_2M_4M_NENTRIES: CpuidFeature = CpuidFeature {
    leaf: 0x80000005,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 16,
    width: 8,
};

/// L1 DTLB associativity, 2M and 4M pages
pub const L1_DTLB_2M_4M_ASSOC: CpuidFeature = CpuidFeature {
    leaf: 0x80000005,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 24,
    width: 8,
};

/// L1 ITLB #entries, 4K pages
pub const L1_ITLB_4K_NENTRIES: CpuidFeature = CpuidFeature {
    leaf: 0x80000005,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 0,
    width: 8,
};

/// L1 ITLB associativity, 4K pages
pub const L1_ITLB_4K_ASSOC: CpuidFeature = CpuidFeature {
    leaf: 0x80000005,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 8,
    width: 8,
};

/// L1 DTLB #entries, 4K pages
pub const L1_DTLB_4K_NENTRIES: CpuidFeature = CpuidFeature {
    leaf: 0x80000005,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 16,
    width: 8,
};

/// L1 DTLB associativity, 4K pages
pub const L1_DTLB_4K_ASSOC: CpuidFeature = CpuidFeature {
    leaf: 0x80000005,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 24,
    width: 8,
};

/// L1 dcache line size, in bytes
pub const L1_DCACHE_LINE_SIZE: CpuidFeature = CpuidFeature {
    leaf: 0x80000005,
    subleaf: 0,
    register: CpuidRegister::Ecx,
    shift: 0,
    width: 8,
};

/// L1 dcache lines per tag
pub const L1_DCACHE_NLINES: CpuidFeature = CpuidFeature {
    leaf: 0x80000005,
    subleaf: 0,
    register: CpuidRegister::Ecx,
    shift: 8,
    width: 8,
};

/// L1 dcache associativity
pub const L1_DCACHE_ASSOC: CpuidFeature = CpuidFeature {
    leaf: 0x80000005,
    subleaf: 0,
    register: CpuidRegister::Ecx,
    shift: 16,
    width: 8,
};

/// L1 dcache size, in KB
pub const L1_DCACHE_SIZE_KB: CpuidFeature = CpuidFeature {
    leaf: 0x80000005,
    subleaf: 0,
    register: CpuidRegister::Ecx,
    shift: 24,
    width: 8,
};

/// L1 icache line size, in bytes
pub const L1_ICACHE_LINE_SIZE: CpuidFeature = CpuidFeature {
    leaf: 0x80000005,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 0,
    width: 8,
};

/// L1 icache lines per tag
pub const L1_ICACHE_NLINES: CpuidFeature = CpuidFeature {
    leaf: 0x80000005,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 8,
    width: 8,
};

/// L1 icache associativity
pub const L1_ICACHE_ASSOC: CpuidFeature = CpuidFeature {
    leaf: 0x80000005,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 16,
    width: 8,
};

/// L1 icache size, in KB
pub const L1_ICACHE_SIZE_KB: CpuidFeature = CpuidFeature {
    leaf: 0x80000005,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 24,
    width: 8,
};

/// CPUID 0x20 max subleaf + 1
pub const HRESET_NR_SUBLEAVES: CpuidFeature = CpuidFeature {
    leaf: 0x20,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 0,
    width: 32,
};

/// Intel thread director HRESET
pub const HRESET_THREAD_DIRECTOR: CpuidFeature = CpuidFeature {
    leaf: 0x20,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 0,
    width: 1,
};

/// CPU info string bytes 32 - 35
pub const CPU_INFO_8: CpuidFeature = CpuidFeature {
    leaf: 0x80860005,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 0,
    width: 32,
};

/// CPU info string bytes 36 - 39
pub const CPU_INFO_9: CpuidFeature = CpuidFeature {
    leaf: 0x80860005,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 0,
    width: 32,
};

/// CPU info string bytes 40 - 43
pub const CPU_INFO_10: CpuidFeature = CpuidFeature {
    leaf: 0x80860005,
    subleaf: 0,
    register: CpuidRegister::Ecx,
    shift: 0,
    width: 32,
};

/// CPU info string bytes 44 - 47
pub const CPU_INFO_11: CpuidFeature = CpuidFeature {
    leaf: 0x80860005,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 0,
    width: 32,
};

/// L2 iTLB #entries, 2M and 4M pages
pub const L2_ITLB_2M_4M_NENTRIES: CpuidFeature = CpuidFeature {
    leaf: 0x80000006,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 0,
    width: 12,
};

/// L2 iTLB associativity, 2M and 4M pages
pub const L2_ITLB_2M_4M_ASSOC: CpuidFeature = CpuidFeature {
    leaf: 0x80000006,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 12,
    width: 4,
};

/// L2 dTLB #entries, 2M and 4M pages
pub const L2_DTLB_2M_4M_NENTRIES: CpuidFeature = CpuidFeature {
    leaf: 0x80000006,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 16,
    width: 12,
};

/// L2 dTLB associativity, 2M and 4M pages
pub const L2_DTLB_2M_4M_ASSOC: CpuidFeature = CpuidFeature {
    leaf: 0x80000006,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 28,
    width: 4,
};

/// L2 iTLB #entries, 4K pages
pub const L2_ITLB_4K_NENTRIES: CpuidFeature = CpuidFeature {
    leaf: 0x80000006,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 0,
    width: 12,
};

/// L2 iTLB associativity, 4K pages
pub const L2_ITLB_4K_ASSOC: CpuidFeature = CpuidFeature {
    leaf: 0x80000006,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 12,
    width: 4,
};

/// L2 dTLB #entries, 4K pages
pub const L2_DTLB_4K_NENTRIES: CpuidFeature = CpuidFeature {
    leaf: 0x80000006,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 16,
    width: 12,
};

/// L2 dTLB associativity, 4K pages
pub const L2_DTLB_4K_ASSOC: CpuidFeature = CpuidFeature {
    leaf: 0x80000006,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 28,
    width: 4,
};

/// L2 cache line size, in bytes
pub const L2_LINE_SIZE: CpuidFeature = CpuidFeature {
    leaf: 0x80000006,
    subleaf: 0,
    register: CpuidRegister::Ecx,
    shift: 0,
    width: 8,
};

/// L2 cache number of lines per tag
pub const L2_NLINES: CpuidFeature = CpuidFeature {
    leaf: 0x80000006,
    subleaf: 0,
    register: CpuidRegister::Ecx,
    shift: 8,
    width: 4,
};

/// L2 cache associativity
pub const L2_ASSOC: CpuidFeature = CpuidFeature {
    leaf: 0x80000006,
    subleaf: 0,
    register: CpuidRegister::Ecx,
    shift: 12,
    width: 4,
};

/// L2 cache size, in KB
pub const L2_SIZE_KB: CpuidFeature = CpuidFeature {
    leaf: 0x80000006,
    subleaf: 0,
    register: CpuidRegister::Ecx,
    shift: 16,
    width: 16,
};

/// L3 cache line size, in bytes
pub const L3_LINE_SIZE: CpuidFeature = CpuidFeature {
    leaf: 0x80000006,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 0,
    width: 8,
};

/// L3 cache number of lines per tag
pub const L3_NLINES: CpuidFeature = CpuidFeature {
    leaf: 0x80000006,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 8,
    width: 4,
};

/// L3 cache associativity
pub const L3_ASSOC: CpuidFeature = CpuidFeature {
    leaf: 0x80000006,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 12,
    width: 4,
};

/// L3 cache size range
pub const L3_SIZE_RANGE: CpuidFeature = CpuidFeature {
    leaf: 0x80000006,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 18,
    width: 14,
};

/// No nested data breakpoints
pub const X86_FEATURE_NO_NESTED_DATA_BP: CpuidFeature = CpuidFeature {
    leaf: 0x80000021,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 0,
    width: 1,
};

/// WRMSR to {FS,GS,KERNEL_GS}_BASE is non-serializing
pub const FSGS_NON_SERIALIZING: CpuidFeature = CpuidFeature {
    leaf: 0x80000021,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 1,
    width: 1,
};

/// LFENCE always serializing / synchronizes RDTSC
pub const X86_FEATURE_LFENCE_RDTSC: CpuidFeature = CpuidFeature {
    leaf: 0x80000021,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 2,
    width: 1,
};

/// SMM paging configuration lock
pub const SMM_PAGE_CFG_LOCK: CpuidFeature = CpuidFeature {
    leaf: 0x80000021,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 3,
    width: 1,
};

/// Null selector clears base
pub const X86_FEATURE_NULL_SEL_CLR_BASE: CpuidFeature = CpuidFeature {
    leaf: 0x80000021,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 6,
    width: 1,
};

/// EFER MSR Upper Address Ignore
pub const UPPER_ADDR_IGNORE: CpuidFeature = CpuidFeature {
    leaf: 0x80000021,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 7,
    width: 1,
};

/// EFER MSR Automatic IBRS
pub const X86_FEATURE_AUTOIBRS: CpuidFeature = CpuidFeature {
    leaf: 0x80000021,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 8,
    width: 1,
};

/// SMM_CTL MSR not available
pub const X86_FEATURE_NO_SMM_CTL_MSR: CpuidFeature = CpuidFeature {
    leaf: 0x80000021,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 9,
    width: 1,
};

/// Fast Short REP STOSB
pub const FSRS: CpuidFeature = CpuidFeature {
    leaf: 0x80000021,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 10,
    width: 1,
};

/// Fast Short REP CMPSB
pub const FSRC: CpuidFeature = CpuidFeature {
    leaf: 0x80000021,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 11,
    width: 1,
};

/// Prefetch control MSR
pub const PREFETCH_CTL_MSR: CpuidFeature = CpuidFeature {
    leaf: 0x80000021,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 13,
    width: 1,
};

/// Reserves opcode space
pub const X86_FEATURE_OPCODE_RECLAIM: CpuidFeature = CpuidFeature {
    leaf: 0x80000021,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 16,
    width: 1,
};

/// #GP when executing CPUID at CPL > 0
pub const USER_CPUID_DISABLE: CpuidFeature = CpuidFeature {
    leaf: 0x80000021,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 17,
    width: 1,
};

/// Enhanced Predictive Store Forwarding
pub const EPSF: CpuidFeature = CpuidFeature {
    leaf: 0x80000021,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 18,
    width: 1,
};

/// Workload-based heuristic feedback to OS
pub const X86_FEATURE_WL_FEEDBACK: CpuidFeature = CpuidFeature {
    leaf: 0x80000021,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 22,
    width: 1,
};

/// Enhanced Return Address Predictor Security
pub const X86_FEATURE_ERAPS: CpuidFeature = CpuidFeature {
    leaf: 0x80000021,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 24,
    width: 1,
};

/// Selective Branch Predictor Barrier
pub const X86_FEATURE_SBPB: CpuidFeature = CpuidFeature {
    leaf: 0x80000021,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 27,
    width: 1,
};

/// Branch predictions flushed from CPU branch predictor
pub const X86_FEATURE_IBPB_BRTYPE: CpuidFeature = CpuidFeature {
    leaf: 0x80000021,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 28,
    width: 1,
};

/// No SRSO vulnerability
pub const X86_FEATURE_SRSO_NO: CpuidFeature = CpuidFeature {
    leaf: 0x80000021,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 29,
    width: 1,
};

/// No SRSO at user-kernel boundary
pub const X86_FEATURE_SRSO_UK_NO: CpuidFeature = CpuidFeature {
    leaf: 0x80000021,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 30,
    width: 1,
};

/// MSR BP_CFG[BpSpecReduce] SRSO mitigation
pub const X86_FEATURE_SRSO_MSR_FIX: CpuidFeature = CpuidFeature {
    leaf: 0x80000021,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 31,
    width: 1,
};

/// Microcode patch size, in 16-byte units
pub const MICROCODE_PATCH_SIZE: CpuidFeature = CpuidFeature {
    leaf: 0x80000021,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 0,
    width: 16,
};

/// Return Address Predictor size
pub const RAP_SIZE: CpuidFeature = CpuidFeature {
    leaf: 0x80000021,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 16,
    width: 8,
};

/// CPU brand ID string, bytes 32 - 35
pub const CPU_BRANDID_8: CpuidFeature = CpuidFeature {
    leaf: 0x80000004,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 0,
    width: 32,
};

/// CPU brand ID string, bytes 36 - 39
pub const CPU_BRANDID_9: CpuidFeature = CpuidFeature {
    leaf: 0x80000004,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 0,
    width: 32,
};

/// CPU brand ID string, bytes 40 - 43
pub const CPU_BRANDID_10: CpuidFeature = CpuidFeature {
    leaf: 0x80000004,
    subleaf: 0,
    register: CpuidRegister::Ecx,
    shift: 0,
    width: 32,
};

/// CPU brand ID string, bytes 44 - 47
pub const CPU_BRANDID_11: CpuidFeature = CpuidFeature {
    leaf: 0x80000004,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 0,
    width: 32,
};

/// XCR0.X87
pub const XCR0_X87: CpuidFeature = CpuidFeature {
    leaf: 0xd,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 0,
    width: 1,
};

/// XCR0.SSE
pub const XCR0_SSE: CpuidFeature = CpuidFeature {
    leaf: 0xd,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 1,
    width: 1,
};

/// XCR0.AVX
pub const XCR0_AVX: CpuidFeature = CpuidFeature {
    leaf: 0xd,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 2,
    width: 1,
};

/// XCR0.BNDREGS: MPX BND0-BND3 registers
pub const XCR0_MPX_BNDREGS: CpuidFeature = CpuidFeature {
    leaf: 0xd,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 3,
    width: 1,
};

/// XCR0.BNDCSR: MPX BNDCFGU/BNDSTATUS registers
pub const XCR0_MPX_BNDCSR: CpuidFeature = CpuidFeature {
    leaf: 0xd,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 4,
    width: 1,
};

/// XCR0.OPMASK: AVX-512 k0-k7 registers
pub const XCR0_AVX512_OPMASK: CpuidFeature = CpuidFeature {
    leaf: 0xd,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 5,
    width: 1,
};

/// XCR0.ZMM_Hi256: AVX-512 ZMM0->ZMM7/15 registers
pub const XCR0_AVX512_ZMM_HI256: CpuidFeature = CpuidFeature {
    leaf: 0xd,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 6,
    width: 1,
};

/// XCR0.HI16_ZMM: AVX-512 ZMM16->ZMM31 registers
pub const XCR0_AVX512_HI16_ZMM: CpuidFeature = CpuidFeature {
    leaf: 0xd,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 7,
    width: 1,
};

/// XCR0.PKRU: XSAVE PKRU registers
pub const XCR0_PKRU: CpuidFeature = CpuidFeature {
    leaf: 0xd,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 9,
    width: 1,
};

/// XCR0.CET_U: CET user state
pub const XCR0_CET_U: CpuidFeature = CpuidFeature {
    leaf: 0xd,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 11,
    width: 1,
};

/// XCR0.CET_S: CET supervisor state
pub const XCR0_CET_S: CpuidFeature = CpuidFeature {
    leaf: 0xd,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 12,
    width: 1,
};

/// XCR0.TILECONFIG: AMX can manage TILECONFIG
pub const XCR0_TILECONFIG: CpuidFeature = CpuidFeature {
    leaf: 0xd,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 17,
    width: 1,
};

/// XCR0.TILEDATA: AMX can manage TILEDATA
pub const XCR0_TILEDATA: CpuidFeature = CpuidFeature {
    leaf: 0xd,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 18,
    width: 1,
};

/// XSAVE/XRSTOR area byte size, for XCR0 enabled features
pub const XSAVE_SZ_XCR0: CpuidFeature = CpuidFeature {
    leaf: 0xd,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 0,
    width: 32,
};

/// XSAVE/XRSTOR area max byte size, all CPU features
pub const XSAVE_SZ_MAX: CpuidFeature = CpuidFeature {
    leaf: 0xd,
    subleaf: 0,
    register: CpuidRegister::Ecx,
    shift: 0,
    width: 32,
};

/// AMD XCR0.LWP: Light-weight Profiling
pub const XCR0_LWP: CpuidFeature = CpuidFeature {
    leaf: 0xd,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 30,
    width: 1,
};

/// XSAVEOPT instruction
pub const X86_FEATURE_XSAVEOPT: CpuidFeature = CpuidFeature {
    leaf: 0xd,
    subleaf: 1,
    register: CpuidRegister::Eax,
    shift: 0,
    width: 1,
};

/// XSAVEC instruction
pub const X86_FEATURE_XSAVEC: CpuidFeature = CpuidFeature {
    leaf: 0xd,
    subleaf: 1,
    register: CpuidRegister::Eax,
    shift: 1,
    width: 1,
};

/// XGETBV instruction with ECX = 1
pub const X86_FEATURE_XGETBV1: CpuidFeature = CpuidFeature {
    leaf: 0xd,
    subleaf: 1,
    register: CpuidRegister::Eax,
    shift: 2,
    width: 1,
};

/// XSAVES/XRSTORS instructions (and XSS MSR)
pub const X86_FEATURE_XSAVES: CpuidFeature = CpuidFeature {
    leaf: 0xd,
    subleaf: 1,
    register: CpuidRegister::Eax,
    shift: 3,
    width: 1,
};

/// Extended feature disable
pub const X86_FEATURE_XFD: CpuidFeature = CpuidFeature {
    leaf: 0xd,
    subleaf: 1,
    register: CpuidRegister::Eax,
    shift: 4,
    width: 1,
};

/// XSAVES/XSAVEC area byte size, for XCR0|XSS enabled features
pub const XSAVE_SZ_XCR0_XSS: CpuidFeature = CpuidFeature {
    leaf: 0xd,
    subleaf: 1,
    register: CpuidRegister::Ebx,
    shift: 0,
    width: 32,
};

/// PT state
pub const XSS_PT: CpuidFeature = CpuidFeature {
    leaf: 0xd,
    subleaf: 1,
    register: CpuidRegister::Ecx,
    shift: 8,
    width: 1,
};

/// PASID state
pub const XSS_PASID: CpuidFeature = CpuidFeature {
    leaf: 0xd,
    subleaf: 1,
    register: CpuidRegister::Ecx,
    shift: 10,
    width: 1,
};

/// CET user state
pub const XSS_CET_U: CpuidFeature = CpuidFeature {
    leaf: 0xd,
    subleaf: 1,
    register: CpuidRegister::Ecx,
    shift: 11,
    width: 1,
};

/// CET supervisor state
pub const XSS_CET_S: CpuidFeature = CpuidFeature {
    leaf: 0xd,
    subleaf: 1,
    register: CpuidRegister::Ecx,
    shift: 12,
    width: 1,
};

/// HDC state
pub const XSS_HDC: CpuidFeature = CpuidFeature {
    leaf: 0xd,
    subleaf: 1,
    register: CpuidRegister::Ecx,
    shift: 13,
    width: 1,
};

/// UINTR state
pub const XSS_UINTR: CpuidFeature = CpuidFeature {
    leaf: 0xd,
    subleaf: 1,
    register: CpuidRegister::Ecx,
    shift: 14,
    width: 1,
};

/// LBR state
pub const XSS_LBR: CpuidFeature = CpuidFeature {
    leaf: 0xd,
    subleaf: 1,
    register: CpuidRegister::Ecx,
    shift: 15,
    width: 1,
};

/// HWP state
pub const XSS_HWP: CpuidFeature = CpuidFeature {
    leaf: 0xd,
    subleaf: 1,
    register: CpuidRegister::Ecx,
    shift: 16,
    width: 1,
};

/// Subleaf-N feature save area size, in bytes
pub const XSAVE_SZ: CpuidFeature = CpuidFeature {
    leaf: 0xd,
    subleaf: 2,
    register: CpuidRegister::Eax,
    shift: 0,
    width: 32,
};

/// Subleaf-N feature save area offset, in bytes
pub const XSAVE_OFFSET: CpuidFeature = CpuidFeature {
    leaf: 0xd,
    subleaf: 2,
    register: CpuidRegister::Ebx,
    shift: 0,
    width: 32,
};

/// Subleaf N describes an XSS bit (otherwise XCR0)
pub const IS_XSS_BIT: CpuidFeature = CpuidFeature {
    leaf: 0xd,
    subleaf: 2,
    register: CpuidRegister::Ecx,
    shift: 0,
    width: 1,
};

/// When compacted, subleaf-N XSAVE area is 64-byte aligned
pub const COMPACTED_XSAVE_64BYTE_ALIGNED: CpuidFeature = CpuidFeature {
    leaf: 0xd,
    subleaf: 2,
    register: CpuidRegister::Ecx,
    shift: 1,
    width: 1,
};

/// Smallest monitor-line size, in bytes
pub const MIN_MON_SIZE: CpuidFeature = CpuidFeature {
    leaf: 0x5,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 0,
    width: 16,
};

/// Largest monitor-line size, in bytes
pub const MAX_MON_SIZE: CpuidFeature = CpuidFeature {
    leaf: 0x5,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 0,
    width: 16,
};

/// MONITOR/MWAIT extensions
pub const MWAIT_EXT: CpuidFeature = CpuidFeature {
    leaf: 0x5,
    subleaf: 0,
    register: CpuidRegister::Ecx,
    shift: 0,
    width: 1,
};

/// Interrupts as a break event for MWAIT
pub const MWAIT_IRQ_BREAK: CpuidFeature = CpuidFeature {
    leaf: 0x5,
    subleaf: 0,
    register: CpuidRegister::Ecx,
    shift: 1,
    width: 1,
};

/// Number of C0 sub C-states
pub const N_C0_SUBSTATES: CpuidFeature = CpuidFeature {
    leaf: 0x5,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 0,
    width: 4,
};

/// Number of C1 sub C-states
pub const N_C1_SUBSTATES: CpuidFeature = CpuidFeature {
    leaf: 0x5,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 4,
    width: 4,
};

/// Number of C2 sub C-states
pub const N_C2_SUBSTATES: CpuidFeature = CpuidFeature {
    leaf: 0x5,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 8,
    width: 4,
};

/// Number of C3 sub C-states
pub const N_C3_SUBSTATES: CpuidFeature = CpuidFeature {
    leaf: 0x5,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 12,
    width: 4,
};

/// Number of C4 sub C-states
pub const N_C4_SUBSTATES: CpuidFeature = CpuidFeature {
    leaf: 0x5,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 16,
    width: 4,
};

/// Number of C5 sub C-states
pub const N_C5_SUBSTATES: CpuidFeature = CpuidFeature {
    leaf: 0x5,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 20,
    width: 4,
};

/// Number of C6 sub C-states
pub const N_C6_SUBSTATES: CpuidFeature = CpuidFeature {
    leaf: 0x5,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 24,
    width: 4,
};

/// Number of C7 sub C-states
pub const N_C7_SUBSTATES: CpuidFeature = CpuidFeature {
    leaf: 0x5,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 28,
    width: 4,
};

/// Max LBR stack depth bitmask
pub const LBR_DEPTH_MASK: CpuidFeature = CpuidFeature {
    leaf: 0x1c,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 0,
    width: 8,
};

/// LBRs may be cleared on MWAIT C-state > C1
pub const LBR_DEEP_C_RESET: CpuidFeature = CpuidFeature {
    leaf: 0x1c,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 30,
    width: 1,
};

/// LBR IP contain Last IP (otherwise effective IP)
pub const LBR_IP_IS_LIP: CpuidFeature = CpuidFeature {
    leaf: 0x1c,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 31,
    width: 1,
};

/// CPL filtering
pub const LBR_CPL: CpuidFeature = CpuidFeature {
    leaf: 0x1c,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 0,
    width: 1,
};

/// Branch filtering
pub const LBR_BRANCH_FILTER: CpuidFeature = CpuidFeature {
    leaf: 0x1c,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 1,
    width: 1,
};

/// Call-stack mode
pub const LBR_CALL_STACK: CpuidFeature = CpuidFeature {
    leaf: 0x1c,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 2,
    width: 1,
};

/// Branch misprediction bit
pub const LBR_MISPREDICT: CpuidFeature = CpuidFeature {
    leaf: 0x1c,
    subleaf: 0,
    register: CpuidRegister::Ecx,
    shift: 0,
    width: 1,
};

/// Timed LBRs (CPU cycles since last LBR entry)
pub const LBR_TIMED_LBR: CpuidFeature = CpuidFeature {
    leaf: 0x1c,
    subleaf: 0,
    register: CpuidRegister::Ecx,
    shift: 1,
    width: 1,
};

/// Branch type field
pub const LBR_BRANCH_TYPE: CpuidFeature = CpuidFeature {
    leaf: 0x1c,
    subleaf: 0,
    register: CpuidRegister::Ecx,
    shift: 2,
    width: 1,
};

/// PMU-events logging support
pub const LBR_EVENTS_GPC_BMP: CpuidFeature = CpuidFeature {
    leaf: 0x1c,
    subleaf: 0,
    register: CpuidRegister::Ecx,
    shift: 16,
    width: 4,
};

/// Bit width of this level (previous levels inclusive)
pub const X2APIC_ID_SHIFT: CpuidFeature = CpuidFeature {
    leaf: 0x1f,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 0,
    width: 5,
};

/// Logical CPUs count across all instances of this domain
pub const DOMAIN_LCPUS_COUNT: CpuidFeature = CpuidFeature {
    leaf: 0x1f,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 0,
    width: 16,
};

/// This domain level (subleaf ID)
pub const DOMAIN_LEVEL: CpuidFeature = CpuidFeature {
    leaf: 0x1f,
    subleaf: 0,
    register: CpuidRegister::Ecx,
    shift: 0,
    width: 8,
};

/// This domain type
pub const DOMAIN_TYPE: CpuidFeature = CpuidFeature {
    leaf: 0x1f,
    subleaf: 0,
    register: CpuidRegister::Ecx,
    shift: 8,
    width: 8,
};

/// x2APIC ID of current logical CPU
pub const X2APIC_ID: CpuidFeature = CpuidFeature {
    leaf: 0x1f,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 0,
    width: 32,
};

/// This domain level (subleaf ID)
pub const DOMAIN_NR: CpuidFeature = CpuidFeature {
    leaf: 0xb,
    subleaf: 0,
    register: CpuidRegister::Ecx,
    shift: 0,
    width: 8,
};

/// OS: LWP is available to application programs
pub const OS_LWP_AVAIL: CpuidFeature = CpuidFeature {
    leaf: 0x8000001c,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 0,
    width: 1,
};

/// OS: LWPVAL instruction
pub const OS_LWPVAL: CpuidFeature = CpuidFeature {
    leaf: 0x8000001c,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 1,
    width: 1,
};

/// OS: Instructions Retired Event
pub const OS_LWP_IRE: CpuidFeature = CpuidFeature {
    leaf: 0x8000001c,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 2,
    width: 1,
};

/// OS: Branch Retired Event
pub const OS_LWP_BRE: CpuidFeature = CpuidFeature {
    leaf: 0x8000001c,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 3,
    width: 1,
};

/// OS: Dcache Miss Event
pub const OS_LWP_DME: CpuidFeature = CpuidFeature {
    leaf: 0x8000001c,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 4,
    width: 1,
};

/// OS: CPU Clocks Not Halted event
pub const OS_LWP_CNH: CpuidFeature = CpuidFeature {
    leaf: 0x8000001c,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 5,
    width: 1,
};

/// OS: CPU Reference clocks Not Halted event
pub const OS_LWP_RNH: CpuidFeature = CpuidFeature {
    leaf: 0x8000001c,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 6,
    width: 1,
};

/// OS: LWP sampling in continuous mode
pub const OS_LWP_CONT: CpuidFeature = CpuidFeature {
    leaf: 0x8000001c,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 29,
    width: 1,
};

/// OS: Performance Time Stamp Counter in event records
pub const OS_LWP_PTSC: CpuidFeature = CpuidFeature {
    leaf: 0x8000001c,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 30,
    width: 1,
};

/// OS: Interrupt on threshold overflow
pub const OS_LWP_INT: CpuidFeature = CpuidFeature {
    leaf: 0x8000001c,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 31,
    width: 1,
};

/// Control Block size, in quadwords
pub const LWP_LWPCB_SZ: CpuidFeature = CpuidFeature {
    leaf: 0x8000001c,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 0,
    width: 8,
};

/// Event record size, in bytes
pub const LWP_EVENT_SZ: CpuidFeature = CpuidFeature {
    leaf: 0x8000001c,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 8,
    width: 8,
};

/// Max EventID supported
pub const LWP_MAX_EVENTS: CpuidFeature = CpuidFeature {
    leaf: 0x8000001c,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 16,
    width: 8,
};

/// Control Block events area offset
pub const LWP_EVENT_OFFSET: CpuidFeature = CpuidFeature {
    leaf: 0x8000001c,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 24,
    width: 8,
};

/// Cache latency counters number of bits
pub const LWP_LATENCY_MAX: CpuidFeature = CpuidFeature {
    leaf: 0x8000001c,
    subleaf: 0,
    register: CpuidRegister::Ecx,
    shift: 0,
    width: 5,
};

/// Cache miss events report data cache address
pub const LWP_DATA_ADDR: CpuidFeature = CpuidFeature {
    leaf: 0x8000001c,
    subleaf: 0,
    register: CpuidRegister::Ecx,
    shift: 5,
    width: 1,
};

/// Cache latency rounding amount
pub const LWP_LATENCY_RND: CpuidFeature = CpuidFeature {
    leaf: 0x8000001c,
    subleaf: 0,
    register: CpuidRegister::Ecx,
    shift: 6,
    width: 3,
};

/// LWP version
pub const LWP_VERSION: CpuidFeature = CpuidFeature {
    leaf: 0x8000001c,
    subleaf: 0,
    register: CpuidRegister::Ecx,
    shift: 9,
    width: 7,
};

/// LWP event ring buffer min size, 32 event record units
pub const LWP_BUF_MIN_SZ: CpuidFeature = CpuidFeature {
    leaf: 0x8000001c,
    subleaf: 0,
    register: CpuidRegister::Ecx,
    shift: 16,
    width: 8,
};

/// Branches Retired events can be filtered
pub const LWP_BRANCH_PREDICT: CpuidFeature = CpuidFeature {
    leaf: 0x8000001c,
    subleaf: 0,
    register: CpuidRegister::Ecx,
    shift: 28,
    width: 1,
};

/// IP filtering (IPI, IPF, BaseIP, and LimitIP @ LWPCP)
pub const LWP_IP_FILTERING: CpuidFeature = CpuidFeature {
    leaf: 0x8000001c,
    subleaf: 0,
    register: CpuidRegister::Ecx,
    shift: 29,
    width: 1,
};

/// Cache-related events: filter by cache level
pub const LWP_CACHE_LEVELS: CpuidFeature = CpuidFeature {
    leaf: 0x8000001c,
    subleaf: 0,
    register: CpuidRegister::Ecx,
    shift: 30,
    width: 1,
};

/// Cache-related events: filter by latency
pub const LWP_CACHE_LATENCY: CpuidFeature = CpuidFeature {
    leaf: 0x8000001c,
    subleaf: 0,
    register: CpuidRegister::Ecx,
    shift: 31,
    width: 1,
};

/// HW: LWP available
pub const HW_LWP_AVAIL: CpuidFeature = CpuidFeature {
    leaf: 0x8000001c,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 0,
    width: 1,
};

/// HW: LWPVAL available
pub const HW_LWPVAL: CpuidFeature = CpuidFeature {
    leaf: 0x8000001c,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 1,
    width: 1,
};

/// HW: Instructions Retired Event
pub const HW_LWP_IRE: CpuidFeature = CpuidFeature {
    leaf: 0x8000001c,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 2,
    width: 1,
};

/// HW: Branch Retired Event
pub const HW_LWP_BRE: CpuidFeature = CpuidFeature {
    leaf: 0x8000001c,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 3,
    width: 1,
};

/// HW: Dcache Miss Event
pub const HW_LWP_DME: CpuidFeature = CpuidFeature {
    leaf: 0x8000001c,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 4,
    width: 1,
};

/// HW: Clocks Not Halted event
pub const HW_LWP_CNH: CpuidFeature = CpuidFeature {
    leaf: 0x8000001c,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 5,
    width: 1,
};

/// HW: Reference clocks Not Halted event
pub const HW_LWP_RNH: CpuidFeature = CpuidFeature {
    leaf: 0x8000001c,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 6,
    width: 1,
};

/// HW: LWP sampling in continuous mode
pub const HW_LWP_CONT: CpuidFeature = CpuidFeature {
    leaf: 0x8000001c,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 29,
    width: 1,
};

/// HW: Performance Time Stamp Counter in event records
pub const HW_LWP_PTSC: CpuidFeature = CpuidFeature {
    leaf: 0x8000001c,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 30,
    width: 1,
};

/// HW: Interrupt on threshold overflow
pub const HW_LWP_INT: CpuidFeature = CpuidFeature {
    leaf: 0x8000001c,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 31,
    width: 1,
};

/// Extended APIC ID
pub const EXT_APIC_ID: CpuidFeature = CpuidFeature {
    leaf: 0x8000001e,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 0,
    width: 32,
};

/// Unique per-socket logical core unit ID
pub const CORE_ID: CpuidFeature = CpuidFeature {
    leaf: 0x8000001e,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 0,
    width: 8,
};

/// #Threads per core (zero-based)
pub const CORE_NTHREADS: CpuidFeature = CpuidFeature {
    leaf: 0x8000001e,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 8,
    width: 8,
};

/// Node (die) ID of invoking logical CPU
pub const NODE_ID: CpuidFeature = CpuidFeature {
    leaf: 0x8000001e,
    subleaf: 0,
    register: CpuidRegister::Ecx,
    shift: 0,
    width: 8,
};

/// #nodes in invoking logical CPU's package/socket
pub const NNODES_PER_SOCKET: CpuidFeature = CpuidFeature {
    leaf: 0x8000001e,
    subleaf: 0,
    register: CpuidRegister::Ecx,
    shift: 8,
    width: 3,
};

/// Processor base frequency, in MHz
pub const CPU_BASE_MHZ: CpuidFeature = CpuidFeature {
    leaf: 0x16,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 0,
    width: 16,
};

/// Processor max frequency, in MHz
pub const CPU_MAX_MHZ: CpuidFeature = CpuidFeature {
    leaf: 0x16,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 0,
    width: 16,
};

/// Bus reference frequency, in MHz
pub const BUS_MHZ: CpuidFeature = CpuidFeature {
    leaf: 0x16,
    subleaf: 0,
    register: CpuidRegister::Ecx,
    shift: 0,
    width: 16,
};

/// Maximum Transmeta leaf
pub const MAX_TRA_LEAF: CpuidFeature = CpuidFeature {
    leaf: 0x80860000,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 0,
    width: 32,
};

/// Transmeta vendor ID string bytes 0 - 3
pub const CPU_VENDORID_0: CpuidFeature = CpuidFeature {
    leaf: 0x80860000,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 0,
    width: 32,
};

/// Transmeta vendor ID string bytes 8 - 11
pub const CPU_VENDORID_2: CpuidFeature = CpuidFeature {
    leaf: 0x80860000,
    subleaf: 0,
    register: CpuidRegister::Ecx,
    shift: 0,
    width: 32,
};

/// Transmeta vendor ID string bytes 4 - 7
pub const CPU_VENDORID_1: CpuidFeature = CpuidFeature {
    leaf: 0x80860000,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 0,
    width: 32,
};

/// Denominator of the TSC/'core crystal clock' ratio
pub const TSC_DENOMINATOR: CpuidFeature = CpuidFeature {
    leaf: 0x15,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 0,
    width: 32,
};

/// Numerator of the TSC/'core crystal clock' ratio
pub const TSC_NUMERATOR: CpuidFeature = CpuidFeature {
    leaf: 0x15,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 0,
    width: 32,
};

/// Core crystal clock nominal frequency, in Hz
pub const CPU_CRYSTAL_HZ: CpuidFeature = CpuidFeature {
    leaf: 0x15,
    subleaf: 0,
    register: CpuidRegister::Ecx,
    shift: 0,
    width: 32,
};

/// Number of leaf 0x7 subleaves
pub const LEAF7_N_SUBLEAVES: CpuidFeature = CpuidFeature {
    leaf: 0x7,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 0,
    width: 32,
};

/// FSBASE/GSBASE read/write
pub const X86_FEATURE_FSGSBASE: CpuidFeature = CpuidFeature {
    leaf: 0x7,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 0,
    width: 1,
};

/// IA32_TSC_ADJUST MSR
pub const X86_FEATURE_TSC_ADJUST: CpuidFeature = CpuidFeature {
    leaf: 0x7,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 1,
    width: 1,
};

/// Intel SGX (Software Guard Extensions)
pub const X86_FEATURE_SGX: CpuidFeature = CpuidFeature {
    leaf: 0x7,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 2,
    width: 1,
};

/// Bit manipulation extensions group 1
pub const X86_FEATURE_BMI1: CpuidFeature = CpuidFeature {
    leaf: 0x7,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 3,
    width: 1,
};

/// Hardware Lock Elision
pub const X86_FEATURE_HLE: CpuidFeature = CpuidFeature {
    leaf: 0x7,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 4,
    width: 1,
};

/// AVX2 instruction set
pub const X86_FEATURE_AVX2: CpuidFeature = CpuidFeature {
    leaf: 0x7,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 5,
    width: 1,
};

/// FPU Data Pointer updated only on x87 exceptions
pub const X86_FEATURE_FDP_EXCPTN_ONLY: CpuidFeature = CpuidFeature {
    leaf: 0x7,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 6,
    width: 1,
};

/// Supervisor Mode Execution Protection
pub const X86_FEATURE_SMEP: CpuidFeature = CpuidFeature {
    leaf: 0x7,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 7,
    width: 1,
};

/// Bit manipulation extensions group 2
pub const X86_FEATURE_BMI2: CpuidFeature = CpuidFeature {
    leaf: 0x7,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 8,
    width: 1,
};

/// Enhanced REP MOVSB/STOSB
pub const X86_FEATURE_ERMS: CpuidFeature = CpuidFeature {
    leaf: 0x7,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 9,
    width: 1,
};

/// INVPCID instruction (Invalidate Processor Context ID)
pub const X86_FEATURE_INVPCID: CpuidFeature = CpuidFeature {
    leaf: 0x7,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 10,
    width: 1,
};

/// Intel restricted transactional memory
pub const X86_FEATURE_RTM: CpuidFeature = CpuidFeature {
    leaf: 0x7,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 11,
    width: 1,
};

/// Intel RDT-CMT / AMD Platform-QoS cache monitoring
pub const X86_FEATURE_CQM: CpuidFeature = CpuidFeature {
    leaf: 0x7,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 12,
    width: 1,
};

/// Deprecated FPU CS/DS (stored as zero)
pub const X86_FEATURE_ZERO_FCS_FDS: CpuidFeature = CpuidFeature {
    leaf: 0x7,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 13,
    width: 1,
};

/// Intel memory protection extensions
pub const X86_FEATURE_MPX: CpuidFeature = CpuidFeature {
    leaf: 0x7,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 14,
    width: 1,
};

/// Intel RDT / AMD Platform-QoS Enforcement
pub const X86_FEATURE_RDT_A: CpuidFeature = CpuidFeature {
    leaf: 0x7,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 15,
    width: 1,
};

/// AVX-512 foundation instructions
pub const X86_FEATURE_AVX512F: CpuidFeature = CpuidFeature {
    leaf: 0x7,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 16,
    width: 1,
};

/// AVX-512 double/quadword instructions
pub const X86_FEATURE_AVX512DQ: CpuidFeature = CpuidFeature {
    leaf: 0x7,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 17,
    width: 1,
};

/// RDSEED instruction
pub const X86_FEATURE_RDSEED: CpuidFeature = CpuidFeature {
    leaf: 0x7,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 18,
    width: 1,
};

/// ADCX/ADOX instructions
pub const X86_FEATURE_ADX: CpuidFeature = CpuidFeature {
    leaf: 0x7,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 19,
    width: 1,
};

/// Supervisor mode access prevention
pub const X86_FEATURE_SMAP: CpuidFeature = CpuidFeature {
    leaf: 0x7,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 20,
    width: 1,
};

/// AVX-512 integer fused multiply add
pub const X86_FEATURE_AVX512IFMA: CpuidFeature = CpuidFeature {
    leaf: 0x7,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 21,
    width: 1,
};

/// CLFLUSHOPT instruction
pub const X86_FEATURE_CLFLUSHOPT: CpuidFeature = CpuidFeature {
    leaf: 0x7,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 23,
    width: 1,
};

/// CLWB instruction
pub const X86_FEATURE_CLWB: CpuidFeature = CpuidFeature {
    leaf: 0x7,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 24,
    width: 1,
};

/// Intel processor trace
pub const X86_FEATURE_INTEL_PT: CpuidFeature = CpuidFeature {
    leaf: 0x7,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 25,
    width: 1,
};

/// AVX-512 prefetch instructions
pub const X86_FEATURE_AVX512PF: CpuidFeature = CpuidFeature {
    leaf: 0x7,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 26,
    width: 1,
};

/// AVX-512 exponent/reciprocal instructions
pub const X86_FEATURE_AVX512ER: CpuidFeature = CpuidFeature {
    leaf: 0x7,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 27,
    width: 1,
};

/// AVX-512 conflict detection instructions
pub const X86_FEATURE_AVX512CD: CpuidFeature = CpuidFeature {
    leaf: 0x7,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 28,
    width: 1,
};

/// SHA/SHA256 instructions
pub const X86_FEATURE_SHA_NI: CpuidFeature = CpuidFeature {
    leaf: 0x7,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 29,
    width: 1,
};

/// AVX-512 byte/word instructions
pub const X86_FEATURE_AVX512BW: CpuidFeature = CpuidFeature {
    leaf: 0x7,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 30,
    width: 1,
};

/// AVX-512 VL (128/256 vector length) extensions
pub const X86_FEATURE_AVX512VL: CpuidFeature = CpuidFeature {
    leaf: 0x7,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 31,
    width: 1,
};

/// PREFETCHWT1 (Intel Xeon Phi only)
pub const PREFETCHWT1: CpuidFeature = CpuidFeature {
    leaf: 0x7,
    subleaf: 0,
    register: CpuidRegister::Ecx,
    shift: 0,
    width: 1,
};

/// AVX-512 Vector byte manipulation instructions
pub const X86_FEATURE_AVX512VBMI: CpuidFeature = CpuidFeature {
    leaf: 0x7,
    subleaf: 0,
    register: CpuidRegister::Ecx,
    shift: 1,
    width: 1,
};

/// User mode instruction protection
pub const X86_FEATURE_UMIP: CpuidFeature = CpuidFeature {
    leaf: 0x7,
    subleaf: 0,
    register: CpuidRegister::Ecx,
    shift: 2,
    width: 1,
};

/// Protection keys for user-space
pub const X86_FEATURE_PKU: CpuidFeature = CpuidFeature {
    leaf: 0x7,
    subleaf: 0,
    register: CpuidRegister::Ecx,
    shift: 3,
    width: 1,
};

/// OS protection keys enable
pub const X86_FEATURE_OSPKE: CpuidFeature = CpuidFeature {
    leaf: 0x7,
    subleaf: 0,
    register: CpuidRegister::Ecx,
    shift: 4,
    width: 1,
};

/// WAITPKG instructions
pub const X86_FEATURE_WAITPKG: CpuidFeature = CpuidFeature {
    leaf: 0x7,
    subleaf: 0,
    register: CpuidRegister::Ecx,
    shift: 5,
    width: 1,
};

/// AVX-512 vector byte manipulation instructions group 2
pub const X86_FEATURE_AVX512_VBMI2: CpuidFeature = CpuidFeature {
    leaf: 0x7,
    subleaf: 0,
    register: CpuidRegister::Ecx,
    shift: 6,
    width: 1,
};

/// CET shadow stack features
pub const CET_SS: CpuidFeature = CpuidFeature {
    leaf: 0x7,
    subleaf: 0,
    register: CpuidRegister::Ecx,
    shift: 7,
    width: 1,
};

/// Galois field new instructions
pub const X86_FEATURE_GFNI: CpuidFeature = CpuidFeature {
    leaf: 0x7,
    subleaf: 0,
    register: CpuidRegister::Ecx,
    shift: 8,
    width: 1,
};

/// Vector AES instructions
pub const X86_FEATURE_VAES: CpuidFeature = CpuidFeature {
    leaf: 0x7,
    subleaf: 0,
    register: CpuidRegister::Ecx,
    shift: 9,
    width: 1,
};

/// VPCLMULQDQ 256-bit instruction
pub const X86_FEATURE_VPCLMULQDQ: CpuidFeature = CpuidFeature {
    leaf: 0x7,
    subleaf: 0,
    register: CpuidRegister::Ecx,
    shift: 10,
    width: 1,
};

/// Vector neural network instructions
pub const X86_FEATURE_AVX512_VNNI: CpuidFeature = CpuidFeature {
    leaf: 0x7,
    subleaf: 0,
    register: CpuidRegister::Ecx,
    shift: 11,
    width: 1,
};

/// AVX-512 bitwise algorithms
pub const X86_FEATURE_AVX512_BITALG: CpuidFeature = CpuidFeature {
    leaf: 0x7,
    subleaf: 0,
    register: CpuidRegister::Ecx,
    shift: 12,
    width: 1,
};

/// Intel total memory encryption
pub const X86_FEATURE_TME: CpuidFeature = CpuidFeature {
    leaf: 0x7,
    subleaf: 0,
    register: CpuidRegister::Ecx,
    shift: 13,
    width: 1,
};

/// AVX-512: POPCNT for vectors of DWORD/QWORD
pub const X86_FEATURE_AVX512_VPOPCNTDQ: CpuidFeature = CpuidFeature {
    leaf: 0x7,
    subleaf: 0,
    register: CpuidRegister::Ecx,
    shift: 14,
    width: 1,
};

/// 57-bit linear addresses (five-level paging)
pub const X86_FEATURE_LA57: CpuidFeature = CpuidFeature {
    leaf: 0x7,
    subleaf: 0,
    register: CpuidRegister::Ecx,
    shift: 16,
    width: 1,
};

/// BNDLDX/BNDSTX MAWAU value in 64-bit mode
pub const MAWAU_VAL_LM: CpuidFeature = CpuidFeature {
    leaf: 0x7,
    subleaf: 0,
    register: CpuidRegister::Ecx,
    shift: 17,
    width: 5,
};

/// RDPID instruction
pub const X86_FEATURE_RDPID: CpuidFeature = CpuidFeature {
    leaf: 0x7,
    subleaf: 0,
    register: CpuidRegister::Ecx,
    shift: 22,
    width: 1,
};

/// Intel key locker
pub const KEY_LOCKER: CpuidFeature = CpuidFeature {
    leaf: 0x7,
    subleaf: 0,
    register: CpuidRegister::Ecx,
    shift: 23,
    width: 1,
};

/// OS bus-lock detection
pub const X86_FEATURE_BUS_LOCK_DETECT: CpuidFeature = CpuidFeature {
    leaf: 0x7,
    subleaf: 0,
    register: CpuidRegister::Ecx,
    shift: 24,
    width: 1,
};

/// CLDEMOTE instruction
pub const X86_FEATURE_CLDEMOTE: CpuidFeature = CpuidFeature {
    leaf: 0x7,
    subleaf: 0,
    register: CpuidRegister::Ecx,
    shift: 25,
    width: 1,
};

/// MOVDIRI instruction
pub const X86_FEATURE_MOVDIRI: CpuidFeature = CpuidFeature {
    leaf: 0x7,
    subleaf: 0,
    register: CpuidRegister::Ecx,
    shift: 27,
    width: 1,
};

/// MOVDIR64B instruction
pub const X86_FEATURE_MOVDIR64B: CpuidFeature = CpuidFeature {
    leaf: 0x7,
    subleaf: 0,
    register: CpuidRegister::Ecx,
    shift: 28,
    width: 1,
};

/// Enqueue stores (ENQCMD{,S})
pub const X86_FEATURE_ENQCMD: CpuidFeature = CpuidFeature {
    leaf: 0x7,
    subleaf: 0,
    register: CpuidRegister::Ecx,
    shift: 29,
    width: 1,
};

/// Intel SGX launch configuration
pub const X86_FEATURE_SGX_LC: CpuidFeature = CpuidFeature {
    leaf: 0x7,
    subleaf: 0,
    register: CpuidRegister::Ecx,
    shift: 30,
    width: 1,
};

/// Protection keys for supervisor-mode pages
pub const PKS: CpuidFeature = CpuidFeature {
    leaf: 0x7,
    subleaf: 0,
    register: CpuidRegister::Ecx,
    shift: 31,
    width: 1,
};

/// Intel SGX attestation services
pub const SGX_KEYS: CpuidFeature = CpuidFeature {
    leaf: 0x7,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 1,
    width: 1,
};

/// AVX-512 neural network instructions
pub const X86_FEATURE_AVX512_4VNNIW: CpuidFeature = CpuidFeature {
    leaf: 0x7,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 2,
    width: 1,
};

/// AVX-512 multiply accumulation single precision
pub const X86_FEATURE_AVX512_4FMAPS: CpuidFeature = CpuidFeature {
    leaf: 0x7,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 3,
    width: 1,
};

/// Fast short REP MOVSB
pub const X86_FEATURE_FSRM: CpuidFeature = CpuidFeature {
    leaf: 0x7,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 4,
    width: 1,
};

/// User interrupts
pub const UINTR: CpuidFeature = CpuidFeature {
    leaf: 0x7,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 5,
    width: 1,
};

/// VP2INTERSECT{D,Q} instructions
pub const X86_FEATURE_AVX512_VP2INTERSECT: CpuidFeature = CpuidFeature {
    leaf: 0x7,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 8,
    width: 1,
};

/// SRBDS mitigation MSR
pub const X86_FEATURE_SRBDS_CTRL: CpuidFeature = CpuidFeature {
    leaf: 0x7,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 9,
    width: 1,
};

/// VERW MD_CLEAR microcode
pub const X86_FEATURE_MD_CLEAR: CpuidFeature = CpuidFeature {
    leaf: 0x7,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 10,
    width: 1,
};

/// XBEGIN (RTM transaction) always aborts
pub const X86_FEATURE_RTM_ALWAYS_ABORT: CpuidFeature = CpuidFeature {
    leaf: 0x7,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 11,
    width: 1,
};

/// MSR TSX_FORCE_ABORT, RTM_ABORT bit
pub const X86_FEATURE_TSX_FORCE_ABORT: CpuidFeature = CpuidFeature {
    leaf: 0x7,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 13,
    width: 1,
};

/// SERIALIZE instruction
pub const X86_FEATURE_SERIALIZE: CpuidFeature = CpuidFeature {
    leaf: 0x7,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 14,
    width: 1,
};

/// The CPU is identified as a 'hybrid part'
pub const X86_FEATURE_HYBRID_CPU: CpuidFeature = CpuidFeature {
    leaf: 0x7,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 15,
    width: 1,
};

/// TSX suspend/resume load address tracking
pub const X86_FEATURE_TSXLDTRK: CpuidFeature = CpuidFeature {
    leaf: 0x7,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 16,
    width: 1,
};

/// PCONFIG instruction
pub const X86_FEATURE_PCONFIG: CpuidFeature = CpuidFeature {
    leaf: 0x7,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 18,
    width: 1,
};

/// Intel architectural LBRs
pub const X86_FEATURE_ARCH_LBR: CpuidFeature = CpuidFeature {
    leaf: 0x7,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 19,
    width: 1,
};

/// CET indirect branch tracking
pub const X86_FEATURE_IBT: CpuidFeature = CpuidFeature {
    leaf: 0x7,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 20,
    width: 1,
};

/// AMX-BF16: tile bfloat16
pub const X86_FEATURE_AMX_BF16: CpuidFeature = CpuidFeature {
    leaf: 0x7,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 22,
    width: 1,
};

/// AVX-512 FP16 instructions
pub const X86_FEATURE_AVX512_FP16: CpuidFeature = CpuidFeature {
    leaf: 0x7,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 23,
    width: 1,
};

/// AMX-TILE: tile architecture
pub const X86_FEATURE_AMX_TILE: CpuidFeature = CpuidFeature {
    leaf: 0x7,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 24,
    width: 1,
};

/// AMX-INT8: tile 8-bit integer
pub const X86_FEATURE_AMX_INT8: CpuidFeature = CpuidFeature {
    leaf: 0x7,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 25,
    width: 1,
};

/// Speculation Control (IBRS/IBPB: indirect branch restrictions)
pub const X86_FEATURE_SPEC_CTRL: CpuidFeature = CpuidFeature {
    leaf: 0x7,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 26,
    width: 1,
};

/// Single thread indirect branch predictors
pub const X86_FEATURE_INTEL_STIBP: CpuidFeature = CpuidFeature {
    leaf: 0x7,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 27,
    width: 1,
};

/// FLUSH L1D cache: IA32_FLUSH_CMD MSR
pub const X86_FEATURE_FLUSH_L1D: CpuidFeature = CpuidFeature {
    leaf: 0x7,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 28,
    width: 1,
};

/// Intel IA32_ARCH_CAPABILITIES MSR
pub const X86_FEATURE_ARCH_CAPABILITIES: CpuidFeature = CpuidFeature {
    leaf: 0x7,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 29,
    width: 1,
};

/// IA32_CORE_CAPABILITIES MSR
pub const X86_FEATURE_CORE_CAPABILITIES: CpuidFeature = CpuidFeature {
    leaf: 0x7,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 30,
    width: 1,
};

/// Speculative store bypass disable
pub const X86_FEATURE_SPEC_CTRL_SSBD: CpuidFeature = CpuidFeature {
    leaf: 0x7,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 31,
    width: 1,
};

/// AVX-VNNI instructions
pub const X86_FEATURE_AVX_VNNI: CpuidFeature = CpuidFeature {
    leaf: 0x7,
    subleaf: 1,
    register: CpuidRegister::Eax,
    shift: 4,
    width: 1,
};

/// AVX-512 bfloat16 instructions
pub const X86_FEATURE_AVX512_BF16: CpuidFeature = CpuidFeature {
    leaf: 0x7,
    subleaf: 1,
    register: CpuidRegister::Eax,
    shift: 5,
    width: 1,
};

/// Linear address space separation
pub const LASS: CpuidFeature = CpuidFeature {
    leaf: 0x7,
    subleaf: 1,
    register: CpuidRegister::Eax,
    shift: 6,
    width: 1,
};

/// CMPccXADD instructions
pub const X86_FEATURE_CMPCCXADD: CpuidFeature = CpuidFeature {
    leaf: 0x7,
    subleaf: 1,
    register: CpuidRegister::Eax,
    shift: 7,
    width: 1,
};

/// ArchPerfmonExt: leaf 0x23
pub const X86_FEATURE_ARCH_PERFMON_EXT: CpuidFeature = CpuidFeature {
    leaf: 0x7,
    subleaf: 1,
    register: CpuidRegister::Eax,
    shift: 8,
    width: 1,
};

/// Fast zero-length REP MOVSB
pub const X86_FEATURE_FZRM: CpuidFeature = CpuidFeature {
    leaf: 0x7,
    subleaf: 1,
    register: CpuidRegister::Eax,
    shift: 10,
    width: 1,
};

/// Fast short REP STOSB
pub const X86_FEATURE_FSRS: CpuidFeature = CpuidFeature {
    leaf: 0x7,
    subleaf: 1,
    register: CpuidRegister::Eax,
    shift: 11,
    width: 1,
};

/// Fast Short REP CMPSB/SCASB
pub const X86_FEATURE_FSRC: CpuidFeature = CpuidFeature {
    leaf: 0x7,
    subleaf: 1,
    register: CpuidRegister::Eax,
    shift: 12,
    width: 1,
};

/// FRED: Flexible return and event delivery transitions
pub const FRED: CpuidFeature = CpuidFeature {
    leaf: 0x7,
    subleaf: 1,
    register: CpuidRegister::Eax,
    shift: 17,
    width: 1,
};

/// LKGS: Load 'kernel' (userspace) GS
pub const X86_FEATURE_LKGS: CpuidFeature = CpuidFeature {
    leaf: 0x7,
    subleaf: 1,
    register: CpuidRegister::Eax,
    shift: 18,
    width: 1,
};

/// WRMSRNS instruction (WRMSR-non-serializing)
pub const WRMSRNS: CpuidFeature = CpuidFeature {
    leaf: 0x7,
    subleaf: 1,
    register: CpuidRegister::Eax,
    shift: 19,
    width: 1,
};

/// NMI-source reporting with FRED event data
pub const NMI_SRC: CpuidFeature = CpuidFeature {
    leaf: 0x7,
    subleaf: 1,
    register: CpuidRegister::Eax,
    shift: 20,
    width: 1,
};

/// AMX-FP16: FP16 tile operations
pub const X86_FEATURE_AMX_FP16: CpuidFeature = CpuidFeature {
    leaf: 0x7,
    subleaf: 1,
    register: CpuidRegister::Eax,
    shift: 21,
    width: 1,
};

/// HRESET (Thread director history reset)
pub const HRESET: CpuidFeature = CpuidFeature {
    leaf: 0x7,
    subleaf: 1,
    register: CpuidRegister::Eax,
    shift: 22,
    width: 1,
};

/// Integer fused multiply add
pub const X86_FEATURE_AVX_IFMA: CpuidFeature = CpuidFeature {
    leaf: 0x7,
    subleaf: 1,
    register: CpuidRegister::Eax,
    shift: 23,
    width: 1,
};

/// Linear address masking
pub const X86_FEATURE_LAM: CpuidFeature = CpuidFeature {
    leaf: 0x7,
    subleaf: 1,
    register: CpuidRegister::Eax,
    shift: 26,
    width: 1,
};

/// RDMSRLIST/WRMSRLIST instructions
pub const RD_WR_MSRLIST: CpuidFeature = CpuidFeature {
    leaf: 0x7,
    subleaf: 1,
    register: CpuidRegister::Eax,
    shift: 27,
    width: 1,
};

/// Protected processor inventory number (PPIN{,_CTL} MSRs)
pub const X86_FEATURE_INTEL_PPIN: CpuidFeature = CpuidFeature {
    leaf: 0x7,
    subleaf: 1,
    register: CpuidRegister::Ebx,
    shift: 0,
    width: 1,
};

/// AVX-VNNI-INT8 instructions
pub const AVX_VNNI_INT8: CpuidFeature = CpuidFeature {
    leaf: 0x7,
    subleaf: 1,
    register: CpuidRegister::Edx,
    shift: 4,
    width: 1,
};

/// AVX-NE-CONVERT instructions
pub const AVX_NE_CONVERT: CpuidFeature = CpuidFeature {
    leaf: 0x7,
    subleaf: 1,
    register: CpuidRegister::Edx,
    shift: 5,
    width: 1,
};

/// AMX-COMPLEX instructions (starting from Granite Rapids)
pub const AMX_COMPLEX: CpuidFeature = CpuidFeature {
    leaf: 0x7,
    subleaf: 1,
    register: CpuidRegister::Edx,
    shift: 8,
    width: 1,
};

/// PREFETCHIT0/1 instructions
pub const PREFETCHIT_0_1: CpuidFeature = CpuidFeature {
    leaf: 0x7,
    subleaf: 1,
    register: CpuidRegister::Edx,
    shift: 14,
    width: 1,
};

/// CET supervisor shadow stacks safe to use
pub const CET_SSS: CpuidFeature = CpuidFeature {
    leaf: 0x7,
    subleaf: 1,
    register: CpuidRegister::Edx,
    shift: 18,
    width: 1,
};

/// Intel predictive store forward disable
pub const INTEL_PSFD: CpuidFeature = CpuidFeature {
    leaf: 0x7,
    subleaf: 2,
    register: CpuidRegister::Edx,
    shift: 0,
    width: 1,
};

/// MSR bits IA32_SPEC_CTRL.IPRED_DIS_{U,S}
pub const IPRED_CTRL: CpuidFeature = CpuidFeature {
    leaf: 0x7,
    subleaf: 2,
    register: CpuidRegister::Edx,
    shift: 1,
    width: 1,
};

/// MSR bits IA32_SPEC_CTRL.RRSBA_DIS_{U,S}
pub const X86_FEATURE_RRSBA_CTRL: CpuidFeature = CpuidFeature {
    leaf: 0x7,
    subleaf: 2,
    register: CpuidRegister::Edx,
    shift: 2,
    width: 1,
};

/// MSR bit IA32_SPEC_CTRL.DDPD_U
pub const DDP_CTRL: CpuidFeature = CpuidFeature {
    leaf: 0x7,
    subleaf: 2,
    register: CpuidRegister::Edx,
    shift: 3,
    width: 1,
};

/// MSR bit IA32_SPEC_CTRL.BHI_DIS_S
pub const BHI_CTRL: CpuidFeature = CpuidFeature {
    leaf: 0x7,
    subleaf: 2,
    register: CpuidRegister::Edx,
    shift: 4,
    width: 1,
};

/// MCDT mitigation not needed
pub const MCDT_NO: CpuidFeature = CpuidFeature {
    leaf: 0x7,
    subleaf: 2,
    register: CpuidRegister::Edx,
    shift: 5,
    width: 1,
};

/// UC-lock disable
pub const UCLOCK_DISABLE: CpuidFeature = CpuidFeature {
    leaf: 0x7,
    subleaf: 2,
    register: CpuidRegister::Edx,
    shift: 6,
    width: 1,
};

/// SVM revision number
pub const SVM_VERSION: CpuidFeature = CpuidFeature {
    leaf: 0x8000000a,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 0,
    width: 8,
};

/// Number of address space identifiers (ASID)
pub const SVM_NASID: CpuidFeature = CpuidFeature {
    leaf: 0x8000000a,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 0,
    width: 32,
};

/// Page Modification Logging (PML)
pub const X86_FEATURE_PML: CpuidFeature = CpuidFeature {
    leaf: 0x8000000a,
    subleaf: 0,
    register: CpuidRegister::Ecx,
    shift: 4,
    width: 1,
};

/// Nested paging
pub const X86_FEATURE_NPT: CpuidFeature = CpuidFeature {
    leaf: 0x8000000a,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 0,
    width: 1,
};

/// LBR virtualization
pub const X86_FEATURE_LBRV: CpuidFeature = CpuidFeature {
    leaf: 0x8000000a,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 1,
    width: 1,
};

/// SVM lock
pub const X86_FEATURE_SVML: CpuidFeature = CpuidFeature {
    leaf: 0x8000000a,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 2,
    width: 1,
};

/// NRIP save support on #VMEXIT
pub const X86_FEATURE_NRIPS: CpuidFeature = CpuidFeature {
    leaf: 0x8000000a,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 3,
    width: 1,
};

/// MSR-based TSC rate control
pub const X86_FEATURE_TSCRATEMSR: CpuidFeature = CpuidFeature {
    leaf: 0x8000000a,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 4,
    width: 1,
};

/// VMCB clean bits support
pub const X86_FEATURE_VMCBCLEAN: CpuidFeature = CpuidFeature {
    leaf: 0x8000000a,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 5,
    width: 1,
};

/// Flush by ASID + Extended VMCB TLB_Control
pub const X86_FEATURE_FLUSHBYASID: CpuidFeature = CpuidFeature {
    leaf: 0x8000000a,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 6,
    width: 1,
};

/// Decode Assists support
pub const X86_FEATURE_DECODEASSISTS: CpuidFeature = CpuidFeature {
    leaf: 0x8000000a,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 7,
    width: 1,
};

/// Pause intercept filter
pub const X86_FEATURE_PAUSEFILTER: CpuidFeature = CpuidFeature {
    leaf: 0x8000000a,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 10,
    width: 1,
};

/// Pause filter threshold
pub const X86_FEATURE_PFTHRESHOLD: CpuidFeature = CpuidFeature {
    leaf: 0x8000000a,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 12,
    width: 1,
};

/// Advanced virtual interrupt controller
pub const X86_FEATURE_AVIC: CpuidFeature = CpuidFeature {
    leaf: 0x8000000a,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 13,
    width: 1,
};

/// Virtual VMSAVE/VMLOAD (nested virtualization)
pub const X86_FEATURE_V_VMSAVE_VMLOAD: CpuidFeature = CpuidFeature {
    leaf: 0x8000000a,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 15,
    width: 1,
};

/// Virtualize the Global Interrupt Flag
pub const X86_FEATURE_VGIF: CpuidFeature = CpuidFeature {
    leaf: 0x8000000a,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 16,
    width: 1,
};

/// Guest mode execution trap
pub const GMET: CpuidFeature = CpuidFeature {
    leaf: 0x8000000a,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 17,
    width: 1,
};

/// Virtual x2APIC
pub const X86_FEATURE_X2AVIC: CpuidFeature = CpuidFeature {
    leaf: 0x8000000a,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 18,
    width: 1,
};

/// Supervisor Shadow Stack restrictions
pub const SSS_CHECK: CpuidFeature = CpuidFeature {
    leaf: 0x8000000a,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 19,
    width: 1,
};

/// Virtual SPEC_CTRL
pub const X86_FEATURE_V_SPEC_CTRL: CpuidFeature = CpuidFeature {
    leaf: 0x8000000a,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 20,
    width: 1,
};

/// Read-Only guest page table support
pub const RO_GPT: CpuidFeature = CpuidFeature {
    leaf: 0x8000000a,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 21,
    width: 1,
};

/// Host MCE override
pub const H_MCE_OVERRIDE: CpuidFeature = CpuidFeature {
    leaf: 0x8000000a,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 23,
    width: 1,
};

/// TLBSYNC intercept + INVLPGB/TLBSYNC in VMCB
pub const TLBSYNC_INT: CpuidFeature = CpuidFeature {
    leaf: 0x8000000a,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 24,
    width: 1,
};

/// NMI virtualization
pub const X86_FEATURE_VNMI: CpuidFeature = CpuidFeature {
    leaf: 0x8000000a,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 25,
    width: 1,
};

/// IBS Virtualization
pub const IBS_VIRT: CpuidFeature = CpuidFeature {
    leaf: 0x8000000a,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 26,
    width: 1,
};

/// Extended LVT offset fault change
pub const EXT_LVT_OFF_CHG: CpuidFeature = CpuidFeature {
    leaf: 0x8000000a,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 27,
    width: 1,
};

/// Guest SVME address check
pub const X86_FEATURE_SVME_ADDR_CHK: CpuidFeature = CpuidFeature {
    leaf: 0x8000000a,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 28,
    width: 1,
};

/// TDX vendor ID string bytes 0 - 3
pub const TDX_VENDORID_0: CpuidFeature = CpuidFeature {
    leaf: 0x21,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 0,
    width: 32,
};

/// TDX vendor ID string bytes 8 - 11
pub const TDX_VENDORID_2: CpuidFeature = CpuidFeature {
    leaf: 0x21,
    subleaf: 0,
    register: CpuidRegister::Ecx,
    shift: 0,
    width: 32,
};

/// TDX vendor ID string bytes 4 - 7
pub const TDX_VENDORID_1: CpuidFeature = CpuidFeature {
    leaf: 0x21,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 0,
    width: 32,
};

/// CCS SM2 instructions
pub const CCS_SM2: CpuidFeature = CpuidFeature {
    leaf: 0xc0000001,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 0,
    width: 1,
};

/// CCS SM2 enabled
pub const CCS_SM2_EN: CpuidFeature = CpuidFeature {
    leaf: 0xc0000001,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 1,
    width: 1,
};

/// Random Number Generator
pub const X86_FEATURE_XSTORE: CpuidFeature = CpuidFeature {
    leaf: 0xc0000001,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 2,
    width: 1,
};

/// RNG enabled
pub const X86_FEATURE_XSTORE_EN: CpuidFeature = CpuidFeature {
    leaf: 0xc0000001,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 3,
    width: 1,
};

/// CCS SM3 and SM4 instructions
pub const CCS_SM3_SM4: CpuidFeature = CpuidFeature {
    leaf: 0xc0000001,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 4,
    width: 1,
};

/// CCS SM3/SM4 enabled
pub const CCS_SM3_SM4_EN: CpuidFeature = CpuidFeature {
    leaf: 0xc0000001,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 5,
    width: 1,
};

/// Advanced Cryptography Engine
pub const X86_FEATURE_XCRYPT: CpuidFeature = CpuidFeature {
    leaf: 0xc0000001,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 6,
    width: 1,
};

/// ACE enabled
pub const X86_FEATURE_XCRYPT_EN: CpuidFeature = CpuidFeature {
    leaf: 0xc0000001,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 7,
    width: 1,
};

/// Advanced Cryptography Engine v2
pub const X86_FEATURE_ACE2: CpuidFeature = CpuidFeature {
    leaf: 0xc0000001,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 8,
    width: 1,
};

/// ACE v2 enabled
pub const X86_FEATURE_ACE2_EN: CpuidFeature = CpuidFeature {
    leaf: 0xc0000001,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 9,
    width: 1,
};

/// PadLock Hash Engine
pub const X86_FEATURE_PHE: CpuidFeature = CpuidFeature {
    leaf: 0xc0000001,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 10,
    width: 1,
};

/// PHE enabled
pub const X86_FEATURE_PHE_EN: CpuidFeature = CpuidFeature {
    leaf: 0xc0000001,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 11,
    width: 1,
};

/// PadLock Montgomery Multiplier
pub const X86_FEATURE_PMM: CpuidFeature = CpuidFeature {
    leaf: 0xc0000001,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 12,
    width: 1,
};

/// PMM enabled
pub const X86_FEATURE_PMM_EN: CpuidFeature = CpuidFeature {
    leaf: 0xc0000001,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 13,
    width: 1,
};

/// Parallax auto adjust processor voltage
pub const PARALLAX: CpuidFeature = CpuidFeature {
    leaf: 0xc0000001,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 16,
    width: 1,
};

/// Parallax enabled
pub const PARALLAX_EN: CpuidFeature = CpuidFeature {
    leaf: 0xc0000001,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 17,
    width: 1,
};

/// Thermal Monitor v3
pub const TM3: CpuidFeature = CpuidFeature {
    leaf: 0xc0000001,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 20,
    width: 1,
};

/// TM v3 enabled
pub const TM3_EN: CpuidFeature = CpuidFeature {
    leaf: 0xc0000001,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 21,
    width: 1,
};

/// PadLock Hash Engine v2 (SHA384/SHA512)
pub const PHE2: CpuidFeature = CpuidFeature {
    leaf: 0xc0000001,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 25,
    width: 1,
};

/// PHE v2 enabled
pub const PHE2_EN: CpuidFeature = CpuidFeature {
    leaf: 0xc0000001,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 26,
    width: 1,
};

/// RSA instructions (XMODEXP/MONTMUL2)
pub const RSA: CpuidFeature = CpuidFeature {
    leaf: 0xc0000001,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 27,
    width: 1,
};

/// RSA instructions enabled
pub const RSA_EN: CpuidFeature = CpuidFeature {
    leaf: 0xc0000001,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 28,
    width: 1,
};

/// Stepping ID
pub const STEPPING: CpuidFeature = CpuidFeature {
    leaf: 0x1,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 0,
    width: 4,
};

/// Base CPU model ID
pub const BASE_MODEL: CpuidFeature = CpuidFeature {
    leaf: 0x1,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 4,
    width: 4,
};

/// Base CPU family ID
pub const BASE_FAMILY_ID: CpuidFeature = CpuidFeature {
    leaf: 0x1,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 8,
    width: 4,
};

/// CPU type
pub const CPU_TYPE: CpuidFeature = CpuidFeature {
    leaf: 0x1,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 12,
    width: 2,
};

/// Extended CPU model ID
pub const EXT_MODEL: CpuidFeature = CpuidFeature {
    leaf: 0x1,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 16,
    width: 4,
};

/// Extended CPU family ID
pub const EXT_FAMILY: CpuidFeature = CpuidFeature {
    leaf: 0x1,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 20,
    width: 8,
};

/// Brand index
pub const BRAND_ID: CpuidFeature = CpuidFeature {
    leaf: 0x1,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 0,
    width: 8,
};

/// CLFLUSH instruction cache line size
pub const CLFLUSH_SIZE: CpuidFeature = CpuidFeature {
    leaf: 0x1,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 8,
    width: 8,
};

/// Logical CPU count
pub const N_LOGICAL_CPU: CpuidFeature = CpuidFeature {
    leaf: 0x1,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 16,
    width: 8,
};

/// Initial local APIC physical ID
pub const LOCAL_APIC_ID: CpuidFeature = CpuidFeature {
    leaf: 0x1,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 24,
    width: 8,
};

/// Streaming SIMD Extensions 3 (SSE3)
pub const X86_FEATURE_XMM3: CpuidFeature = CpuidFeature {
    leaf: 0x1,
    subleaf: 0,
    register: CpuidRegister::Ecx,
    shift: 0,
    width: 1,
};

/// PCLMULQDQ instruction support
pub const X86_FEATURE_PCLMULQDQ: CpuidFeature = CpuidFeature {
    leaf: 0x1,
    subleaf: 0,
    register: CpuidRegister::Ecx,
    shift: 1,
    width: 1,
};

/// 64-bit DS save area
pub const X86_FEATURE_DTES64: CpuidFeature = CpuidFeature {
    leaf: 0x1,
    subleaf: 0,
    register: CpuidRegister::Ecx,
    shift: 2,
    width: 1,
};

/// MONITOR/MWAIT support
pub const X86_FEATURE_MWAIT: CpuidFeature = CpuidFeature {
    leaf: 0x1,
    subleaf: 0,
    register: CpuidRegister::Ecx,
    shift: 3,
    width: 1,
};

/// CPL Qualified Debug Store
pub const X86_FEATURE_DSCPL: CpuidFeature = CpuidFeature {
    leaf: 0x1,
    subleaf: 0,
    register: CpuidRegister::Ecx,
    shift: 4,
    width: 1,
};

/// Virtual Machine Extensions
pub const X86_FEATURE_VMX: CpuidFeature = CpuidFeature {
    leaf: 0x1,
    subleaf: 0,
    register: CpuidRegister::Ecx,
    shift: 5,
    width: 1,
};

/// Safer Mode Extensions
pub const X86_FEATURE_SMX: CpuidFeature = CpuidFeature {
    leaf: 0x1,
    subleaf: 0,
    register: CpuidRegister::Ecx,
    shift: 6,
    width: 1,
};

/// Enhanced Intel SpeedStep
pub const X86_FEATURE_EST: CpuidFeature = CpuidFeature {
    leaf: 0x1,
    subleaf: 0,
    register: CpuidRegister::Ecx,
    shift: 7,
    width: 1,
};

/// Thermal Monitor 2
pub const X86_FEATURE_TM2: CpuidFeature = CpuidFeature {
    leaf: 0x1,
    subleaf: 0,
    register: CpuidRegister::Ecx,
    shift: 8,
    width: 1,
};

/// Supplemental SSE3
pub const X86_FEATURE_SSSE3: CpuidFeature = CpuidFeature {
    leaf: 0x1,
    subleaf: 0,
    register: CpuidRegister::Ecx,
    shift: 9,
    width: 1,
};

/// L1 Context ID
pub const X86_FEATURE_CID: CpuidFeature = CpuidFeature {
    leaf: 0x1,
    subleaf: 0,
    register: CpuidRegister::Ecx,
    shift: 10,
    width: 1,
};

/// Silicon Debug
pub const X86_FEATURE_SDBG: CpuidFeature = CpuidFeature {
    leaf: 0x1,
    subleaf: 0,
    register: CpuidRegister::Ecx,
    shift: 11,
    width: 1,
};

/// FMA extensions using YMM state
pub const X86_FEATURE_FMA: CpuidFeature = CpuidFeature {
    leaf: 0x1,
    subleaf: 0,
    register: CpuidRegister::Ecx,
    shift: 12,
    width: 1,
};

/// CMPXCHG16B instruction support
pub const X86_FEATURE_CX16: CpuidFeature = CpuidFeature {
    leaf: 0x1,
    subleaf: 0,
    register: CpuidRegister::Ecx,
    shift: 13,
    width: 1,
};

/// xTPR Update Control
pub const X86_FEATURE_XTPR: CpuidFeature = CpuidFeature {
    leaf: 0x1,
    subleaf: 0,
    register: CpuidRegister::Ecx,
    shift: 14,
    width: 1,
};

/// Perfmon and Debug Capability
pub const X86_FEATURE_PDCM: CpuidFeature = CpuidFeature {
    leaf: 0x1,
    subleaf: 0,
    register: CpuidRegister::Ecx,
    shift: 15,
    width: 1,
};

/// Process-context identifiers
pub const X86_FEATURE_PCID: CpuidFeature = CpuidFeature {
    leaf: 0x1,
    subleaf: 0,
    register: CpuidRegister::Ecx,
    shift: 17,
    width: 1,
};

/// Direct Cache Access
pub const X86_FEATURE_DCA: CpuidFeature = CpuidFeature {
    leaf: 0x1,
    subleaf: 0,
    register: CpuidRegister::Ecx,
    shift: 18,
    width: 1,
};

/// SSE4.1
pub const X86_FEATURE_XMM4_1: CpuidFeature = CpuidFeature {
    leaf: 0x1,
    subleaf: 0,
    register: CpuidRegister::Ecx,
    shift: 19,
    width: 1,
};

/// SSE4.2
pub const X86_FEATURE_XMM4_2: CpuidFeature = CpuidFeature {
    leaf: 0x1,
    subleaf: 0,
    register: CpuidRegister::Ecx,
    shift: 20,
    width: 1,
};

/// X2APIC support
pub const X86_FEATURE_X2APIC: CpuidFeature = CpuidFeature {
    leaf: 0x1,
    subleaf: 0,
    register: CpuidRegister::Ecx,
    shift: 21,
    width: 1,
};

/// MOVBE instruction support
pub const X86_FEATURE_MOVBE: CpuidFeature = CpuidFeature {
    leaf: 0x1,
    subleaf: 0,
    register: CpuidRegister::Ecx,
    shift: 22,
    width: 1,
};

/// POPCNT instruction support
pub const X86_FEATURE_POPCNT: CpuidFeature = CpuidFeature {
    leaf: 0x1,
    subleaf: 0,
    register: CpuidRegister::Ecx,
    shift: 23,
    width: 1,
};

/// APIC timer one-shot operation
pub const X86_FEATURE_TSC_DEADLINE_TIMER: CpuidFeature = CpuidFeature {
    leaf: 0x1,
    subleaf: 0,
    register: CpuidRegister::Ecx,
    shift: 24,
    width: 1,
};

/// AES instructions
pub const X86_FEATURE_AES: CpuidFeature = CpuidFeature {
    leaf: 0x1,
    subleaf: 0,
    register: CpuidRegister::Ecx,
    shift: 25,
    width: 1,
};

/// XSAVE (and related instructions) support
pub const X86_FEATURE_XSAVE: CpuidFeature = CpuidFeature {
    leaf: 0x1,
    subleaf: 0,
    register: CpuidRegister::Ecx,
    shift: 26,
    width: 1,
};

/// XSAVE (and related instructions) are enabled by OS
pub const X86_FEATURE_OSXSAVE: CpuidFeature = CpuidFeature {
    leaf: 0x1,
    subleaf: 0,
    register: CpuidRegister::Ecx,
    shift: 27,
    width: 1,
};

/// AVX instructions support
pub const X86_FEATURE_AVX: CpuidFeature = CpuidFeature {
    leaf: 0x1,
    subleaf: 0,
    register: CpuidRegister::Ecx,
    shift: 28,
    width: 1,
};

/// Half-precision floating-point conversion support
pub const X86_FEATURE_F16C: CpuidFeature = CpuidFeature {
    leaf: 0x1,
    subleaf: 0,
    register: CpuidRegister::Ecx,
    shift: 29,
    width: 1,
};

/// RDRAND instruction support
pub const X86_FEATURE_RDRAND: CpuidFeature = CpuidFeature {
    leaf: 0x1,
    subleaf: 0,
    register: CpuidRegister::Ecx,
    shift: 30,
    width: 1,
};

/// System is running as guest; (para-)virtualized system
pub const GUEST_STATUS: CpuidFeature = CpuidFeature {
    leaf: 0x1,
    subleaf: 0,
    register: CpuidRegister::Ecx,
    shift: 31,
    width: 1,
};

/// Floating-Point Unit on-chip (x87)
pub const X86_FEATURE_FPU: CpuidFeature = CpuidFeature {
    leaf: 0x1,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 0,
    width: 1,
};

/// Virtual-8086 Mode Extensions
pub const X86_FEATURE_VME: CpuidFeature = CpuidFeature {
    leaf: 0x1,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 1,
    width: 1,
};

/// Debugging Extensions
pub const X86_FEATURE_DE: CpuidFeature = CpuidFeature {
    leaf: 0x1,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 2,
    width: 1,
};

/// Page Size Extension
pub const X86_FEATURE_PSE: CpuidFeature = CpuidFeature {
    leaf: 0x1,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 3,
    width: 1,
};

/// Time Stamp Counter
pub const X86_FEATURE_TSC: CpuidFeature = CpuidFeature {
    leaf: 0x1,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 4,
    width: 1,
};

/// Model-Specific Registers (RDMSR and WRMSR support)
pub const X86_FEATURE_MSR: CpuidFeature = CpuidFeature {
    leaf: 0x1,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 5,
    width: 1,
};

/// Physical Address Extensions
pub const X86_FEATURE_PAE: CpuidFeature = CpuidFeature {
    leaf: 0x1,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 6,
    width: 1,
};

/// Machine Check Exception
pub const X86_FEATURE_MCE: CpuidFeature = CpuidFeature {
    leaf: 0x1,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 7,
    width: 1,
};

/// CMPXCHG8B instruction
pub const X86_FEATURE_CX8: CpuidFeature = CpuidFeature {
    leaf: 0x1,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 8,
    width: 1,
};

/// APIC on-chip
pub const X86_FEATURE_APIC: CpuidFeature = CpuidFeature {
    leaf: 0x1,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 9,
    width: 1,
};

/// SYSENTER, SYSEXIT, and associated MSRs
pub const X86_FEATURE_SEP: CpuidFeature = CpuidFeature {
    leaf: 0x1,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 11,
    width: 1,
};

/// Memory Type Range Registers
pub const X86_FEATURE_MTRR: CpuidFeature = CpuidFeature {
    leaf: 0x1,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 12,
    width: 1,
};

/// Page Global Extensions
pub const X86_FEATURE_PGE: CpuidFeature = CpuidFeature {
    leaf: 0x1,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 13,
    width: 1,
};

/// Machine Check Architecture
pub const X86_FEATURE_MCA: CpuidFeature = CpuidFeature {
    leaf: 0x1,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 14,
    width: 1,
};

/// Conditional Move Instruction
pub const X86_FEATURE_CMOV: CpuidFeature = CpuidFeature {
    leaf: 0x1,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 15,
    width: 1,
};

/// Page Attribute Table
pub const X86_FEATURE_PAT: CpuidFeature = CpuidFeature {
    leaf: 0x1,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 16,
    width: 1,
};

/// Page Size Extension (36-bit)
pub const X86_FEATURE_PSE36: CpuidFeature = CpuidFeature {
    leaf: 0x1,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 17,
    width: 1,
};

/// Processor Serial Number
pub const X86_FEATURE_PN: CpuidFeature = CpuidFeature {
    leaf: 0x1,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 18,
    width: 1,
};

/// CLFLUSH instruction
pub const X86_FEATURE_CLFLUSH: CpuidFeature = CpuidFeature {
    leaf: 0x1,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 19,
    width: 1,
};

/// Debug Store
pub const X86_FEATURE_DS: CpuidFeature = CpuidFeature {
    leaf: 0x1,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 21,
    width: 1,
};

/// Thermal monitor and clock control
pub const X86_FEATURE_ACPI: CpuidFeature = CpuidFeature {
    leaf: 0x1,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 22,
    width: 1,
};

/// MMX instructions
pub const X86_FEATURE_MMX: CpuidFeature = CpuidFeature {
    leaf: 0x1,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 23,
    width: 1,
};

/// FXSAVE and FXRSTOR instructions
pub const X86_FEATURE_FXSR: CpuidFeature = CpuidFeature {
    leaf: 0x1,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 24,
    width: 1,
};

/// SSE instructions
pub const X86_FEATURE_XMM: CpuidFeature = CpuidFeature {
    leaf: 0x1,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 25,
    width: 1,
};

/// SSE2 instructions
pub const X86_FEATURE_XMM2: CpuidFeature = CpuidFeature {
    leaf: 0x1,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 26,
    width: 1,
};

/// Self Snoop
pub const X86_FEATURE_SELFSNOOP: CpuidFeature = CpuidFeature {
    leaf: 0x1,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 27,
    width: 1,
};

/// Hyper-threading
pub const X86_FEATURE_HT: CpuidFeature = CpuidFeature {
    leaf: 0x1,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 28,
    width: 1,
};

/// Thermal Monitor
pub const X86_FEATURE_ACC: CpuidFeature = CpuidFeature {
    leaf: 0x1,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 29,
    width: 1,
};

/// Legacy IA-64 (Itanium) support bit, now reserved
pub const X86_FEATURE_IA64: CpuidFeature = CpuidFeature {
    leaf: 0x1,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 30,
    width: 1,
};

/// Pending Break Enable
pub const X86_FEATURE_PBE: CpuidFeature = CpuidFeature {
    leaf: 0x1,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 31,
    width: 1,
};

/// Maximum leaf 0x18 subleaf
pub const TLB_MAX_SUBLEAF: CpuidFeature = CpuidFeature {
    leaf: 0x18,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 0,
    width: 32,
};

/// TLB supports 4KB-page entries
pub const TLB_4K_PAGE: CpuidFeature = CpuidFeature {
    leaf: 0x18,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 0,
    width: 1,
};

/// TLB supports 2MB-page entries
pub const TLB_2M_PAGE: CpuidFeature = CpuidFeature {
    leaf: 0x18,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 1,
    width: 1,
};

/// TLB supports 4MB-page entries
pub const TLB_4M_PAGE: CpuidFeature = CpuidFeature {
    leaf: 0x18,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 2,
    width: 1,
};

/// TLB supports 1GB-page entries
pub const TLB_1G_PAGE: CpuidFeature = CpuidFeature {
    leaf: 0x18,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 3,
    width: 1,
};

/// Partitioning between logical CPUs
pub const HARD_PARTITIONING: CpuidFeature = CpuidFeature {
    leaf: 0x18,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 8,
    width: 3,
};

/// Ways of associativity
pub const N_WAY_ASSOCIATIVE: CpuidFeature = CpuidFeature {
    leaf: 0x18,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 16,
    width: 16,
};

/// Number of sets
pub const N_SETS: CpuidFeature = CpuidFeature {
    leaf: 0x18,
    subleaf: 0,
    register: CpuidRegister::Ecx,
    shift: 0,
    width: 32,
};

/// Translation cache type (TLB type)
pub const TLB_TYPE: CpuidFeature = CpuidFeature {
    leaf: 0x18,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 0,
    width: 5,
};

/// Translation cache level (1-based)
pub const TLB_CACHE_LEVEL: CpuidFeature = CpuidFeature {
    leaf: 0x18,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 5,
    width: 3,
};

/// Fully-associative
pub const IS_FULLY_ASSOCIATIVE: CpuidFeature = CpuidFeature {
    leaf: 0x18,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 8,
    width: 1,
};

/// Max number of addressable IDs - 1
pub const TLB_MAX_ADDRESSABLE_IDS: CpuidFeature = CpuidFeature {
    leaf: 0x18,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 14,
    width: 12,
};

/// L3 Cache Allocation Technology
pub const X86_FEATURE_CAT_L3: CpuidFeature = CpuidFeature {
    leaf: 0x10,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 1,
    width: 1,
};

/// L2 Cache Allocation Technology
pub const X86_FEATURE_CAT_L2: CpuidFeature = CpuidFeature {
    leaf: 0x10,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 2,
    width: 1,
};

/// L3/L2_CAT capacity bitmask length, minus-one notation
pub const CAT_CBM_LEN: CpuidFeature = CpuidFeature {
    leaf: 0x10,
    subleaf: 1,
    register: CpuidRegister::Eax,
    shift: 0,
    width: 5,
};

/// L3/L2_CAT allocation units bitmap
pub const CAT_UNITS_BITMAP: CpuidFeature = CpuidFeature {
    leaf: 0x10,
    subleaf: 1,
    register: CpuidRegister::Ebx,
    shift: 0,
    width: 32,
};

/// L3_CAT COS updates should be infrequent
pub const L3_CAT_COS_INFREQ_UPDATES: CpuidFeature = CpuidFeature {
    leaf: 0x10,
    subleaf: 1,
    register: CpuidRegister::Ecx,
    shift: 1,
    width: 1,
};

/// L3/L2_CAT Code and Data Prioritization
pub const X86_FEATURE_CDP_L3: CpuidFeature = CpuidFeature {
    leaf: 0x10,
    subleaf: 1,
    register: CpuidRegister::Ecx,
    shift: 2,
    width: 1,
};

/// L3/L2_CAT non-contiguous 1s value
pub const CAT_SPARSE_1S: CpuidFeature = CpuidFeature {
    leaf: 0x10,
    subleaf: 1,
    register: CpuidRegister::Ecx,
    shift: 3,
    width: 1,
};

/// L3/L2_CAT max Class of Service
pub const CAT_COS_MAX: CpuidFeature = CpuidFeature {
    leaf: 0x10,
    subleaf: 1,
    register: CpuidRegister::Edx,
    shift: 0,
    width: 16,
};

/// Max MBA throttling value; minus-one notation
pub const MBA_MAX_DELAY: CpuidFeature = CpuidFeature {
    leaf: 0x10,
    subleaf: 3,
    register: CpuidRegister::Eax,
    shift: 0,
    width: 12,
};

/// Per-thread MBA controls
pub const X86_FEATURE_PER_THREAD_MBA: CpuidFeature = CpuidFeature {
    leaf: 0x10,
    subleaf: 3,
    register: CpuidRegister::Ecx,
    shift: 0,
    width: 1,
};

/// Delay values are linear
pub const MBA_DELAY_LINEAR: CpuidFeature = CpuidFeature {
    leaf: 0x10,
    subleaf: 3,
    register: CpuidRegister::Ecx,
    shift: 2,
    width: 1,
};

/// MBA max Class of Service
pub const MBA_COS_MAX: CpuidFeature = CpuidFeature {
    leaf: 0x10,
    subleaf: 3,
    register: CpuidRegister::Edx,
    shift: 0,
    width: 16,
};

/// DCA is enabled in BIOS
pub const DCA_ENABLED_IN_BIOS: CpuidFeature = CpuidFeature {
    leaf: 0x9,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 0,
    width: 1,
};

/// Intel F00F
pub const X86_FEATURE_F00F: CpuidFeature = CpuidFeature {
    leaf: 0x4c780002,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 0,
    width: 1,
};

/// FPU FDIV
pub const X86_FEATURE_FDIV: CpuidFeature = CpuidFeature {
    leaf: 0x4c780002,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 1,
    width: 1,
};

/// Cyrix 6x86 coma
pub const X86_FEATURE_COMA: CpuidFeature = CpuidFeature {
    leaf: 0x4c780002,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 2,
    width: 1,
};

/// AMD Erratum 383
pub const X86_FEATURE_AMD_TLB_MMATCH: CpuidFeature = CpuidFeature {
    leaf: 0x4c780002,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 3,
    width: 1,
};

/// AMD Erratum 400
pub const X86_FEATURE_AMD_APIC_C1E: CpuidFeature = CpuidFeature {
    leaf: 0x4c780002,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 4,
    width: 1,
};

/// Bad local APIC aka 11AP
pub const X86_FEATURE__11AP: CpuidFeature = CpuidFeature {
    leaf: 0x4c780002,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 5,
    width: 1,
};

/// FXSAVE leaks FOP/FIP/FOP
pub const X86_FEATURE_FXSAVE_LEAK: CpuidFeature = CpuidFeature {
    leaf: 0x4c780002,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 6,
    width: 1,
};

/// AAI65, CLFLUSH required before MONITOR
pub const X86_FEATURE_CLFLUSH_MONITOR: CpuidFeature = CpuidFeature {
    leaf: 0x4c780002,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 7,
    width: 1,
};

/// SYSRET does not fix up SS attributes
pub const X86_FEATURE_SYSRET_SS_ATTRS: CpuidFeature = CpuidFeature {
    leaf: 0x4c780002,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 8,
    width: 1,
};

/// IRET to 16-bit SS corrupts ESP/RSP high bits (x86-32)
pub const X86_FEATURE_ESPFIX: CpuidFeature = CpuidFeature {
    leaf: 0x4c780002,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 9,
    width: 1,
};

/// Setting a selector to NULL preserves the base
pub const X86_FEATURE_NULL_SEG: CpuidFeature = CpuidFeature {
    leaf: 0x4c780002,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 10,
    width: 1,
};

/// SWAPGS without input dep on GS
pub const X86_FEATURE_SWAPGS_FENCE: CpuidFeature = CpuidFeature {
    leaf: 0x4c780002,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 11,
    width: 1,
};

/// IPI required to wake up remote CPU
pub const X86_FEATURE_MONITOR: CpuidFeature = CpuidFeature {
    leaf: 0x4c780002,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 12,
    width: 1,
};

/// CPU is among the affected by Erratum 400
pub const X86_FEATURE_AMD_E400: CpuidFeature = CpuidFeature {
    leaf: 0x4c780002,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 13,
    width: 1,
};

/// CPU affected by meltdown; needs kernel page table isolation
pub const X86_FEATURE_CPU_MELTDOWN: CpuidFeature = CpuidFeature {
    leaf: 0x4c780002,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 14,
    width: 1,
};

/// CPU affected by Spectre variant 1 with conditional branches
pub const X86_FEATURE_SPECTRE_V1: CpuidFeature = CpuidFeature {
    leaf: 0x4c780002,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 15,
    width: 1,
};

/// CPU affected by Spectre variant 2 with indirect branches
pub const X86_FEATURE_SPECTRE_V2: CpuidFeature = CpuidFeature {
    leaf: 0x4c780002,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 16,
    width: 1,
};

/// CPU affected by speculative store bypass attack
pub const X86_FEATURE_SPEC_STORE_BYPASS: CpuidFeature = CpuidFeature {
    leaf: 0x4c780002,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 17,
    width: 1,
};

/// CPU affected by L1 Terminal Fault
pub const X86_FEATURE_L1TF: CpuidFeature = CpuidFeature {
    leaf: 0x4c780002,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 18,
    width: 1,
};

/// CPU affected by Microarchitectural data sampling
pub const X86_FEATURE_MDS: CpuidFeature = CpuidFeature {
    leaf: 0x4c780002,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 19,
    width: 1,
};

/// Microarchitectural data sampling: CPU only affected by the MSBDS variant
pub const X86_FEATURE_MSBDS_ONLY: CpuidFeature = CpuidFeature {
    leaf: 0x4c780002,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 20,
    width: 1,
};

/// CPU affected by speculation through SWAPGS
pub const X86_FEATURE_SWAPGS: CpuidFeature = CpuidFeature {
    leaf: 0x4c780002,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 21,
    width: 1,
};

/// CPU is affected by TSX Async Abort (TAA)
pub const X86_FEATURE_TAA: CpuidFeature = CpuidFeature {
    leaf: 0x4c780002,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 22,
    width: 1,
};

/// CPU may incur MCE during certain page attribute changes
pub const X86_FEATURE_ITLB_MULTIHIT: CpuidFeature = CpuidFeature {
    leaf: 0x4c780002,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 23,
    width: 1,
};

/// CPU may leak RNG bits if not mitigated
pub const X86_FEATURE_SRBDS: CpuidFeature = CpuidFeature {
    leaf: 0x4c780002,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 24,
    width: 1,
};

/// CPU affected by Processor MMIO Stale Data vulnerabilities
pub const X86_FEATURE_MMIO_STALE_DATA: CpuidFeature = CpuidFeature {
    leaf: 0x4c780002,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 25,
    width: 1,
};

/// CPU affected by Retbleed
pub const X86_FEATURE_RETBLEED: CpuidFeature = CpuidFeature {
    leaf: 0x4c780002,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 27,
    width: 1,
};

/// EIBRS is vulnerable to Post Barrier RSB Predictions
pub const X86_FEATURE_EIBRS_PBRSB: CpuidFeature = CpuidFeature {
    leaf: 0x4c780002,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 28,
    width: 1,
};

/// CPU vulnerable to Cross-Thread Return Address Predictions
pub const X86_FEATURE_SMT_RSB: CpuidFeature = CpuidFeature {
    leaf: 0x4c780002,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 29,
    width: 1,
};

/// CPU affected by Gather Data Sampling
pub const X86_FEATURE_GDS: CpuidFeature = CpuidFeature {
    leaf: 0x4c780002,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 30,
    width: 1,
};

/// CPU may incur #MC if non-TD software does partial write to TDX private memory
pub const X86_FEATURE_TDX_PW_MCE: CpuidFeature = CpuidFeature {
    leaf: 0x4c780002,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 31,
    width: 1,
};

/// AMD SRSO bug
pub const X86_FEATURE_SRSO: CpuidFeature = CpuidFeature {
    leaf: 0x4c780002,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 0,
    width: 1,
};

/// AMD DIV0 speculation bug
pub const X86_FEATURE_DIV0: CpuidFeature = CpuidFeature {
    leaf: 0x4c780002,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 1,
    width: 1,
};

/// CPU vulnerable to Register File Data Sampling
pub const X86_FEATURE_RFDS: CpuidFeature = CpuidFeature {
    leaf: 0x4c780002,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 2,
    width: 1,
};

/// CPU affected by Branch History Injection
pub const X86_FEATURE_BHI: CpuidFeature = CpuidFeature {
    leaf: 0x4c780002,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 3,
    width: 1,
};

/// IBPB omits return target predictions
pub const X86_FEATURE_IBPB_NO_RET: CpuidFeature = CpuidFeature {
    leaf: 0x4c780002,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 4,
    width: 1,
};

/// CPU affected by Spectre variant 2 between user processes
pub const X86_FEATURE_SPECTRE_V2_USER: CpuidFeature = CpuidFeature {
    leaf: 0x4c780002,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 5,
    width: 1,
};

/// CPU has old microcode; it must be vulnerable to something
pub const X86_FEATURE_OLD_MICROCODE: CpuidFeature = CpuidFeature {
    leaf: 0x4c780002,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 6,
    width: 1,
};

/// CPU affected by Indirect Target Selection
pub const X86_FEATURE_ITS: CpuidFeature = CpuidFeature {
    leaf: 0x4c780002,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 7,
    width: 1,
};

/// CPU affected by ITS; VMX is not affected
pub const X86_FEATURE_ITS_NATIVE_ONLY: CpuidFeature = CpuidFeature {
    leaf: 0x4c780002,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 8,
    width: 1,
};

/// CPU affected by Transient Scheduler Attacks
pub const X86_FEATURE_TSA: CpuidFeature = CpuidFeature {
    leaf: 0x4c780002,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 9,
    width: 1,
};

/// CPU affected by VMSCAPE attacks from guests
pub const X86_FEATURE_VMSCAPE: CpuidFeature = CpuidFeature {
    leaf: 0x4c780002,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 10,
    width: 1,
};

/// Slow Memory Bandwidth Allocation support
pub const X86_FEATURE_SMBA: CpuidFeature = CpuidFeature {
    leaf: 0x80000020,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 2,
    width: 1,
};

/// Bandwidth Monitoring Event Configuration support
pub const X86_FEATURE_BMEC: CpuidFeature = CpuidFeature {
    leaf: 0x80000020,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 3,
    width: 1,
};

/// L3 Range Reservation support
pub const L3RR: CpuidFeature = CpuidFeature {
    leaf: 0x80000020,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 4,
    width: 1,
};

/// Assignable Bandwidth Monitoring Counters
pub const X86_FEATURE_ABMC: CpuidFeature = CpuidFeature {
    leaf: 0x80000020,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 5,
    width: 1,
};

/// Smart Data Cache Injection (SDCI) Allocation Enforcement
pub const X86_FEATURE_SDCIAE: CpuidFeature = CpuidFeature {
    leaf: 0x80000020,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 6,
    width: 1,
};

/// MBA enforcement limit size
pub const MBA_LIMIT_LEN: CpuidFeature = CpuidFeature {
    leaf: 0x80000020,
    subleaf: 1,
    register: CpuidRegister::Eax,
    shift: 0,
    width: 32,
};

/// SMBA enforcement limit size
pub const SMBA_LIMIT_LEN: CpuidFeature = CpuidFeature {
    leaf: 0x80000020,
    subleaf: 2,
    register: CpuidRegister::Eax,
    shift: 0,
    width: 32,
};

/// SMBA max Class of Service number (zero-based)
pub const SMBA_COS_MAX: CpuidFeature = CpuidFeature {
    leaf: 0x80000020,
    subleaf: 2,
    register: CpuidRegister::Edx,
    shift: 0,
    width: 32,
};

/// BMEC number of bandwidth events available
pub const BMEC_NUM_EVENTS: CpuidFeature = CpuidFeature {
    leaf: 0x80000020,
    subleaf: 3,
    register: CpuidRegister::Ebx,
    shift: 0,
    width: 8,
};

/// Local NUMA reads can be tracked
pub const BMEC_LOCAL_READS: CpuidFeature = CpuidFeature {
    leaf: 0x80000020,
    subleaf: 3,
    register: CpuidRegister::Ecx,
    shift: 0,
    width: 1,
};

/// Remote NUMA reads can be tracked
pub const BMEC_REMOTE_READS: CpuidFeature = CpuidFeature {
    leaf: 0x80000020,
    subleaf: 3,
    register: CpuidRegister::Ecx,
    shift: 1,
    width: 1,
};

/// Local NUMA non-temporal writes can be tracked
pub const BMEC_LOCAL_NONTEMP_WR: CpuidFeature = CpuidFeature {
    leaf: 0x80000020,
    subleaf: 3,
    register: CpuidRegister::Ecx,
    shift: 2,
    width: 1,
};

/// Remote NUMA non-temporal writes can be tracked
pub const BMEC_REMOTE_NONTEMP_WR: CpuidFeature = CpuidFeature {
    leaf: 0x80000020,
    subleaf: 3,
    register: CpuidRegister::Ecx,
    shift: 3,
    width: 1,
};

/// Local NUMA slow-memory reads can be tracked
pub const BMEC_LOCAL_SLOW_MEM_RD: CpuidFeature = CpuidFeature {
    leaf: 0x80000020,
    subleaf: 3,
    register: CpuidRegister::Ecx,
    shift: 4,
    width: 1,
};

/// Remote NUMA slow-memory reads can be tracked
pub const BMEC_REMOTE_SLOW_MEM_RD: CpuidFeature = CpuidFeature {
    leaf: 0x80000020,
    subleaf: 3,
    register: CpuidRegister::Ecx,
    shift: 5,
    width: 1,
};

/// Dirty QoS victims to all types of memory can be tracked
pub const BMEC_ALL_DIRTY_VICTIMS: CpuidFeature = CpuidFeature {
    leaf: 0x80000020,
    subleaf: 3,
    register: CpuidRegister::Ecx,
    shift: 6,
    width: 1,
};

/// RMID max within this core (0-based)
pub const CORE_RMID_MAX: CpuidFeature = CpuidFeature {
    leaf: 0xf,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 0,
    width: 32,
};

/// LLC QoS-monitoring
pub const X86_FEATURE_CQM_LLC: CpuidFeature = CpuidFeature {
    leaf: 0xf,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 1,
    width: 1,
};

/// L3 QoS-monitoring counter bitwidth (24-based)
pub const L3C_QM_BITWIDTH: CpuidFeature = CpuidFeature {
    leaf: 0xf,
    subleaf: 1,
    register: CpuidRegister::Eax,
    shift: 0,
    width: 8,
};

/// QM_CTR MSR bit 61 is an overflow bit
pub const L3C_QM_OVERFLOW_BIT: CpuidFeature = CpuidFeature {
    leaf: 0xf,
    subleaf: 1,
    register: CpuidRegister::Eax,
    shift: 8,
    width: 1,
};

/// non-CPU agent supporting Intel RDT CMT present
pub const IO_RDT_CMT: CpuidFeature = CpuidFeature {
    leaf: 0xf,
    subleaf: 1,
    register: CpuidRegister::Eax,
    shift: 9,
    width: 1,
};

/// non-CPU agent supporting Intel RDT MBM present
pub const IO_RDT_MBM: CpuidFeature = CpuidFeature {
    leaf: 0xf,
    subleaf: 1,
    register: CpuidRegister::Eax,
    shift: 10,
    width: 1,
};

/// QM_CTR MSR conversion factor to bytes
pub const L3C_QM_CONVER_FACTOR: CpuidFeature = CpuidFeature {
    leaf: 0xf,
    subleaf: 1,
    register: CpuidRegister::Ebx,
    shift: 0,
    width: 32,
};

/// L3 QoS-monitoring max RMID
pub const L3C_QM_RMID_MAX: CpuidFeature = CpuidFeature {
    leaf: 0xf,
    subleaf: 1,
    register: CpuidRegister::Ecx,
    shift: 0,
    width: 32,
};

/// L3 QoS occupancy monitoring
pub const X86_FEATURE_CQM_OCCUP_LLC: CpuidFeature = CpuidFeature {
    leaf: 0xf,
    subleaf: 1,
    register: CpuidRegister::Edx,
    shift: 0,
    width: 1,
};

/// L3 QoS total bandwidth monitoring
pub const X86_FEATURE_CQM_MBM_TOTAL: CpuidFeature = CpuidFeature {
    leaf: 0xf,
    subleaf: 1,
    register: CpuidRegister::Edx,
    shift: 1,
    width: 1,
};

/// L3 QoS local bandwidth monitoring
pub const X86_FEATURE_CQM_MBM_LOCAL: CpuidFeature = CpuidFeature {
    leaf: 0xf,
    subleaf: 1,
    register: CpuidRegister::Edx,
    shift: 2,
    width: 1,
};

/// CPU brand ID string, bytes 0 - 3
pub const CPU_BRANDID_0: CpuidFeature = CpuidFeature {
    leaf: 0x80000002,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 0,
    width: 32,
};

/// CPU brand ID string, bytes 4 - 7
pub const CPU_BRANDID_1: CpuidFeature = CpuidFeature {
    leaf: 0x80000002,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 0,
    width: 32,
};

/// CPU brand ID string, bytes 8 - 11
pub const CPU_BRANDID_2: CpuidFeature = CpuidFeature {
    leaf: 0x80000002,
    subleaf: 0,
    register: CpuidRegister::Ecx,
    shift: 0,
    width: 32,
};

/// CPU brand ID string, bytes 12 - 15
pub const CPU_BRANDID_3: CpuidFeature = CpuidFeature {
    leaf: 0x80000002,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 0,
    width: 32,
};

/// CPU revision ID, mask minor
pub const CPU_REV_MASK_MINOR: CpuidFeature = CpuidFeature {
    leaf: 0x80860001,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 0,
    width: 8,
};

/// CPU revision ID, mask major
pub const CPU_REV_MASK_MAJOR: CpuidFeature = CpuidFeature {
    leaf: 0x80860001,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 8,
    width: 8,
};

/// CPU revision ID, minor
pub const CPU_REV_MINOR: CpuidFeature = CpuidFeature {
    leaf: 0x80860001,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 16,
    width: 8,
};

/// CPU revision ID, major
pub const CPU_REV_MAJOR: CpuidFeature = CpuidFeature {
    leaf: 0x80860001,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 24,
    width: 8,
};

/// Recovery CMS is active (after bad flush)
pub const X86_FEATURE_RECOVERY: CpuidFeature = CpuidFeature {
    leaf: 0x80860001,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 0,
    width: 1,
};

/// LongRun power management capabilities
pub const X86_FEATURE_LONGRUN: CpuidFeature = CpuidFeature {
    leaf: 0x80860001,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 1,
    width: 1,
};

/// LongRun Table Interface
pub const X86_FEATURE_LRTI: CpuidFeature = CpuidFeature {
    leaf: 0x80860001,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 3,
    width: 1,
};

/// CPU info string bytes 16 - 19
pub const CPU_INFO_4: CpuidFeature = CpuidFeature {
    leaf: 0x80860004,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 0,
    width: 32,
};

/// CPU info string bytes 20 - 23
pub const CPU_INFO_5: CpuidFeature = CpuidFeature {
    leaf: 0x80860004,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 0,
    width: 32,
};

/// CPU info string bytes 24 - 27
pub const CPU_INFO_6: CpuidFeature = CpuidFeature {
    leaf: 0x80860004,
    subleaf: 0,
    register: CpuidRegister::Ecx,
    shift: 0,
    width: 32,
};

/// CPU info string bytes 28 - 31
pub const CPU_INFO_7: CpuidFeature = CpuidFeature {
    leaf: 0x80860004,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 0,
    width: 32,
};

/// Highest palette ID / subleaf ID
pub const AMX_MAX_PALETTE: CpuidFeature = CpuidFeature {
    leaf: 0x1d,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 0,
    width: 32,
};

/// AMX palette total tiles size, in bytes
pub const AMX_PALETTE_SIZE: CpuidFeature = CpuidFeature {
    leaf: 0x1d,
    subleaf: 1,
    register: CpuidRegister::Eax,
    shift: 0,
    width: 16,
};

/// AMX single tile's size, in bytes
pub const AMX_TILE_SIZE: CpuidFeature = CpuidFeature {
    leaf: 0x1d,
    subleaf: 1,
    register: CpuidRegister::Eax,
    shift: 16,
    width: 16,
};

/// AMX tile single row's size, in bytes
pub const AMX_TILE_ROW_SIZE: CpuidFeature = CpuidFeature {
    leaf: 0x1d,
    subleaf: 1,
    register: CpuidRegister::Ebx,
    shift: 0,
    width: 16,
};

/// AMX palette number of tiles
pub const AMX_PALETTE_NR_TILES: CpuidFeature = CpuidFeature {
    leaf: 0x1d,
    subleaf: 1,
    register: CpuidRegister::Ebx,
    shift: 16,
    width: 16,
};

/// AMX tile max number of rows
pub const AMX_TILE_NR_ROWS: CpuidFeature = CpuidFeature {
    leaf: 0x1d,
    subleaf: 1,
    register: CpuidRegister::Ecx,
    shift: 0,
    width: 16,
};

/// This core's native model ID
pub const CORE_NATIVE_MODEL: CpuidFeature = CpuidFeature {
    leaf: 0x1a,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 0,
    width: 24,
};

/// This core's type
pub const CORE_TYPE: CpuidFeature = CpuidFeature {
    leaf: 0x1a,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 24,
    width: 8,
};

/// Cyrix MMX extensions
pub const X86_FEATURE_CXMMX: CpuidFeature = CpuidFeature {
    leaf: 0x4c780001,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 0,
    width: 1,
};

/// AMD K6 nonstandard MTRRs
pub const X86_FEATURE_K6_MTRR: CpuidFeature = CpuidFeature {
    leaf: 0x4c780001,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 1,
    width: 1,
};

/// Cyrix ARRs (= MTRRs)
pub const X86_FEATURE_CYRIX_ARR: CpuidFeature = CpuidFeature {
    leaf: 0x4c780001,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 2,
    width: 1,
};

/// Centaur MCRs (= MTRRs)
pub const X86_FEATURE_CENTAUR_MCR: CpuidFeature = CpuidFeature {
    leaf: 0x4c780001,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 3,
    width: 1,
};

/// Opteron, Athlon64
pub const X86_FEATURE_K8: CpuidFeature = CpuidFeature {
    leaf: 0x4c780001,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 4,
    width: 1,
};

/// CPU based on Zen5 micro-architecture
pub const X86_FEATURE_ZEN5: CpuidFeature = CpuidFeature {
    leaf: 0x4c780001,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 5,
    width: 1,
};

/// CPU based on Zen6 micro-architecture
pub const X86_FEATURE_ZEN6: CpuidFeature = CpuidFeature {
    leaf: 0x4c780001,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 6,
    width: 1,
};

/// TSC ticks at a constant rate
pub const X86_FEATURE_CONSTANT_TSC: CpuidFeature = CpuidFeature {
    leaf: 0x4c780001,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 8,
    width: 1,
};

/// SMP kernel running on UP
pub const X86_FEATURE_UP: CpuidFeature = CpuidFeature {
    leaf: 0x4c780001,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 9,
    width: 1,
};

/// Always running timer (ART)
pub const X86_FEATURE_ART: CpuidFeature = CpuidFeature {
    leaf: 0x4c780001,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 10,
    width: 1,
};

/// Intel Architectural PerfMon
pub const X86_FEATURE_ARCH_PERFMON: CpuidFeature = CpuidFeature {
    leaf: 0x4c780001,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 11,
    width: 1,
};

/// Precise-Event Based Sampling
pub const X86_FEATURE_PEBS: CpuidFeature = CpuidFeature {
    leaf: 0x4c780001,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 12,
    width: 1,
};

/// Branch Trace Store
pub const X86_FEATURE_BTS: CpuidFeature = CpuidFeature {
    leaf: 0x4c780001,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 13,
    width: 1,
};

/// SYSCALL in IA32 userspace
pub const X86_FEATURE_SYSCALL32: CpuidFeature = CpuidFeature {
    leaf: 0x4c780001,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 14,
    width: 1,
};

/// SYSENTER in IA32 userspace
pub const X86_FEATURE_SYSENTER32: CpuidFeature = CpuidFeature {
    leaf: 0x4c780001,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 15,
    width: 1,
};

/// REP microcode works well
pub const X86_FEATURE_REP_GOOD: CpuidFeature = CpuidFeature {
    leaf: 0x4c780001,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 16,
    width: 1,
};

/// AMD Last Branch Record Extension version 2
pub const X86_FEATURE_AMD_LBR_V2: CpuidFeature = CpuidFeature {
    leaf: 0x4c780001,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 17,
    width: 1,
};

/// Clear CPU buffers using VERW
pub const X86_FEATURE_CLEAR_CPU_BUF: CpuidFeature = CpuidFeature {
    leaf: 0x4c780001,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 18,
    width: 1,
};

/// AMD Accumulated Power Mechanism
pub const X86_FEATURE_ACC_POWER: CpuidFeature = CpuidFeature {
    leaf: 0x4c780001,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 19,
    width: 1,
};

/// The NOPL instructions
pub const X86_FEATURE_NOPL: CpuidFeature = CpuidFeature {
    leaf: 0x4c780001,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 20,
    width: 1,
};

/// Always-present feature
pub const X86_FEATURE_ALWAYS: CpuidFeature = CpuidFeature {
    leaf: 0x4c780001,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 21,
    width: 1,
};

/// CPU topology enumeration extensions
pub const X86_FEATURE_XTOPOLOGY: CpuidFeature = CpuidFeature {
    leaf: 0x4c780001,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 22,
    width: 1,
};

/// TSC is known to be reliable
pub const X86_FEATURE_TSC_RELIABLE: CpuidFeature = CpuidFeature {
    leaf: 0x4c780001,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 23,
    width: 1,
};

/// TSC does not stop in C states
pub const X86_FEATURE_NONSTOP_TSC: CpuidFeature = CpuidFeature {
    leaf: 0x4c780001,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 24,
    width: 1,
};

/// CPU has the CPUID instruction
pub const X86_FEATURE_CPUID: CpuidFeature = CpuidFeature {
    leaf: 0x4c780001,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 25,
    width: 1,
};

/// Extended APIC ID (8 bits)
pub const X86_FEATURE_EXTD_APICID: CpuidFeature = CpuidFeature {
    leaf: 0x4c780001,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 26,
    width: 1,
};

/// AMD multi-node processor
pub const X86_FEATURE_AMD_DCM: CpuidFeature = CpuidFeature {
    leaf: 0x4c780001,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 27,
    width: 1,
};

/// APERF/MPERF MSRs: P-State hardware coordination feedback
pub const X86_FEATURE_APERFMPERF: CpuidFeature = CpuidFeature {
    leaf: 0x4c780001,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 28,
    width: 1,
};

/// AMD/Hygon RAPL interface
pub const X86_FEATURE_RAPL: CpuidFeature = CpuidFeature {
    leaf: 0x4c780001,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 29,
    width: 1,
};

/// TSC does not stop in S3 state
pub const X86_FEATURE_NONSTOP_TSC_S3: CpuidFeature = CpuidFeature {
    leaf: 0x4c780001,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 30,
    width: 1,
};

/// TSC has known frequency
pub const X86_FEATURE_TSC_KNOWN_FREQ: CpuidFeature = CpuidFeature {
    leaf: 0x4c780001,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 31,
    width: 1,
};

/// Ring 3 MONITOR/MWAIT instructions
pub const X86_FEATURE_RING3MWAIT: CpuidFeature = CpuidFeature {
    leaf: 0x4c780001,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 0,
    width: 1,
};

/// Intel CPUID faulting
pub const X86_FEATURE_CPUID_FAULT: CpuidFeature = CpuidFeature {
    leaf: 0x4c780001,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 1,
    width: 1,
};

/// AMD Core Performance Boost
pub const X86_FEATURE_CPB: CpuidFeature = CpuidFeature {
    leaf: 0x4c780001,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 2,
    width: 1,
};

/// IA32_ENERGY_PERF_BIAS support
pub const X86_FEATURE_EPB: CpuidFeature = CpuidFeature {
    leaf: 0x4c780001,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 3,
    width: 1,
};

/// Platform supports being a TDX host
pub const X86_FEATURE_TDX_HOST_PLATFORM: CpuidFeature = CpuidFeature {
    leaf: 0x4c780001,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 7,
    width: 1,
};

/// AMD Hardware P-state control
pub const X86_FEATURE_HW_PSTATE: CpuidFeature = CpuidFeature {
    leaf: 0x4c780001,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 8,
    width: 1,
};

/// AMD Processor Feedback Interface
pub const X86_FEATURE_PROC_FEEDBACK: CpuidFeature = CpuidFeature {
    leaf: 0x4c780001,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 9,
    width: 1,
};

/// Use compacted XSTATE (XSAVES or XSAVEC)
pub const X86_FEATURE_XCOMPACTED: CpuidFeature = CpuidFeature {
    leaf: 0x4c780001,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 10,
    width: 1,
};

/// Kernel Page Table Isolation enabled
pub const X86_FEATURE_PTI: CpuidFeature = CpuidFeature {
    leaf: 0x4c780001,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 11,
    width: 1,
};

/// Set/clear IBRS on kernel entry/exit
pub const X86_FEATURE_KERNEL_IBRS: CpuidFeature = CpuidFeature {
    leaf: 0x4c780001,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 12,
    width: 1,
};

/// Fill RSB on VM-Exit
pub const X86_FEATURE_RSB_VMEXIT: CpuidFeature = CpuidFeature {
    leaf: 0x4c780001,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 13,
    width: 1,
};

/// Code and Data Prioritization L2
pub const X86_FEATURE_CDP_L2: CpuidFeature = CpuidFeature {
    leaf: 0x4c780001,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 15,
    width: 1,
};

/// MSR SPEC_CTRL is implemented
pub const X86_FEATURE_MSR_SPEC_CTRL: CpuidFeature = CpuidFeature {
    leaf: 0x4c780001,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 16,
    width: 1,
};

/// Speculative Store Bypass Disable
pub const X86_FEATURE_SSBD: CpuidFeature = CpuidFeature {
    leaf: 0x4c780001,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 17,
    width: 1,
};

/// Fill RSB on context switches
pub const X86_FEATURE_RSB_CTXSW: CpuidFeature = CpuidFeature {
    leaf: 0x4c780001,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 19,
    width: 1,
};

/// Use IBRS during runtime firmware calls
pub const X86_FEATURE_USE_IBRS_FW: CpuidFeature = CpuidFeature {
    leaf: 0x4c780001,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 22,
    width: 1,
};

/// Disable Speculative Store Bypass
pub const X86_FEATURE_SPEC_STORE_BYPASS_DISABLE: CpuidFeature = CpuidFeature {
    leaf: 0x4c780001,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 23,
    width: 1,
};

/// AMD SSBD implementation via LS_CFG MSR
pub const X86_FEATURE_LS_CFG_SSBD: CpuidFeature = CpuidFeature {
    leaf: 0x4c780001,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 24,
    width: 1,
};

/// Indirect Branch Restricted Speculation
pub const X86_FEATURE_IBRS: CpuidFeature = CpuidFeature {
    leaf: 0x4c780001,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 25,
    width: 1,
};

/// Indirect Branch Prediction Barrier (without RSB flush guarantee)
pub const X86_FEATURE_IBPB: CpuidFeature = CpuidFeature {
    leaf: 0x4c780001,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 26,
    width: 1,
};

/// Single Thread Indirect Branch Predictors
pub const X86_FEATURE_STIBP: CpuidFeature = CpuidFeature {
    leaf: 0x4c780001,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 27,
    width: 1,
};

/// Generic flag for all Zen and newer
pub const X86_FEATURE_ZEN: CpuidFeature = CpuidFeature {
    leaf: 0x4c780001,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 28,
    width: 1,
};

/// L1TF workaround PTE inversion
pub const X86_FEATURE_L1TF_PTEINV: CpuidFeature = CpuidFeature {
    leaf: 0x4c780001,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 29,
    width: 1,
};

/// Enhanced IBRS
pub const X86_FEATURE_IBRS_ENHANCED: CpuidFeature = CpuidFeature {
    leaf: 0x4c780001,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 30,
    width: 1,
};

/// MSR IA32_FEAT_CTL configured
pub const X86_FEATURE_MSR_IA32_FEAT_CTL: CpuidFeature = CpuidFeature {
    leaf: 0x4c780001,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 31,
    width: 1,
};

/// Intel TPR Shadow
pub const X86_FEATURE_TPR_SHADOW: CpuidFeature = CpuidFeature {
    leaf: 0x4c780001,
    subleaf: 0,
    register: CpuidRegister::Ecx,
    shift: 0,
    width: 1,
};

/// Intel FlexPriority
pub const X86_FEATURE_FLEXPRIORITY: CpuidFeature = CpuidFeature {
    leaf: 0x4c780001,
    subleaf: 0,
    register: CpuidRegister::Ecx,
    shift: 1,
    width: 1,
};

/// Intel Extended Page Table
pub const X86_FEATURE_EPT: CpuidFeature = CpuidFeature {
    leaf: 0x4c780001,
    subleaf: 0,
    register: CpuidRegister::Ecx,
    shift: 2,
    width: 1,
};

/// Intel Virtual Processor ID
pub const X86_FEATURE_VPID: CpuidFeature = CpuidFeature {
    leaf: 0x4c780001,
    subleaf: 0,
    register: CpuidRegister::Ecx,
    shift: 3,
    width: 1,
};

/// SNP cache coherency software workaround not needed
pub const X86_FEATURE_COHERENCY_SFW_NO: CpuidFeature = CpuidFeature {
    leaf: 0x4c780001,
    subleaf: 0,
    register: CpuidRegister::Ecx,
    shift: 4,
    width: 1,
};

/// Prefer VMMCALL to VMCALL
pub const X86_FEATURE_VMMCALL: CpuidFeature = CpuidFeature {
    leaf: 0x4c780001,
    subleaf: 0,
    register: CpuidRegister::Ecx,
    shift: 15,
    width: 1,
};

/// Xen paravirtual guest
pub const X86_FEATURE_XENPV: CpuidFeature = CpuidFeature {
    leaf: 0x4c780001,
    subleaf: 0,
    register: CpuidRegister::Ecx,
    shift: 16,
    width: 1,
};

/// Intel Extended Page Table access-dirty bit
pub const X86_FEATURE_EPT_AD: CpuidFeature = CpuidFeature {
    leaf: 0x4c780001,
    subleaf: 0,
    register: CpuidRegister::Ecx,
    shift: 17,
    width: 1,
};

/// Hypervisor supports the VMCALL instruction
pub const X86_FEATURE_VMCALL: CpuidFeature = CpuidFeature {
    leaf: 0x4c780001,
    subleaf: 0,
    register: CpuidRegister::Ecx,
    shift: 18,
    width: 1,
};

/// VMware prefers the VMMCALL instruction
pub const X86_FEATURE_VMW_VMMCALL: CpuidFeature = CpuidFeature {
    leaf: 0x4c780001,
    subleaf: 0,
    register: CpuidRegister::Ecx,
    shift: 19,
    width: 1,
};

/// PV unlock function
pub const X86_FEATURE_PVUNLOCK: CpuidFeature = CpuidFeature {
    leaf: 0x4c780001,
    subleaf: 0,
    register: CpuidRegister::Ecx,
    shift: 20,
    width: 1,
};

/// PV vcpu_is_preempted function
pub const X86_FEATURE_VCPUPREEMPT: CpuidFeature = CpuidFeature {
    leaf: 0x4c780001,
    subleaf: 0,
    register: CpuidRegister::Ecx,
    shift: 21,
    width: 1,
};

/// Intel Trust Domain Extensions Guest
pub const X86_FEATURE_TDX_GUEST: CpuidFeature = CpuidFeature {
    leaf: 0x4c780001,
    subleaf: 0,
    register: CpuidRegister::Ecx,
    shift: 22,
    width: 1,
};

/// LFENCE in user entry SWAPGS path
pub const X86_FEATURE_FENCE_SWAPGS_USER: CpuidFeature = CpuidFeature {
    leaf: 0x4c780001,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 4,
    width: 1,
};

/// LFENCE in kernel entry SWAPGS path
pub const X86_FEATURE_FENCE_SWAPGS_KERNEL: CpuidFeature = CpuidFeature {
    leaf: 0x4c780001,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 5,
    width: 1,
};

/// #AC for split lock
pub const X86_FEATURE_SPLIT_LOCK_DETECT: CpuidFeature = CpuidFeature {
    leaf: 0x4c780001,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 6,
    width: 1,
};

/// SGX Basic
pub const X86_FEATURE_SGX1: CpuidFeature = CpuidFeature {
    leaf: 0x4c780001,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 8,
    width: 1,
};

/// SGX Enclave Dynamic Memory Management (EDMM)
pub const X86_FEATURE_SGX2: CpuidFeature = CpuidFeature {
    leaf: 0x4c780001,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 9,
    width: 1,
};

/// Issue an IBPB on kernel entry
pub const X86_FEATURE_ENTRY_IBPB: CpuidFeature = CpuidFeature {
    leaf: 0x4c780001,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 10,
    width: 1,
};

/// Generic Retpoline mitigation for Spectre variant 2
pub const X86_FEATURE_RETPOLINE: CpuidFeature = CpuidFeature {
    leaf: 0x4c780001,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 12,
    width: 1,
};

/// Use LFENCE for Spectre variant 2
pub const X86_FEATURE_RETPOLINE_LFENCE: CpuidFeature = CpuidFeature {
    leaf: 0x4c780001,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 13,
    width: 1,
};

/// Use Return THUNK
pub const X86_FEATURE_RETHUNK: CpuidFeature = CpuidFeature {
    leaf: 0x4c780001,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 14,
    width: 1,
};

/// AMD BTB untrain return
pub const X86_FEATURE_UNRET: CpuidFeature = CpuidFeature {
    leaf: 0x4c780001,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 15,
    width: 1,
};

/// Use IBPB during runtime firmware calls
pub const X86_FEATURE_USE_IBPB_FW: CpuidFeature = CpuidFeature {
    leaf: 0x4c780001,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 16,
    width: 1,
};

/// Fill RSB on VM exit when EIBRS is enabled
pub const X86_FEATURE_RSB_VMEXIT_LITE: CpuidFeature = CpuidFeature {
    leaf: 0x4c780001,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 17,
    width: 1,
};

/// SGX EDECCSSA user leaf function
pub const X86_FEATURE_SGX_EDECCSSA: CpuidFeature = CpuidFeature {
    leaf: 0x4c780001,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 18,
    width: 1,
};

/// Call depth tracking for RSB stuffing
pub const X86_FEATURE_CALL_DEPTH: CpuidFeature = CpuidFeature {
    leaf: 0x4c780001,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 19,
    width: 1,
};

/// MSR IA32_TSX_CTRL (Intel) implemented
pub const X86_FEATURE_MSR_TSX_CTRL: CpuidFeature = CpuidFeature {
    leaf: 0x4c780001,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 20,
    width: 1,
};

/// Shadow stack support for user mode applications
pub const X86_FEATURE_USER_SHSTK: CpuidFeature = CpuidFeature {
    leaf: 0x4c780001,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 23,
    width: 1,
};

/// AMD BTB untrain RETs through aliasing
pub const X86_FEATURE_SRSO_ALIAS: CpuidFeature = CpuidFeature {
    leaf: 0x4c780001,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 25,
    width: 1,
};

/// Issue an IBPB only on VMEXIT
pub const X86_FEATURE_IBPB_ON_VMEXIT: CpuidFeature = CpuidFeature {
    leaf: 0x4c780001,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 26,
    width: 1,
};

/// IA32_TSC_DEADLINE and X2APIC MSRs need fencing
pub const X86_FEATURE_APIC_MSRS_FENCE: CpuidFeature = CpuidFeature {
    leaf: 0x4c780001,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 27,
    width: 1,
};

/// CPU based on Zen2 microarchitecture
pub const X86_FEATURE_ZEN2: CpuidFeature = CpuidFeature {
    leaf: 0x4c780001,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 28,
    width: 1,
};

/// CPU based on Zen3 microarchitecture
pub const X86_FEATURE_ZEN3: CpuidFeature = CpuidFeature {
    leaf: 0x4c780001,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 29,
    width: 1,
};

/// CPU based on Zen4 microarchitecture
pub const X86_FEATURE_ZEN4: CpuidFeature = CpuidFeature {
    leaf: 0x4c780001,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 30,
    width: 1,
};

/// CPU based on Zen1 microarchitecture
pub const X86_FEATURE_ZEN1: CpuidFeature = CpuidFeature {
    leaf: 0x4c780001,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 31,
    width: 1,
};

/// MCA overflow recovery support
pub const X86_FEATURE_OVERFLOW_RECOV: CpuidFeature = CpuidFeature {
    leaf: 0x4c780001,
    subleaf: 1,
    register: CpuidRegister::Eax,
    shift: 0,
    width: 1,
};

/// Uncorrectable error containment and recovery
pub const X86_FEATURE_SUCCOR: CpuidFeature = CpuidFeature {
    leaf: 0x4c780001,
    subleaf: 1,
    register: CpuidRegister::Eax,
    shift: 1,
    width: 1,
};

/// Scalable MCA
pub const X86_FEATURE_SMCA: CpuidFeature = CpuidFeature {
    leaf: 0x4c780001,
    subleaf: 1,
    register: CpuidRegister::Eax,
    shift: 3,
    width: 1,
};

/// AMD LBR and PMC Freeze
pub const X86_FEATURE_AMD_LBR_PMC_FREEZE: CpuidFeature = CpuidFeature {
    leaf: 0x4c780001,
    subleaf: 1,
    register: CpuidRegister::Ebx,
    shift: 0,
    width: 1,
};

/// Clear branch history at SYSCALL entry using SW loop
pub const X86_FEATURE_CLEAR_BHB_LOOP: CpuidFeature = CpuidFeature {
    leaf: 0x4c780001,
    subleaf: 1,
    register: CpuidRegister::Ebx,
    shift: 1,
    width: 1,
};

/// BHI_DIS_S HW control available
pub const X86_FEATURE_BHI_CTRL: CpuidFeature = CpuidFeature {
    leaf: 0x4c780001,
    subleaf: 1,
    register: CpuidRegister::Ebx,
    shift: 2,
    width: 1,
};

/// BHI_DIS_S HW control enabled
pub const X86_FEATURE_CLEAR_BHB_HW: CpuidFeature = CpuidFeature {
    leaf: 0x4c780001,
    subleaf: 1,
    register: CpuidRegister::Ebx,
    shift: 3,
    width: 1,
};

/// Clear branch history at VMEXIT using SW loop
pub const X86_FEATURE_CLEAR_BHB_VMEXIT: CpuidFeature = CpuidFeature {
    leaf: 0x4c780001,
    subleaf: 1,
    register: CpuidRegister::Ebx,
    shift: 4,
    width: 1,
};

/// AMD fast Collaborative Processor Performance Control
pub const X86_FEATURE_AMD_FAST_CPPC: CpuidFeature = CpuidFeature {
    leaf: 0x4c780001,
    subleaf: 1,
    register: CpuidRegister::Ebx,
    shift: 5,
    width: 1,
};

/// Heterogeneous Core Topology
pub const X86_FEATURE_AMD_HTR_CORES: CpuidFeature = CpuidFeature {
    leaf: 0x4c780001,
    subleaf: 1,
    register: CpuidRegister::Ebx,
    shift: 6,
    width: 1,
};

/// Workload Classification
pub const X86_FEATURE_AMD_WORKLOAD_CLASS: CpuidFeature = CpuidFeature {
    leaf: 0x4c780001,
    subleaf: 1,
    register: CpuidRegister::Ebx,
    shift: 7,
    width: 1,
};

/// Avoid ZMM registers due to downclocking
pub const X86_FEATURE_PREFER_YMM: CpuidFeature = CpuidFeature {
    leaf: 0x4c780001,
    subleaf: 1,
    register: CpuidRegister::Ebx,
    shift: 8,
    width: 1,
};

/// Advanced Performance Extensions
pub const X86_FEATURE_APX: CpuidFeature = CpuidFeature {
    leaf: 0x4c780001,
    subleaf: 1,
    register: CpuidRegister::Ebx,
    shift: 9,
    width: 1,
};

/// Use thunk for indirect branches in lower half of cache line
pub const X86_FEATURE_INDIRECT_THUNK_ITS: CpuidFeature = CpuidFeature {
    leaf: 0x4c780001,
    subleaf: 1,
    register: CpuidRegister::Ebx,
    shift: 10,
    width: 1,
};

/// AMD CPU not vulnerable to TSA-SQ
pub const X86_FEATURE_TSA_SQ_NO: CpuidFeature = CpuidFeature {
    leaf: 0x4c780001,
    subleaf: 1,
    register: CpuidRegister::Ebx,
    shift: 11,
    width: 1,
};

/// AMD CPU not vulnerable to TSA-L1
pub const X86_FEATURE_TSA_L1_NO: CpuidFeature = CpuidFeature {
    leaf: 0x4c780001,
    subleaf: 1,
    register: CpuidRegister::Ebx,
    shift: 12,
    width: 1,
};

/// Clear CPU buffers using VERW before VMRUN
pub const X86_FEATURE_CLEAR_CPU_BUF_VM: CpuidFeature = CpuidFeature {
    leaf: 0x4c780001,
    subleaf: 1,
    register: CpuidRegister::Ebx,
    shift: 13,
    width: 1,
};

/// Use IBPB on exit-to-userspace, see VMSCAPE bug
pub const X86_FEATURE_IBPB_EXIT_TO_USER: CpuidFeature = CpuidFeature {
    leaf: 0x4c780001,
    subleaf: 1,
    register: CpuidRegister::Ebx,
    shift: 14,
    width: 1,
};

/// ENCLV leaves
pub const ENCLV_LEAVES: CpuidFeature = CpuidFeature {
    leaf: 0x12,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 5,
    width: 1,
};

/// ENCLS leaves
pub const ENCLS_LEAVES: CpuidFeature = CpuidFeature {
    leaf: 0x12,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 6,
    width: 1,
};

/// ENCLU leaf EVERIFYREPORT2
pub const ENCLU_EVERIFYREPORT2: CpuidFeature = CpuidFeature {
    leaf: 0x12,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 7,
    width: 1,
};

/// ENCLS leaf EUPDATESVN
pub const ENCLS_EUPDATESVN: CpuidFeature = CpuidFeature {
    leaf: 0x12,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 10,
    width: 1,
};

/// SSA.MISC frame: Enclave #PF and #GP reporting
pub const MISCSELECT_EXINFO: CpuidFeature = CpuidFeature {
    leaf: 0x12,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 0,
    width: 1,
};

/// SSA.MISC frame: Enclave #CP reporting
pub const MISCSELECT_CPINFO: CpuidFeature = CpuidFeature {
    leaf: 0x12,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 1,
    width: 1,
};

/// Maximum enclave size in non-64-bit mode (log2)
pub const MAX_ENCLAVE_SZ_NOT64: CpuidFeature = CpuidFeature {
    leaf: 0x12,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 0,
    width: 8,
};

/// Maximum enclave size in 64-bit mode (log2)
pub const MAX_ENCLAVE_SZ_64: CpuidFeature = CpuidFeature {
    leaf: 0x12,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 8,
    width: 8,
};

/// Enclave initialized by EINIT
pub const SECS_ATTR_INIT: CpuidFeature = CpuidFeature {
    leaf: 0x12,
    subleaf: 1,
    register: CpuidRegister::Eax,
    shift: 0,
    width: 1,
};

/// Enclave permits debugger read/write
pub const SECS_ATTR_DEBUG: CpuidFeature = CpuidFeature {
    leaf: 0x12,
    subleaf: 1,
    register: CpuidRegister::Eax,
    shift: 1,
    width: 1,
};

/// Enclave runs in 64-bit mode
pub const SECS_ATTR_MODE64BIT: CpuidFeature = CpuidFeature {
    leaf: 0x12,
    subleaf: 1,
    register: CpuidRegister::Eax,
    shift: 2,
    width: 1,
};

/// Provisioning key
pub const SECS_ATTR_PROVISIONKEY: CpuidFeature = CpuidFeature {
    leaf: 0x12,
    subleaf: 1,
    register: CpuidRegister::Eax,
    shift: 4,
    width: 1,
};

/// EINIT token key
pub const SECS_ATTR_EINITTOKEN_KEY: CpuidFeature = CpuidFeature {
    leaf: 0x12,
    subleaf: 1,
    register: CpuidRegister::Eax,
    shift: 5,
    width: 1,
};

/// CET attributes
pub const SECS_ATTR_CET: CpuidFeature = CpuidFeature {
    leaf: 0x12,
    subleaf: 1,
    register: CpuidRegister::Eax,
    shift: 6,
    width: 1,
};

/// Key Separation and Sharing
pub const SECS_ATTR_KSS: CpuidFeature = CpuidFeature {
    leaf: 0x12,
    subleaf: 1,
    register: CpuidRegister::Eax,
    shift: 7,
    width: 1,
};

/// Enclave threads: AEX notifications
pub const SECS_ATTR_AEXNOTIFY: CpuidFeature = CpuidFeature {
    leaf: 0x12,
    subleaf: 1,
    register: CpuidRegister::Eax,
    shift: 10,
    width: 1,
};

/// Enclave XFRM.X87
pub const XFRM_X87: CpuidFeature = CpuidFeature {
    leaf: 0x12,
    subleaf: 1,
    register: CpuidRegister::Ecx,
    shift: 0,
    width: 1,
};

/// Enclave XFRM.SSE
pub const XFRM_SSE: CpuidFeature = CpuidFeature {
    leaf: 0x12,
    subleaf: 1,
    register: CpuidRegister::Ecx,
    shift: 1,
    width: 1,
};

/// Enclave XFRM.AVX
pub const XFRM_AVX: CpuidFeature = CpuidFeature {
    leaf: 0x12,
    subleaf: 1,
    register: CpuidRegister::Ecx,
    shift: 2,
    width: 1,
};

/// Enclave XFRM.BNDREGS (MPX BND0-BND3 registers)
pub const XFRM_MPX_BNDREGS: CpuidFeature = CpuidFeature {
    leaf: 0x12,
    subleaf: 1,
    register: CpuidRegister::Ecx,
    shift: 3,
    width: 1,
};

/// Enclave XFRM.BNDCSR (MPX BNDCFGU/BNDSTATUS registers)
pub const XFRM_MPX_BNDCSR: CpuidFeature = CpuidFeature {
    leaf: 0x12,
    subleaf: 1,
    register: CpuidRegister::Ecx,
    shift: 4,
    width: 1,
};

/// Enclave XFRM.OPMASK (AVX-512 k0-k7 registers)
pub const XFRM_AVX512_OPMASK: CpuidFeature = CpuidFeature {
    leaf: 0x12,
    subleaf: 1,
    register: CpuidRegister::Ecx,
    shift: 5,
    width: 1,
};

/// Enclave XFRM.ZMM_Hi256 (AVX-512 ZMM0->ZMM7/15 registers)
pub const XFRM_AVX512_ZMM_HI256: CpuidFeature = CpuidFeature {
    leaf: 0x12,
    subleaf: 1,
    register: CpuidRegister::Ecx,
    shift: 6,
    width: 1,
};

/// Enclave XFRM.HI16_ZMM (AVX-512 ZMM16->ZMM31 registers)
pub const XFRM_AVX512_HI16_ZMM: CpuidFeature = CpuidFeature {
    leaf: 0x12,
    subleaf: 1,
    register: CpuidRegister::Ecx,
    shift: 7,
    width: 1,
};

/// Enclave XFRM.PKRU (XSAVE PKRU registers)
pub const XFRM_PKRU: CpuidFeature = CpuidFeature {
    leaf: 0x12,
    subleaf: 1,
    register: CpuidRegister::Ecx,
    shift: 9,
    width: 1,
};

/// Enclave XFRM.TILECONFIG (AMX can manage TILECONFIG)
pub const XFRM_TILECONFIG: CpuidFeature = CpuidFeature {
    leaf: 0x12,
    subleaf: 1,
    register: CpuidRegister::Ecx,
    shift: 17,
    width: 1,
};

/// Enclave XFRM.TILEDATA (AMX can manage TILEDATA)
pub const XFRM_TILEDATA: CpuidFeature = CpuidFeature {
    leaf: 0x12,
    subleaf: 1,
    register: CpuidRegister::Ecx,
    shift: 18,
    width: 1,
};

/// Subleaf type
pub const SUBLEAF_TYPE: CpuidFeature = CpuidFeature {
    leaf: 0x12,
    subleaf: 2,
    register: CpuidRegister::Eax,
    shift: 0,
    width: 4,
};

/// EPC section base address, bits[12:31]
pub const EPC_SEC_BASE_ADDR_0: CpuidFeature = CpuidFeature {
    leaf: 0x12,
    subleaf: 2,
    register: CpuidRegister::Eax,
    shift: 12,
    width: 20,
};

/// EPC section base address, bits[32:51]
pub const EPC_SEC_BASE_ADDR_1: CpuidFeature = CpuidFeature {
    leaf: 0x12,
    subleaf: 2,
    register: CpuidRegister::Ebx,
    shift: 0,
    width: 20,
};

/// EPC section type / property encoding
pub const EPC_SEC_TYPE: CpuidFeature = CpuidFeature {
    leaf: 0x12,
    subleaf: 2,
    register: CpuidRegister::Ecx,
    shift: 0,
    width: 4,
};

/// EPC section size, bits[12:31]
pub const EPC_SEC_SIZE_0: CpuidFeature = CpuidFeature {
    leaf: 0x12,
    subleaf: 2,
    register: CpuidRegister::Ecx,
    shift: 12,
    width: 20,
};

/// EPC section size, bits[32:51]
pub const EPC_SEC_SIZE_1: CpuidFeature = CpuidFeature {
    leaf: 0x12,
    subleaf: 2,
    register: CpuidRegister::Edx,
    shift: 0,
    width: 20,
};

/// Stepping ID
pub const E_STEPPING_ID: CpuidFeature = CpuidFeature {
    leaf: 0x80000001,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 0,
    width: 4,
};

/// Base processor model
pub const E_BASE_MODEL: CpuidFeature = CpuidFeature {
    leaf: 0x80000001,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 4,
    width: 4,
};

/// Base processor family
pub const E_BASE_FAMILY: CpuidFeature = CpuidFeature {
    leaf: 0x80000001,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 8,
    width: 4,
};

/// Base processor type (Transmeta)
pub const E_BASE_TYPE: CpuidFeature = CpuidFeature {
    leaf: 0x80000001,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 12,
    width: 2,
};

/// Extended processor model
pub const E_EXT_MODEL: CpuidFeature = CpuidFeature {
    leaf: 0x80000001,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 16,
    width: 4,
};

/// Extended processor family
pub const E_EXT_FAMILY: CpuidFeature = CpuidFeature {
    leaf: 0x80000001,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 20,
    width: 8,
};

/// Package type
pub const PKG_TYPE: CpuidFeature = CpuidFeature {
    leaf: 0x80000001,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 28,
    width: 4,
};

/// LAHF and SAHF in 64-bit mode
pub const X86_FEATURE_LAHF_LM: CpuidFeature = CpuidFeature {
    leaf: 0x80000001,
    subleaf: 0,
    register: CpuidRegister::Ecx,
    shift: 0,
    width: 1,
};

/// Multi-processing legacy mode (No HT)
pub const X86_FEATURE_CMP_LEGACY: CpuidFeature = CpuidFeature {
    leaf: 0x80000001,
    subleaf: 0,
    register: CpuidRegister::Ecx,
    shift: 1,
    width: 1,
};

/// Secure Virtual Machine
pub const X86_FEATURE_SVM: CpuidFeature = CpuidFeature {
    leaf: 0x80000001,
    subleaf: 0,
    register: CpuidRegister::Ecx,
    shift: 2,
    width: 1,
};

/// Extended APIC space
pub const X86_FEATURE_EXTAPIC: CpuidFeature = CpuidFeature {
    leaf: 0x80000001,
    subleaf: 0,
    register: CpuidRegister::Ecx,
    shift: 3,
    width: 1,
};

/// LOCK MOV CR0 means MOV CR8
pub const X86_FEATURE_CR8_LEGACY: CpuidFeature = CpuidFeature {
    leaf: 0x80000001,
    subleaf: 0,
    register: CpuidRegister::Ecx,
    shift: 4,
    width: 1,
};

/// LZCNT advanced bit manipulation
pub const X86_FEATURE_ABM: CpuidFeature = CpuidFeature {
    leaf: 0x80000001,
    subleaf: 0,
    register: CpuidRegister::Ecx,
    shift: 5,
    width: 1,
};

/// SSE4A support
pub const X86_FEATURE_SSE4A: CpuidFeature = CpuidFeature {
    leaf: 0x80000001,
    subleaf: 0,
    register: CpuidRegister::Ecx,
    shift: 6,
    width: 1,
};

/// Misaligned SSE mode
pub const X86_FEATURE_MISALIGNSSE: CpuidFeature = CpuidFeature {
    leaf: 0x80000001,
    subleaf: 0,
    register: CpuidRegister::Ecx,
    shift: 7,
    width: 1,
};

/// 3DNow PREFETCH/PREFETCHW support
pub const X86_FEATURE__3DNOWPREFETCH: CpuidFeature = CpuidFeature {
    leaf: 0x80000001,
    subleaf: 0,
    register: CpuidRegister::Ecx,
    shift: 8,
    width: 1,
};

/// OS visible workaround
pub const X86_FEATURE_OSVW: CpuidFeature = CpuidFeature {
    leaf: 0x80000001,
    subleaf: 0,
    register: CpuidRegister::Ecx,
    shift: 9,
    width: 1,
};

/// Instruction based sampling
pub const X86_FEATURE_IBS: CpuidFeature = CpuidFeature {
    leaf: 0x80000001,
    subleaf: 0,
    register: CpuidRegister::Ecx,
    shift: 10,
    width: 1,
};

/// XOP: extended operation (AVX instructions)
pub const X86_FEATURE_XOP: CpuidFeature = CpuidFeature {
    leaf: 0x80000001,
    subleaf: 0,
    register: CpuidRegister::Ecx,
    shift: 11,
    width: 1,
};

/// SKINIT/STGI support
pub const X86_FEATURE_SKINIT: CpuidFeature = CpuidFeature {
    leaf: 0x80000001,
    subleaf: 0,
    register: CpuidRegister::Ecx,
    shift: 12,
    width: 1,
};

/// Watchdog timer support
pub const X86_FEATURE_WDT: CpuidFeature = CpuidFeature {
    leaf: 0x80000001,
    subleaf: 0,
    register: CpuidRegister::Ecx,
    shift: 13,
    width: 1,
};

/// Lightweight profiling
pub const X86_FEATURE_LWP: CpuidFeature = CpuidFeature {
    leaf: 0x80000001,
    subleaf: 0,
    register: CpuidRegister::Ecx,
    shift: 15,
    width: 1,
};

/// 4-operand FMA instruction
pub const X86_FEATURE_FMA4: CpuidFeature = CpuidFeature {
    leaf: 0x80000001,
    subleaf: 0,
    register: CpuidRegister::Ecx,
    shift: 16,
    width: 1,
};

/// Translation cache extension
pub const X86_FEATURE_TCE: CpuidFeature = CpuidFeature {
    leaf: 0x80000001,
    subleaf: 0,
    register: CpuidRegister::Ecx,
    shift: 17,
    width: 1,
};

/// NodeId MSR (0xc001100c)
pub const X86_FEATURE_NODEID_MSR: CpuidFeature = CpuidFeature {
    leaf: 0x80000001,
    subleaf: 0,
    register: CpuidRegister::Ecx,
    shift: 19,
    width: 1,
};

/// Trailing bit manipulations
pub const X86_FEATURE_TBM: CpuidFeature = CpuidFeature {
    leaf: 0x80000001,
    subleaf: 0,
    register: CpuidRegister::Ecx,
    shift: 21,
    width: 1,
};

/// Topology Extensions (leaf 0x8000001d)
pub const X86_FEATURE_TOPOEXT: CpuidFeature = CpuidFeature {
    leaf: 0x80000001,
    subleaf: 0,
    register: CpuidRegister::Ecx,
    shift: 22,
    width: 1,
};

/// Core performance counter extensions
pub const X86_FEATURE_PERFCTR_CORE: CpuidFeature = CpuidFeature {
    leaf: 0x80000001,
    subleaf: 0,
    register: CpuidRegister::Ecx,
    shift: 23,
    width: 1,
};

/// NB/DF performance counter extensions
pub const X86_FEATURE_PERFCTR_NB: CpuidFeature = CpuidFeature {
    leaf: 0x80000001,
    subleaf: 0,
    register: CpuidRegister::Ecx,
    shift: 24,
    width: 1,
};

/// Data access breakpoint extension
pub const X86_FEATURE_BPEXT: CpuidFeature = CpuidFeature {
    leaf: 0x80000001,
    subleaf: 0,
    register: CpuidRegister::Ecx,
    shift: 26,
    width: 1,
};

/// Performance time-stamp counter
pub const X86_FEATURE_PTSC: CpuidFeature = CpuidFeature {
    leaf: 0x80000001,
    subleaf: 0,
    register: CpuidRegister::Ecx,
    shift: 27,
    width: 1,
};

/// LLC (L3) performance counter extensions
pub const X86_FEATURE_PERFCTR_LLC: CpuidFeature = CpuidFeature {
    leaf: 0x80000001,
    subleaf: 0,
    register: CpuidRegister::Ecx,
    shift: 28,
    width: 1,
};

/// MWAITX/MONITORX support
pub const X86_FEATURE_MWAITX: CpuidFeature = CpuidFeature {
    leaf: 0x80000001,
    subleaf: 0,
    register: CpuidRegister::Ecx,
    shift: 29,
    width: 1,
};

/// Breakpoint address mask extension (to bit 31)
pub const ADDR_MASK_EXT: CpuidFeature = CpuidFeature {
    leaf: 0x80000001,
    subleaf: 0,
    register: CpuidRegister::Ecx,
    shift: 30,
    width: 1,
};

/// Floating-Point Unit on-chip (x87)
pub const E_FPU: CpuidFeature = CpuidFeature {
    leaf: 0x80000001,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 0,
    width: 1,
};

/// Virtual-8086 Mode Extensions
pub const E_VME: CpuidFeature = CpuidFeature {
    leaf: 0x80000001,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 1,
    width: 1,
};

/// Debugging Extensions
pub const E_DE: CpuidFeature = CpuidFeature {
    leaf: 0x80000001,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 2,
    width: 1,
};

/// Page Size Extension
pub const E_PSE: CpuidFeature = CpuidFeature {
    leaf: 0x80000001,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 3,
    width: 1,
};

/// Time Stamp Counter
pub const E_TSC: CpuidFeature = CpuidFeature {
    leaf: 0x80000001,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 4,
    width: 1,
};

/// Model-Specific Registers (RDMSR and WRMSR support)
pub const E_MSR: CpuidFeature = CpuidFeature {
    leaf: 0x80000001,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 5,
    width: 1,
};

/// Physical Address Extensions
pub const PAE: CpuidFeature = CpuidFeature {
    leaf: 0x80000001,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 6,
    width: 1,
};

/// Machine Check Exception
pub const MCE: CpuidFeature = CpuidFeature {
    leaf: 0x80000001,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 7,
    width: 1,
};

/// CMPXCHG8B instruction
pub const CX8: CpuidFeature = CpuidFeature {
    leaf: 0x80000001,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 8,
    width: 1,
};

/// APIC on-chip
pub const APIC: CpuidFeature = CpuidFeature {
    leaf: 0x80000001,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 9,
    width: 1,
};

/// SYSCALL and SYSRET instructions
pub const X86_FEATURE_SYSCALL: CpuidFeature = CpuidFeature {
    leaf: 0x80000001,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 11,
    width: 1,
};

/// Memory Type Range Registers
pub const MTRR: CpuidFeature = CpuidFeature {
    leaf: 0x80000001,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 12,
    width: 1,
};

/// Page Global Extensions
pub const PGE: CpuidFeature = CpuidFeature {
    leaf: 0x80000001,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 13,
    width: 1,
};

/// Machine Check Architecture
pub const MCA: CpuidFeature = CpuidFeature {
    leaf: 0x80000001,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 14,
    width: 1,
};

/// Conditional Move Instruction
pub const CMOV: CpuidFeature = CpuidFeature {
    leaf: 0x80000001,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 15,
    width: 1,
};

/// Page Attribute Table
pub const PAT: CpuidFeature = CpuidFeature {
    leaf: 0x80000001,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 16,
    width: 1,
};

/// Page Size Extension (36-bit)
pub const PSE36: CpuidFeature = CpuidFeature {
    leaf: 0x80000001,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 17,
    width: 1,
};

/// Out-of-spec AMD Multiprocessing bit
pub const X86_FEATURE_MP: CpuidFeature = CpuidFeature {
    leaf: 0x80000001,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 19,
    width: 1,
};

/// No-execute page protection
pub const X86_FEATURE_NX: CpuidFeature = CpuidFeature {
    leaf: 0x80000001,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 20,
    width: 1,
};

/// AMD MMX extensions
pub const X86_FEATURE_MMXEXT: CpuidFeature = CpuidFeature {
    leaf: 0x80000001,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 22,
    width: 1,
};

/// MMX instructions
pub const E_MMX: CpuidFeature = CpuidFeature {
    leaf: 0x80000001,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 23,
    width: 1,
};

/// FXSAVE and FXRSTOR instructions
pub const E_FXSR: CpuidFeature = CpuidFeature {
    leaf: 0x80000001,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 24,
    width: 1,
};

/// FXSAVE and FXRSTOR optimizations
pub const X86_FEATURE_FXSR_OPT: CpuidFeature = CpuidFeature {
    leaf: 0x80000001,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 25,
    width: 1,
};

/// 1-GB large page support
pub const X86_FEATURE_GBPAGES: CpuidFeature = CpuidFeature {
    leaf: 0x80000001,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 26,
    width: 1,
};

/// RDTSCP instruction
pub const X86_FEATURE_RDTSCP: CpuidFeature = CpuidFeature {
    leaf: 0x80000001,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 27,
    width: 1,
};

/// Long mode (x86-64, 64-bit support)
pub const X86_FEATURE_LM: CpuidFeature = CpuidFeature {
    leaf: 0x80000001,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 29,
    width: 1,
};

/// AMD 3DNow extensions
pub const X86_FEATURE__3DNOWEXT: CpuidFeature = CpuidFeature {
    leaf: 0x80000001,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 30,
    width: 1,
};

/// 3DNow instructions
pub const X86_FEATURE__3DNOW: CpuidFeature = CpuidFeature {
    leaf: 0x80000001,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 31,
    width: 1,
};

/// TMUL unit maximum height, K (rows or columns)
pub const TMUL_MAXK: CpuidFeature = CpuidFeature {
    leaf: 0x1e,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 0,
    width: 8,
};

/// TMUL unit maximum SIMD dimension, N (column bytes)
pub const TMUL_MAXN: CpuidFeature = CpuidFeature {
    leaf: 0x1e,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 8,
    width: 16,
};

/// Highest standard CPUID leaf
pub const MAX_STD_LEAF: CpuidFeature = CpuidFeature {
    leaf: 0x0,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 0,
    width: 32,
};

/// Hardware assert MSRs
pub const HW_ASSERT: CpuidFeature = CpuidFeature {
    leaf: 0x80000007,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 2,
    width: 1,
};

/// CPU power sample time ratio
pub const CPU_PWR_SAMPLE_RATIO: CpuidFeature = CpuidFeature {
    leaf: 0x80000007,
    subleaf: 0,
    register: CpuidRegister::Ecx,
    shift: 0,
    width: 32,
};

/// Digital temperature sensor
pub const DIGITAL_TEMP: CpuidFeature = CpuidFeature {
    leaf: 0x80000007,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 0,
    width: 1,
};

/// PowerNOW! frequency scaling
pub const POWERNOW_FREQ_ID: CpuidFeature = CpuidFeature {
    leaf: 0x80000007,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 1,
    width: 1,
};

/// PowerNOW! voltage scaling
pub const POWERNOW_VOLT_ID: CpuidFeature = CpuidFeature {
    leaf: 0x80000007,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 2,
    width: 1,
};

/// THERMTRIP (Thermal Trip)
pub const THERMAL_TRIP: CpuidFeature = CpuidFeature {
    leaf: 0x80000007,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 3,
    width: 1,
};

/// Hardware thermal control
pub const HW_THERMAL_CONTROL: CpuidFeature = CpuidFeature {
    leaf: 0x80000007,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 4,
    width: 1,
};

/// Software thermal control
pub const SW_THERMAL_CONTROL: CpuidFeature = CpuidFeature {
    leaf: 0x80000007,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 5,
    width: 1,
};

/// 100 MHz multiplier control
pub const _100MHZ_STEPS: CpuidFeature = CpuidFeature {
    leaf: 0x80000007,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 6,
    width: 1,
};

/// Read-only effective frequency interface
pub const EFF_FREQ_RO: CpuidFeature = CpuidFeature {
    leaf: 0x80000007,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 10,
    width: 1,
};

/// CPU Connected Standby support
pub const CONNECTED_STANDBY: CpuidFeature = CpuidFeature {
    leaf: 0x80000007,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 13,
    width: 1,
};

/// Internal FP/SIMD exec data path is 128-bits wide
pub const FP_128: CpuidFeature = CpuidFeature {
    leaf: 0x8000001a,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 0,
    width: 1,
};

/// SSE: MOVU* better than MOVL*/MOVH*
pub const MOVU_PREFERRED: CpuidFeature = CpuidFeature {
    leaf: 0x8000001a,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 1,
    width: 1,
};

/// Internal FP/SSE exec data path is 256-bits wide
pub const FP_256: CpuidFeature = CpuidFeature {
    leaf: 0x8000001a,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 2,
    width: 1,
};

/// Maximum leaf 0x17 subleaf
pub const SOC_MAX_SUBLEAF: CpuidFeature = CpuidFeature {
    leaf: 0x17,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 0,
    width: 32,
};

/// SoC vendor ID
pub const SOC_VENDOR_ID: CpuidFeature = CpuidFeature {
    leaf: 0x17,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 0,
    width: 16,
};

/// Assigned by industry enumeration scheme (not Intel)
pub const IS_VENDOR_SCHEME: CpuidFeature = CpuidFeature {
    leaf: 0x17,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 16,
    width: 1,
};

/// SoC project ID, assigned by vendor
pub const SOC_PROJ_ID: CpuidFeature = CpuidFeature {
    leaf: 0x17,
    subleaf: 0,
    register: CpuidRegister::Ecx,
    shift: 0,
    width: 32,
};

/// SoC project stepping ID, assigned by vendor
pub const SOC_STEPPING_ID: CpuidFeature = CpuidFeature {
    leaf: 0x17,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 0,
    width: 32,
};

/// Vendor Brand ID string, bytes subleaf_nr * (0 -> 3)
pub const VENDOR_BRAND_A: CpuidFeature = CpuidFeature {
    leaf: 0x17,
    subleaf: 1,
    register: CpuidRegister::Eax,
    shift: 0,
    width: 32,
};

/// Vendor Brand ID string, bytes subleaf_nr * (4 -> 7)
pub const VENDOR_BRAND_B: CpuidFeature = CpuidFeature {
    leaf: 0x17,
    subleaf: 1,
    register: CpuidRegister::Ebx,
    shift: 0,
    width: 32,
};

/// Vendor Brand ID string, bytes subleaf_nr * (8 -> 11)
pub const VENDOR_BRAND_C: CpuidFeature = CpuidFeature {
    leaf: 0x17,
    subleaf: 1,
    register: CpuidRegister::Ecx,
    shift: 0,
    width: 32,
};

/// Vendor Brand ID string, bytes subleaf_nr * (12 -> 15)
pub const VENDOR_BRAND_D: CpuidFeature = CpuidFeature {
    leaf: 0x17,
    subleaf: 1,
    register: CpuidRegister::Edx,
    shift: 0,
    width: 32,
};

/// Performance monitoring unit version ID
pub const PMU_VERSION: CpuidFeature = CpuidFeature {
    leaf: 0xa,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 0,
    width: 8,
};

/// Number of general-purpose PMU counters per logical CPU
pub const NUM_COUNTERS_GP: CpuidFeature = CpuidFeature {
    leaf: 0xa,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 8,
    width: 8,
};

/// Bitwidth of PMU general-purpose counters
pub const BIT_WIDTH_GP: CpuidFeature = CpuidFeature {
    leaf: 0xa,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 16,
    width: 8,
};

/// Length of CPUID(0xa).EBX bit vector
pub const EVENTS_MASK_LEN: CpuidFeature = CpuidFeature {
    leaf: 0xa,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 24,
    width: 8,
};

/// Core cycle event not available
pub const NO_CORE_CYCLE: CpuidFeature = CpuidFeature {
    leaf: 0xa,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 0,
    width: 1,
};

/// Instruction retired event not available
pub const NO_INSTRUCTION_RETIRED: CpuidFeature = CpuidFeature {
    leaf: 0xa,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 1,
    width: 1,
};

/// Reference cycles event not available
pub const NO_REFERENCE_CYCLES: CpuidFeature = CpuidFeature {
    leaf: 0xa,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 2,
    width: 1,
};

/// LLC-reference event not available
pub const NO_LLC_REFERENCE: CpuidFeature = CpuidFeature {
    leaf: 0xa,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 3,
    width: 1,
};

/// LLC-misses event not available
pub const NO_LLC_MISSES: CpuidFeature = CpuidFeature {
    leaf: 0xa,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 4,
    width: 1,
};

/// Branch instruction retired event not available
pub const NO_BR_INSN_RETIRED: CpuidFeature = CpuidFeature {
    leaf: 0xa,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 5,
    width: 1,
};

/// Branch mispredict retired event not available
pub const NO_BR_MISSES_RETIRED: CpuidFeature = CpuidFeature {
    leaf: 0xa,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 6,
    width: 1,
};

/// Topdown slots event not available
pub const NO_TOPDOWN_SLOTS: CpuidFeature = CpuidFeature {
    leaf: 0xa,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 7,
    width: 1,
};

/// Topdown backend bound not available
pub const NO_BACKEND_BOUND: CpuidFeature = CpuidFeature {
    leaf: 0xa,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 8,
    width: 1,
};

/// Topdown bad speculation not available
pub const NO_BAD_SPECULATION: CpuidFeature = CpuidFeature {
    leaf: 0xa,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 9,
    width: 1,
};

/// Topdown frontend bound not available
pub const NO_FRONTEND_BOUND: CpuidFeature = CpuidFeature {
    leaf: 0xa,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 10,
    width: 1,
};

/// Topdown retiring not available
pub const NO_RETIRING: CpuidFeature = CpuidFeature {
    leaf: 0xa,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 11,
    width: 1,
};

/// LBR inserts not available
pub const NO_LBR_INSERTS: CpuidFeature = CpuidFeature {
    leaf: 0xa,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 12,
    width: 1,
};

/// Fixed-function PMU counters support bitmap
pub const PMU_FCOUNTERS_BITMAP: CpuidFeature = CpuidFeature {
    leaf: 0xa,
    subleaf: 0,
    register: CpuidRegister::Ecx,
    shift: 0,
    width: 32,
};

/// Number of fixed PMU counters
pub const NUM_COUNTERS_FIXED: CpuidFeature = CpuidFeature {
    leaf: 0xa,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 0,
    width: 5,
};

/// Bitwidth of PMU fixed counters
pub const BITWIDTH_FIXED: CpuidFeature = CpuidFeature {
    leaf: 0xa,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 5,
    width: 8,
};

/// AnyThread mode deprecation
pub const ANYTHREAD_DEPRECATION: CpuidFeature = CpuidFeature {
    leaf: 0xa,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 15,
    width: 1,
};

/// CPUID 0x1b subleaf type
pub const PCONFIG_SUBLEAF_TYPE: CpuidFeature = CpuidFeature {
    leaf: 0x1b,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 0,
    width: 12,
};

/// A supported PCONFIG target ID
pub const PCONFIG_TARGET_ID_X: CpuidFeature = CpuidFeature {
    leaf: 0x1b,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 0,
    width: 32,
};

/// A supported PCONFIG target ID
pub const PCONFIG_TARGET_ID_Y: CpuidFeature = CpuidFeature {
    leaf: 0x1b,
    subleaf: 0,
    register: CpuidRegister::Ecx,
    shift: 0,
    width: 32,
};

/// A supported PCONFIG target ID
pub const PCONFIG_TARGET_ID_Z: CpuidFeature = CpuidFeature {
    leaf: 0x1b,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 0,
    width: 32,
};

/// CPU info string bytes 48 - 51
pub const CPU_INFO_12: CpuidFeature = CpuidFeature {
    leaf: 0x80860006,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 0,
    width: 32,
};

/// CPU info string bytes 52 - 55
pub const CPU_INFO_13: CpuidFeature = CpuidFeature {
    leaf: 0x80860006,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 0,
    width: 32,
};

/// CPU info string bytes 56 - 59
pub const CPU_INFO_14: CpuidFeature = CpuidFeature {
    leaf: 0x80860006,
    subleaf: 0,
    register: CpuidRegister::Ecx,
    shift: 0,
    width: 32,
};

/// CPU info string bytes 60 - 63
pub const CPU_INFO_15: CpuidFeature = CpuidFeature {
    leaf: 0x80860006,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 0,
    width: 32,
};

/// CPU revision ID
pub const CPU_REV_ID: CpuidFeature = CpuidFeature {
    leaf: 0x80860002,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 0,
    width: 32,
};

/// CMS revision ID, mask component 2
pub const CMS_REV_MASK_2: CpuidFeature = CpuidFeature {
    leaf: 0x80860002,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 0,
    width: 8,
};

/// CMS revision ID, mask component 1
pub const CMS_REV_MASK_1: CpuidFeature = CpuidFeature {
    leaf: 0x80860002,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 8,
    width: 8,
};

/// CMS revision ID, minor
pub const CMS_REV_MINOR: CpuidFeature = CpuidFeature {
    leaf: 0x80860002,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 16,
    width: 8,
};

/// CMS revision ID, major
pub const CMS_REV_MAJOR: CpuidFeature = CpuidFeature {
    leaf: 0x80860002,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 24,
    width: 8,
};

/// CMS revision ID, mask component 3
pub const CMS_REV_MASK_3: CpuidFeature = CpuidFeature {
    leaf: 0x80860002,
    subleaf: 0,
    register: CpuidRegister::Ecx,
    shift: 0,
    width: 32,
};

/// Maximum Centaur/Zhaoxin leaf
pub const MAX_CNTR_LEAF: CpuidFeature = CpuidFeature {
    leaf: 0xc0000000,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 0,
    width: 32,
};

/// This core has a power efficiency ranking
pub const CORE_HAS_PWREFF_RANKING: CpuidFeature = CpuidFeature {
    leaf: 0x80000026,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 29,
    width: 1,
};

/// This domain level has hybrid (E, P) cores
pub const DOMAIN_HAS_HYBRID_CORES: CpuidFeature = CpuidFeature {
    leaf: 0x80000026,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 30,
    width: 1,
};

/// The 'Core' domain has asymmetric cores count
pub const DOMAIN_CORE_COUNT_ASYMM: CpuidFeature = CpuidFeature {
    leaf: 0x80000026,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 31,
    width: 1,
};

/// This core's static power efficiency ranking
pub const CORE_PWREFF_RANKING: CpuidFeature = CpuidFeature {
    leaf: 0x80000026,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 16,
    width: 8,
};

/// This core's native model ID
pub const CORE_NATIVE_MODEL_ID: CpuidFeature = CpuidFeature {
    leaf: 0x80000026,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 24,
    width: 4,
};

/// Maximum extended CPUID leaf
pub const MAX_EXT_LEAF: CpuidFeature = CpuidFeature {
    leaf: 0x80000000,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 0,
    width: 32,
};

/// Digital temperature sensor
pub const X86_FEATURE_DTHERM: CpuidFeature = CpuidFeature {
    leaf: 0x6,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 0,
    width: 1,
};

/// Intel Turbo Boost
pub const TURBO_BOOST: CpuidFeature = CpuidFeature {
    leaf: 0x6,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 1,
    width: 1,
};

/// Always-Running APIC Timer (not affected by p-state)
pub const X86_FEATURE_ARAT: CpuidFeature = CpuidFeature {
    leaf: 0x6,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 2,
    width: 1,
};

/// Power Limit Notification (PLN) event
pub const X86_FEATURE_PLN: CpuidFeature = CpuidFeature {
    leaf: 0x6,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 4,
    width: 1,
};

/// Clock modulation duty cycle extension
pub const ECMD: CpuidFeature = CpuidFeature {
    leaf: 0x6,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 5,
    width: 1,
};

/// Package thermal management
pub const X86_FEATURE_PTS: CpuidFeature = CpuidFeature {
    leaf: 0x6,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 6,
    width: 1,
};

/// HWP (Hardware P-states) base registers
pub const X86_FEATURE_HWP: CpuidFeature = CpuidFeature {
    leaf: 0x6,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 7,
    width: 1,
};

/// HWP notification (IA32_HWP_INTERRUPT MSR)
pub const X86_FEATURE_HWP_NOTIFY: CpuidFeature = CpuidFeature {
    leaf: 0x6,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 8,
    width: 1,
};

/// HWP activity window (IA32_HWP_REQUEST[bits 41:32])
pub const X86_FEATURE_HWP_ACT_WINDOW: CpuidFeature = CpuidFeature {
    leaf: 0x6,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 9,
    width: 1,
};

/// HWP Energy Performance Preference
pub const X86_FEATURE_HWP_EPP: CpuidFeature = CpuidFeature {
    leaf: 0x6,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 10,
    width: 1,
};

/// HWP Package Level Request
pub const X86_FEATURE_HWP_PKG_REQ: CpuidFeature = CpuidFeature {
    leaf: 0x6,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 11,
    width: 1,
};

/// HDC base registers
pub const HDC_BASE_REGS: CpuidFeature = CpuidFeature {
    leaf: 0x6,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 13,
    width: 1,
};

/// Intel Turbo Boost Max 3.0
pub const TURBO_BOOST_3_0: CpuidFeature = CpuidFeature {
    leaf: 0x6,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 14,
    width: 1,
};

/// HWP Highest Performance change
pub const HWP_CAPABILITIES: CpuidFeature = CpuidFeature {
    leaf: 0x6,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 15,
    width: 1,
};

/// HWP PECI override
pub const HWP_PECI_OVERRIDE: CpuidFeature = CpuidFeature {
    leaf: 0x6,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 16,
    width: 1,
};

/// Flexible HWP
pub const HWP_FLEXIBLE: CpuidFeature = CpuidFeature {
    leaf: 0x6,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 17,
    width: 1,
};

/// IA32_HWP_REQUEST MSR fast access mode
pub const HWP_FAST: CpuidFeature = CpuidFeature {
    leaf: 0x6,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 18,
    width: 1,
};

/// HW_FEEDBACK MSRs
pub const X86_FEATURE_HFI: CpuidFeature = CpuidFeature {
    leaf: 0x6,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 19,
    width: 1,
};

/// Ignoring idle logical CPU HWP request is supported
pub const HWP_IGNORE_IDLE: CpuidFeature = CpuidFeature {
    leaf: 0x6,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 20,
    width: 1,
};

/// IA32_HWP_CTL MSR
pub const HWP_CTL: CpuidFeature = CpuidFeature {
    leaf: 0x6,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 22,
    width: 1,
};

/// Intel thread director
pub const THREAD_DIRECTOR: CpuidFeature = CpuidFeature {
    leaf: 0x6,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 23,
    width: 1,
};

/// IA32_THERM_INTERRUPT MSR bit 25
pub const THERM_INTERRUPT_BIT25: CpuidFeature = CpuidFeature {
    leaf: 0x6,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 24,
    width: 1,
};

/// Digital thermometer thresholds
pub const N_THERM_THRESHOLDS: CpuidFeature = CpuidFeature {
    leaf: 0x6,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 0,
    width: 4,
};

/// Number of Intel Thread Director classes
pub const HW_FEEDBACK_NCLASSES: CpuidFeature = CpuidFeature {
    leaf: 0x6,
    subleaf: 0,
    register: CpuidRegister::Ecx,
    shift: 8,
    width: 8,
};

/// Performance capability reporting
pub const PERFCAP_REPORTING: CpuidFeature = CpuidFeature {
    leaf: 0x6,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 0,
    width: 1,
};

/// Energy efficiency capability reporting
pub const ENCAP_REPORTING: CpuidFeature = CpuidFeature {
    leaf: 0x6,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 1,
    width: 1,
};

/// Feedback interface structure size, in 4K pages
pub const FEEDBACK_SZ: CpuidFeature = CpuidFeature {
    leaf: 0x6,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 8,
    width: 4,
};

/// This logical CPU hardware feedback interface index
pub const THIS_LCPU_HWFDBK_IDX: CpuidFeature = CpuidFeature {
    leaf: 0x6,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 16,
    width: 16,
};

/// Cache type field
pub const CACHE_TYPE: CpuidFeature = CpuidFeature {
    leaf: 0x8000001d,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 0,
    width: 5,
};

/// Cache level (1-based)
pub const CACHE_LEVEL: CpuidFeature = CpuidFeature {
    leaf: 0x8000001d,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 5,
    width: 3,
};

/// Self-initializing cache level
pub const CACHE_SELF_INIT: CpuidFeature = CpuidFeature {
    leaf: 0x8000001d,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 8,
    width: 1,
};

/// Fully-associative cache
pub const FULLY_ASSOCIATIVE: CpuidFeature = CpuidFeature {
    leaf: 0x8000001d,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 9,
    width: 1,
};

/// Number of logical CPUs sharing cache
pub const NUM_THREADS_SHARING: CpuidFeature = CpuidFeature {
    leaf: 0x8000001d,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 14,
    width: 12,
};

/// System coherency line size (0-based)
pub const CACHE_LINESIZE: CpuidFeature = CpuidFeature {
    leaf: 0x8000001d,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 0,
    width: 12,
};

/// Physical line partitions (0-based)
pub const CACHE_NPARTITIONS: CpuidFeature = CpuidFeature {
    leaf: 0x8000001d,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 12,
    width: 10,
};

/// Ways of associativity (0-based)
pub const CACHE_NWAYS: CpuidFeature = CpuidFeature {
    leaf: 0x8000001d,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 22,
    width: 10,
};

/// Cache number of sets (0-based)
pub const CACHE_NSETS: CpuidFeature = CpuidFeature {
    leaf: 0x8000001d,
    subleaf: 0,
    register: CpuidRegister::Ecx,
    shift: 0,
    width: 31,
};

/// WBINVD/INVD not guaranteed for Remote Lower-Level caches
pub const WBINVD_RLL_NO_GUARANTEE: CpuidFeature = CpuidFeature {
    leaf: 0x8000001d,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 0,
    width: 1,
};

/// Cache is inclusive of Lower-Level caches
pub const LL_INCLUSIVE: CpuidFeature = CpuidFeature {
    leaf: 0x8000001d,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 1,
    width: 1,
};

/// Secure Memory Encryption
pub const X86_FEATURE_SME: CpuidFeature = CpuidFeature {
    leaf: 0x8000001f,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 0,
    width: 1,
};

/// Secure Encrypted Virtualization
pub const X86_FEATURE_SEV: CpuidFeature = CpuidFeature {
    leaf: 0x8000001f,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 1,
    width: 1,
};

/// VM Page Flush MSR
pub const X86_FEATURE_VM_PAGE_FLUSH: CpuidFeature = CpuidFeature {
    leaf: 0x8000001f,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 2,
    width: 1,
};

/// SEV Encrypted State
pub const X86_FEATURE_SEV_ES: CpuidFeature = CpuidFeature {
    leaf: 0x8000001f,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 3,
    width: 1,
};

/// SEV secure nested paging
pub const SEV_NESTED_PAGING: CpuidFeature = CpuidFeature {
    leaf: 0x8000001f,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 4,
    width: 1,
};

/// VMPL
pub const VM_PERMISSION_LEVELS: CpuidFeature = CpuidFeature {
    leaf: 0x8000001f,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 5,
    width: 1,
};

/// RPMQUERY instruction
pub const RPMQUERY: CpuidFeature = CpuidFeature {
    leaf: 0x8000001f,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 6,
    width: 1,
};

/// VMPL supervisor shadow stack
pub const VMPL_SSS: CpuidFeature = CpuidFeature {
    leaf: 0x8000001f,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 7,
    width: 1,
};

/// Secure TSC
pub const SECURE_TSC: CpuidFeature = CpuidFeature {
    leaf: 0x8000001f,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 8,
    width: 1,
};

/// Hardware virtualizes TSC_AUX
pub const X86_FEATURE_V_TSC_AUX: CpuidFeature = CpuidFeature {
    leaf: 0x8000001f,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 9,
    width: 1,
};

/// Cache coherency enforcement across encryption domains
pub const X86_FEATURE_SME_COHERENT: CpuidFeature = CpuidFeature {
    leaf: 0x8000001f,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 10,
    width: 1,
};

/// SEV guest mandates 64-bit hypervisor
pub const REQ_64BIT_HYPERVISOR: CpuidFeature = CpuidFeature {
    leaf: 0x8000001f,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 11,
    width: 1,
};

/// Restricted Injection supported
pub const RESTRICTED_INJECTION: CpuidFeature = CpuidFeature {
    leaf: 0x8000001f,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 12,
    width: 1,
};

/// Alternate Injection supported
pub const ALTERNATE_INJECTION: CpuidFeature = CpuidFeature {
    leaf: 0x8000001f,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 13,
    width: 1,
};

/// SEV-ES: Full debug state swap
pub const DEBUG_SWAP: CpuidFeature = CpuidFeature {
    leaf: 0x8000001f,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 14,
    width: 1,
};

/// SEV-ES: Disallowing IBS use by the host
pub const DISALLOW_HOST_IBS: CpuidFeature = CpuidFeature {
    leaf: 0x8000001f,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 15,
    width: 1,
};

/// Virtual Transparent Encryption
pub const VIRT_TRANSPARENT_ENC: CpuidFeature = CpuidFeature {
    leaf: 0x8000001f,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 16,
    width: 1,
};

/// SEV_FEATURES: VmgexitParameter
pub const VMGEXIT_PARAMETER: CpuidFeature = CpuidFeature {
    leaf: 0x8000001f,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 17,
    width: 1,
};

/// Virtual TOM MSR
pub const VIRT_TOM_MSR: CpuidFeature = CpuidFeature {
    leaf: 0x8000001f,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 18,
    width: 1,
};

/// SEV-ES guests: IBS state virtualization
pub const VIRT_IBS: CpuidFeature = CpuidFeature {
    leaf: 0x8000001f,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 19,
    width: 1,
};

/// VMSA register protection
pub const VMSA_REG_PROTECTION: CpuidFeature = CpuidFeature {
    leaf: 0x8000001f,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 24,
    width: 1,
};

/// SMT protection
pub const SMT_PROTECTION: CpuidFeature = CpuidFeature {
    leaf: 0x8000001f,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 25,
    width: 1,
};

/// SVSM communication page MSR
pub const SVSM_PAGE_MSR: CpuidFeature = CpuidFeature {
    leaf: 0x8000001f,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 28,
    width: 1,
};

/// VIRT_RMPUPDATE/VIRT_PSMASH MSRs
pub const NESTED_VIRT_SNP_MSR: CpuidFeature = CpuidFeature {
    leaf: 0x8000001f,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 29,
    width: 1,
};

/// PTE bit number to enable memory encryption
pub const PTE_CBIT_POS: CpuidFeature = CpuidFeature {
    leaf: 0x8000001f,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 0,
    width: 6,
};

/// Reduction of phys address space in bits
pub const PHYS_ADDR_REDUCTION_NBITS: CpuidFeature = CpuidFeature {
    leaf: 0x8000001f,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 6,
    width: 6,
};

/// Number of VM permission levels (VMPL)
pub const VMPL_COUNT: CpuidFeature = CpuidFeature {
    leaf: 0x8000001f,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 12,
    width: 4,
};

/// Max number of simultaneous encrypted guests
pub const ENC_GUESTS_MAX: CpuidFeature = CpuidFeature {
    leaf: 0x8000001f,
    subleaf: 0,
    register: CpuidRegister::Ecx,
    shift: 0,
    width: 32,
};

/// Minimum ASID for SEV-enabled SEV-ES-disabled guest
pub const MIN_SEV_ASID_NO_SEV_ES: CpuidFeature = CpuidFeature {
    leaf: 0x8000001f,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 0,
    width: 32,
};

/// Number of cores in the physical package
pub const NUM_CORES_ON_DIE: CpuidFeature = CpuidFeature {
    leaf: 0x4,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 26,
    width: 6,
};

/// Not a direct-mapped cache (complex function)
pub const COMPLEX_INDEXING: CpuidFeature = CpuidFeature {
    leaf: 0x4,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 2,
    width: 1,
};

/// IBS feature flags
pub const IBS_FLAGS: CpuidFeature = CpuidFeature {
    leaf: 0x8000001b,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 0,
    width: 1,
};

/// IBS fetch sampling
pub const IBS_FETCH_SAMPLING: CpuidFeature = CpuidFeature {
    leaf: 0x8000001b,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 1,
    width: 1,
};

/// IBS execution sampling
pub const IBS_OP_SAMPLING: CpuidFeature = CpuidFeature {
    leaf: 0x8000001b,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 2,
    width: 1,
};

/// IBS read/write of op counter
pub const IBS_RDWR_OP_COUNTER: CpuidFeature = CpuidFeature {
    leaf: 0x8000001b,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 3,
    width: 1,
};

/// IBS OP counting mode
pub const IBS_OP_COUNT: CpuidFeature = CpuidFeature {
    leaf: 0x8000001b,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 4,
    width: 1,
};

/// IBS branch target address reporting
pub const IBS_BRANCH_TARGET: CpuidFeature = CpuidFeature {
    leaf: 0x8000001b,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 5,
    width: 1,
};

/// IBS IbsOpCurCnt/IbsOpMaxCnt extend by 7 bits
pub const IBS_OP_COUNTERS_EXT: CpuidFeature = CpuidFeature {
    leaf: 0x8000001b,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 6,
    width: 1,
};

/// IBS invalid RIP indication
pub const IBS_RIP_INVALID_CHK: CpuidFeature = CpuidFeature {
    leaf: 0x8000001b,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 7,
    width: 1,
};

/// IBS fused branch micro-op indication
pub const IBS_OP_BRANCH_FUSE: CpuidFeature = CpuidFeature {
    leaf: 0x8000001b,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 8,
    width: 1,
};

/// IBS Fetch Control Extended MSR
pub const IBS_FETCH_CTL_EXT: CpuidFeature = CpuidFeature {
    leaf: 0x8000001b,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 9,
    width: 1,
};

/// IBS op data 4 MSR
pub const IBS_OP_DATA_4: CpuidFeature = CpuidFeature {
    leaf: 0x8000001b,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 10,
    width: 1,
};

/// IBS L3-miss filtering (Zen4+)
pub const IBS_L3_MISS_FILTER: CpuidFeature = CpuidFeature {
    leaf: 0x8000001b,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 11,
    width: 1,
};

/// Subleaf 0, this subleaf
pub const SUBLEAF_0: CpuidFeature = CpuidFeature {
    leaf: 0x23,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 0,
    width: 1,
};

/// Subleaf 1, PMU counter bitmaps
pub const COUNTERS_SUBLEAF: CpuidFeature = CpuidFeature {
    leaf: 0x23,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 1,
    width: 1,
};

/// Subleaf 2, Auto Counter Reload bitmaps
pub const ACR_SUBLEAF: CpuidFeature = CpuidFeature {
    leaf: 0x23,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 2,
    width: 1,
};

/// Subleaf 3, PMU event bitmaps
pub const EVENTS_SUBLEAF: CpuidFeature = CpuidFeature {
    leaf: 0x23,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 3,
    width: 1,
};

/// Subleaf 4, PEBS capabilities
pub const PEBS_CAPS_SUBLEAF: CpuidFeature = CpuidFeature {
    leaf: 0x23,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 4,
    width: 1,
};

/// Subleaf 5, Arch PEBS bitmaps
pub const PEBS_SUBLEAF: CpuidFeature = CpuidFeature {
    leaf: 0x23,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 5,
    width: 1,
};

/// IA32_PERFEVTSELx MSRs UnitMask2 bit
pub const UNITMASK2: CpuidFeature = CpuidFeature {
    leaf: 0x23,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 0,
    width: 1,
};

/// IA32_PERFEVTSELx MSRs EQ bit
pub const EQ: CpuidFeature = CpuidFeature {
    leaf: 0x23,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 1,
    width: 1,
};

/// RDPMC userspace disable
pub const RDPMC_USER_DISABLE: CpuidFeature = CpuidFeature {
    leaf: 0x23,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 2,
    width: 1,
};

/// Bitmap of general-purpose PMU counters
pub const GP_COUNTERS: CpuidFeature = CpuidFeature {
    leaf: 0x23,
    subleaf: 1,
    register: CpuidRegister::Eax,
    shift: 0,
    width: 32,
};

/// Bitmap of fixed PMU counters
pub const FIXED_COUNTERS: CpuidFeature = CpuidFeature {
    leaf: 0x23,
    subleaf: 1,
    register: CpuidRegister::Ebx,
    shift: 0,
    width: 32,
};

/// Bitmap of general-purpose counters that can be reloaded
pub const ACR_GP_RELOAD: CpuidFeature = CpuidFeature {
    leaf: 0x23,
    subleaf: 2,
    register: CpuidRegister::Eax,
    shift: 0,
    width: 32,
};

/// Bitmap of fixed counters that can be reloaded
pub const ACR_FIXED_RELOAD: CpuidFeature = CpuidFeature {
    leaf: 0x23,
    subleaf: 2,
    register: CpuidRegister::Ebx,
    shift: 0,
    width: 32,
};

/// Bitmap of general-purpose counters that can trigger reloads
pub const ACR_GP_TRIGGER: CpuidFeature = CpuidFeature {
    leaf: 0x23,
    subleaf: 2,
    register: CpuidRegister::Ecx,
    shift: 0,
    width: 32,
};

/// Bitmap of fixed counters that can trigger reloads
pub const ACR_FIXED_TRIGGER: CpuidFeature = CpuidFeature {
    leaf: 0x23,
    subleaf: 2,
    register: CpuidRegister::Edx,
    shift: 0,
    width: 32,
};

/// Core cycles event
pub const CORE_CYCLES_EVT: CpuidFeature = CpuidFeature {
    leaf: 0x23,
    subleaf: 3,
    register: CpuidRegister::Eax,
    shift: 0,
    width: 1,
};

/// Instructions retired event
pub const INSN_RETIRED_EVT: CpuidFeature = CpuidFeature {
    leaf: 0x23,
    subleaf: 3,
    register: CpuidRegister::Eax,
    shift: 1,
    width: 1,
};

/// Reference cycles event
pub const REF_CYCLES_EVT: CpuidFeature = CpuidFeature {
    leaf: 0x23,
    subleaf: 3,
    register: CpuidRegister::Eax,
    shift: 2,
    width: 1,
};

/// Last-level cache references event
pub const LLC_REFS_EVT: CpuidFeature = CpuidFeature {
    leaf: 0x23,
    subleaf: 3,
    register: CpuidRegister::Eax,
    shift: 3,
    width: 1,
};

/// Last-level cache misses event
pub const LLC_MISSES_EVT: CpuidFeature = CpuidFeature {
    leaf: 0x23,
    subleaf: 3,
    register: CpuidRegister::Eax,
    shift: 4,
    width: 1,
};

/// Branch instruction retired event
pub const BR_INSN_RET_EVT: CpuidFeature = CpuidFeature {
    leaf: 0x23,
    subleaf: 3,
    register: CpuidRegister::Eax,
    shift: 5,
    width: 1,
};

/// Branch mispredict retired event
pub const BR_MISPR_EVT: CpuidFeature = CpuidFeature {
    leaf: 0x23,
    subleaf: 3,
    register: CpuidRegister::Eax,
    shift: 6,
    width: 1,
};

/// Topdown slots event
pub const TD_SLOTS_EVT: CpuidFeature = CpuidFeature {
    leaf: 0x23,
    subleaf: 3,
    register: CpuidRegister::Eax,
    shift: 7,
    width: 1,
};

/// Topdown backend bound event
pub const TD_BACKEND_BOUND_EVT: CpuidFeature = CpuidFeature {
    leaf: 0x23,
    subleaf: 3,
    register: CpuidRegister::Eax,
    shift: 8,
    width: 1,
};

/// Topdown bad speculation event
pub const TD_BAD_SPEC_EVT: CpuidFeature = CpuidFeature {
    leaf: 0x23,
    subleaf: 3,
    register: CpuidRegister::Eax,
    shift: 9,
    width: 1,
};

/// Topdown frontend bound event
pub const TD_FRONTEND_BOUND_EVT: CpuidFeature = CpuidFeature {
    leaf: 0x23,
    subleaf: 3,
    register: CpuidRegister::Eax,
    shift: 10,
    width: 1,
};

/// Topdown retiring event
pub const TD_RETIRING_EVT: CpuidFeature = CpuidFeature {
    leaf: 0x23,
    subleaf: 3,
    register: CpuidRegister::Eax,
    shift: 11,
    width: 1,
};

/// ALLOW_IN_RECORD bit in MSRs
pub const ALLOW_IN_RECORD: CpuidFeature = CpuidFeature {
    leaf: 0x23,
    subleaf: 4,
    register: CpuidRegister::Ebx,
    shift: 3,
    width: 1,
};

/// Counters group sub-group general-purpose counters
pub const COUNTERS_GP: CpuidFeature = CpuidFeature {
    leaf: 0x23,
    subleaf: 4,
    register: CpuidRegister::Ebx,
    shift: 4,
    width: 1,
};

/// Counters group sub-group fixed-function counters
pub const COUNTERS_FIXED: CpuidFeature = CpuidFeature {
    leaf: 0x23,
    subleaf: 4,
    register: CpuidRegister::Ebx,
    shift: 5,
    width: 1,
};

/// Counters group sub-group performance metrics
pub const COUNTERS_METRICS: CpuidFeature = CpuidFeature {
    leaf: 0x23,
    subleaf: 4,
    register: CpuidRegister::Ebx,
    shift: 6,
    width: 1,
};

/// LBR group
pub const LBR: CpuidFeature = CpuidFeature {
    leaf: 0x23,
    subleaf: 4,
    register: CpuidRegister::Ebx,
    shift: 8,
    width: 2,
};

/// XER group
pub const XER: CpuidFeature = CpuidFeature {
    leaf: 0x23,
    subleaf: 4,
    register: CpuidRegister::Ebx,
    shift: 16,
    width: 8,
};

/// GPR group
pub const GPR: CpuidFeature = CpuidFeature {
    leaf: 0x23,
    subleaf: 4,
    register: CpuidRegister::Ebx,
    shift: 29,
    width: 1,
};

/// AUX group
pub const AUX: CpuidFeature = CpuidFeature {
    leaf: 0x23,
    subleaf: 4,
    register: CpuidRegister::Ebx,
    shift: 30,
    width: 1,
};

/// Architectural PEBS general-purpose counters
pub const PEBS_GP: CpuidFeature = CpuidFeature {
    leaf: 0x23,
    subleaf: 5,
    register: CpuidRegister::Eax,
    shift: 0,
    width: 32,
};

/// Architectural PEBS PDIST general-purpose counters
pub const PEBS_PDIST_GP: CpuidFeature = CpuidFeature {
    leaf: 0x23,
    subleaf: 5,
    register: CpuidRegister::Ebx,
    shift: 0,
    width: 32,
};

/// Architectural PEBS fixed counters
pub const PEBS_FIXED: CpuidFeature = CpuidFeature {
    leaf: 0x23,
    subleaf: 5,
    register: CpuidRegister::Ecx,
    shift: 0,
    width: 32,
};

/// Architectural PEBS PDIST fixed counters
pub const PEBS_PDIST_FIXED: CpuidFeature = CpuidFeature {
    leaf: 0x23,
    subleaf: 5,
    register: CpuidRegister::Edx,
    shift: 0,
    width: 32,
};

/// Maximum leaf 0x14 subleaf
pub const PT_MAX_SUBLEAF: CpuidFeature = CpuidFeature {
    leaf: 0x14,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 0,
    width: 32,
};

/// IA32_RTIT_CR3_MATCH is accessible
pub const CR3_FILTERING: CpuidFeature = CpuidFeature {
    leaf: 0x14,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 0,
    width: 1,
};

/// Configurable PSB and cycle-accurate mode
pub const PSB_CYC: CpuidFeature = CpuidFeature {
    leaf: 0x14,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 1,
    width: 1,
};

/// IP/TraceStop filtering; Warm-reset PT MSRs preservation
pub const IP_FILTERING: CpuidFeature = CpuidFeature {
    leaf: 0x14,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 2,
    width: 1,
};

/// MTC timing packet; COFI-based packets suppression
pub const MTC_TIMING: CpuidFeature = CpuidFeature {
    leaf: 0x14,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 3,
    width: 1,
};

/// PTWRITE instruction
pub const PTWRITE: CpuidFeature = CpuidFeature {
    leaf: 0x14,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 4,
    width: 1,
};

/// Power Event Trace
pub const POWER_EVENT_TRACE: CpuidFeature = CpuidFeature {
    leaf: 0x14,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 5,
    width: 1,
};

/// PSB and PMI preservation
pub const PSB_PMI_PRESERVE: CpuidFeature = CpuidFeature {
    leaf: 0x14,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 6,
    width: 1,
};

/// Event Trace packet generation
pub const EVENT_TRACE: CpuidFeature = CpuidFeature {
    leaf: 0x14,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 7,
    width: 1,
};

/// TNT packet generation disable
pub const TNT_DISABLE: CpuidFeature = CpuidFeature {
    leaf: 0x14,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 8,
    width: 1,
};

/// ToPA output scheme
pub const TOPA_OUTPUT: CpuidFeature = CpuidFeature {
    leaf: 0x14,
    subleaf: 0,
    register: CpuidRegister::Ecx,
    shift: 0,
    width: 1,
};

/// ToPA tables can hold multiple entries
pub const TOPA_MULTIPLE_ENTRIES: CpuidFeature = CpuidFeature {
    leaf: 0x14,
    subleaf: 0,
    register: CpuidRegister::Ecx,
    shift: 1,
    width: 1,
};

/// Single-range output
pub const SINGLE_RANGE_OUTPUT: CpuidFeature = CpuidFeature {
    leaf: 0x14,
    subleaf: 0,
    register: CpuidRegister::Ecx,
    shift: 2,
    width: 1,
};

/// Trace Transport subsystem output
pub const TRACE_TRANSPORT_OUTPUT: CpuidFeature = CpuidFeature {
    leaf: 0x14,
    subleaf: 0,
    register: CpuidRegister::Ecx,
    shift: 3,
    width: 1,
};

/// IP payloads have LIP values (CS base included)
pub const IP_PAYLOADS_LIP: CpuidFeature = CpuidFeature {
    leaf: 0x14,
    subleaf: 0,
    register: CpuidRegister::Ecx,
    shift: 31,
    width: 1,
};

/// Number of configurable address ranges
pub const NUM_ADDRESS_RANGES: CpuidFeature = CpuidFeature {
    leaf: 0x14,
    subleaf: 1,
    register: CpuidRegister::Eax,
    shift: 0,
    width: 3,
};

/// MTC period encodings bitmap
pub const MTC_PERIODS_BMP: CpuidFeature = CpuidFeature {
    leaf: 0x14,
    subleaf: 1,
    register: CpuidRegister::Eax,
    shift: 16,
    width: 16,
};

/// Cycle Threshold encodings bitmap
pub const CYCLE_THRESHOLDS_BMP: CpuidFeature = CpuidFeature {
    leaf: 0x14,
    subleaf: 1,
    register: CpuidRegister::Ebx,
    shift: 0,
    width: 16,
};

/// Configurable PSB frequency encodings bitmap
pub const PSB_PERIODS_BMP: CpuidFeature = CpuidFeature {
    leaf: 0x14,
    subleaf: 1,
    register: CpuidRegister::Ebx,
    shift: 16,
    width: 16,
};

/// CPL0-only key locker restriction
pub const KL_CPL0_ONLY: CpuidFeature = CpuidFeature {
    leaf: 0x19,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 0,
    width: 1,
};

/// No-encrypt key locker restriction
pub const KL_NO_ENCRYPT: CpuidFeature = CpuidFeature {
    leaf: 0x19,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 1,
    width: 1,
};

/// No-decrypt key locker restriction
pub const KL_NO_DECRYPT: CpuidFeature = CpuidFeature {
    leaf: 0x19,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 2,
    width: 1,
};

/// AES key locker instructions
pub const AES_KEYLOCKER: CpuidFeature = CpuidFeature {
    leaf: 0x19,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 0,
    width: 1,
};

/// AES wide key locker instructions
pub const AES_KEYLOCKER_WIDE: CpuidFeature = CpuidFeature {
    leaf: 0x19,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 2,
    width: 1,
};

/// Key locker MSRs and IWKEY backups
pub const KL_MSR_IWKEY: CpuidFeature = CpuidFeature {
    leaf: 0x19,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 4,
    width: 1,
};

/// LOADIWKEY NoBackup parameter
pub const LOADIWKEY_NO_BACKUP: CpuidFeature = CpuidFeature {
    leaf: 0x19,
    subleaf: 0,
    register: CpuidRegister::Ecx,
    shift: 0,
    width: 1,
};

/// IWKEY randomization
pub const IWKEY_RAND: CpuidFeature = CpuidFeature {
    leaf: 0x19,
    subleaf: 0,
    register: CpuidRegister::Ecx,
    shift: 1,
    width: 1,
};

/// CPU info string bytes 0 - 3
pub const CPU_INFO_0: CpuidFeature = CpuidFeature {
    leaf: 0x80860003,
    subleaf: 0,
    register: CpuidRegister::Eax,
    shift: 0,
    width: 32,
};

/// CPU info string bytes 4 - 7
pub const CPU_INFO_1: CpuidFeature = CpuidFeature {
    leaf: 0x80860003,
    subleaf: 0,
    register: CpuidRegister::Ebx,
    shift: 0,
    width: 32,
};

/// CPU info string bytes 8 - 11
pub const CPU_INFO_2: CpuidFeature = CpuidFeature {
    leaf: 0x80860003,
    subleaf: 0,
    register: CpuidRegister::Ecx,
    shift: 0,
    width: 32,
};

/// CPU info string bytes 12 - 15
pub const CPU_INFO_3: CpuidFeature = CpuidFeature {
    leaf: 0x80860003,
    subleaf: 0,
    register: CpuidRegister::Edx,
    shift: 0,
    width: 32,
};
