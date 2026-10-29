//! Binary ceiling arithmetic preserves the pinned Dart buffer fixtures.

use core::{fmt, num::NonZeroU32};

/// Maximum requested compute units used by the pinned smart-transaction helper.
pub const MAX_SMART_TRANSACTION_COMPUTE_UNITS: u32 = 1_400_000;
const DEFAULT_MINIMUM: u32 = 1000;
const DEFAULT_BUFFER_PERCENT: f64 = 0.1;

/// Caller-selected minimum and binary floating-point compute-unit buffer.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ComputeUnitBuffer {
    /// Requested floor, applied before the runtime maximum; zero is allowed.
    pub minimum: u32,
    /// Added fraction; finite negative percentages retain the source floor behavior.
    pub buffer_percent: f64,
}

impl Default for ComputeUnitBuffer {
    fn default() -> Self {
        Self {
            minimum: DEFAULT_MINIMUM,
            buffer_percent: DEFAULT_BUFFER_PERCENT,
        }
    }
}

/// The buffer cannot be evaluated or represented as a positive runtime compute limit.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComputeUnitBufferError {
    /// The caller supplied NaN or infinity.
    NonFinitePercent,
    /// Finite inputs overflowed binary floating-point multiplication.
    NonFiniteResult,
    /// The clamped result cannot be represented as a positive runtime compute limit.
    OutOfRange,
}

impl fmt::Display for ComputeUnitBufferError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::NonFinitePercent => "compute buffer percentage must be finite",
            Self::NonFiniteResult => "compute buffer arithmetic must remain finite",
            Self::OutOfRange => "compute buffer result is outside the runtime range",
        })
    }
}

impl std::error::Error for ComputeUnitBufferError {}

/// Applies binary ceil(raw * (1 + percentage)), then the floor and runtime maximum.
///
/// The result is always positive, including for zero raw units and a zero minimum.
/// The default intentionally maps 100000 to 110001, matching the source's binary
/// arithmetic. Fee rates and totals use separate exact integer arithmetic.
pub fn buffer_compute_units(
    raw_units: u64,
    options: ComputeUnitBuffer,
) -> Result<NonZeroU32, ComputeUnitBufferError> {
    if !options.buffer_percent.is_finite() {
        return Err(ComputeUnitBufferError::NonFinitePercent);
    }

    let buffered = (raw_units as f64) * (1.0 + options.buffer_percent);

    if !buffered.is_finite() {
        return Err(ComputeUnitBufferError::NonFiniteResult);
    }

    let maximum = f64::from(MAX_SMART_TRANSACTION_COMPUTE_UNITS);
    let units = buffered
        .ceil()
        .max(f64::from(options.minimum))
        .clamp(1.0, maximum);

    if !(1.0..=maximum).contains(&units) {
        return Err(ComputeUnitBufferError::OutOfRange);
    }

    // Finite, integral and clamped to 1..=1_400_000 before narrowing.
    NonZeroU32::new(units as u32).ok_or(ComputeUnitBufferError::OutOfRange)
}
