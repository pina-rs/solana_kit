//! Every instruction uses independent literal bytes, nonzero fields and exact role matrices.
#[path = "support/mod.rs"]
mod support;

use solana_kit_core::{
    AccountMeta, AccountRole, Address, Instruction,
    codecs::{Decoder, Encoder, Limits},
};
use solana_kit_squads::*;

const LIMITS: Limits = Limits::new(8192, 4096);

fn verify(index: usize) {
    let case = &support::CASES[index];
    let mut bytes = support::hex(case.header);
    bytes.extend(support::payload(case.payload));
    let accounts = support::accounts(case.roles);
    let value = SquadsMultisigInstructionDataCodec
        .decode(&bytes, LIMITS)
        .unwrap();
    assert_eq!(identify_instruction(&bytes).unwrap(), &bytes[..8]);
    assert_eq!(
        SquadsMultisigInstructionDataCodec
            .encode(&value, LIMITS.max_bytes)
            .unwrap(),
        bytes
    );
    let instruction = rebuild(&value, &accounts);

    assert_eq!(instruction.data, bytes);
    assert_eq!(instruction.accounts, accounts);
    assert_eq!(parse_instruction(&instruction, LIMITS).unwrap(), value);
    malformed(&instruction);
    privileges(&instruction, case.remaining);
    optional_roles(&instruction, case.optional);
    payload_controls(&instruction, case.payload);
}

fn malformed(instruction: &Instruction) {
    for end in 0..instruction.data.len() {
        let mut short = instruction.clone();
        short.data.truncate(end);
        assert!(parse_instruction(&short, LIMITS).is_err());
    }

    let mut changed = instruction.clone();
    changed.data.push(0);
    assert!(parse_instruction(&changed, LIMITS).is_err());
    changed = instruction.clone();
    changed.data[0] ^= 0x80;
    assert!(parse_instruction(&changed, LIMITS).is_err());
    changed = instruction.clone();
    changed.accounts.pop();

    assert!(parse_instruction(&changed, LIMITS).is_err());
    assert!(parse_instruction(instruction, Limits::new(instruction.data.len() - 1, 4096)).is_err());

    let mut overridden = instruction.clone();
    overridden.program_address = Address::from_bytes([97; 32]);
    assert!(parse_instruction(&overridden, LIMITS).is_err());
    assert_eq!(
        parse_instruction_with_program(overridden.program_address, &overridden, LIMITS).unwrap(),
        parse_instruction(instruction, LIMITS).unwrap()
    );
}

/// Fixed accounts reject every privilege change; only declared remaining tails may extend them.
fn privileges(instruction: &Instruction, remaining: bool) {
    for index in 0..instruction.accounts.len() {
        for role in [
            AccountRole::Readonly,
            AccountRole::Writable,
            AccountRole::ReadonlySigner,
            AccountRole::WritableSigner,
        ] {
            if role == instruction.accounts[index].role {
                continue;
            }

            let mut changed = instruction.clone();
            changed.accounts[index].role = role;
            assert!(parse_instruction(&changed, LIMITS).is_err());
        }
    }

    let mut extended = instruction.clone();
    extended.accounts.extend([
        AccountMeta {
            address: Address::from_bytes([88; 32]),
            role: AccountRole::Writable,
        },
        AccountMeta {
            address: Address::from_bytes([89; 32]),
            role: AccountRole::ReadonlySigner,
        },
    ]);
    assert_eq!(parse_instruction(&extended, LIMITS).is_ok(), remaining);
    assert!(parse_instruction(&extended, Limits::new(8192, extended.accounts.len() - 1)).is_err());
}

fn optional_roles(instruction: &Instruction, optional: &[usize]) {
    for combination in 0..(1usize << optional.len()) {
        let mut selected = instruction.clone();

        for (bit, index) in optional.iter().enumerate() {
            if combination & (1 << bit) != 0 {
                selected.accounts[*index] = AccountMeta {
                    address: PROGRAM,
                    role: AccountRole::Readonly,
                };
            }
        }

        assert!(parse_instruction(&selected, LIMITS).is_ok());
    }
}

fn payload_controls(instruction: &Instruction, payload: support::Payload) {
    use support::Payload;
    let has_memo = matches!(
        payload,
        Payload::Create
            | Payload::Member
            | Payload::Remove
            | Payload::TimeLock
            | Payload::Threshold
            | Payload::Memo
            | Payload::Rent
            | Payload::Spending
            | Payload::Actions
            | Payload::Vault
            | Payload::Batch
            | Payload::Use
    );

    if has_memo {
        memo_controls(instruction);
    }

    specific_payload_controls(instruction, payload);
}

