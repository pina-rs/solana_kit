//! Independent nonzero data literals and the full ordered privilege policy.
use solana_kit_attestation_service::*;
use solana_kit_core::{
    AccountMeta, AccountRole, Address, Instruction,
    codecs::{Decoder, Encoder, Limits},
};

const BYTE_BUDGET: usize = 4096;
const ITEM_BUDGET: usize = 2048;
const LIMITS: Limits = Limits::new(BYTE_BUDGET, ITEM_BUDGET);

fn accounts(bits: &[u8]) -> Vec<AccountMeta> {
    bits.iter()
        .enumerate()
        .map(|(index, bits)| AccountMeta {
            address: Address::from_bytes([u8::try_from(index + 31).unwrap(); 32]),
            role: AccountRole::from_privileges(bits & 2 != 0, bits & 1 != 0),
        })
        .collect()
}

/// Exercise every alternate account privilege because the parser must reject escalation and loss.
fn verify(bytes: &[u8], bits: &[u8]) {
    let metas = accounts(bits);
    let value = AttestationServiceInstructionDataCodec
        .decode(bytes, LIMITS)
        .unwrap();
    assert_eq!(
        AttestationServiceInstructionDataCodec
            .encode(&value, BYTE_BUDGET)
            .unwrap(),
        bytes
    );
    let invocation = rebuild(&value, PROGRAM, &metas);
    assert_eq!(invocation.data, bytes);
    assert_eq!(invocation.accounts, metas);
    assert_eq!(parse_instruction(&invocation, LIMITS).unwrap(), value);
    malformed(&invocation);

    for (index, meta) in metas.iter().enumerate() {
        for role in [
            AccountRole::Readonly,
            AccountRole::Writable,
            AccountRole::ReadonlySigner,
            AccountRole::WritableSigner,
        ] {
            if role == meta.role {
                continue;
            }

            let mut changed = invocation.clone();
            changed.accounts[index].role = role;
            assert!(parse_instruction(&changed, LIMITS).is_err());
        }
    }

    let alternate = Address::from_bytes([91; 32]);
    let explicit = rebuild(&value, alternate, &metas);
    assert_eq!(explicit.program_address, alternate);
    assert_eq!(explicit.data, bytes);
    assert_eq!(explicit.accounts, metas);
    assert!(parse_instruction(&explicit, LIMITS).is_err());
    assert_eq!(
        parse_instruction_with_program(alternate, &explicit, LIMITS).unwrap(),
        value
    );
}

fn malformed(invocation: &Instruction) {
    for size in 0..invocation.data.len() {
        let mut changed = invocation.clone();
        changed.data.truncate(size);
        assert!(parse_instruction(&changed, LIMITS).is_err());
    }

    let mut changed = invocation.clone();
    changed.data.push(0);
    assert!(parse_instruction(&changed, LIMITS).is_err());
    changed = invocation.clone();
    changed.accounts.pop();
    assert!(parse_instruction(&changed, LIMITS).is_err());
    changed = invocation.clone();
    changed.accounts.push(AccountMeta {
        address: PROGRAM,
        role: AccountRole::Readonly,
    });

    assert!(parse_instruction(&changed, LIMITS).is_err());
    assert!(
        parse_instruction(
            invocation,
            Limits::new(invocation.data.len() - 1, ITEM_BUDGET)
        )
        .is_err()
    );
    assert!(
        parse_instruction(
            invocation,
            Limits::new(BYTE_BUDGET, invocation.accounts.len() - 1)
        )
        .is_err()
    );
}

// Both public builder paths must be exercised for each independently decoded operation.
macro_rules! build {
    ($value:expr, $program:expr, $metas:expr; $($name:ident),+) => {
        match $value { $(AttestationServiceInstructionData::$name(value) =>
            if $program == PROGRAM { value.instruction($metas, LIMITS).unwrap() }
            else { value.instruction_with_program($program, $metas, LIMITS).unwrap() }),+ }
    };
}

fn rebuild(
    value: &AttestationServiceInstructionData,
    program: Address,
    metas: &[AccountMeta],
) -> Instruction {
    build!(value, program, metas;
        CreateCredential,
        CreateSchema,
        ChangeSchemaStatus,
        ChangeAuthorizedSigners,
        ChangeSchemaDescription,
        ChangeSchemaVersion,
        CreateAttestation,
        CloseAttestation,
        TokenizeSchema,
        CreateTokenizedAttestation,
        CloseTokenizedAttestation,
        EmitEvent
    )
}

#[cfg_attr(test, test)]
pub fn create_credential() {
    let mut bytes = vec![0, 2, 0, 0, 0, 0xc3, 0xa9, 2, 0, 0, 0];
    bytes.extend([71; 32]);
    bytes.extend([72; 32]);
    verify(&bytes, &[3, 1, 2, 0]);
    assert_eq!(
        CreateCredentialCodec.decode(&bytes, LIMITS).unwrap().name,
        "é"
    );
    bytes[5] = 0xff;
    assert!(CreateCredentialCodec.decode(&bytes, LIMITS).is_err());
}

