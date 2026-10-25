use super::*;
const TAG: u8 = 12;
const WIRE_SIZE: usize = 6;
const TAIL: AccountMeta = AccountMeta {
    address: Address::from_bytes([99; 32]),
    role: AccountRole::WritableSigner,
};

fn accounts(mask: usize) -> SetLockupCheckedAccounts {
    SetLockupCheckedAccounts {
        stake: ACCOUNT_1,
        authority: ACCOUNT_2,
        new_authority: optional(mask, 0, ACCOUNT_3),
    }
}

fn expected(mask: usize) -> Vec<AccountMeta> {
    [
        Some(AccountMeta {
            address: ACCOUNT_1,
            role: AccountRole::Writable,
        }),
        Some(AccountMeta {
            address: ACCOUNT_2,
            role: AccountRole::ReadonlySigner,
        }),
        optional(mask, 0, ACCOUNT_3).map(|_| AccountMeta {
            address: ACCOUNT_3,
            role: AccountRole::ReadonlySigner,
        }),
        Some(TAIL),
    ]
    .into_iter()
    .flatten()
    .collect()
}

pub fn run() {
    for mask in 0..2 {
        let mut bytes = vec![0; WIRE_SIZE];
        bytes[0] = TAG;
        let value = SetLockupCheckedCodec.decode(&bytes, LIMITS).unwrap();
        let accounts = accounts(mask);

        let instruction = check(
            Expected {
                accounts: expected(mask),
                bytes,
                value: value.clone(),
            },
            |limits| accounts.instruction(OVERRIDE, &value, core::slice::from_ref(&TAIL), limits),
            SetLockupChecked::parse_instruction,
        );

        let parsed = SetLockupChecked::parse_with_metadata(&instruction, OVERRIDE, LIMITS).unwrap();
        assert_eq!(parsed.data, value);
        assert!(core::ptr::eq(parsed.instruction, &instruction));
    }
}
