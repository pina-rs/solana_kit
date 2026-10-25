use super::*;
const TAG: u8 = 3;
const WIRE_SIZE: usize = 4;
const TAIL: AccountMeta = AccountMeta {
    address: Address::from_bytes([99; 32]),
    role: AccountRole::WritableSigner,
};

fn accounts() -> UpgradeAccounts {
    UpgradeAccounts {
        program_data_account: ACCOUNT_1,
        program_account: ACCOUNT_2,
        buffer_account: ACCOUNT_3,
        spill_account: ACCOUNT_4,
        rent_sysvar: ACCOUNT_5,
        clock_sysvar: ACCOUNT_6,
        authority: ACCOUNT_7,
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
            role: AccountRole::Writable,
        }),
        Some(AccountMeta {
            address: ACCOUNT_3,
            role: AccountRole::Writable,
        }),
        Some(AccountMeta {
            address: ACCOUNT_4,
            role: AccountRole::Writable,
        }),
        Some(AccountMeta {
            address: ACCOUNT_5,
            role: AccountRole::Readonly,
        }),
        Some(AccountMeta {
            address: ACCOUNT_6,
            role: AccountRole::Readonly,
        }),
        Some(AccountMeta {
            address: ACCOUNT_7,
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
    let value = UpgradeCodec.decode(&bytes, LIMITS).unwrap();
    let accounts = accounts();

    let instruction = check(
        Expected {
            accounts: expected(),
            bytes,
            value: value.clone(),
        },
        |limits| accounts.instruction(OVERRIDE, &value, core::slice::from_ref(&TAIL), limits),
        Upgrade::parse_instruction,
    );

    let parsed = Upgrade::parse_with_metadata(&instruction, OVERRIDE, LIMITS).unwrap();
    assert_eq!(parsed.data, value);
    assert!(core::ptr::eq(parsed.instruction, &instruction));
}
