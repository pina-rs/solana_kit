//! New metadata bodies append after the complete original 307-case prefix.

#[path = "../../../crates/solana_kit_core/tests/codec_size/algebra.rs"]
mod algebra;
#[path = "../../../crates/solana_kit_core/tests/codec_size/overrides.rs"]
mod overrides;
#[path = "../../../crates/solana_kit_core/tests/codec_size/pairing.rs"]
mod pairing;
#[path = "../../../crates/solana_kit_core/tests/codec_size/primitives.rs"]
mod primitives;

pub(crate) fn cases() -> alloc::vec::Vec<crate::CaseResult> {
    let bodies: [(&str, fn()); 8] = [
        ("codec-size-algebra", algebra::kinds_and_identities),
        ("codec-size-overflow", algebra::overflow),
        ("codec-size-primitives", primitives::declarations),
        (
            "codec-size-compact-boundaries",
            primitives::compact_boundaries,
        ),
        ("codec-size-pair-matrix", pairing::constructor_matrix),
        ("codec-size-local-callbacks", pairing::local_callbacks),
        ("codec-size-error-facts", pairing::error_facts),
        ("codec-size-owned-overrides", overrides::complete_operations),
    ];
    bodies
        .into_iter()
        .map(|(name, body)| {
            body();
            crate::case(name, Ok(()))
        })
        .collect()
}
