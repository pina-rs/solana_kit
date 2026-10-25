use solana_kit_address_lookup_table::{Funding, Operation, PROGRAM};
use solana_kit_core::{AccountMeta, AccountRole, Address, codecs::Limits};

#[cfg_attr(test, test)]
pub fn create_and_extend_preserve_distinct_positional_addresses() {
    let table = Address::from_bytes([1; 32]);
    let authority = Address::from_bytes([2; 32]);
    let payer = Address::from_bytes([3; 32]);
    let system = Address::from_bytes([4; 32]);
    let funding = Funding {
        payer,
        system_program: system,
    };
    let operation = Operation::Create {
        table,
        authority,
        funding,
        recent_slot: 0,
        bump: 0,
    };
    let instruction = operation.build(PROGRAM, Limits::new(100, 10)).unwrap();
    let expected = vec![
        AccountMeta {
            address: table,
            role: AccountRole::Writable,
        },
        AccountMeta {
            address: authority,
            role: AccountRole::Readonly,
        },
        AccountMeta {
            address: payer,
            role: AccountRole::WritableSigner,
        },
        AccountMeta {
            address: system,
            role: AccountRole::Readonly,
        },
    ];

    assert_eq!(instruction.accounts, expected);
    let extend = Operation::Extend {
        table,
        authority,
        funding: Some(funding),
        addresses: vec![],
    };
    let instruction = extend.build(PROGRAM, Limits::new(100, 10)).unwrap();
    let mut expected_extend = expected;
    expected_extend[1].role = AccountRole::ReadonlySigner;
    assert_eq!(instruction.accounts, expected_extend);
}
