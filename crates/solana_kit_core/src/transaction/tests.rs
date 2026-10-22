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
