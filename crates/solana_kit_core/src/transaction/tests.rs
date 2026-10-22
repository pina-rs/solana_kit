use super::*;

// This explicit legacy wire fixture follows the pinned Dart message codec layout.
// Header, payer, program, blockhash, then one no-account/no-data instruction.
fn legacy_fixture() -> Vec<u8> {
    let mut bytes = alloc::vec![1, 0, 1, 2];
    bytes.extend([1; 32]);
    bytes.extend([2; 32]);
    bytes.extend([3; 32]);
    bytes.extend([1, 1, 0, 0]);
    bytes
}

#[test]
fn legacy_compile_matches_independent_layout() {
    let instruction = Instruction {
        program_address: Address::from_bytes([2; 32]),
        accounts: Vec::new(),
        data: Vec::new(),
    };
    let message = Message::compile(
        Version::Legacy,
        Address::from_bytes([1; 32]),
        Lifetime::Blockhash {
            blockhash: Hash::from_bytes([3; 32]),
            last_valid_block_height: 42,
        },
        &[instruction],
        &[],
    )
    .unwrap();
    assert_eq!(message.bytes(), legacy_fixture());
    assert_eq!(
        Message::decode(message.bytes()).unwrap().bytes(),
        message.bytes()
    );
}

#[test]
fn external_wire_has_separate_validation() {
    let mut bytes = legacy_fixture();
    bytes[2] = 0; // Writable invoked programs are protocol-valid incoming wire.
    assert!(Message::decode(&bytes).is_ok());
    bytes.push(0);
    assert_eq!(Message::decode(&bytes), Err(Error::TrailingBytes));
}

#[test]
fn unsigned_packet_has_exact_slots() {
    let transaction = Transaction::new(Message::decode(&legacy_fixture()).unwrap());
    assert_eq!(transaction.encode(true), Err(Error::MissingSignature));
    let mut expected = alloc::vec![1];
    expected.extend([0; 64]);
    expected.extend(legacy_fixture());
    assert_eq!(transaction.encode(false).unwrap(), expected);
    assert_eq!(Transaction::decode(&expected).unwrap(), transaction);
}

fn compile_fixture(
    version: Version,
    accounts: Vec<crate::AccountMeta>,
    tables: &[LookupTable],
    data: Vec<u8>,
) -> Result<Message, Error> {
    Message::compile(
        version,
        Address::from_bytes([1; 32]),
        Lifetime::Blockhash {
            blockhash: Hash::from_bytes([3; 32]),
            last_valid_block_height: 42,
        },
        &[Instruction {
            program_address: Address::from_bytes([2; 32]),
            accounts,
            data,
        }],
        tables,
    )
}

#[test]
fn v0_lookup_order_and_duplicate_positions() {
    use crate::{AccountMeta, AccountRole};
    let writable = Address::from_bytes([4; 32]);
    let readonly = Address::from_bytes([5; 32]);
    let table = LookupTable {
        address: Address::from_bytes([6; 32]),
        addresses: alloc::vec![readonly, writable, writable],
    };
    let accounts = alloc::vec![
        AccountMeta {
            address: readonly,
            role: AccountRole::Readonly
        },
        AccountMeta {
            address: writable,
            role: AccountRole::Readonly
        },
        AccountMeta {
            address: writable,
            role: AccountRole::Writable
        },
    ];
    let message = compile_fixture(Version::V0, accounts, &[table], alloc::vec![9]).unwrap();
    let mut expected = alloc::vec![128, 1, 0, 1, 2];
    expected.extend([1; 32]);
    expected.extend([2; 32]);
    expected.extend([3; 32]);
    expected.extend([1, 1, 3, 3, 2, 2, 1, 9, 1]);
    expected.extend([6; 32]);
    expected.extend([1, 1, 1, 0]);
    assert_eq!(message.bytes(), expected);
    assert_eq!(
        Message::decode(&expected).unwrap().instructions()[0].accounts(),
        [3, 2, 2]
    );
}

#[test]
fn lookup_signers_and_invoked_programs_stay_static() {
    use crate::{AccountMeta, AccountRole};
    let signer = Address::from_bytes([7; 32]);
    let table = LookupTable {
        address: Address::from_bytes([6; 32]),
        addresses: alloc::vec![signer, Address::from_bytes([2; 32])],
    };
    let message = compile_fixture(
        Version::V0,
        alloc::vec![AccountMeta {
            address: signer,
            role: AccountRole::ReadonlySigner
        }],
        &[table],
        Vec::new(),
    )
    .unwrap();
    assert_eq!(message.signers(), [Address::from_bytes([1; 32]), signer]);
    assert!(message.lookups().is_empty());
    let writable_program = alloc::vec![AccountMeta {
        address: Address::from_bytes([2; 32]),
        role: AccountRole::Writable
    }];
    assert_eq!(
        compile_fixture(Version::V0, writable_program, &[], Vec::new()),
        Err(Error::WritableProgram)
    );
}

