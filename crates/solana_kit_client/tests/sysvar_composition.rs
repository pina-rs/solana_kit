//! Ordinary public client account-fetch/sysvar-codec composition callers.
#[path = "sysvar_composition/cases.rs"]
mod cases;

#[test]
fn clock_composition() {
    cases::clock_composition();
}
#[test]
fn history_composition() {
    cases::history_composition();
}
#[test]
fn missing_and_owner() {
    cases::missing_and_owner();
}
#[test]
fn decoding_failures() {
    cases::decoding_failures();
}
#[test]
fn rpc_failures() {
    cases::rpc_failures();
}
