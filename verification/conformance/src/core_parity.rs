//! Source-proposed contracts share their exact ordinary public-core assertions.

#[path = "../../../crates/solana_kit_core/tests/core_parity/bigints.rs"]
mod bigints;
#[path = "../../../crates/solana_kit_core/tests/core_parity/constants.rs"]
mod constants;
#[path = "../../../crates/solana_kit_core/tests/core_parity/dynamic.rs"]
mod dynamic;
#[path = "../../../crates/solana_kit_core/tests/core_parity/fixed_points.rs"]
mod fixed_points;
#[path = "../../../crates/solana_kit_core/tests/core_parity/keccak.rs"]
mod keccak;
#[path = "../../../crates/solana_kit_core/tests/core_parity/selection.rs"]
mod selection;
#[path = "../../../crates/solana_kit_core/tests/core_parity/utf16.rs"]
mod utf16;
#[path = "../../../crates/solana_kit_core/tests/core_parity/values.rs"]
mod values;

pub(crate) fn cases() -> alloc::vec::Vec<crate::CaseResult> {
    let bodies: [(&str, fn()); 15] = [
        ("predicate-untagged-union", selection::untagged_selection),
        (
            "contextual-decode-transform",
            selection::contextual_decoding,
        ),
        ("dynamic-tuple-schema", dynamic::dynamic_tuples),
        ("dynamic-record-schema", dynamic::dynamic_records),
        ("utf16-radix-surrogate-positions", utf16::surrogate_symbols),
        ("utf16-radix-shared-limits", utf16::unicode_limits),
        (
            "utf16-surrogate-bit-reslicing",
            utf16::surrogate_bit_reslicing,
        ),
        ("lookup-account-metadata", values::lookup_metadata),
        (
            "public-address-collation-display",
            values::text_collation_and_display,
        ),
        ("float32-extreme-conversion", values::floating_conversion),
        (
            "canonical-address-constant-literals",
            constants::constant_literals,
        ),
        (
            "arbitrary-integer-width-literals",
            bigints::arbitrary_width_literals,
        ),
        (
            "arbitrary-integer-width-rejections",
            bigints::arbitrary_width_rejections,
        ),
        ("keccak-original-padding-literals", keccak::keccak_literals),
        (
            "fixed-point-extreme-presentation",
            fixed_points::extreme_fixed_point_conversion,
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
