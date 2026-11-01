#![doc(
    html_logo_url = "https://raw.githubusercontent.com/pina-rs/solana_kit/main/assets/brand/solana-kit-mark.png"
)]
#![doc(
    html_favicon_url = "https://raw.githubusercontent.com/pina-rs/solana_kit/main/assets/brand/solana-kit-favicon-32.png"
)]
#![no_std]
#![forbid(unsafe_code)]
#![warn(missing_docs)]
#![doc = include_str!("../README.md")]

extern crate alloc;

mod generated;
mod helpers;

pub use generated::*;
pub use helpers::*;