#[test]
fn lookup_last_index_and_invalid_snapshots() {
    use crate::{AccountMeta, AccountRole};
    let account = Address::from_bytes([9; 32]);
    let mut table = LookupTable {
        address: Address::from_bytes([6; 32]),
        addresses: alloc::vec![Address::from_bytes([8; 32]); 256],
    };
    table.addresses[255] = account;
    let accounts = alloc::vec![AccountMeta {
        address: account,
        role: AccountRole::Writable
    }];
    let message = compile_fixture(
        Version::V0,
        accounts.clone(),
        core::slice::from_ref(&table),
        Vec::new(),
    )
    .unwrap();
    assert_eq!(message.lookups()[0].writable_indices(), [255]);
    assert_eq!(
        compile_fixture(
            Version::V0,
            accounts.clone(),
            &[table.clone(), table.clone()],
            Vec::new()
        ),
        Err(Error::InvalidLookup)
    );
    table.addresses.push(account);
    assert_eq!(
        compile_fixture(Version::V0, accounts, &[table], Vec::new()),
        Err(Error::InvalidLookup)
    );
}

#[test]
fn strict_lengths_headers_indices_and_truncation() {
    let fixture = legacy_fixture();

    for end in 0..fixture.len() {
        assert!(Message::decode(&fixture[..end]).is_err());
    }

    for length in [&[130, 0][..], &[130, 128, 0][..], &[255, 255, 4][..]] {
        let mut bytes = fixture[..3].to_vec();
        bytes.extend(length);
        bytes.extend(&fixture[4..]);
        assert_eq!(Message::decode(&bytes), Err(Error::InvalidLength));
    }

    for prefix in [&[][..], &[128][..], &[128, 128][..]] {
        assert_eq!(Transaction::decode(prefix), Err(Error::Truncated));
    }

    for prefix in [&[128, 0][..], &[128, 128, 0][..], &[255, 255, 4][..]] {
        assert_eq!(Transaction::decode(prefix), Err(Error::InvalidLength));
    }

    let mut bytes = fixture.clone();
    bytes[0] = 129;
    assert_eq!(Message::decode(&bytes), Err(Error::UnsupportedVersion));
    bytes = fixture.clone();
    bytes[1] = 1;
    assert_eq!(Message::decode(&bytes), Err(Error::InvalidHeader));
    bytes = fixture;
    bytes[101] = 0;
    assert_eq!(Message::decode(&bytes), Err(Error::InvalidIndex));
}

#[test]
fn packet_limit_reserves_signature_space() {
    let message = compile_fixture(Version::Legacy, Vec::new(), &[], alloc::vec![0; 1062]).unwrap();
    let packet = Transaction::new(message).encode(false).unwrap();
    assert_eq!(packet.len(), PACKET_SIZE);
    assert!(Transaction::decode(&packet).is_ok());
    assert_eq!(
        compile_fixture(Version::Legacy, Vec::new(), &[], alloc::vec![0; 1063]),
        Err(Error::PacketTooLarge)
    );
    assert_eq!(
        Transaction::decode(&alloc::vec![0; PACKET_SIZE + 1]),
        Err(Error::PacketTooLarge)
    );
}

#[test]
fn signature_ownership_and_frozen_message_are_enforced() {
    let transaction = Transaction::new(Message::decode(&legacy_fixture()).unwrap());
    let signer = transaction.message().signers()[0];
    let signature = Signature::from_bytes([1; 64]);
    assert_eq!(
        transaction.with_signature(signer, b"changed", signature),
        Err(Error::ChangedMessage)
    );
    assert_eq!(
        transaction.with_signature(
            Address::from_bytes([99; 32]),
            transaction.message().bytes(),
            signature
        ),
        Err(Error::UnknownSigner)
    );
    let signed = transaction
        .with_signature(signer, transaction.message().bytes(), signature)
        .unwrap();
    assert_eq!(transaction.signatures(), [None]);
    assert_eq!(signed.signatures(), [Some(signature)]);
    let mut extra = signed.encode(true).unwrap();
    extra[0] = 2;
    extra.splice(65..65, [1; 64]);
    assert!(Transaction::decode(&extra).is_err());
}

