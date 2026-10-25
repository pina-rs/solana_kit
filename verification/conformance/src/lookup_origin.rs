//! Source-proposed lookup compilation bodies through the real public compiler.
#[path = "../../../crates/solana_kit_core/tests/lookup_origin/cases.rs"]
mod public;

pub(crate) fn cases() -> alloc::vec::Vec<crate::CaseResult> {
    let bodies: [(&str, fn()); 5] = [
        (
            "lookup-origin-literal-compiled-wire",
            public::explicit_origin_wire,
        ),
        (
            "lookup-origin-invalid-snapshot-conflict",
            public::invalid_origins,
        ),
        (
            "lookup-origin-static-requirements",
            public::static_requirements,
        ),
        ("lookup-origin-u8-table-bounds", public::lookup_bounds),
        (
            "lookup-origin-ordinary-source-compatibility",
            public::ordinary_source_compatibility,
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
