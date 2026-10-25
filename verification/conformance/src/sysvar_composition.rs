//! Source-proposed public Client fetch/typed sysvar decode composition callers.
#[path = "../../../crates/solana_kit_client/tests/sysvar_composition/cases.rs"]
mod public;

pub(crate) fn cases() -> alloc::vec::Vec<crate::CaseResult> {
    let bodies: [(&str, fn()); 5] = [
        (
            "sysvar-public-clock-fetch-composition",
            public::clock_composition,
        ),
        (
            "sysvar-public-history-fetch-composition",
            public::history_composition,
        ),
        (
            "sysvar-public-fetch-absence-owner",
            public::missing_and_owner,
        ),
        (
            "sysvar-public-fetch-decoder-failures",
            public::decoding_failures,
        ),
        (
            "sysvar-public-fetch-rpc-causes-limits",
            public::rpc_failures,
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
