extern crate alloc;

#[path = "ordered_collections/domain.rs"]
mod domain;
#[path = "ordered_collections/framing.rs"]
mod framing;
#[path = "ordered_collections/limits.rs"]
mod limits;
#[path = "ordered_collections/operations.rs"]
mod operations;

#[test]
fn map_replacements_keep_original_keys_and_positions() {
    domain::map_replacements();
}

#[test]
fn sets_keep_original_values_and_insertion_order() {
    domain::set_membership();
}

#[test]
fn collection_counts_reuse_existing_framing() {
    framing::count_strategies();
}

#[test]
fn markers_and_optional_counts_preserve_boundary_policy() {
    framing::termination_strategies();
}

#[test]
fn duplicate_wire_entries_keep_consumption_and_claims() {
    limits::duplicate_claims();
}

#[test]
fn malformed_and_nested_collections_share_limits() {
    limits::malformed_and_nested();
}

#[test]
fn borrowed_nonclone_values_and_metadata_keep_selected_operations() {
    operations::borrowed_operations();
}

#[test]
fn every_small_wire_sequence_normalizes_in_first_seen_order() {
    limits::small_sequences();
}
