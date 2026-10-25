use super::*;
const TAG: u8 = 1;
const WIRE_SIZE: usize = 40;
const TAIL: AccountMeta = AccountMeta {
    address: Address::from_bytes([99; 32]),
    role: AccountRole::WritableSigner,
};

fn accounts(mask: usize) -> AuthorizeAccounts {
    AuthorizeAccounts {
        stake: ACCOUNT_1,
        clock_sysvar: ACCOUNT_2,
        authority: ACCOUNT_3,
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
            role: AccountRole::Readonly,
        }),
        Some(AccountMeta {
            address: ACCOUNT_3,
            role: AccountRole::ReadonlySigner,
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
        let value = AuthorizeCodec.decode(&bytes, LIMITS).unwrap();
        let accounts = accounts(mask);

        let instruction = check(
            Expected {
                accounts: expected(mask),
                bytes,
                value: value.clone(),
            },
            |limits| accounts.instruction(OVERRIDE, &value, core::slice::from_ref(&TAIL), limits),
            Authorize::parse_instruction,
        );

        let parsed = Authorize::parse_with_metadata(&instruction, OVERRIDE, LIMITS).unwrap();
        assert_eq!(parsed.data, value);
        assert!(core::ptr::eq(parsed.instruction, &instruction));
    }
}
