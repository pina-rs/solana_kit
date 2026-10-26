//! Every allocated account and custom error uses independent nonzero wire values.
use solana_kit_attestation_service::*;
use solana_kit_core::{
    Address,
    accounts::{Account, AccountError, EncodedAccount},
    codecs::{Decoder, Encoder, Limits},
    program_error::{InstructionFailure, ProgramError},
};

const BYTE_BUDGET: usize = 4096;
const ITEM_BUDGET: usize = 2048;
const LIMITS: Limits = Limits::new(BYTE_BUDGET, ITEM_BUDGET);
const STRING_SCHEMA_TAG: u8 = 12;

fn snapshot(data: Vec<u8>) -> EncodedAccount {
    Account {
        address: Address::from_bytes([81; 32]),
        owner: PROGRAM,
        lamports: 91,
        executable: false,
        space: u64::try_from(data.len()).unwrap(),
        rent_epoch: Some(7),
        data,
    }
}

fn wire<T: core::fmt::Debug + PartialEq>(
    codec: &(impl Encoder<T> + Decoder<T>),
    bytes: &[u8],
    expected: &T,
) {
    assert_eq!(codec.encode(expected, BYTE_BUDGET).unwrap(), bytes);
    assert_eq!(&codec.decode(bytes, LIMITS).unwrap(), expected);

    for size in 0..bytes.len() {
        assert!(codec.decode(&bytes[..size], LIMITS).is_err());
    }

    let mut trailing = bytes.to_vec();
    trailing.push(0);
    assert!(codec.decode(&trailing, LIMITS).is_err());
    assert!(
        codec
            .decode(bytes, Limits::new(bytes.len() - 1, ITEM_BUDGET))
            .is_err()
    );
    assert!(codec.decode(bytes, Limits::new(BYTE_BUDGET, 0)).is_err());
}

fn ownership<T: core::fmt::Debug>(
    bytes: Vec<u8>,
    decode: impl Fn(EncodedAccount) -> Result<Account<T>, AccountError>,
) {
    let original = snapshot(bytes);
    let decoded = decode(original.clone()).unwrap();
    assert_eq!(decoded.address, original.address);
    assert_eq!(decoded.owner, original.owner);
    assert_eq!(decoded.lamports, 91);
    assert_eq!(decoded.space, original.space);
    assert_eq!(decoded.rent_epoch, Some(7));
    let mut wrong = original.clone();

    wrong.owner = Address::from_bytes([99; 32]);
    assert!(matches!(
        decode(wrong),
        Err(AccountError::OwnerMismatch { .. })
    ));
    wrong = original.clone();
    wrong.space += 1;
    assert!(matches!(
        decode(wrong),
        Err(AccountError::DataSizeMismatch(_))
    ));
    wrong = original;
    wrong.data[0] = 99;
    assert!(matches!(decode(wrong), Err(AccountError::Decode { .. })));
}

#[cfg_attr(test, test)]
pub fn credential() {
    let mut bytes = vec![0];
    bytes.extend([31; 32]);
    bytes.extend([2, 0, 0, 0, 0xff, 0x80, 2, 0, 0, 0]);
    bytes.extend([32; 32]);
    bytes.extend([33; 32]);
    let value = Credential {
        authority: Address::from_bytes([31; 32]),
        name: vec![0xff, 0x80],
        authorized_signers: vec![Address::from_bytes([32; 32]), Address::from_bytes([33; 32])],
    };
    wire(&CredentialCodec, &bytes, &value);
    ownership(bytes.clone(), |account| decode_credential(account, LIMITS));

    assert_eq!(identify_account(&bytes).unwrap(), AccountKind::Credential);
    let mut alternate = snapshot(bytes);
    alternate.owner = Address::from_bytes([90; 32]);
    assert_eq!(
        decode_credential_with_program(alternate.clone(), alternate.owner, LIMITS)
            .unwrap()
            .data,
        value
    );
}

#[cfg_attr(test, test)]
pub fn schema() {
    let mut bytes = vec![1];
    bytes.extend([34; 32]);
    bytes.extend([
        1, 0, 0, 0, b'n', 2, 0, 0, 0, 0xff, 0xfe, 2, 0, 0, 0, 12, 0, 10, 0, 0, 0, 1, 0, 0, 0, b'x',
        1, 0, 0, 0, b'y', 1, 255,
    ]);
    let value = Schema {
        credential: Address::from_bytes([34; 32]),
        name: vec![b'n'],
        description: vec![0xff, 0xfe],
        layout: vec![STRING_SCHEMA_TAG, 0],
        field_names: vec![1, 0, 0, 0, b'x', 1, 0, 0, 0, b'y'],
        is_paused: true,
        version: 255,
    };
    wire(&SchemaCodec, &bytes, &value);
    ownership(bytes.clone(), |account| decode_schema(account, LIMITS));
    assert_eq!(identify_account(&bytes).unwrap(), AccountKind::Schema);
    let mut alternate = snapshot(bytes.clone());

    alternate.owner = Address::from_bytes([90; 32]);
    assert_eq!(
        decode_schema_with_program(alternate.clone(), alternate.owner, LIMITS)
            .unwrap()
            .data,
        value
    );
    let boolean = bytes.len() - 2;
    bytes[boolean] = 2;
    assert!(SchemaCodec.decode(&bytes, LIMITS).is_err());
}