#[cfg_attr(test, test)]
pub fn create_schema() {
    let bytes = [
        1, 1, 0, 0, 0, b'n', 1, 0, 0, 0, b'd', 2, 0, 0, 0, 0, 25, 2, 0, 0, 0, 1, 0, 0, 0, b'a', 1,
        0, 0, 0, b'b',
    ];
    verify(&bytes, &[3, 2, 0, 1, 0]);
    let mut bad = bytes;
    bad[15] = 26;
    assert!(CreateSchemaCodec.decode(&bad, LIMITS).is_err());
}

#[cfg_attr(test, test)]
pub fn change_schema_status() {
    verify(&[2, 1], &[2, 0, 1]);
    verify(&[2, 0], &[2, 0, 1]);
    assert!(ChangeSchemaStatusCodec.decode(&[2, 2], LIMITS).is_err());
}

#[cfg_attr(test, test)]
pub fn change_authorized_signers() {
    let mut bytes = vec![3, 1, 0, 0, 0];
    bytes.extend([73; 32]);
    verify(&bytes, &[3, 2, 1, 0]);
    verify(&[3, 0, 0, 0, 0], &[3, 2, 1, 0]);
    bytes[1..5].copy_from_slice(&u32::MAX.to_le_bytes());
    assert!(ChangeAuthorizedSignersCodec.decode(&bytes, LIMITS).is_err());
}

#[cfg_attr(test, test)]
pub fn change_schema_description() {
    let bytes = [4, 2, 0, 0, 0, b'x', b'y'];
    verify(&bytes, &[3, 2, 0, 1, 0]);
}

#[cfg_attr(test, test)]
pub fn change_schema_version() {
    verify(
        &[5, 1, 0, 0, 0, 9, 1, 0, 0, 0, 1, 0, 0, 0, b'z'],
        &[3, 2, 0, 0, 1, 0],
    );
}

#[cfg_attr(test, test)]
pub fn create_attestation() {
    const NEGATIVE_EXPIRY: [u8; 8] = [0xfe, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff];
    let mut bytes = vec![6];
    bytes.extend([74; 32]);
    bytes.extend([3, 0, 0, 0, 0xff, 0x80, 9]);
    bytes.extend(NEGATIVE_EXPIRY);
    verify(&bytes, &[3, 2, 0, 0, 1, 0]);
    assert_eq!(
        CreateAttestationCodec
            .decode(&bytes, LIMITS)
            .unwrap()
            .expiry,
        -2
    );

    for expiry in [i64::MIN, 0, i64::MAX] {
        let value = CreateAttestation {
            nonce: Address::from_bytes([74; 32]),
            data: vec![0xff, 0x80, 9],
            expiry,
        };
        let encoded = CreateAttestationCodec.encode(&value, BYTE_BUDGET).unwrap();
        assert_eq!(&encoded[40..48], &expiry.to_le_bytes());
    }
}

#[cfg_attr(test, test)]
pub fn close_attestation() {
    verify(&[7], &[3, 2, 0, 1, 0, 0, 0]);
}

#[cfg_attr(test, test)]
pub fn tokenize_schema() {
    let bytes = [9, 1, 2, 3, 4, 5, 6, 7, 0x80];
    let roles = [3, 2, 0, 0, 1, 0, 0, 0];
    verify(&bytes, &roles);
    assert_eq!(
        TokenizeSchemaCodec.decode(&bytes, LIMITS).unwrap().max_size,
        0x8007_0605_0403_0201
    );
}

#[cfg_attr(test, test)]
pub fn create_tokenized_attestation() {
    let mut bytes = vec![10];
    bytes.extend([75; 32]);
    bytes.extend([1, 0, 0, 0, 0xff]);
    bytes.extend([1, 2, 3, 4, 5, 6, 7, 0]);
    bytes.extend([
        1, 0, 0, 0, b'n', 1, 0, 0, 0, b'u', 1, 0, 0, 0, b's', 0x34, 0x12,
    ]);
    let roles = [3, 2, 0, 0, 1, 0, 1, 1, 0, 1, 0, 0, 0];
    verify(&bytes, &roles);
    let value = CreateTokenizedAttestationCodec
        .decode(&bytes, LIMITS)
        .unwrap();

    assert_eq!(value.mint_account_space, 0x1234);

    assert_eq!(value.expiry, 0x0007_0605_0403_0201);
}

#[cfg_attr(test, test)]
pub fn close_tokenized_attestation() {
    let roles = [3, 2, 0, 1, 0, 0, 0, 1, 0, 1, 0];
    verify(&[11], &roles);
}

#[cfg_attr(test, test)]
pub fn emit_event() {
    verify(&[228], &[2]);
    assert!(identify_instruction(&[]).is_err());

    for tag in [8, 12, 200, 255] {
        assert!(identify_instruction(&[tag]).is_err());
    }
}
