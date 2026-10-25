use super::*;
const TAG: u8 = 13;
const WIRE_SIZE: usize = 4;
const TAIL: AccountMeta = AccountMeta {
    address: Address::from_bytes([99; 32]),
    role: AccountRole::WritableSigner,
};

fn accounts() -> GetMinimumDelegationAccounts {
    GetMinimumDelegationAccounts {}
}

fn expected() -> Vec<AccountMeta> {
    [Some(TAIL)].into_iter().flatten().collect()
}

pub fn run() {
    let mut bytes = vec![0; WIRE_SIZE];
    bytes[0] = TAG;
    let value = GetMinimumDelegationCodec.decode(&bytes, LIMITS).unwrap();
    let accounts = accounts();

    let instruction = check(
        Expected {
            accounts: expected(),
            bytes,
            value: value.clone(),
        },
        |limits| accounts.instruction(OVERRIDE, &value, core::slice::from_ref(&TAIL), limits),
        GetMinimumDelegation::parse_instruction,
    );

    let parsed = GetMinimumDelegation::parse_with_metadata(&instruction, OVERRIDE, LIMITS).unwrap();
    assert_eq!(parsed.data, value);
    assert!(core::ptr::eq(parsed.instruction, &instruction));
}
