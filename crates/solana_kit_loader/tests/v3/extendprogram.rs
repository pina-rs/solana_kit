use super::*;
const TAG: u8 = 6;
const WIRE_SIZE: usize = 8;
const TAIL: AccountMeta = AccountMeta {
    address: Address::from_bytes([99; 32]),
    role: AccountRole::WritableSigner,
};

fn accounts(mask: usize) -> ExtendProgramAccounts {
    ExtendProgramAccounts {
        program_data_account: ACCOUNT_1,
        program_account: ACCOUNT_2,
        system_program: optional(mask, 0, ACCOUNT_3),
        payer: optional(mask, 1, ACCOUNT_4),
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
            role: AccountRole::Writable,
        }),
        optional(mask, 0, ACCOUNT_3).map(|_| AccountMeta {
            address: ACCOUNT_3,
            role: AccountRole::Readonly,
        }),
        optional(mask, 1, ACCOUNT_4).map(|_| AccountMeta {
            address: ACCOUNT_4,
            role: AccountRole::WritableSigner,
        }),
        Some(TAIL),
    ]
    .into_iter()
    .flatten()
    .collect()
}

pub fn run() {
    for mask in 0..4 {
        let mut bytes = vec![0; WIRE_SIZE];
        bytes[0] = TAG;
        let value = ExtendProgramCodec.decode(&bytes, LIMITS).unwrap();
        let accounts = accounts(mask);

        let instruction = check(
            Expected {
                accounts: expected(mask),
                bytes,
                value: value.clone(),
            },
            |limits| accounts.instruction(OVERRIDE, &value, core::slice::from_ref(&TAIL), limits),
            ExtendProgram::parse_instruction,
        );

        let parsed = ExtendProgram::parse_with_metadata(&instruction, OVERRIDE, LIMITS).unwrap();
        assert_eq!(parsed.data, value);
        assert!(core::ptr::eq(parsed.instruction, &instruction));
    }
}
