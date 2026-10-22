use std::collections::{BTreeMap, BTreeSet};

const CANONICAL: &str = include_str!("../../../docs/parity/capabilities.tsv");
const RECEIPT: &str = include_str!("../../../docs/parity/codec-framing.tsv");
const REMAINING: &str = include_str!("../../../docs/parity/codec-framing-remaining.tsv");

#[test]
fn exact_framing_receipt_preserves_source_ids_hashes_and_residuals() {
    let canonical: BTreeMap<_, _> = CANONICAL
        .lines()
        .skip(1)
        .filter_map(|line| {
            let fields: Vec<_> = line.split('\t').collect();
            (fields[1] == "solana_kit_codecs_core").then_some((fields[0], line))
        })
        .collect();
    let mut seen = BTreeSet::new();
    let mut partial = 0;

    for line in RECEIPT.lines().skip(1) {
        let fields: Vec<_> = line.split('\t').collect();
        assert_eq!(fields.len(), 17);
        assert!(seen.insert(fields[0]));
        assert_eq!(fields[..14].join("\t"), canonical[fields[0]]);
        assert_eq!(fields[12].len(), 64);

        match fields[14] {
            "partial" => {
                partial += 1;
                assert_ne!(fields[15], "-");
            }
            "not-covered-by-unit131" => assert_eq!(fields[15], "-"),
            _ => panic!("unexpected framing status"),
        }
    }

    assert_eq!(seen.len(), 113);
    assert_eq!(partial, 39);
    assert_eq!(seen, canonical.keys().copied().collect());
    let remaining: BTreeSet<_> = REMAINING
        .lines()
        .skip(1)
        .map(|line| line.split('\t').next().unwrap())
        .collect();
    assert_eq!(remaining, seen);
}
