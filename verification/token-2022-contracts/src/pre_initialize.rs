use solana_kit_core::{
    AccountMeta, AccountRole, Address, Instruction,
    codecs::{Decoder, Error, Limits},
};
use solana_kit_token_2022::extensions::pre_initialize_mint_extensions;
use solana_kit_token_2022::*;
const ORACLE: &str = include_str!("../../../tools/codama-renderer/fixtures/token-2022-oracle.tsv");
const ITEMS: usize = 4096;
const RADIX: u32 = 16;
const CASES: [(&str, &str); 15] = [
    (
        "confidentialTransferMint",
        "initializeConfidentialTransferMint",
    ),
    ("defaultAccountState", "initializeDefaultAccountState"),
    ("transferFeeConfig", "initializeTransferFeeConfig"),
    ("metadataPointer", "initializeMetadataPointer"),
    ("interestBearingConfig", "initializeInterestBearingMint"),
    ("scaledUiAmountConfig", "initializeScaledUiAmountMint"),
    ("pausableConfig", "initializePausableConfig"),
    ("permissionedBurn", "initializePermissionedBurn"),
    ("groupPointer", "initializeGroupPointer"),
    ("groupMemberPointer", "initializeGroupMemberPointer"),
    ("nonTransferable", "initializeNonTransferableMint"),
    ("transferHook", "initializeTransferHook"),
    ("permanentDelegate", "initializePermanentDelegate"),
    (
        "confidentialTransferFee",
        "initializeConfidentialTransferFee",
    ),
    ("mintCloseAuthority", "initializeMintCloseAuthority"),
];
fn limits() -> Limits {
    Limits::new(1000, ITEMS)
}
fn literal(name: &str) -> Vec<u8> {
    let row = ORACLE
        .lines()
        .find(|row| row.split('\t').next() == Some(name))
        .unwrap();
    row.split('\t')
        .nth(1)
        .unwrap()
        .as_bytes()
        .as_chunks::<2>()
        .0
        .iter()
        .map(|pair| u8::from_str_radix(std::str::from_utf8(pair).unwrap(), RADIX).unwrap())
        .collect()
}
fn extension(name: &str, case: u8) -> Extension {
    let bare = literal(&format!("extension-{name}-{case}"));
    let mut framed = bare[..2].to_vec();
    framed.extend_from_slice(&u16::try_from(bare.len() - 2).unwrap().to_le_bytes());
    framed.extend_from_slice(&bare[2..]);
    ExtensionCodec.decode(&framed, limits()).unwrap()
}
fn assert_malformed(instruction: &Instruction) {
    for end in 0..instruction.data.len() {
        let mut bad = instruction.clone();
        bad.data.truncate(end);
        assert!(
            parse_instruction_with_program(instruction.program_address, &bad, limits()).is_err()
        );
    }

    let mut bad = instruction.clone();
    bad.data[0] = 250;
    assert!(parse_instruction_with_program(instruction.program_address, &bad, limits()).is_err());
    bad = instruction.clone();

    bad.accounts[0].role = AccountRole::Readonly;
    assert!(parse_instruction_with_program(instruction.program_address, &bad, limits()).is_err());
    assert!(parse_instruction(instruction, limits()).is_err());
}
pub(crate) fn all_fifteen_branches_match_independent_literal_outputs_and_crosschecks() {
    let mint = Address::from_bytes([3; 32]);
    let program = Address::from_bytes([9; 32]);
    let extensions: Vec<_> = CASES.iter().map(|(name, _)| extension(name, 1)).collect();
    let instructions =
        pre_initialize_mint_extensions(mint, &extensions, Some(program), limits()).unwrap();
    assert_eq!(instructions.len(), 15);

    for (instruction, (_, name)) in instructions.iter().zip(CASES) {
        assert_eq!(instruction.program_address, program);
        assert_eq!(
            instruction.accounts,
            [AccountMeta {
                address: mint,
                role: AccountRole::Writable
            }]
        );
        assert_eq!(instruction.data, literal(&format!("{name}-1")), "{name}");
        assert_malformed(instruction);
    }
}
pub(crate) fn invalid_authority_limits_and_unmatched_extensions_are_explicit() {
    let mint = Address::from_bytes([3; 32]);
    let missing = extension("permissionedBurn", 0);
    assert_eq!(
        pre_initialize_mint_extensions(mint, &[missing], None, limits()),
        Err(Error::InvalidValue)
    );
    let value = extension("nonTransferable", 1);
    assert_eq!(
        pre_initialize_mint_extensions(
            mint,
            std::slice::from_ref(&value),
            None,
            Limits::new(1000, 0)
        ),
        Err(Error::LimitExceeded)
    );
    assert!(pre_initialize_mint_extensions(mint, &[value], None, Limits::new(0, ITEMS)).is_err());
    let unmatched = Extension::TransferFeeAmount(ExtensionTransferFeeAmountPayload {
        withheld_amount: u64::MAX,
    });
    assert_eq!(
        pre_initialize_mint_extensions(mint, &[unmatched], None, limits()).unwrap(),
        []
    );
}
