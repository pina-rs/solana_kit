//! The exact public Surfpool contract bodies run on each portability host.

extern crate std;

#[path = "../../../crates/solana_kit_surfpool/tests/support/mod.rs"]
mod support;

pub(crate) fn cases() -> alloc::vec::Vec<crate::CaseResult> {
    support::groups()
        .into_iter()
        .map(|(name, contract)| crate::case(name, contract()))
        .collect()
}
