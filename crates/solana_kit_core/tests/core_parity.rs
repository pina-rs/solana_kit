extern crate alloc;

#[cfg(feature = "bigint-codecs")]
#[path = "core_parity/bigints.rs"]
mod bigints;
#[path = "core_parity/constants.rs"]
mod constants;
#[path = "core_parity/dynamic.rs"]
mod dynamic;
#[cfg(feature = "fixed-points")]
#[path = "core_parity/fixed_points.rs"]
mod fixed_points;
#[cfg(feature = "keccak")]
#[path = "core_parity/keccak.rs"]
mod keccak;
#[path = "core_parity/selection.rs"]
mod selection;
#[path = "core_parity/utf16.rs"]
mod utf16;
#[path = "core_parity/values.rs"]
mod values;

#[test]
fn untagged_unions_select_before_reading() {
    selection::untagged_selection();
}

#[test]
fn transforms_retain_original_byte_context() {
    selection::contextual_decoding();
}

#[test]
fn runtime_tuples_keep_schema_order_and_shared_limits() {
    dynamic::dynamic_tuples();
}

#[test]
fn runtime_records_keep_names_and_schema_order() {
    dynamic::dynamic_records();
}

#[test]
fn utf16_radix_retains_surrogate_positions() {
    utf16::surrogate_symbols();
}

#[test]
fn utf16_radix_bounds_work_and_allocations() {
    utf16::unicode_limits();
}

#[test]
fn utf16_bit_reslicing_retains_padding_policy() {
    utf16::surrogate_bit_reslicing();
}

#[test]
fn lookup_metadata_cannot_request_signatures() {
    values::lookup_metadata();
}

#[test]
fn public_collation_and_display_match_reference() {
    values::text_collation_and_display();
}

#[test]
fn float32_mapping_matches_extreme_conversion() {
    values::floating_conversion();
}

#[test]
fn canonical_address_constants_match_literal_text() {
    constants::constant_literals();
}

#[cfg(feature = "bigint-codecs")]
#[test]
fn arbitrary_integer_widths_keep_exact_literals() {
    bigints::arbitrary_width_literals();
}

#[cfg(feature = "bigint-codecs")]
#[test]
fn arbitrary_integer_widths_reject_overflow_and_limits() {
    bigints::arbitrary_width_rejections();
}

#[cfg(feature = "keccak")]
#[test]
fn keccak_matches_original_padding_vectors() {
    keccak::keccak_literals();
}

#[cfg(feature = "fixed-points")]
#[test]
fn fixed_point_presentation_scales_before_float_conversion() {
    fixed_points::extreme_fixed_point_conversion();
}
