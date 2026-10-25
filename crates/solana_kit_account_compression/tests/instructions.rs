//! Full primary literal headers and nonzero payloads, independent of the renderer.
use solana_kit_account_compression::*;
use solana_kit_core::{
    AccountMeta, AccountRole, Address, Instruction,
    codecs::{Decoder, Encoder, Limits},
};

const LIMITS: Limits = Limits::new(4096, 1024);
const BYTE_LIMIT: usize = 4096;
const SECOND_PROOF: u8 = 5;

fn meta(byte: u8, role: AccountRole) -> AccountMeta {
    AccountMeta {
        address: Address::from_bytes([byte; 32]),
        role,
    }
}

fn modification() -> Vec<AccountMeta> {
    vec![
        meta(1, AccountRole::Writable),
        meta(2, AccountRole::ReadonlySigner),
        meta(3, AccountRole::Readonly),
    ]
}

fn verify(instruction: Instruction, value: SplAccountCompressionInstructionData, bytes: Vec<u8>) {
    assert_eq!(instruction.data, bytes);
    assert_eq!(parse_instruction(&instruction, LIMITS).unwrap(), value);
    assert_eq!(
        SplAccountCompressionInstructionDataCodec
            .encode(&value, BYTE_LIMIT)
            .unwrap(),
        bytes
    );
    assert_eq!(
        SplAccountCompressionInstructionDataCodec
            .decode(&bytes, LIMITS)
            .unwrap(),
        value
    );
    assert_eq!(identify_instruction(&bytes).unwrap(), &bytes[..8]);

    malformed(&instruction);
    role_matrix(&instruction);

    let foreign = Address::from_bytes([99; 32]);
    let mut overridden = instruction.clone();
    overridden.program_address = foreign;
    assert!(parse_instruction(&overridden, LIMITS).is_err());
    assert_eq!(
        parse_instruction_with_program(foreign, &overridden, LIMITS).unwrap(),
        value
    );
    assert!(
        parse_instruction(
            &instruction,
            Limits::new(BYTE_LIMIT, instruction.accounts.len() - 1)
        )
        .is_err()
    );
    assert!(parse_instruction(&instruction, Limits::new(bytes.len() - 1, 1024)).is_err());
}

fn malformed(instruction: &Instruction) {
    for end in 0..instruction.data.len() {
        let mut short = instruction.clone();
        short.data.truncate(end);
        assert!(parse_instruction(&short, LIMITS).is_err());
    }

    let mut trailing = instruction.clone();
    trailing.data.push(0);
    assert!(parse_instruction(&trailing, LIMITS).is_err());
    let mut wrong = instruction.clone();
    wrong.data[0] ^= 0x80;
    assert!(parse_instruction(&wrong, LIMITS).is_err());
    let mut missing = instruction.clone();
    missing.accounts.clear();

    assert!(parse_instruction(&missing, LIMITS).is_err());
}

fn role_matrix(instruction: &Instruction) {
    for index in 0..instruction.accounts.len() {
        for role in [
            AccountRole::Readonly,
            AccountRole::Writable,
            AccountRole::ReadonlySigner,
            AccountRole::WritableSigner,
        ] {
            let mut changed = instruction.clone();
            changed.accounts[index].role = role;
            assert_eq!(
                parse_instruction(&changed, LIMITS).is_ok(),
                role == instruction.accounts[index].role
            );
        }
    }
}

#[cfg_attr(test, test)]
pub fn initialize_transfer_append_and_close_match_primary_bytes() {
    initialize_and_transfer();
    append_and_close();
}

fn initialize_and_transfer() {
    let accounts = modification();
    let value = InitEmptyMerkleTree {
        max_depth: 14,
        max_buffer_size: 256,
    };
    verify(
        value.instruction(&accounts, LIMITS).unwrap(),
        SplAccountCompressionInstructionData::InitEmptyMerkleTree(value),
        vec![191, 11, 119, 7, 180, 107, 220, 110, 14, 0, 0, 0, 0, 1, 0, 0],
    );

    let value = TransferAuthority {
        new_authority: Address::from_bytes([71; 32]),
    };
    let mut bytes = vec![48, 169, 76, 72, 229, 180, 55, 161];
    bytes.extend_from_slice(&[71; 32]);
    verify(
        value.instruction(&accounts[..2], LIMITS).unwrap(),
        SplAccountCompressionInstructionData::TransferAuthority(value),
        bytes,
    );
}

