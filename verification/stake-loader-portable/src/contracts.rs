//! Shared observable invocation assertions for independently pinned matrices.
use solana_kit_core::{
    AccountMeta, Address, Instruction,
    codecs::{Error, Limits},
};
use std::fmt::Debug;

pub const LIMITS: Limits = Limits::new(4096, 4096);
pub const OVERRIDE: Address = Address::from_bytes([90; 32]);
pub const OTHER: Address = Address::from_bytes([91; 32]);

pub fn optional(mask: usize, bit: usize, value: Address) -> Option<Address> {
    (mask & (1 << bit) != 0).then_some(value)
}

pub struct Expected<T> {
    pub accounts: Vec<AccountMeta>,
    pub bytes: Vec<u8>,
    pub value: T,
}

pub fn check<T: Debug + Eq>(
    expected: Expected<T>,
    build: impl Fn(Limits) -> Result<Instruction, Error>,
    parse: fn(&Instruction, Address, Limits) -> Result<T, Error>,
) -> Instruction {
    let instruction = build(LIMITS).unwrap();
    assert_eq!(instruction.accounts, expected.accounts);
    assert_eq!(instruction.data, expected.bytes);
    assert_eq!(
        parse(&instruction, OVERRIDE, LIMITS).unwrap(),
        expected.value
    );
    assert_eq!(parse(&instruction, OTHER, LIMITS), Err(Error::InvalidValue));

    let no_accounts = Limits::new(LIMITS.max_bytes, 0);
    assert_eq!(
        parse(&instruction, OVERRIDE, no_accounts),
        Err(Error::LimitExceeded)
    );
    assert_eq!(
        build(Limits::new(0, LIMITS.max_items)),
        Err(Error::LimitExceeded)
    );
    instruction
}
