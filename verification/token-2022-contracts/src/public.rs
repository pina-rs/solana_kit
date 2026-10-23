use solana_kit_core::{
    AccountMeta, AccountRole, Address, Instruction,
    codecs::{Decoder, Encoder, Limits},
};
use solana_kit_token_2022::*;
const HEX_RADIX: u32 = 16;

fn address(byte: u8) -> Address {
    Address::from_bytes([byte; 32])
}
fn limits() -> Limits {
    Limits {
        max_bytes: 1_000_000,
        max_items: 4096,
    }
}
fn hex(text: &str) -> Vec<u8> {
    text.as_bytes()
        .as_chunks::<2>()
        .0
        .iter()
        .map(|pair| u8::from_str_radix(std::str::from_utf8(pair).unwrap(), HEX_RADIX).unwrap())
        .collect()
}

pub(crate) fn all_100_dispatch_and_400_independent_frames_preserve_accounts() {
    let mut count = 0;
    let mut strict = 0;

    for row in
        include_str!("../../../verification/codegen-design/fixtures/token-2022-oracle.tsv").lines()
    {
        let fields: Vec<_> = row.split('\t').collect();

        if fields[0].starts_with("account-") || fields[0].starts_with("extension-") {
            continue;
        }

        let instruction = oracle_instruction(&fields);
        assert_data_frame(&instruction.data, fields[0]);
        strict += usize::from(assert_invocation(&instruction, fields[0]));
        count += 1;
    }

    assert_eq!(count, 400);
    assert_eq!(strict, 398);
}

fn oracle_instruction(fields: &[&str]) -> Instruction {
    let accounts = fields[3]
        .split(',')
        .filter(|item| !item.is_empty())
        .map(|item| {
            let (key, role) = item.rsplit_once(':').unwrap();
            let bits = role.parse::<u8>().unwrap();
            AccountMeta {
                address: key.parse().unwrap(),
                role: AccountRole::from_privileges(bits & 2 != 0, bits & 1 != 0),
            }
        })
        .collect();
    Instruction {
        program_address: fields[2].parse().unwrap(),
        accounts,
        data: hex(fields[1]),
    }
}

fn assert_data_frame(data: &[u8], name: &str) {
    let value = Token2022InstructionDataCodec
        .decode(data, limits())
        .unwrap();
    assert_eq!(
        Token2022InstructionDataCodec
            .encode(&value, limits().max_bytes)
            .unwrap(),
        data,
        "{name}"
    );
}

fn assert_invocation(instruction: &Instruction, name: &str) -> bool {
    let parsed = parse_instruction_with_program(instruction.program_address, instruction, limits());

    // The original Dart builder accepts impossible count/source combinations.
    // Exact data parity remains; strict public invocation parsing rejects them.
    if name == "withdrawWithheldTokensFromAccounts-1"
        || name == "withdrawWithheldTokensFromAccounts-2"
    {
        assert!(parsed.is_err());

        return false;
    }

    let parsed = parsed.unwrap_or_else(|error| panic!("{name}: {error:?}"));
    assert_eq!(parsed.accounts, instruction.accounts);
    true
}

