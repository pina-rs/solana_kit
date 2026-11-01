#![doc(
    html_logo_url = "https://raw.githubusercontent.com/pina-rs/solana_kit/main/assets/brand/solana-kit-mark.png"
)]
#![doc(
    html_favicon_url = "https://raw.githubusercontent.com/pina-rs/solana_kit/main/assets/brand/solana-kit-favicon-32.png"
)]
#![no_std]
#![forbid(unsafe_code)]
#![deny(missing_docs)]
#![doc = include_str!("../README.md")]

extern crate alloc;

mod coder;
mod decode;
mod discriminator;
mod encode;
mod errors;
mod events;

mod idl;
mod parse;
mod scalar;
mod types;
mod value;

pub use coder::{Coder, DecodedAccount, DecodedValue, ValueCodec};
pub use discriminator::{
    DISCRIMINATOR_LENGTH, account_discriminator, anchor_sighash, event_discriminator,
    has_discriminator, instruction_discriminator, sha256,
};
pub use errors::{Error, ProgramError, STANDARD_ERRORS, StandardError, program_error};
pub use events::DecodedEvent;
pub use idl::{CustomError, Definition, Field, Idl, Instruction, NamedDiscriminator, Variant};
pub use types::{Primitive, Type, type_name};
pub use value::{Limits, Value, Values};
