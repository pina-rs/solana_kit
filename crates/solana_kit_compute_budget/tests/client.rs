use solana_kit_compute_budget::*;
use solana_kit_core::{
    AccountMeta, AccountRole, Address, Instruction,
    codecs::{Decoder, Encoder, Limits},
};

const LIMITS: Limits = Limits::new(64, 0);
fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

fn fixture_value(tag: u8, amount: u64) -> ComputeBudgetInstructionData {
    match tag {
        0 => ComputeBudgetInstructionData::RequestUnits(RequestUnits {
            units: amount.try_into().unwrap(),
            additional_fee: u32::MAX - u32::try_from(amount).unwrap(),
        }),
        1 => ComputeBudgetInstructionData::RequestHeapFrame(RequestHeapFrame {
            bytes: amount.try_into().unwrap(),
        }),
        2 => ComputeBudgetInstructionData::SetComputeUnitLimit(SetComputeUnitLimit {
            units: amount.try_into().unwrap(),
        }),
        3 => ComputeBudgetInstructionData::SetComputeUnitPrice(SetComputeUnitPrice {
            micro_lamports: amount,
        }),
        4 => ComputeBudgetInstructionData::SetLoadedAccountsDataSizeLimit(
            SetLoadedAccountsDataSizeLimit {
                account_data_size_limit: amount.try_into().unwrap(),
            },
        ),
        _ => panic!("unexpected fixture tag"),
    }
}

fn build(value: &ComputeBudgetInstructionData) -> Instruction {
    match value {
        ComputeBudgetInstructionData::RequestUnits(value) => value.instruction(LIMITS),
        ComputeBudgetInstructionData::RequestHeapFrame(value) => value.instruction(LIMITS),
        ComputeBudgetInstructionData::SetComputeUnitLimit(value) => value.instruction(LIMITS),
        ComputeBudgetInstructionData::SetComputeUnitPrice(value) => value.instruction(LIMITS),
        ComputeBudgetInstructionData::SetLoadedAccountsDataSizeLimit(value) => {
            value.instruction(LIMITS)
        }
    }
    .unwrap()
}

#[test]
fn actual_pinned_dart_all_five_constructors_and_full_width_values() {
    let mut count = 0;

    for row in include_str!("fixtures/dart.tsv")
        .lines()
        .filter(|row| !row.starts_with("helpers"))
    {
        let fields: Vec<_> = row.split('\t').collect();
        let value = fixture_value(fields[0].parse().unwrap(), fields[1].parse().unwrap());
        let instruction = build(&value);
        assert_eq!(instruction.program_address.to_string(), fields[2]);
        assert_eq!(instruction.accounts.len().to_string(), fields[3]);

        assert_eq!(hex(&instruction.data), fields[4]);
        assert_eq!(parse_instruction(&instruction, LIMITS).unwrap(), value);
        assert_eq!(
            ComputeBudgetInstructionDataCodec
                .encode(&value, LIMITS.max_bytes)
                .unwrap(),
            instruction.data
        );
        count += 1;
    }
    assert_eq!(count, 21);
}

fn malformed(value: ComputeBudgetInstructionData) {
    let instruction = build(&value);

    for length in 0..instruction.data.len() {
        let mut bad = instruction.clone();
        bad.data.truncate(length);
        assert!(parse_instruction(&bad, LIMITS).is_err());
    }

    let mut bad = instruction.clone();
    bad.data.push(0);
    assert!(parse_instruction(&bad, LIMITS).is_err());

    let mut bad = instruction.clone();
    bad.data[0] = 255;
    assert!(parse_instruction(&bad, LIMITS).is_err());
    let mut bad = instruction.clone();
    bad.program_address = Address::from_bytes([0; 32]);
    assert!(parse_instruction(&bad, LIMITS).is_err());
    let mut bad = instruction.clone();
    bad.accounts.push(AccountMeta {
        address: PROGRAM,
        role: AccountRole::Readonly,
    });

    assert!(parse_instruction(&bad, LIMITS).is_err());
    assert!(parse_instruction(&instruction, Limits::new(instruction.data.len() - 1, 0)).is_err());
    assert!(
        ComputeBudgetInstructionDataCodec
            .encode(&value, instruction.data.len() - 1)
            .is_err()
    );
}