pub(crate) fn typed_builder_defaults_authority_and_two_segment_boundary_are_literal() {
    let value = WithdrawWithheldTokensFromAccounts {
        num_token_accounts: 2,
    };
    let instruction = value
        .instruction(
            WithdrawWithheldTokensFromAccountsAccounts {
                mint: address(1),
                fee_receiver: address(2),
                withdraw_withheld_authority: Authority {
                    address: address(3),
                    is_signer: false,
                },
                multi_signers: &[address(4), address(4)],
                sources: &[address(5), address(6)],
            },
            limits(),
        )
        .unwrap();
    assert_eq!(instruction.data, [26, 3, 2]);
    assert_eq!(
        instruction
            .accounts
            .iter()
            .map(|meta| meta.role)
            .collect::<Vec<_>>(),
        [
            AccountRole::Readonly,
            AccountRole::Writable,
            AccountRole::Readonly,
            AccountRole::ReadonlySigner,
            AccountRole::ReadonlySigner,
            AccountRole::Writable,
            AccountRole::Writable,
        ]
    );
    assert_eq!(instruction.accounts[3], instruction.accounts[4]);
    let mut wrong = instruction.clone();
    wrong.accounts.swap(4, 5);
    assert!(parse_instruction(&wrong, limits()).is_err());

    wrong = instruction.clone();
    wrong.data[2] = 3;
    assert!(parse_instruction(&wrong, limits()).is_err());
    wrong = instruction.clone();
    wrong.program_address = address(8);
    assert!(parse_instruction(&wrong, limits()).is_err());
    assert!(parse_instruction_with_program(address(8), &wrong, limits()).is_ok());
    wrong.data.push(0);

    assert!(parse_instruction_with_program(address(8), &wrong, limits()).is_err());
}

pub(crate) fn rent_default_is_literal() {
    let mint = InitializeMint {
        decimals: 9,
        mint_authority: address(1),
        freeze_authority: None,
    };
    let instruction = mint
        .instruction(
            InitializeMintAccounts {
                mint: address(2),
                rent: None,
            },
            limits(),
        )
        .unwrap();
    assert_eq!(
        instruction.accounts[1].address,
        "SysvarRent111111111111111111111111111111111"
            .parse()
            .unwrap()
    );
    assert_eq!(instruction.accounts[1].role, AccountRole::Readonly);
}

pub(crate) fn omitted_middle_accounts_keep_actual_positions() {
    let value = ConfidentialWithdraw {
        amount: u64::MAX,
        decimals: 9,
        new_decryptable_available_balance: DecryptableBalance { value: [7; 36] },
        equality_proof_instruction_offset: i8::MIN,
        range_proof_instruction_offset: i8::MAX,
    };
    let instruction = value
        .instruction(
            ConfidentialWithdrawAccounts {
                token: address(1),
                mint: address(2),
                instructions_sysvar: None,
                equality_record: Some(address(3)),
                range_record: None,
                authority: Authority {
                    address: address(4),
                    is_signer: false,
                },
                multi_signers: &[address(5)],
            },
            limits(),
        )
        .unwrap();
    assert_eq!(
        instruction
            .accounts
            .iter()
            .map(|meta| meta.address)
            .collect::<Vec<_>>(),
        [address(1), address(2), address(3), address(4), address(5)]
    );
    assert_eq!(
        ConfidentialWithdraw::parse_with_program(PROGRAM, &instruction, limits()).unwrap(),
        value
    );
    let mut bad = instruction;
    bad.accounts[4].role = AccountRole::Writable;
    assert!(parse_instruction(&bad, limits()).is_err());
}

pub(crate) fn all_errors_preserve_unknown_numeric_codes_and_actual_index() {
    for row in include_str!(
        "../../../verification/codegen-design/fixtures/token-2022-rust-error-labels.tsv"
    )
    .lines()
    {
        let (code, label) = row.split_once('\t').unwrap();
        let code = code.parse().unwrap();
        let error = errors::Token2022Error::from_code(code);
        assert_eq!(error.code(), code);
        assert_eq!(error.label(), Some(label));
    }

    let failure = solana_kit_core::program_error::InstructionFailure {
        index: 1,
        error: solana_kit_core::program_error::ProgramError::Custom(u32::MAX),
    };
    let programs = [PROGRAM, address(8)];
    assert!(errors::classify_failure(&failure, &programs, PROGRAM).is_none());
    let error = errors::classify_failure(&failure, &programs, address(8)).unwrap();
    assert_eq!(error, errors::Token2022Error::Unknown(u32::MAX));
    assert_eq!(error.code(), u32::MAX);
}
