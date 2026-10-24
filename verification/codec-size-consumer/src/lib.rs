#![no_std]
#![forbid(unsafe_code)]
//! Ordinary outside-consumer proof source for selected metadata contracts.
//!
//! Validating one Encoder implementation cannot grant another input type.
//!
//! ```compile_fail
//! use solana_kit_codec_size_consumer::Multi;
//! use solana_kit_core::codecs::{Encoder, Number, Paired};
//! let pair = Paired::<_, _, u8, u8>::try_new(Multi, Number::<u8>::LE).unwrap();
//! pair.encode(&513u16, 2).unwrap();
//! ```
//!
//! Raw tuple construction is unavailable.
//!
//! ```compile_fail
//! use solana_kit_codec_size_consumer::Multi;
//! use solana_kit_core::codecs::{Number, Paired};
//! let pair = Paired(Multi, Number::<u8>::LE);
//! ```
//!
//! The validated parts cannot be replaced through fields.
//!
//! ```compile_fail
//! use solana_kit_codec_size_consumer::Multi;
//! use solana_kit_core::codecs::{Number, Paired};
//! let mut pair = Paired::<_, _, u8, u8>::try_new(Multi, Number::<u8>::LE).unwrap();
//! pair.encoder = Multi;
//! ```
//!
//! Decoder replacement is independently unavailable.
//!
//! ```compile_fail
//! use solana_kit_codec_size_consumer::Multi;
//! use solana_kit_core::codecs::{Number, Paired};
//! let mut pair = Paired::<_, _, u8, u8>::try_new(Multi, Number::<u8>::LE).unwrap();
//! pair.decoder = Number::<u8>::BE;
//! ```
//!
//! A struct literal cannot assemble the parts directly.
//!
//! ```compile_fail
//! use solana_kit_codec_size_consumer::Multi;
//! use solana_kit_core::codecs::{Number, Paired};
//! let pair = Paired::<_, _, u8, u8> {
//!     encoder: Multi,
//!     decoder: Number::<u8>::LE,
//!     values: Default::default(),
//! };
//! ```
//!
//! There is no unchecked constructor.
//!
//! ```compile_fail
//! use solana_kit_codec_size_consumer::Multi;
//! use solana_kit_core::codecs::{Number, Paired};
//! let pair = Paired::<_, _, u8, u8>::new_unchecked(Multi, Number::<u8>::LE);
//! ```
//!
//! A tuple conversion cannot skip validation.
//!
//! ```compile_fail
//! use solana_kit_codec_size_consumer::Multi;
//! use solana_kit_core::codecs::{Number, Paired};
//! let pair: Paired<Multi, Number<u8>, u8, u8> = (Multi, Number::<u8>::LE).into();
//! ```
//!
//! Default cannot create an unchecked identity.
//!
//! ```compile_fail
//! use solana_kit_codec_size_consumer::Multi;
//! use solana_kit_core::codecs::{Number, Paired};
//! let pair: Paired<Multi, Number<u8>, u8, u8> = Default::default();
//! ```
extern crate alloc;

use core::{fmt::Debug, marker::PhantomData};
use solana_kit_core::codecs::{
    Decoder, Encoder, Error, Limits, Number, PairError, Paired, Reader, Size, Writer,
};

#[path = "../../../crates/solana_kit_core/tests/codec_size/algebra.rs"]
mod algebra;
#[path = "../../../crates/solana_kit_core/tests/codec_size/overrides.rs"]
mod overrides;
#[path = "../../../crates/solana_kit_core/tests/codec_size/pairing.rs"]
mod pairing;
#[path = "../../../crates/solana_kit_core/tests/codec_size/primitives.rs"]
mod primitives;

/// One encoder with two distinct input-type declarations.
#[derive(Clone, Copy, Debug, Default)]
pub struct Multi;

impl Encoder<u8> for Multi {
    fn encode_size(&self) -> Result<Size, Error> {
        Number::<u8>::LE.encode_size()
    }

