//! Ordinary public instruction/compiler integration callers.
#[path = "lookup_origin/cases.rs"]
mod cases;

#[test]
fn explicit_origin_wire() {
    cases::explicit_origin_wire();
}
#[test]
fn invalid_origins() {
    cases::invalid_origins();
}
#[test]
fn static_requirements() {
    cases::static_requirements();
}
#[test]
fn lookup_bounds() {
    cases::lookup_bounds();
}
#[test]
fn ordinary_source_compatibility() {
    cases::ordinary_source_compatibility();
}
