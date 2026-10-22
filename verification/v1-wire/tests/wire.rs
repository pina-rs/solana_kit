use base64::{Engine, engine::general_purpose::STANDARD};
use solana_kit_v1_wire_proof::{Message, decode, encode};

fn minimal() -> Message {
    Message {
        header: [1, 0, 0],
        mask: 0,
        lifetime: [2; 32],
        addresses: vec![[3; 32]],
        config: vec![],
        instructions: vec![],
    }
}

#[test]
fn literal_message_first_layout_and_official_oracle() {
    let message = minimal();
    let mut literal = vec![129, 1, 0, 0, 0, 0, 0, 0];
    literal.extend([2; 32]);
    literal.extend([0, 1]);
    literal.extend([3; 32]);
    literal.extend([4; 64]);
    assert_eq!(literal.len(), 138);
    assert_eq!(encode(&message, &[[4; 64]]).unwrap(), literal);
    assert_eq!(decode(&literal).unwrap(), (message, vec![[4; 64]]));
    official(&literal);
}

fn official(wire: &[u8]) {
    let (message, signatures) = decode(wire).unwrap();
    let message_end = wire.len() - signatures.len() * 64;
    let official = solana_message::v1::deserialize(&wire[1..message_end]).unwrap();
    official.validate().unwrap();
    assert_eq!(official.serialize(), wire[..message_end]);
    assert_eq!(encode(&message, &signatures).unwrap(), wire);
}

#[test]
fn pinned_dart_vectors_match_official_rust_and_exact_envelope() {
    let vectors: serde_json::Value =
        serde_json::from_str(include_str!("../fixtures/dart-v1.json")).unwrap();

    for (name, fixture) in vectors["cases"].as_object().unwrap() {
        let wire = STANDARD.decode(fixture["wire"].as_str().unwrap()).unwrap();
        assert_eq!(
            wire.len(),
            fixture["wireBytes"].as_u64().unwrap() as usize,
            "{name}"
        );
        official(&wire);

        for length in 0..wire.len() {
            assert!(decode(&wire[..length]).is_err(), "{name} prefix {length}");
        }

        let mut trailing = wire.clone();
        trailing.push(0);
        assert!(decode(&trailing).is_err());
    }
}

#[test]
fn masks_version_and_semantic_boundaries_reject() {
    let mut message = minimal();

    for mask in [1, 2, 32, 0x80000000, 0xffffffff] {
        message.mask = mask;
        assert!(encode(&message, &[[0; 64]]).is_err());
    }

    message = minimal();
    message.header = [0, 0, 0];
    assert!(encode(&message, &[]).is_err());
    message = minimal();
    message.addresses.push([3; 32]);
    assert!(encode(&message, &[[0; 64]]).is_err());
    let wire = encode(&minimal(), &[[0; 64]]).unwrap();

    for first in [0, 1, 128, 130, 255] {
        let mut changed = wire.clone();
        changed[0] = first;
        assert!(decode(&changed).is_err());
    }
}

#[test]
fn inclusive_packet_cap_includes_signature_and_fixed_lengths() {
    let mut message = minimal();
    message.header[2] = 1;
    message.addresses.push([5; 32]);
    message
        .instructions
        .push(solana_kit_v1_wire_proof::Instruction {
            program: 1,
            accounts: vec![],
            data: vec![7; 3922],
        });
    let wire = encode(&message, &[[0; 64]]).unwrap();
    assert_eq!(wire.len(), 4096);
    official(&wire);
    message.instructions[0].data.push(7);
    assert!(encode(&message, &[[0; 64]]).is_err());
    let mut oversized = wire;
    oversized.push(0);
    assert!(decode(&oversized).is_err());
}
