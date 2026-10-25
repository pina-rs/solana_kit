use super::*;
const TAG: u8 = 6;
const WIRE_SIZE: usize = 7;
const TAIL: AccountMeta = AccountMeta {
    address: Address::from_bytes([99; 32]),
    role: AccountRole::WritableSigner,
};

fn accounts() -> SetLockupAccounts {
    SetLockupAccounts {
        stake: ACCOUNT_1,
        authority: ACCOUNT_2,
    }
}

fn expected() -> Vec<AccountMeta> {
    [
        Some(AccountMeta {
            address: ACCOUNT_1,
            role: AccountRole::Writable,
        }),
        Some(AccountMeta {
            address: ACCOUNT_2,
            role: AccountRole::ReadonlySigner,
        }),
        Some(TAIL),
    ]
    .into_iter()
    .flatten()
    .collect()
}

pub fn run() {
    let mut bytes = vec![0; WIRE_SIZE];
    bytes[0] = TAG;
    let value = SetLockupCodec.decode(&bytes, LIMITS).unwrap();
    let accounts = accounts();

    let instruction = check(
        Expected {
            accounts: expected(),
            bytes,
            value: value.clone(),
        },
        |limits| accounts.instruction(OVERRIDE, &value, core::slice::from_ref(&TAIL), limits),
        SetLockup::parse_instruction,
    );

    let parsed = SetLockup::parse_with_metadata(&instruction, OVERRIDE, LIMITS).unwrap();
    assert_eq!(parsed.data, value);
    assert!(core::ptr::eq(parsed.instruction, &instruction));
}