fn memo_controls(instruction: &Instruction) {
    const MEMO_SUFFIX_BYTES: usize = 7;
    let marker = instruction.data.len() - MEMO_SUFFIX_BYTES;
    let mut none = instruction.clone();
    none.data.truncate(marker);
    none.data.push(0);
    assert!(parse_instruction(&none, LIMITS).is_ok());
    let mut changed = instruction.clone();
    changed.data[marker] = 2;

    assert!(parse_instruction(&changed, LIMITS).is_err());
    changed = instruction.clone();
    changed.data[marker + 5] = 0xff;
    assert!(parse_instruction(&changed, LIMITS).is_err());
    changed = instruction.clone();
    changed.data[marker + 1..marker + 5].fill(0xff);
    assert!(parse_instruction(&changed, LIMITS).is_err());
}

fn specific_payload_controls(instruction: &Instruction, payload: support::Payload) {
    use support::Payload;
    match payload {
        Payload::Create | Payload::Rent => {
            let mut invalid = instruction.clone();
            invalid.data[8] = 2;
            assert!(parse_instruction(&invalid, LIMITS).is_err());
            let mut none = instruction.clone();
            none.data = [&instruction.data[..8], &[0], &instruction.data[41..]].concat();
            assert!(parse_instruction(&none, LIMITS).is_ok());
        }
        Payload::Proposal => {
            let mut invalid = instruction.clone();
            invalid.data[16] = 2;
            assert!(parse_instruction(&invalid, LIMITS).is_err());
        }
        Payload::Actions => {
            let mut invalid = instruction.clone();
            invalid.data[8..12].fill(0xff);
            assert!(parse_instruction(&invalid, LIMITS).is_err());
        }
        _ => {}
    }

    if matches!(payload, Payload::Create) {
        const RENT_OPTION_OFFSET: usize = 84;
        let mut some_rent = instruction.clone();
        some_rent.data = [
            instruction.data[..RENT_OPTION_OFFSET].to_vec(),
            vec![1],
            vec![9; 32],
            instruction.data[85..].to_vec(),
        ]
        .concat();
        assert!(parse_instruction(&some_rent, LIMITS).is_ok());
    }
}

macro_rules! builders {
    ($($variant:ident),* $(,)?) => {
        fn rebuild(value: &SquadsMultisigInstructionData, accounts: &[AccountMeta]) -> Instruction {
            match value {
                $(SquadsMultisigInstructionData::$variant(value) => {
                    value.instruction(accounts, LIMITS).unwrap()
                },)*
            }
        }
    };
}

builders!(
    ProgramConfigInit,
    ProgramConfigSetAuthority,
    ProgramConfigSetMultisigCreationFee,
    ProgramConfigSetTreasury,
    MultisigCreate,
    MultisigCreateV2,
    MultisigAddMember,
    MultisigRemoveMember,
    MultisigSetTimeLock,
    MultisigChangeThreshold,
    MultisigSetConfigAuthority,
    MultisigSetRentCollector,
    MultisigAddSpendingLimit,
    MultisigRemoveSpendingLimit,
    ConfigTransactionCreate,
    ConfigTransactionExecute,
    VaultTransactionCreate,
    TransactionBufferCreate,
    TransactionBufferClose,
    TransactionBufferExtend,
    VaultTransactionCreateFromBuffer,
    VaultTransactionExecute,
    BatchCreate,
    BatchAddTransaction,
    BatchExecuteTransaction,
    ProposalCreate,
    ProposalActivate,
    ProposalApprove,
    ProposalReject,
    ProposalCancel,
    ProposalCancelV2,
    SpendingLimitUse,
    ConfigTransactionAccountsClose,
    VaultTransactionAccountsClose,
    VaultBatchTransactionAccountClose,
    BatchAccountsClose
);

macro_rules! cases {
    ($($name:ident = $index:literal),* $(,)?) => {
        $(#[cfg_attr(test, test)] pub fn $name() { verify($index); })*
    };
}

cases!(
    program_config_init = 0,
    program_config_set_authority = 1,
    program_config_set_multisig_creation_fee = 2,
    program_config_set_treasury = 3,
    multisig_create = 4,
    multisig_create_v2 = 5,
    multisig_add_member = 6,
    multisig_remove_member = 7,
    multisig_set_time_lock = 8,
    multisig_change_threshold = 9,
    multisig_set_config_authority = 10,
    multisig_set_rent_collector = 11,
    multisig_add_spending_limit = 12,
    multisig_remove_spending_limit = 13,
    config_transaction_create = 14,
    config_transaction_execute = 15,
    vault_transaction_create = 16,
    transaction_buffer_create = 17,
    transaction_buffer_close = 18,
    transaction_buffer_extend = 19,
    vault_transaction_create_from_buffer = 20,
    vault_transaction_execute = 21,
    batch_create = 22,
    batch_add_transaction = 23,
    batch_execute_transaction = 24,
    proposal_create = 25,
    proposal_activate = 26,
    proposal_approve = 27,
    proposal_reject = 28,
    proposal_cancel = 29,
    proposal_cancel_v2 = 30,
    spending_limit_use = 31,
    config_transaction_accounts_close = 32,
    vault_transaction_accounts_close = 33,
    vault_batch_transaction_account_close = 34,
    batch_accounts_close = 35
);
