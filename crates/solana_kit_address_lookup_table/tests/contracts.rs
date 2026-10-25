use solana_kit_address_lookup_table::*;
use solana_kit_core::{
    AccountRole, Address,
    codecs::{Decoder, Encoder, Error, Limits},
};
const LIMITS: Limits = Limits::new(10_000, 1000);
const ZERO: Address = Address::from_bytes([0; 32]);

#[cfg_attr(test, test)]
pub fn literal_all_five_variants() {
    assert_eq!(
        PROGRAM.to_string(),
        "AddressLookupTab1e1111111111111111111111111"
    );
    let funding = Funding {
        payer: ZERO,
        system_program: ZERO,
    };
    let operations = [
        Operation::Create {
            table: ZERO,
            authority: ZERO,
            funding,
            recent_slot: 0x0807060504030201,
            bump: 254,
        },
        Operation::Freeze {
            table: ZERO,
            authority: ZERO,
        },
        Operation::Extend {
            table: ZERO,
            authority: ZERO,
            funding: Some(funding),
            addresses: vec![ZERO],
        },
        Operation::Deactivate {
            table: ZERO,
            authority: ZERO,
        },
        Operation::Close {
            table: ZERO,
            authority: ZERO,
            recipient: ZERO,
        },
    ];
    let mut extend = vec![2, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0];
    extend.extend([0; 32]);
    let literals = [
        vec![0, 0, 0, 0, 1, 2, 3, 4, 5, 6, 7, 8, 254],
        vec![1, 0, 0, 0],
        extend,
        vec![3, 0, 0, 0],
        vec![4, 0, 0, 0],
    ];

    for (operation, literal) in operations.iter().zip(literals) {
        assert_literal(operation, literal);
    }
}

fn assert_literal(operation: &Operation, literal: Vec<u8>) {
    let instruction = operation.build(PROGRAM, LIMITS).unwrap();
    assert_eq!(instruction.data, literal);
    let parsed = parse_instruction(&instruction, PROGRAM, LIMITS).unwrap();
    assert_eq!(
        InstructionDataCodec
            .encode(&parsed, LIMITS.max_bytes)
            .unwrap(),
        literal
    );
    assert_eq!(instruction.accounts[0].role, AccountRole::Writable);
    assert!(parse_instruction(&instruction, ZERO, LIMITS).is_err());

    for length in 0..literal.len() {
        assert!(
            InstructionDataCodec
                .decode(&literal[..length], LIMITS)
                .is_err()
        );
    }

    let mut trailing = literal.clone();
    trailing.push(0);
    assert!(InstructionDataCodec.decode(&trailing, LIMITS).is_err());
}

#[cfg_attr(test, test)]
pub fn optional_funding_and_privileges() {
    let operation = Operation::Extend {
        table: ZERO,
        authority: ZERO,
        funding: None,
        addresses: vec![],
    };
    let mut instruction = operation.build(ZERO, LIMITS).unwrap();
    assert_eq!(instruction.accounts.len(), 2);
    assert!(parse_instruction(&instruction, ZERO, LIMITS).is_ok());
    instruction.accounts[1].role = AccountRole::WritableSigner;
    assert!(parse_instruction(&instruction, ZERO, LIMITS).is_ok());
    instruction.accounts[1].role = AccountRole::Readonly;
    assert!(parse_instruction(&instruction, ZERO, LIMITS).is_err());

    assert!(InstructionDataCodec.decode(&[5, 0, 0, 0], LIMITS).is_err());
    assert!(
        InstructionDataCodec
            .decode(
                &[2, 0, 0, 0, 255, 255, 255, 255, 255, 255, 255, 255],
                LIMITS
            )
            .is_err()
    );
}

fn table(authority: Option<Address>) -> AddressLookupTable {
    AddressLookupTable {
        deactivation_slot: u64::MAX,
        last_extended_slot: 42,
        last_extended_slot_start_index: 1,
        authority,
        padding: 0,
        addresses: vec![ZERO],
    }
}

#[cfg_attr(test, test)]
pub fn literal_header_none_and_table_boundaries() {
    for authority in [None, Some(ZERO)] {
        let value = table(authority);
        let mut literal = vec![1, 0, 0, 0];
        literal.extend([255; 8]);
        literal.extend([42, 0, 0, 0, 0, 0, 0, 0, 1, u8::from(authority.is_some())]);
        literal.extend([0; 34]);
        literal.extend([0; 32]);
        assert_eq!(
            AddressLookupTableCodec
                .encode(&value, LIMITS.max_bytes)
                .unwrap(),
            literal
        );

        assert_eq!(
            AddressLookupTableCodec.decode(&literal, LIMITS).unwrap(),
            value
        );

        for length in 0..56 {
            assert!(
                AddressLookupTableCodec
                    .decode(&literal[..length], LIMITS)
                    .is_err()
            );
        }

        literal[21] = 2;
        assert!(AddressLookupTableCodec.decode(&literal, LIMITS).is_err());
    }

    let mut value = table(None);
    value.addresses = vec![ZERO; 256];
    let encoded = AddressLookupTableCodec
        .encode(&value, LIMITS.max_bytes)
        .unwrap();
    assert_eq!(
        AddressLookupTableCodec.decode(&encoded, LIMITS).unwrap(),
        value
    );
    assert_eq!(
        value.clone().into_lookup_table(PROGRAM).addresses.len(),
        256
    );
    value.addresses.push(ZERO);
    assert!(
        AddressLookupTableCodec
            .encode(&value, LIMITS.max_bytes)
            .is_err()
    );
    assert_eq!(
        AddressLookupTableCodec.decode(&encoded, Limits::new(LIMITS.max_bytes, 255)),
        Err(Error::LimitExceeded)
    );
}

#[cfg_attr(test, test)]
pub fn absent_authority_padding_is_rust_source_layout() {
    let mut value = table(None);
    value.padding = 0x1234;
    let mut encoded = AddressLookupTableCodec
        .encode(&value, LIMITS.max_bytes)
        .unwrap();
    assert_eq!(&encoded[22..24], &[0x34, 0x12]);
    encoded[24..56].fill(7);
    assert_eq!(
        AddressLookupTableCodec.decode(&encoded, LIMITS).unwrap(),
        value
    );
    encoded.push(0);
    assert!(AddressLookupTableCodec.decode(&encoded, LIMITS).is_err());
}

#[cfg(feature = "pda")]
#[cfg_attr(test, test)]
pub fn pda_uses_actual_authority_slot_seeds() {
    let derived = find_lookup_table(ZERO, 42, PROGRAM).unwrap();
    assert_eq!(
        derived.address().to_string(),
        "HcKrHbCzJLv9DFB3qXLiS5RrneibGRP9oeBeDRpcBKgA"
    );
    assert_eq!(derived.bump(), 255);
    let explicit = solana_kit_core::crypto::find_program_address(
        &[ZERO.as_bytes(), &[42, 0, 0, 0, 0, 0, 0, 0]],
        &PROGRAM,
    )
    .unwrap();
    assert_eq!(derived, explicit);
    assert_ne!(derived, find_lookup_table(ZERO, 43, PROGRAM).unwrap());
    assert_ne!(derived, find_lookup_table(ZERO, 42, ZERO).unwrap());
}
