extern crate std;
use std::format;
use std::prelude::v1::*;
// Independently produced Dart wire bytes are checked against literal Rust packets.
use super::fixtures::literal_wire;
use solana_kit_client::Presence;
use solana_kit_client::introspection::{InstructionTrace, decode_rpc_transaction};
use solana_kit_client::methods::transaction_results::TransactionEncoding;
use std::hash::{DefaultHasher, Hash, Hasher};

pub(super) fn actual_pinned_dart_bytes_equal_all_three_independent_literal_envelopes()
-> Result<(), &'static str> {
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
        check_eq!(literal_wire(version), expected);
        check!(include_str!("dart-oracle.tsv").contains(&format!("trace\t{name}\t0,0,3")));
    }

    Ok(())
}

fn fingerprint(value: &impl Hash) -> u64 {
    let mut hasher = DefaultHasher::new();
    value.hash(&mut hasher);
    hasher.finish()
}

pub(super) fn value_identity_includes_trace_depth_presence_and_ordered_instruction_content()
-> Result<(), &'static str> {
    use super::fixtures::{parse, raw};
    let source = parse(raw(), TransactionEncoding::Json);
    let decoded = decode_rpc_transaction(&source).unwrap();
    let left = decoded.walk_instructions().unwrap();
    let right = decoded.walk_instructions().unwrap();
    check_eq!(left, right);
    check_eq!(fingerprint(&left), fingerprint(&right));
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

    check_ne!(missing, null);
    check_ne!(fingerprint(&missing), fingerprint(&null));
    check_ne!(left[0], left[1]);
    check!(format!("{:?}", left[0]).contains("Outer"));
    Ok(())
}
