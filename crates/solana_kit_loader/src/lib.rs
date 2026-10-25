#![no_std]
#![forbid(unsafe_code)]
#![deny(missing_docs)]
#![doc = include_str!("../README.md")]
extern crate alloc;

/// Upgradeable Loader v3 data and ordered account builders.
pub mod v3 {
    include!("loaderV3.rs");
    mod instructions {
        include!("loaderV3_instructions.rs");
    }
    pub use instructions::*;
}
/// Loader v4 data and ordered account builders.
pub mod v4 {
    include!("loaderV4.rs");
    mod instructions {
        include!("loaderV4_instructions.rs");
    }
    pub use instructions::*;
}
mod accounts;
pub use accounts::*;
mod helpers;
pub use helpers::*;
