extern crate alloc;

#[path = "codec_size/algebra.rs"]
mod algebra;
#[path = "codec_size/overrides.rs"]
mod overrides;
#[path = "codec_size/pairing.rs"]
mod pairing;
#[path = "codec_size/primitives.rs"]
mod primitives;

#[test]
fn declaration_algebra_preserves_kinds_and_identities() {
    algebra::kinds_and_identities();
}

#[test]
fn known_subtotal_overflow_is_order_independent() {
    algebra::overflow();
}

#[test]
fn primitives_declare_their_actual_formats() {
    primitives::declarations();
}

#[test]
fn compact_boundaries_preserve_bytes_and_offsets() {
    primitives::compact_boundaries();
}

#[test]
fn constructor_matrix_preserves_complete_optional_maxima() {
    pairing::constructor_matrix();
}

#[test]
fn local_mapping_does_not_run_callbacks_during_pairing() {
    pairing::local_callbacks();
}

#[test]
fn constructor_facts_retain_null_zero_and_causes() {
    pairing::error_facts();
}

#[test]
fn paired_preserves_every_selected_operation_override() {
    overrides::complete_operations();
}
