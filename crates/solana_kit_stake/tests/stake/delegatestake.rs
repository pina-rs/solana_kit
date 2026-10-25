use super::*;
const TAG: u8 = 2;
const WIRE_SIZE: usize = 4;
const TAIL: AccountMeta = AccountMeta {
    address: Address::from_bytes([99; 32]),
    role: AccountRole::WritableSigner,
};

fn accounts() -> DelegateStakeAccounts {
    DelegateStakeAccounts {
        stake: ACCOUNT_1,
        vote: ACCOUNT_2,
        clock_sysvar: ACCOUNT_3,
        stake_history: ACCOUNT_4,
        unused: ACCOUNT_5,
        stake_authority: ACCOUNT_6,
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
            role: AccountRole::Readonly,
        }),
        Some(AccountMeta {
            address: ACCOUNT_3,
            role: AccountRole::Readonly,
        }),
        Some(AccountMeta {
            address: ACCOUNT_4,
            role: AccountRole::Readonly,
        }),
        Some(AccountMeta {
            address: ACCOUNT_5,
            role: AccountRole::Readonly,
        }),
        Some(AccountMeta {
            address: ACCOUNT_6,
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
    let value = DelegateStakeCodec.decode(&bytes, LIMITS).unwrap();
    let accounts = accounts();

    let instruction = check(
        Expected {
            accounts: expected(),
            bytes,
            value: value.clone(),
        },
        |limits| accounts.instruction(OVERRIDE, &value, core::slice::from_ref(&TAIL), limits),
        DelegateStake::parse_instruction,
    );

    let parsed = DelegateStake::parse_with_metadata(&instruction, OVERRIDE, LIMITS).unwrap();
    assert_eq!(parsed.data, value);
    assert!(core::ptr::eq(parsed.instruction, &instruction));
}
