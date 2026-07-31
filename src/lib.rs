// SPDX-License-Identifier: MIT
//

#![no_std]

mod common;

// `leaves.rs` is auto-generated; its doc comments contain bracketed bitfield
// notation (e.g. `RAX[5]`, `bits[12:31]`) that rustdoc reads as intra-doc links.
#[allow(rustdoc::broken_intra_doc_links)]
pub mod leaves;

pub use common::{CpuidFeature, CpuidRegister};

pub mod backend;
