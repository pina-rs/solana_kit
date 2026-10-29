//! Pure source fee and buffer vectors execute unchanged on every maintained host.

use serde::Deserialize;
use solana_kit_helius::{
    Decimal,
    transactions::{
        ComputeUnitBuffer, ComputeUnitBufferError, PriorityFeeError, ResolvePriorityFeeInput,
        ResolvedPriorityFee, buffer_compute_units, resolve_priority_fee,
    },
};

const FIXTURES: &str = include_str!("../fixtures/priority-fee-resolution.json");
const SIMULATED_UNITS: u64 = 100_000;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Vectors {
    fees: Vec<FeeVector>,
    buffers: Vec<BufferVector>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct FeeVector {
    name: String,
    estimate: Decimal,
    units: u64,
    rate_cap: Option<Decimal>,
    lamports_cap: Option<Decimal>,
    rate: u64,
    lamports: u64,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct BufferVector {
    name: String,
    raw: u64,
    minimum: Option<u32>,
    buffer_percent: Option<f64>,
    units: u32,
}

/// Source fee vectors, exact large caps and binary buffer rounding form one portable group.
pub fn resolution() {
    let vectors: Vectors = serde_json::from_str(FIXTURES).expect("literal typed fee vectors");

    for vector in vectors.fees {
        let result = resolve_priority_fee(ResolvePriorityFeeInput {
            estimate: &vector.estimate,
            units: vector.units,
            rate_cap: vector.rate_cap.as_ref(),
            lamports_cap: vector.lamports_cap.as_ref(),
        });
        assert_eq!(
            result,
            Ok(ResolvedPriorityFee {
                rate: vector.rate,
                lamports: vector.lamports
            }),
            "{}",
            vector.name
        );
    }

    buffer_vectors(vectors.buffers);
    overflow_and_invalid_inputs();
    invalid_buffers();
}

fn buffer_vectors(vectors: Vec<BufferVector>) {
    for vector in vectors {
        let defaults = ComputeUnitBuffer::default();
        let result = buffer_compute_units(
            vector.raw,
            ComputeUnitBuffer {
                minimum: vector.minimum.unwrap_or(defaults.minimum),
                buffer_percent: vector.buffer_percent.unwrap_or(defaults.buffer_percent),
            },
        )
        .expect("source compute buffer");
        assert_eq!(result.get(), vector.units, "{}", vector.name);
    }

    let estimate: Decimal = "2000".parse().expect("exact estimate");
    let units =
        buffer_compute_units(SIMULATED_UNITS, ComputeUnitBuffer::default()).expect("binary buffer");
    assert_eq!(
        resolve_priority_fee(ResolvePriorityFeeInput {
            estimate: &estimate,
            units: u64::from(units.get()),
            rate_cap: None,
            lamports_cap: None,
        }),
        Ok(ResolvedPriorityFee {
            rate: 2000,
            lamports: 221
        })
    );
}

fn overflow_and_invalid_inputs() {
    for estimate in ["1e99999999999999999999", "18446744073709551616"] {
        let estimate = estimate.parse().expect("exact large estimate");
        assert_eq!(
            resolve_priority_fee(ResolvePriorityFeeInput {
                estimate: &estimate,
                units: 0,
                rate_cap: None,
                lamports_cap: None,
            }),
            Err(PriorityFeeError::RateOverflow)
        );
    }

    let estimate = "18446744073709551615".parse().expect("maximum rate");
    assert_eq!(
        resolve_priority_fee(ResolvePriorityFeeInput {
            estimate: &estimate,
            units: u64::MAX,
            rate_cap: None,
            lamports_cap: None,
        }),
        Err(PriorityFeeError::TotalOverflow)
    );

    for invalid in ["\"not-a-number\"", "NaN", "Infinity", "null", "{}"] {
        assert!(invalid.parse::<Decimal>().is_err());
    }
}

fn invalid_buffers() {
    for buffer_percent in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        assert_eq!(
            buffer_compute_units(
                0,
                ComputeUnitBuffer {
                    minimum: 0,
                    buffer_percent
                }
            ),
            Err(ComputeUnitBufferError::NonFinitePercent)
        );
    }

    for buffer_percent in [f64::MAX, -f64::MAX] {
        assert_eq!(
            buffer_compute_units(
                2,
                ComputeUnitBuffer {
                    minimum: 0,
                    buffer_percent
                }
            ),
            Err(ComputeUnitBufferError::NonFiniteResult)
        );
    }
}
