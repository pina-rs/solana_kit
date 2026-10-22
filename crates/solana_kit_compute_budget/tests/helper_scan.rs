use solana_kit_compute_budget::*;
use solana_kit_core::{
    Address, Instruction,
    codecs::{Error, Limits},
};

const LIMIT_UNITS: u32 = 7;
const LIMIT_SIZE: usize = 5;
const PRICE_SIZE: usize = 9;
const MAX_PROBE_SIZE: usize = 12;
const LAST_KNOWN_TAG: u8 = 4;

const LIMIT_TAG: u8 = 2;
const PRICE_TAG: u8 = 3;
const FOREIGN_PROGRAM: Address = Address::from_bytes([93; 32]);
const LIMITS: Limits = Limits::new(PRICE_SIZE, 0);

fn limit() -> Instruction {
    SetComputeUnitLimit { units: LIMIT_UNITS }
        .instruction(LIMITS)
        .unwrap()
}

fn price() -> Instruction {
    SetComputeUnitPrice {
        micro_lamports: u64::MAX,
    }
    .instruction(LIMITS)
    .unwrap()
}

#[test]
fn unknown_qualifying_tag_rejects_before_callbacks_or_fill() {
    let unknown = Instruction {
        program_address: PROGRAM,
        accounts: Vec::new(),
        data: vec![255; PRICE_SIZE],
    };
    let instructions = [unknown, limit(), price()];
    let original = instructions.clone();
    let mut called = false;
    let result = update_or_append_compute_unit_limit(&instructions, |_| {
        called = true;
        1
    });
    assert_eq!(result, Err(Error::InvalidDiscriminator));
    assert!(!called);

    let result = update_or_append_compute_unit_price(&instructions, |_| {
        called = true;
        1
    });
    assert_eq!(result, Err(Error::InvalidDiscriminator));
    assert!(!called);
    assert_eq!(
        fill_provisory_compute_unit_limit(&instructions),
        Err(Error::InvalidDiscriminator)
    );
    assert_eq!(instructions, original);
}

fn probe(tag: u8, size: usize, program_address: Address) -> Instruction {
    let mut data = vec![0; size];

    if let Some(first) = data.first_mut() {
        *first = tag;
    }

    Instruction {
        program_address,
        accounts: Vec::new(),
        data,
    }
}

#[test]
fn scan_threshold_program_and_first_match_matrix() {
    let cases = (0..=255).flat_map(|tag| (0..=MAX_PROBE_SIZE).map(move |size| (tag, size)));

    for (tag, size) in cases {
        for program in [PROGRAM, FOREIGN_PROGRAM] {
            let instruction = probe(tag, size, program);
            check_limit(&instruction, tag, size, program == FOREIGN_PROGRAM);
            check_price(&instruction, tag, size, program == FOREIGN_PROGRAM);
        }
    }
}

fn check_limit(probe: &Instruction, tag: u8, size: usize, foreign: bool) {
    let qualifying = !foreign && size >= LIMIT_SIZE;

    let expected = match (qualifying, tag) {
        (true, tag) if tag > LAST_KNOWN_TAG => Err(Error::InvalidDiscriminator),
        (true, LIMIT_TAG) => Ok(Some((0, 0))),
        _ => Ok(Some((1, LIMIT_UNITS))),
    };

    assert_eq!(find_compute_unit_limit(&[probe.clone(), limit()]), expected);
    assert_eq!(
        find_compute_unit_limit(&[limit(), probe.clone()]),
        Ok(Some((0, LIMIT_UNITS)))
    );
}

fn check_price(probe: &Instruction, tag: u8, size: usize, foreign: bool) {
    let qualifying = !foreign && size >= PRICE_SIZE;

    let expected = match (qualifying, tag, size) {
        (true, tag, _) if tag > LAST_KNOWN_TAG => Err(Error::InvalidDiscriminator),
        (true, PRICE_TAG, _) => Ok(Some((0, 0))),
        _ => Ok(Some((1, u64::MAX))),
    };

    assert_eq!(find_compute_unit_price(&[probe.clone(), price()]), expected);
    assert_eq!(
        find_compute_unit_price(&[price(), probe.clone()]),
        Ok(Some((0, u64::MAX)))
    );
}
