//! Independently produced Dart wire bytes are checked against literal Rust packets.

use super::fixtures::literal_wire;
use solana_kit_client::Presence;
use solana_kit_client::introspection::{InstructionTrace, decode_rpc_transaction};
use solana_kit_client::methods::transaction_results::TransactionEncoding;
use std::hash::{DefaultHasher, Hash, Hasher};

#[test]
fn actual_pinned_dart_bytes_equal_all_three_independent_literal_envelopes() {
    for (name, version) in [("legacy", u8::MAX), ("v0", 0), ("v1", 1)] {
        let line = include_str!("dart-oracle.tsv")
            .lines()
            .find(|line| line.starts_with(&format!("wire\t{name}\t")))
            .unwrap();
        let encoded = line.split('\t').nth(2).unwrap();
        let expected = (0..encoded.len())
            .step_by(2)
            .map(|index| u8::from_str_radix(&encoded[index..index + 2], 16).unwrap())
            .collect::<Vec<_>>();
        assert_eq!(literal_wire(version), expected);
        assert!(include_str!("dart-oracle.tsv").contains(&format!("trace\t{name}\t0,0,3")));
    }
}

fn fingerprint(value: &impl Hash) -> u64 {
    let mut hasher = DefaultHasher::new();
    value.hash(&mut hasher);
    hasher.finish()
}

#[test]
fn value_identity_includes_trace_depth_presence_and_ordered_instruction_content() {
    use super::fixtures::{parse, raw};
    let source = parse(raw(), TransactionEncoding::Json);
    let decoded = decode_rpc_transaction(&source).unwrap();
    let left = decoded.walk_instructions().unwrap();
    let right = decoded.walk_instructions().unwrap();
    assert_eq!(left, right);
    assert_eq!(fingerprint(&left), fingerprint(&right));
    let missing = InstructionTrace::Inner {
        outer_index: 0,
        inner_index: 0,
        stack_height: Presence::Missing,
    };
    let null = InstructionTrace::Inner {
        outer_index: 0,
        inner_index: 0,
        stack_height: Presence::Present(None),
    };

    assert_ne!(missing, null);
    assert_ne!(fingerprint(&missing), fingerprint(&null));
    assert_ne!(left[0], left[1]);
    assert!(format!("{:?}", left[0]).contains("Outer"));
}