fn append_and_close() {
    let accounts = modification();
    let value = Append { leaf: vec![72; 32] };
    let mut bytes = vec![149, 120, 18, 222, 236, 225, 88, 203];
    bytes.extend_from_slice(&[72; 32]);
    verify(
        value.instruction(&accounts, LIMITS).unwrap(),
        SplAccountCompressionInstructionData::Append(value),
        bytes,
    );

    let value = CloseEmptyTree {};
    let accounts = [accounts[0], accounts[1], meta(4, AccountRole::Writable)];
    verify(
        value.instruction(&accounts, LIMITS).unwrap(),
        SplAccountCompressionInstructionData::CloseEmptyTree(value),
        vec![50, 14, 219, 107, 78, 103, 16, 103],
    );
}

#[cfg_attr(test, test)]
pub fn every_proof_instruction_matches_primary_payload_and_remaining_roles() {
    let mut accounts = modification();
    accounts.push(meta(4, AccountRole::Readonly));
    accounts.push(meta(SECOND_PROOF, AccountRole::Readonly));
    replace(&accounts);
    insert(&accounts);
    verify_leaf();
}

fn replace(accounts: &[AccountMeta]) {
    let value = ReplaceLeaf {
        root: vec![11; 32],
        previous_leaf: vec![12; 32],
        new_leaf: vec![13; 32],
        index: 0x12345678,
    };
    let mut bytes = vec![204, 165, 76, 100, 73, 147, 0, 128];
    bytes.extend_from_slice(&[11; 32]);
    bytes.extend_from_slice(&[12; 32]);
    bytes.extend_from_slice(&[13; 32]);
    bytes.extend_from_slice(&[0x78, 0x56, 0x34, 0x12]);
    verify(
        value.instruction(accounts, LIMITS).unwrap(),
        SplAccountCompressionInstructionData::ReplaceLeaf(value.clone()),
        bytes,
    );
    assert!(value.instruction(&accounts[..3], LIMITS).is_ok());
}

fn insert(accounts: &[AccountMeta]) {
    let value = InsertOrAppend {
        root: vec![21; 32],
        leaf: vec![22; 32],
        index: 0x87654321,
    };
    let mut bytes = vec![6, 42, 50, 190, 51, 109, 178, 168];
    bytes.extend_from_slice(&[21; 32]);
    bytes.extend_from_slice(&[22; 32]);
    bytes.extend_from_slice(&[0x21, 0x43, 0x65, 0x87]);
    verify(
        value.instruction(accounts, LIMITS).unwrap(),
        SplAccountCompressionInstructionData::InsertOrAppend(value.clone()),
        bytes,
    );
    assert!(value.instruction(&accounts[..3], LIMITS).is_ok());
}

fn verify_leaf() {
    let value = VerifyLeaf {
        root: vec![31; 32],
        leaf: vec![32; 32],
        index: 0x76543210,
    };
    let accounts = [
        meta(1, AccountRole::Readonly),
        meta(4, AccountRole::Readonly),
        meta(SECOND_PROOF, AccountRole::Readonly),
    ];
    let mut bytes = vec![124, 220, 22, 223, 104, 10, 250, 224];
    bytes.extend_from_slice(&[31; 32]);
    bytes.extend_from_slice(&[32; 32]);
    bytes.extend_from_slice(&[0x10, 0x32, 0x54, 0x76]);
    verify(
        value.instruction(&accounts, LIMITS).unwrap(),
        SplAccountCompressionInstructionData::VerifyLeaf(value.clone()),
        bytes,
    );
    assert!(value.instruction(&accounts[..1], LIMITS).is_ok());
}
