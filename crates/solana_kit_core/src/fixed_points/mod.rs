//! Exact bounded decimal and binary fixed points for off-chain clients.
//!
//! Storage width is independent of scale. Widths are 1..=4096 bits; scales are
//! 0..=4096; decimal input is at most 8192 bytes. These explicit resource bounds
//! replace the reference's unbounded allocations. All constructors validate.

mod codec;
mod shape;
mod text;
pub use codec::FixedPointCodec;
pub use text::{Base10, FormatOptions};

mod value;
pub use value::FixedPoint;

pub use num_bigint::BigInt;
pub use shape::{Error, MAX_INPUT_BYTES, MAX_SCALE, MAX_TOTAL_BITS, Rounding, Shape, Signedness};

/// Validated decimal storage shape.
pub type DecimalShape = Shape<10>;
/// Validated binary storage shape.
pub type BinaryShape = Shape<2>;

/// Exact decimal fixed-point value.
pub type DecimalFixedPoint = FixedPoint<10>;
/// Exact binary fixed-point value.
pub type BinaryFixedPoint = FixedPoint<2>;