#[cfg_attr(test, test)]
pub fn attestation() {
    const NEGATIVE_EXPIRY: [u8; 8] = [0xfe, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff];
    let mut bytes = vec![2];
    bytes.extend([35; 32]);
    bytes.extend([36; 32]);
    bytes.extend([37; 32]);
    bytes.extend([2, 0, 0, 0, 0xff, 0x80]);
    bytes.extend([38; 32]);
    bytes.extend(NEGATIVE_EXPIRY);

    bytes.extend([39; 32]);

    let value = Attestation {
        nonce: Address::from_bytes([35; 32]),
        credential: Address::from_bytes([36; 32]),
        schema: Address::from_bytes([37; 32]),
        data: vec![0xff, 0x80],
        signer: Address::from_bytes([38; 32]),
        expiry: -2,
        token_account: Address::from_bytes([39; 32]),
    };
    wire(&AttestationCodec, &bytes, &value);
    ownership(bytes.clone(), |account| decode_attestation(account, LIMITS));
    assert_eq!(identify_account(&bytes).unwrap(), AccountKind::Attestation);
    let mut alternate = snapshot(bytes);
    alternate.owner = Address::from_bytes([90; 32]);
    assert_eq!(
        decode_attestation_with_program(alternate.clone(), alternate.owner, LIMITS)
            .unwrap()
            .data,
        value
    );
}

#[cfg_attr(test, test)]
pub fn close_event_and_all_errors() {
    let mut bytes = vec![7];
    bytes.extend([40; 32]);
    bytes.extend([2, 0, 0, 0, 9, 0xff]);
    let value = CloseAttestationEvent {
        discriminator: 7,
        schema: Address::from_bytes([40; 32]),
        attestation_data: vec![9, 0xff],
    };
    wire(&CloseAttestationEventCodec, &bytes, &value);
    known_errors();
    unknown_errors();
    unrelated_errors();

    assert!(identify_account(&[]).is_err());
    assert!(identify_account(&[3]).is_err());
}

fn known_errors() {
    let errors = [
        AttestationServiceError::InvalidCredential,
        AttestationServiceError::InvalidSchema,
        AttestationServiceError::InvalidAttestation,
        AttestationServiceError::InvalidAuthority,
        AttestationServiceError::InvalidSchemaDataType,
        AttestationServiceError::SignerNotAuthorized,
        AttestationServiceError::InvalidAttestationData,
        AttestationServiceError::InvalidEventAuthority,
        AttestationServiceError::InvalidMint,
        AttestationServiceError::InvalidProgramSigner,
        AttestationServiceError::InvalidTokenAccount,
        AttestationServiceError::SchemaPaused,
    ];

    for (code, expected) in errors.into_iter().enumerate() {
        let code = u32::try_from(code).unwrap();
        error_source(code, expected);
    }
}

fn error_source(code: u32, expected: AttestationServiceError) {
    assert_eq!(AttestationServiceError::from_code(code), expected);
    assert_eq!(expected.code(), code);
    let failure = InstructionFailure {
        index: 1,
        error: ProgramError::Custom(code),
    };
    assert!(is_attestation_service_error(
        &failure,
        &[Address::from_bytes([1; 32]), PROGRAM],
        expected
    ));
    assert!(!is_attestation_service_error(
        &failure,
        &[PROGRAM],
        expected
    ));
    assert!(!is_attestation_service_error(
        &failure,
        &[PROGRAM, Address::from_bytes([1; 32])],
        expected
    ));

    assert!(!is_attestation_service_error(
        &InstructionFailure {
            index: 0,
            error: ProgramError::Custom(code + 1)
        },
        &[PROGRAM],
        expected
    ));
}

fn unknown_errors() {
    const FIRST_UNKNOWN: u32 = 12;
    const UNRELATED_CODE: u32 = 99;

    for code in [FIRST_UNKNOWN, UNRELATED_CODE, u32::MAX] {
        assert_eq!(
            AttestationServiceError::from_code(code),
            AttestationServiceError::Unknown(code)
        );
        assert_eq!(AttestationServiceError::from_code(code).code(), code);
        assert_eq!(AttestationServiceError::from_code(code).message(), None);
    }
}

fn unrelated_errors() {
    for error in [
        ProgramError::Unknown("unrelated".into()),
        ProgramError::Known(solana_kit_core::program_error::KnownProgramError::InvalidArgument),
    ] {
        assert!(!is_attestation_service_error(
            &InstructionFailure { index: 0, error },
            &[PROGRAM],
            AttestationServiceError::SchemaPaused
        ));
    }
}