#[cfg(feature = "crypto")]
#[test]
fn exact_message_signing_matches_openssl_oracle() {
    // RFC 8032 test 1 seed; signature generated separately by Node/OpenSSL.
    let seed = [
        157, 97, 177, 157, 239, 253, 90, 96, 186, 132, 74, 244, 146, 236, 44, 196, 68, 73, 197,
        105, 123, 50, 105, 25, 112, 59, 172, 3, 28, 174, 127, 96,
    ];
    let keypair = crate::crypto::Keypair::from_seed(&seed);
    let message = Message::compile(
        Version::Legacy,
        keypair.address(),
        Lifetime::Blockhash {
            blockhash: Hash::from_bytes([3; 32]),
            last_valid_block_height: 42,
        },
        &[Instruction {
            program_address: Address::from_bytes([2; 32]),
            accounts: Vec::new(),
            data: Vec::new(),
        }],
        &[],
    )
    .unwrap();
    let unsigned = Transaction::new(message);
    assert_eq!(unsigned.verify(), Err(Error::MissingSignature));
    let signed = unsigned.sign(&keypair).unwrap();
    let expected = Signature::from_bytes([
        179, 0, 170, 254, 234, 124, 169, 9, 108, 48, 231, 29, 49, 65, 187, 140, 250, 66, 209, 172,
        185, 218, 153, 109, 237, 47, 94, 221, 29, 25, 48, 15, 7, 49, 230, 121, 66, 20, 16, 106,
        134, 62, 167, 158, 192, 234, 125, 116, 47, 157, 240, 177, 177, 156, 47, 219, 251, 178, 18,
        117, 245, 113, 167, 10,
    ]);
    assert_eq!(signed.signatures(), [Some(expected)]);
    signed.verify().unwrap();
    let decoded = Transaction::decode(&signed.encode(true).unwrap()).unwrap();
    decoded.verify().unwrap();
    let mut altered = signed.encode(true).unwrap();
    altered[1] ^= 1;
    assert_eq!(
        Transaction::decode(&altered).unwrap().verify(),
        Err(Error::InvalidSignature)
    );
}

#[test]
fn base58_address_sorting_differs_from_raw_byte_order() {
    use crate::{AccountMeta, AccountRole};
    let low_bytes = Address::from_bytes([9; 32]);
    let high_bytes = Address::from_bytes([15; 32]);
    let accounts = alloc::vec![
        AccountMeta {
            address: low_bytes,
            role: AccountRole::Writable
        },
        AccountMeta {
            address: high_bytes,
            role: AccountRole::Writable
        }
    ];
    let message = compile_fixture(Version::Legacy, accounts, &[], Vec::new()).unwrap();
    assert_eq!(
        message.static_accounts(),
        [
            Address::from_bytes([1; 32]),
            high_bytes,
            low_bytes,
            Address::from_bytes([2; 32])
        ]
    );
}

#[test]
fn combined_account_indices_cover_the_u8_boundary() {
    use crate::{AccountMeta, AccountRole};
    let addresses: Vec<_> = (u8::MIN..=u8::MAX)
        .map(|byte| Address::from_bytes([byte; 32]))
        .collect();
    let table = LookupTable {
        address: Address::from_bytes([6; 32]),
        addresses: addresses.clone(),
    };
    let mut accounts: Vec<_> = addresses
        .into_iter()
        .filter(|address| {
            *address != Address::from_bytes([1; 32]) && *address != Address::from_bytes([2; 32])
        })
        .map(|address| AccountMeta {
            address,
            role: AccountRole::Writable,
        })
        .collect();
    let message = compile_fixture(
        Version::V0,
        accounts.clone(),
        core::slice::from_ref(&table),
        Vec::new(),
    )
    .unwrap();
    assert!(message.instructions()[0].accounts().contains(&255));
    assert!(Message::decode(message.bytes()).is_ok());
    let mut extra = [0; 32];
    extra[0] = 1;
    accounts.push(AccountMeta {
        address: Address::from_bytes(extra),
        role: AccountRole::Readonly,
    });
    assert_eq!(
        compile_fixture(Version::V0, accounts, &[table], Vec::new()),
        Err(Error::InvalidIndex)
    );
}

#[test]
fn incoming_lookup_constraints_are_protocol_specific() {
    let mut bytes = alloc::vec![128];
    bytes.extend(legacy_fixture());
    bytes.extend([1]);
    bytes.extend([6; 32]);
    bytes.extend([0, 0]);
    assert_eq!(Message::decode(&bytes), Err(Error::InvalidLookup));
    bytes.pop();
    bytes.extend([1, 255]);
    assert!(Message::decode(&bytes).is_ok());
    bytes[102] = 2;
    assert_eq!(Message::decode(&bytes), Err(Error::InvalidIndex));
}

#[test]
fn incoming_duplicate_signers_fill_every_owned_slot() {
    let mut bytes = alloc::vec![2, 0, 0, 2];
    bytes.extend([1; 32]);
    bytes.extend([1; 32]);
    bytes.extend([3; 32]);
    bytes.push(0);
    let transaction = Transaction::new(Message::decode(&bytes).unwrap());
    let signature = Signature::from_bytes([9; 64]);
    let signed = transaction
        .with_signature(Address::from_bytes([1; 32]), &bytes, signature)
        .unwrap();
    assert_eq!(signed.signatures(), [Some(signature), Some(signature)]);
    assert!(signed.encode(true).is_ok());
}
