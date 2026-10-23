use alloc::vec::Vec;

use alloc::collections::{BTreeMap, BTreeSet};

const CANONICAL: &str = include_str!("../../../../docs/parity/capabilities.tsv");
const RECEIPT: &str = include_str!("../../../../docs/parity/composite-codecs.tsv");
const REMAINING: &str = include_str!("../../../../docs/parity/composite-codecs-remaining.tsv");
const CAPABILITY_COUNT: usize = 244;
const ORIGINAL_FIELDS: usize = 14;
const RECEIPT_FIELDS: usize = 17;
const COMPLETE: usize = 9;
const PARTIAL: usize = 24;
const UNCOVERED: usize = 211;

fn scoped_row(line: &str) -> Option<(&str, &str)> {
    let fields: Vec<_> = line.split('\t').collect();

    if fields[1] == "solana_kit_codecs_core" || fields[1] == "solana_kit_codecs_data_structures" {
        return Some((fields[0], line));
    }

    None
}

pub(super) fn exact_composite_receipt_preserves_every_original_field_and_remaining_id() {
    let canonical: BTreeMap<_, _> = CANONICAL.lines().skip(1).filter_map(scoped_row).collect();
    let mut seen = BTreeSet::new();
    let mut expected_remaining = BTreeSet::new();
    let mut statuses = BTreeMap::new();

    for line in RECEIPT.lines().skip(1) {
        let fields: Vec<_> = line.split('\t').collect();
        assert_eq!(fields.len(), RECEIPT_FIELDS);
        assert!(seen.insert(fields[0]));
        assert_eq!(fields[..ORIGINAL_FIELDS].join("\t"), canonical[fields[0]]);
        *statuses.entry(fields[ORIGINAL_FIELDS]).or_insert(0) += 1;

        if fields[ORIGINAL_FIELDS] != "implemented-bounded" {
            expected_remaining.insert(fields[0]);
        }
    }

    let remaining: BTreeSet<_> = REMAINING
        .lines()
        .skip(1)
        .map(|line| line.split('\t').next().unwrap())
        .collect();
    assert_eq!(seen.len(), CAPABILITY_COUNT);
    assert_eq!(canonical.len(), CAPABILITY_COUNT);
    assert_eq!(remaining, expected_remaining);
    assert_eq!(
        statuses,
        BTreeMap::from([
            ("implemented-bounded", COMPLETE),
            ("partial", PARTIAL),
            ("not-covered-by-unit73", UNCOVERED)
        ])
    );
}
