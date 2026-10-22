use solana_kit_core::{
    AccountMeta, AccountRole, Address, Hash, Instruction,
    transaction::{Error, HeapSize, Lifetime, Message, Transaction, V1Config, Version},
};

const LOADED_BIT: u32 = 8;
const HEAP_BIT: u32 = 16;
const MIN_HEAP: u32 = 32768;
const MAX_HEAP: u32 = 262144;

fn lifetime() -> Lifetime {
    Lifetime::Blockhash {
        blockhash: Hash::from_bytes([2; 32]),
        last_valid_block_height: 42,
    }
}

fn minimal() -> Vec<u8> {
    let mut bytes = vec![129, 1, 0, 0, 0, 0, 0, 0];
    bytes.extend([2; 32]);
    bytes.extend([0, 1]);
    bytes.extend([3; 32]);
    bytes
}

#[test]
fn literal_message_envelope_and_unsigned_slots() {
    let message = Message::compile_v1(
        Address::from_bytes([3; 32]),
        lifetime(),
        &[],
        V1Config::default(),
    )
    .unwrap();
    assert_eq!(message.bytes(), minimal());
    assert_eq!(message.version(), Version::V1);
    assert_eq!(message.config(), Some(V1Config::default()));

    let tx = Transaction::new(message);
    let mut wire = minimal();
    wire.extend([0; 64]);
    assert_eq!(tx.encode(false).unwrap(), wire);
    assert_eq!(tx.encode(true), Err(Error::MissingSignature));

    let decoded = Transaction::decode(&wire).unwrap();
    assert_eq!(decoded.encode(false).unwrap(), wire);
    assert_eq!(decoded.signatures(), [None]);
    assert_eq!(
        decoded.message().lifetime(),
        Lifetime::Unresolved(Hash::from_bytes([2; 32]))
    );
}

#[test]
fn all_config_masks_preserve_absence_and_full_width_values() {
    for mask in 0u32..32 {
        let mut bytes = minimal();
        bytes[4..8].copy_from_slice(&mask.to_le_bytes());

        if matches!(mask & 3, 1 | 2) {
            assert_eq!(Message::decode(&bytes), Err(Error::InvalidConfig));
            continue;
        }

        let (config, bytes) = config_fixture(mask, bytes);

        let decoded = Message::decode(&bytes).unwrap();
        assert_eq!(decoded.config(), Some(config));
        let built =
            Message::compile_v1(Address::from_bytes([3; 32]), lifetime(), &[], config).unwrap();
        assert_eq!(built.bytes(), bytes);
    }

    for mask in [32u32, 0x80000000, u32::MAX] {
        let mut bytes = minimal();
        bytes[4..8].copy_from_slice(&mask.to_le_bytes());
        assert_eq!(Message::decode(&bytes), Err(Error::InvalidConfig));
    }

    for value in [0, MIN_HEAP - 1, MIN_HEAP + 1, MAX_HEAP + 1, u32::MAX] {
        assert_eq!(HeapSize::new(value), Err(Error::InvalidConfig));
    }
}

fn config_fixture(mask: u32, mut bytes: Vec<u8>) -> (V1Config, Vec<u8>) {
    let mut config = V1Config::default();

    if mask & 3 != 0 {
        bytes.extend(u64::MAX.to_le_bytes());
        config = config.with_priority_fee(u64::MAX);
    }

    if mask & 4 != 0 {
        bytes.extend(u32::MAX.to_le_bytes());
        config = config.with_compute_unit_limit(u32::MAX);
    }

    if mask & LOADED_BIT != 0 {
        bytes.extend(0u32.to_le_bytes());
        config = config.with_loaded_accounts_data_limit(0);
    }

    if mask & HEAP_BIT != 0 {
        bytes.extend(MAX_HEAP.to_le_bytes());
        config = config.with_heap_size(HeapSize::new(MAX_HEAP).unwrap());
    }

    (config, bytes)
}

fn instruction(data: usize, accounts: usize) -> Instruction {
    Instruction {
        program_address: Address::from_bytes([5; 32]),
        accounts: vec![
            AccountMeta {
                address: Address::from_bytes([3; 32]),
                role: AccountRole::WritableSigner
            };
            accounts
        ],
        data: vec![7; data],
    }
}

#[test]
fn version_one_packet_and_instruction_limits_are_exact() {
    let build = |instructions: &[Instruction]| {
        Message::compile_v1(
            Address::from_bytes([3; 32]),
            lifetime(),
            instructions,
            V1Config::default(),
        )
    };
    let message = build(&[instruction(3922, 0)]).unwrap();
    let wire = Transaction::new(message).encode(false).unwrap();
    assert_eq!(wire.len(), 4096);
    assert!(Transaction::decode(&wire).is_ok());
    assert_eq!(build(&[instruction(3923, 0)]), Err(Error::PacketTooLarge));

    assert!(build(&vec![instruction(0, 0); 64]).is_ok());
    assert_eq!(
        build(&vec![instruction(0, 0); 65]),
        Err(Error::InvalidV1Limits)
    );
    assert!(build(&[instruction(0, 255)]).is_ok());
    assert_eq!(build(&[instruction(0, 256)]), Err(Error::InvalidLength));

    let mut over = wire;
    over.push(0);
    assert_eq!(Transaction::decode(&over), Err(Error::PacketTooLarge));
}

#[test]
fn malformed_received_fields_and_distinct_addresses_reject() {
    let wire = Transaction::new(Message::decode(&minimal()).unwrap())
        .encode(false)
        .unwrap();

    for end in 0..wire.len() {
        assert!(Transaction::decode(&wire[..end]).is_err());
    }

    let mut trailing = wire;
    trailing.push(0);
    assert_eq!(Transaction::decode(&trailing), Err(Error::TrailingBytes));
    let mut duplicate = minimal();
    duplicate[41] = 2;
    duplicate.extend([3; 32]);
    assert_eq!(Message::decode(&duplicate), Err(Error::InvalidV1Limits));
}

#[cfg(feature = "crypto")]
#[test]
fn signatures_cover_message_prefix_and_are_immutable() {
    let key = solana_kit_core::crypto::Keypair::from_seed(&[9; 32]);
    let message = Message::compile_v1(key.address(), lifetime(), &[], V1Config::default()).unwrap();
    let tx = Transaction::new(message).sign(&key).unwrap();
    tx.verify().unwrap();
    let mut wire = tx.encode(true).unwrap();
    assert_eq!(&wire[..tx.message().bytes().len()], tx.message().bytes());
    assert_eq!(
        &wire[tx.message().bytes().len()..],
        tx.signatures()[0].unwrap().as_bytes()
    );
    *wire.last_mut().unwrap() ^= 1;
    assert_eq!(
        Transaction::decode(&wire).unwrap().verify(),
        Err(Error::InvalidSignature)
    );
}
