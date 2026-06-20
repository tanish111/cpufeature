// SPDX-License-Identifier: MIT
//

#![no_std]

pub mod leaves {
    pub mod common;

    pub use common::{CpuidFeature, CpuidRegister};

    include!(concat!(env!("OUT_DIR"), "/leaves_registry.rs"));
}

pub mod backend;
