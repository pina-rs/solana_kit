extern crate alloc;
include!("generated.rs");
const MAX_BYTES: usize = 4096;
const MAX_ITEMS: usize = 128;

fn main() {
    let limits = Limits::new(MAX_BYTES, MAX_ITEMS);
    let address = Address::from_bytes([7; 32]);
    let roles = [
        AccountMeta {
            address,
            role: AccountRole::Writable,
        },
        AccountMeta {
            address,
            role: AccountRole::ReadonlySigner,
        },
        AccountMeta {
            address,
            role: AccountRole::Readonly,
        },
    ];
    let value = ReplaceLeaf {
        root: vec![1; 32],
        previous_leaf: vec![2; 32],
        new_leaf: vec![3; 32],
        index: 0x12345678,
    };
    verify_wire(&value, &roles, limits);
    verify_roles(&value, roles, limits);
    assert_eq!(SplAccountCompressionError::from_code(6000).code(), 6000);
    assert_eq!(
        SplAccountCompressionError::from_code(u32::MAX),
        SplAccountCompressionError::Unknown(u32::MAX)
    );
}

fn verify_wire(value: &ReplaceLeaf, roles: &[AccountMeta], limits: Limits) {
    let instruction = value.instruction(roles, limits).unwrap();
    let mut expected = vec![204, 165, 76, 100, 73, 147, 0, 128];
    expected.extend([1; 32]);
    expected.extend([2; 32]);

    expected.extend([3; 32]);
    expected.extend([0x78, 0x56, 0x34, 0x12]);
    assert_eq!(instruction.data, expected);
    assert!(matches!(
        parse_instruction(&instruction, limits).unwrap(),
        SplAccountCompressionInstructionData::ReplaceLeaf(parsed) if &parsed == value
    ));

    for length in 0..expected.len() {
        assert!(
            ReplaceLeafCodec
                .decode(&expected[..length], limits)
                .is_err()
        );
    }

    let mut trailing = expected.clone();
    trailing.push(0);
    assert!(ReplaceLeafCodec.decode(&trailing, limits).is_err());
}

fn verify_roles(value: &ReplaceLeaf, roles: [AccountMeta; 3], limits: Limits) {
    let address = roles[0].address;
    let mut proof = roles.to_vec();
    proof.push(AccountMeta {
        address,
        role: AccountRole::Readonly,
    });
    assert!(value.instruction(&proof, limits).is_ok());
    proof[3].role = AccountRole::Writable;
    assert!(value.instruction(&proof, limits).is_err());

    let mut invalid = roles;
    invalid[1].role = AccountRole::Readonly;
    assert!(value.instruction(&invalid, limits).is_err());
    assert!(
        value
            .instruction(&roles, Limits::new(MAX_BYTES, 2))
            .is_err()
    );
}
