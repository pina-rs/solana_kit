use alloc::{string::ToString, vec::Vec};
use solana_kit_anchor::{Idl, Limits, Value, Values};

pub const LIMITS: Limits = Limits::new(1_000_000, 100_000, 64);
pub const ADDRESS: &str = "Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS";

pub fn mini() -> Idl {
    Idl::parse(
        include_str!("../../../../crates/solana_kit_anchor/tests/fixtures/mini.json"),
        LIMITS,
    )
    .unwrap()
}

pub fn fields<const N: usize>(entries: [(&str, Value); N]) -> Values {
    entries
        .into_iter()
        .map(|(name, value)| (name.to_string(), value))
        .collect()
}

pub fn bytes(text: &str) -> Vec<u8> {
    const HEX_RADIX: u32 = 16;
    text.trim()
        .as_bytes()
        .as_chunks::<2>()
        .0
        .iter()
        .map(|pair| u8::from_str_radix(core::str::from_utf8(pair).unwrap(), HEX_RADIX).unwrap())
        .collect()
}
