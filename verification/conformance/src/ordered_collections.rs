//! New ordered collection bodies are shared with the ordinary public-core tests.

#[path = "../../../crates/solana_kit_core/tests/ordered_collections/domain.rs"]
mod domain;
#[path = "../../../crates/solana_kit_core/tests/ordered_collections/framing.rs"]
mod framing;
#[path = "../../../crates/solana_kit_core/tests/ordered_collections/limits.rs"]
mod limits;
#[path = "../../../crates/solana_kit_core/tests/ordered_collections/operations.rs"]
mod operations;

pub(crate) fn cases() -> alloc::vec::Vec<crate::CaseResult> {
    let bodies: [(&str, fn()); 8] = [
        ("ordered-map-key-retention", domain::map_replacements),
        ("ordered-set-value-retention", domain::set_membership),
        ("ordered-collection-counts", framing::count_strategies),
        (
            "ordered-collection-termination",
            framing::termination_strategies,
        ),
        (
            "ordered-collection-duplicate-claims",
            limits::duplicate_claims,
        ),
        (
            "ordered-collection-malformed-nested",
            limits::malformed_and_nested,
        ),
        (
            "ordered-collection-borrowed-operations",
            operations::borrowed_operations,
        ),
        (
            "ordered-collection-small-wire-orders",
            limits::small_sequences,
        ),
    ];
    bodies
        .into_iter()
        .map(|(name, body)| {
            body();
            crate::case(name, Ok(()))
        })
        .collect()
}