    fn encoded_len(&self, value: &u8) -> Result<usize, Error> {
        Number::<u8>::LE.encoded_len(value)
    }

    fn write(&self, value: &u8, writer: &mut Writer<'_>) -> Result<(), Error> {
        Number::<u8>::LE.write(value, writer)
    }
}

impl Encoder<u16> for Multi {
    fn encode_size(&self) -> Result<Size, Error> {
        Number::<u16>::LE.encode_size()
    }

    fn encoded_len(&self, value: &u16) -> Result<usize, Error> {
        Number::<u16>::LE.encoded_len(value)
    }

    fn write(&self, value: &u16, writer: &mut Writer<'_>) -> Result<(), Error> {
        Number::<u16>::LE.write(value, writer)
    }
}

/// A preexisting custom implementation can retain the default unknown class.
struct Legacy;

impl Encoder<u8> for Legacy {
    fn encoded_len(&self, value: &u8) -> Result<usize, Error> {
        Number::<u8>::LE.encoded_len(value)
    }

    fn write(&self, value: &u8, writer: &mut Writer<'_>) -> Result<(), Error> {
        Number::<u8>::LE.write(value, writer)
    }
}

impl Decoder<u8> for Legacy {
    fn read(&self, reader: &mut Reader<'_>) -> Result<u8, Error> {
        Number::<u8>::LE.read(reader)
    }
}

/// No Clone, Copy, Debug, Send or Sync implementation is needed on value types.
struct OpaqueValue(PhantomData<alloc::rc::Rc<()>>);

#[derive(Clone, Copy, Debug)]
struct Empty;

impl Encoder<OpaqueValue> for Empty {
    fn encode_size(&self) -> Result<Size, Error> {
        Ok(Size::Fixed(0))
    }

    fn encoded_len(&self, _: &OpaqueValue) -> Result<usize, Error> {
        Ok(0)
    }

    fn write(&self, _: &OpaqueValue, _: &mut Writer<'_>) -> Result<(), Error> {
        Ok(())
    }
}

impl Decoder<OpaqueValue> for Empty {
    fn decode_size(&self) -> Result<Size, Error> {
        Ok(Size::Fixed(0))
    }

    fn read(&self, _: &mut Reader<'_>) -> Result<OpaqueValue, Error> {
        Ok(OpaqueValue(PhantomData))
    }
}

fn part_bounds<T: Copy + Clone + Debug + Send + Sync>(_: &T) {}

/// Runs the same maintained behavior bodies through an ordinary dependency.
pub fn run() {
    algebra::kinds_and_identities();
    algebra::overflow();
    primitives::declarations();
    primitives::compact_boundaries();
    pairing::constructor_matrix();
    pairing::local_callbacks();
    pairing::error_facts();
    overrides::complete_operations();
    let selected = Paired::<_, _, u8, u8>::try_new(Multi, Number::<u8>::LE).unwrap();
    assert_eq!(selected.encode(&7, 1), Ok(alloc::vec![7]));
    assert_eq!(Multi.encode(&513u16, 2), Ok(alloc::vec![1, 2]));
    assert_eq!(Legacy.encode_size(), Ok(Size::Unknown));
    assert_eq!(Legacy.decode_size(), Ok(Size::Unknown));
    assert_eq!(Legacy.encode(&7, 1), Ok(alloc::vec![7]));
    assert_eq!(Legacy.decode(&[7], Limits::new(1, 0)), Ok(7));
    assert!(matches!(
        Paired::try_new(Legacy, Number::<u8>::LE),
        Err(PairError::UnknownSize)
    ));
    let empty = Paired::<_, _, OpaqueValue, OpaqueValue>::try_new(Empty, Empty).unwrap();
    part_bounds(&empty);
    assert_eq!(
        empty.encode(&OpaqueValue(PhantomData), 0),
        Ok(alloc::vec![])
    );
}

#[cfg(test)]
mod tests {
    #[test]
    fn ordinary_dependency_contracts() {
        super::run();
    }
}
