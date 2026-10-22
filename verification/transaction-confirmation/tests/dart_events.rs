//! Literal expectations were emitted independently by the pinned Dart package.
use solana_kit_client::{
    confirmation::{Confirmation, Event, Failure, Policy, commitment_comparator, timeout_millis},
    methods::Commitment,
    subscriptions::SignatureInfo,
};
use solana_kit_core::{Hash, Signature, transaction::Lifetime};
use std::fmt::Write;
const LEVELS: [Commitment; 3] = [
    Commitment::Finalized,
    Commitment::Confirmed,
    Commitment::Processed,
];
const NAMES: [&str; 3] = ["finalized", "confirmed", "processed"];

fn confirmation() -> Confirmation {
    let lifetime = Lifetime::Blockhash {
        blockhash: Hash::from_bytes([1; 32]),
        last_valid_block_height: u64::MAX,
    };
    Confirmation::new(Signature::from_bytes([7; 64]), lifetime, Policy::default()).unwrap()
}
fn commitments(actual: &mut String) {
    for (i, left) in LEVELS.iter().enumerate() {
        for (j, right) in LEVELS.iter().enumerate() {
            let sign = match commitment_comparator(*left, *right) {
                core::cmp::Ordering::Less => -1,
                core::cmp::Ordering::Equal => 0,
                core::cmp::Ordering::Greater => 1,
            };

            writeln!(actual, "commitment\t{}\t{}\t{sign}", NAMES[i], NAMES[j]).unwrap();
        }
    }
}
fn receipt(actual: &mut String) {
    let mut confirmation = confirmation();
    let signature = Signature::from_bytes([7; 64]);
    let received: SignatureInfo = serde_json::from_str(r#""receivedSignature""#).unwrap();
    let receipt = Event::Signature {
        signature,
        slot: 1,
        commitment: Commitment::Confirmed,
        value: received,
    };
    assert!(confirmation.observe(receipt).is_none());
    let processed = Event::Signature {
        signature,
        slot: 1,
        commitment: Commitment::Confirmed,
        value: SignatureInfo::Processed { err: None },
    };
    assert!(matches!(confirmation.observe(processed), Some(Ok(()))));
    actual.push_str("receipt\tpending>confirmed\n");
}
fn timeouts(actual: &mut String) {
    for (i, commitment) in LEVELS.iter().enumerate() {
        let mut confirmation = confirmation();
        assert!(matches!(
            confirmation.observe(Event::Timeout),
            Some(Err(Failure::Timeout))
        ));
        writeln!(
            actual,
            "timeout\t{}\tpending>timeout\t{}",
            NAMES[i],
            timeout_millis(*commitment)
        )
        .unwrap();
    }
}
#[test]
fn exact_pinned_dart_event_sequences() {
    let mut actual = String::new();
    commitments(&mut actual);
    receipt(&mut actual);
    assert!(solana_kit_confirmation_verification::expiry_boundary());
    actual.push_str("expiry\t41:pending>42:expired\n");
    timeouts(&mut actual);
    assert_eq!(actual, include_str!("../dart-events.tsv"));
    assert!(solana_kit_confirmation_verification::commitment_order());
}
