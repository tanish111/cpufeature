# cpufeature

[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

A `#![no_std]` Rust library for querying x86/x86-64 CPU features via `CPUID`.

Feature descriptors are auto-generated from the [x86-cpuid.org](https://x86-cpuid.org)
database, so each field knows exactly which leaf, subleaf, register, and bit range
it lives in. A small [`CpuidBackend`] trait runs `CPUID` and extracts the field.

## Features

- `no_std`, zero dependencies.

## Getting started

Add the crate to your `Cargo.toml`:

```toml
[dependencies]
cpufeature = "0.1"
```

Query a feature by implementing the backend trait (the default methods do the work):

```rust
use cpufeature::backend::CpuidBackend;
use cpufeature::leaves;

struct Cpu;
impl CpuidBackend for Cpu {}

// Read the highest supported standard CPUID leaf.
if let Some(max_leaf) = Cpu::cpuid_value(&leaves::MAX_STD_LEAF) {
    // ...
}
```

To extract a field manually, [`CpuidFeature`](CpuidFeature) exposes its `leaf`, `subleaf`,
`register`, `shift`, and `width`.

> **Note:** This crate targets `x86`/`x86_64` and uses `core::arch::x86_64`.

## License

Licensed under the [MIT License](LICENSE).
