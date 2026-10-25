use solana_kit_config::*;
use solana_kit_core::{
    AccountRole, Address,
    codecs::{Decoder, Encoder, Error, Limits},
};

const LIMITS: Limits = Limits::new(3_000_000, 100_000);
const ZERO: Address = Address::from_bytes([0; 32]);

#[cfg_attr(test, test)]
pub fn literal_keys_data_and_roles() {
    assert_eq!(
        PROGRAM.to_string(),
        "Config1111111111111111111111111111111111111"
    );
    let value = ConfigData {
        keys: vec![
            ConfigKey {
                address: ZERO,
                is_signer: true,
            },
            ConfigKey {
                address: ZERO,
                is_signer: false,
            },
        ],
        data: vec![9, 8, 7],
    };
    let mut expected = vec![2];
    expected.extend([0; 32]);
    expected.push(1);
    expected.extend([0; 33]);
    expected.extend([9, 8, 7]);
    assert_eq!(
        ConfigCodec.encode(&value, LIMITS.max_bytes).unwrap(),
        expected
    );

    assert_eq!(ConfigCodec.decode(&expected, LIMITS).unwrap(), value);
    let instruction = store_config(PROGRAM, ZERO, true, &value, LIMITS).unwrap();
    assert_eq!(instruction.data, expected);
    assert_eq!(instruction.accounts.len(), 2);
    assert_eq!(instruction.accounts[0].role, AccountRole::WritableSigner);
    assert_eq!(instruction.accounts[1].role, AccountRole::ReadonlySigner);
    assert_eq!(instruction.accounts[1].address, ZERO);
    assert_eq!(parse_store(&instruction, PROGRAM, LIMITS).unwrap(), value);
    assert_eq!(
        parse_store(&instruction, ZERO, LIMITS),
        Err(Error::InvalidValue)
    );
}

#[cfg_attr(test, test)]
pub fn compact_boundaries_and_failures() {
    for (count, prefix) in [
        (0, vec![0]),
        (127, vec![127]),
        (128, vec![128, 1]),
        (16383, vec![255, 127]),
        (16384, vec![128, 128, 1]),
        (65535, vec![255, 255, 3]),
    ] {
        let value = ConfigData {
            keys: vec![
                ConfigKey {
                    address: ZERO,
                    is_signer: false
                };
                count
            ],
            data: vec![],
        };
        let encoded = ConfigCodec.encode(&value, LIMITS.max_bytes).unwrap();
        assert!(encoded.starts_with(&prefix));
        assert_eq!(ConfigCodec.decode(&encoded, LIMITS).unwrap(), value);
    }

    for malformed in [&[128, 0][..], &[128][..], &[255, 255, 4][..], &[1, 0][..]] {
        assert!(ConfigCodec.decode(malformed, LIMITS).is_err());
    }

    let mut bad_bool = vec![1];
    bad_bool.extend([0; 32]);
    bad_bool.push(2);
    assert_eq!(
        ConfigCodec.decode(&bad_bool, LIMITS),
        Err(Error::InvalidBoolean { value: 2 })
    );
    assert_eq!(
        ConfigCodec.decode(&[0, 7], Limits::new(1, 100)),
        Err(Error::LimitExceeded)
    );
    assert_eq!(
        ConfigCodec.decode(&[0, 7], Limits::new(2, 0)),
        Err(Error::LimitExceeded)
    );
}

#[cfg_attr(test, test)]
pub fn signer_duplicates_and_privilege_negatives() {
    let value = ConfigData {
        keys: vec![
            ConfigKey {
                address: ZERO,
                is_signer: true
            };
            2
        ],
        data: vec![],
    };
    let mut instruction = store_config(ZERO, ZERO, false, &value, LIMITS).unwrap();
    assert_eq!(instruction.accounts.len(), 3);
    assert_eq!(parse_store(&instruction, ZERO, LIMITS).unwrap(), value);
    instruction.accounts[1].role = AccountRole::WritableSigner;
    assert!(parse_store(&instruction, ZERO, LIMITS).is_ok());
    instruction.accounts[1].role = AccountRole::Readonly;
    assert!(parse_store(&instruction, ZERO, LIMITS).is_err());

    instruction.accounts.clear();
    assert!(parse_store(&instruction, ZERO, LIMITS).is_err());
    let large = ConfigData {
        keys: vec![
            ConfigKey {
                address: ZERO,
                is_signer: false
            };
            65536
        ],
        data: vec![],
    };
    assert_eq!(ConfigCodec.encoded_len(&large), Err(Error::Overflow));
}
