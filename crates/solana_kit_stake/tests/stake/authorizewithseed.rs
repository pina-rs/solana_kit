use super::*;
const TAG: u8 = 8;
const WIRE_SIZE: usize = 80;
const TAIL: AccountMeta = AccountMeta {
    address: Address::from_bytes([99; 32]),
    role: AccountRole::WritableSigner,
};

fn accounts(mask: usize) -> AuthorizeWithSeedAccounts {
    AuthorizeWithSeedAccounts {
        stake: ACCOUNT_1,
        base: ACCOUNT_2,
        clock_sysvar: ACCOUNT_3,
        lockup_authority: optional(mask, 0, ACCOUNT_4),
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
        Some(AccountMeta {
            address: ACCOUNT_3,
            role: AccountRole::Readonly,
        }),
        optional(mask, 0, ACCOUNT_4).map(|_| AccountMeta {
            address: ACCOUNT_4,
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
        let value = AuthorizeWithSeedCodec.decode(&bytes, LIMITS).unwrap();
        let accounts = accounts(mask);

        let instruction = check(
            Expected {
                accounts: expected(mask),
                bytes,
                value: value.clone(),
            },
            |limits| accounts.instruction(OVERRIDE, &value, core::slice::from_ref(&TAIL), limits),
            AuthorizeWithSeed::parse_instruction,
        );

        let parsed =
            AuthorizeWithSeed::parse_with_metadata(&instruction, OVERRIDE, LIMITS).unwrap();
        assert_eq!(parsed.data, value);
        assert!(core::ptr::eq(parsed.instruction, &instruction));
    }
}