#[test]
fn every_variant_rejects_wrong_program_accounts_tags_lengths_trailing_and_budgets() {
    for tag in 0..5 {
        malformed(fixture_value(tag, u32::MAX.into()));
    }

    assert!(identify_instruction(&[]).is_err());
    assert!(identify_instruction(&[5]).is_err());
    assert_eq!(identify_instruction(&[0]).unwrap(), 0);
    let wrong = SetComputeUnitPriceCodec
        .encode(&SetComputeUnitPrice { micro_lamports: 0 }, 9)
        .unwrap();
    assert!(RequestUnitsCodec.decode(&wrong, LIMITS).is_err());
}

#[test]
fn helper_ordering_duplicates_callbacks_and_u64_max_match_executed_dart() {
    let mut instructions = update_or_append_compute_unit_limit(&[], |_| 5).unwrap();
    instructions.push(
        SetComputeUnitLimit { units: 9 }
            .instruction(LIMITS)
            .unwrap(),
    );
    instructions =
        update_or_append_compute_unit_limit(&instructions, |old| old.unwrap() + 2).unwrap();
    instructions = append_compute_unit_price(&instructions, u64::MAX).unwrap();
    instructions = append_compute_unit_price(&instructions, 1).unwrap();
    instructions =
        update_or_append_compute_unit_price(&instructions, |old| old.unwrap() - 1).unwrap();

    let expected = include_str!("fixtures/dart.tsv")
        .lines()
        .find_map(|row| row.strip_prefix("helpers\t"))
        .unwrap();
    assert_eq!(
        instructions
            .iter()
            .map(|ix| hex(&ix.data))
            .collect::<Vec<_>>()
            .join(","),
        expected
    );
    assert_eq!(find_compute_unit_limit(&instructions), Ok(Some((0, 7))));
    assert_eq!(
        find_compute_unit_price(&instructions).unwrap(),
        Some((2, u64::MAX - 1))
    );
    assert_eq!(
        fill_provisory_compute_unit_limit(&instructions).unwrap(),
        instructions
    );
    assert_eq!(
        update_or_append_compute_unit_price(&instructions, |old| old.unwrap()).unwrap(),
        instructions
    );
    assert_eq!(
        find_compute_unit_limit(&fill_provisory_compute_unit_limit(&[]).unwrap()),
        Ok(Some((0, 0)))
    );
    assert_eq!(MAX_COMPUTE_UNIT_LIMIT, 1_400_000);
}

#[test]
fn helper_matching_preserves_source_short_foreign_and_trailing_contracts() {
    let mut short = SetComputeUnitLimit { units: 5 }
        .instruction(LIMITS)
        .unwrap();
    short.data.truncate(1);
    let mut foreign = SetComputeUnitPrice { micro_lamports: 8 }
        .instruction(LIMITS)
        .unwrap();
    foreign.program_address = Address::from_bytes([2; 32]);
    let instructions = [short, foreign];
    assert_eq!(find_compute_unit_limit(&instructions), Ok(None));
    assert_eq!(find_compute_unit_price(&instructions).unwrap(), None);
    assert_eq!(
        find_compute_unit_price(
            &update_or_append_compute_unit_price(&instructions, |old| {
                assert_eq!(old, None);
                u64::MAX
            })
            .unwrap()
        )
        .unwrap(),
        Some((2, u64::MAX))
    );

    let mut limit = SetComputeUnitLimit { units: 99 }
        .instruction(LIMITS)
        .unwrap();
    limit.data.push(0);
    assert_eq!(find_compute_unit_limit(&[limit]), Ok(Some((0, 99))));
    let mut price = SetComputeUnitPrice { micro_lamports: 99 }
        .instruction(LIMITS)
        .unwrap();
    price.data.push(0);
    assert_eq!(find_compute_unit_price(&[price]), Ok(Some((0, 99))));
}
