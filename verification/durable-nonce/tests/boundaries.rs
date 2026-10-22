use solana_kit_core::transaction::{
    DurableNonce, Error, Lifetime, LookupTable, Message, PACKET_SIZE, Transaction, V1_PACKET_SIZE,
    Version,
};
use solana_kit_core::{Address, Hash, Instruction};

const PREFIX_PROGRAM: Address = Address::from_bytes([9; 32]);

fn nonce() -> DurableNonce {
    DurableNonce::new(
        Hash::from_bytes([7; 32]),
        Address::from_bytes([2; 32]),
        Address::from_bytes([3; 32]),
    )
}

fn packet(version: Version, length: usize) -> Result<Transaction, Error> {
    let payload = Instruction {
        program_address: PREFIX_PROGRAM,
        accounts: vec![],
        data: vec![0; length],
    };
    let nonce = nonce();
    let message = Message::compile(
        version,
        Address::from_bytes([1; 32]),
        Lifetime::DurableNonce(nonce),
        &nonce.prepare_instructions(&[payload]),
        &[],
    )?;
    Ok(Transaction::new(message))
}

#[test]
fn exact_packet_capacity_includes_nonce_advance_and_signatures() {
    for version in [Version::Legacy, Version::V0, Version::V1] {
        let limit = if version == Version::V1 {
            V1_PACKET_SIZE
        } else {
            PACKET_SIZE
        };
        let largest = (0..limit)
            .rev()
            .find_map(|length| packet(version, length).ok().map(|packet| (length, packet)))
            .unwrap();
        assert_eq!(largest.1.encode(false).unwrap().len(), limit);
        assert_eq!(packet(version, largest.0 + 1), Err(Error::PacketTooLarge));
    }
}

#[test]
fn loaded_nonce_identities_require_explicit_resolution() {
    let nonce = nonce();
    let tables = [LookupTable {
        address: Address::from_bytes([8; 32]),
        addresses: vec![nonce.advance().nonce_account],
    }];
    let message = Message::compile(
        Version::V0,
        Address::from_bytes([1; 32]),
        Lifetime::DurableNonce(nonce),
        &nonce.prepare_instructions(&[]),
        &tables,
    )
    .unwrap();
    assert_eq!(message.nonce_advance(), Err(Error::InvalidLookup));
    let received = Message::decode(message.bytes()).unwrap();
    assert_eq!(received.lifetime(), Lifetime::Unresolved(nonce.token()));
    assert_eq!(received.nonce_advance(), Err(Error::InvalidLookup));
}

#[test]
fn missing_or_misordered_advance_and_reserved_identity_mismatch_reject() {
    let nonce = nonce();
    let payer = Address::from_bytes([1; 32]);
    let prefix = Instruction {
        program_address: PREFIX_PROGRAM,
        accounts: vec![],
        data: vec![],
    };
    let invalid = [vec![], vec![prefix, nonce.advance().instruction()]];

    for instructions in invalid {
        assert_eq!(
            Message::compile(
                Version::V0,
                payer,
                Lifetime::DurableNonce(nonce),
                &instructions,
                &[]
            ),
            Err(Error::InvalidNonceAdvance)
        );
    }

    let reserved = DurableNonce::new(
        nonce.token(),
        solana_kit_core::sysvars::RECENT_BLOCKHASHES_ADDRESS,
        nonce.advance().nonce_authority,
    );
    assert_eq!(
        Message::compile(
            Version::V0,
            payer,
            Lifetime::DurableNonce(reserved),
            &nonce.prepare_instructions(&[]),
            &[]
        ),
        Err(Error::InvalidNonceAdvance)
    );
}

#[test]
fn existing_system_constructor_agrees_without_a_core_program_dependency() {
    use solana_kit_core::codecs::Limits;
    use solana_kit_system::{AdvanceNonceAccount, AdvanceNonceAccountAccounts};
    let nonce = nonce();
    let advance = nonce.advance();
    let system = AdvanceNonceAccount {}
        .instruction(
            AdvanceNonceAccountAccounts {
                nonce_account: advance.nonce_account,
                nonce_authority: advance.nonce_authority,
            },
            Limits::new(4, 0),
        )
        .unwrap();
    assert_eq!(system, advance.instruction());
}
