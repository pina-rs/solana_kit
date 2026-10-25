use solana_kit_config::{
    ConfigCodec, ConfigData, ConfigKey, PROGRAM, StoreAccounts, generated, parse_store, store,
    store_config,
};
use solana_kit_core::{
    AccountRole, Address,
    codecs::{Decoder, Encoder, Error, Limits, Writer},
};

const CONFIG: Address = Address::from_bytes([17; 32]);
const OTHER: Address = Address::from_bytes([42; 32]);
const LIMITS: Limits = Limits::new(4096, 4096);

fn sample() -> ConfigData {
    ConfigData {
        keys: vec![
            ConfigKey {
                address: CONFIG,
                is_signer: true,
            },
            ConfigKey {
                address: OTHER,
                is_signer: false,
            },
            ConfigKey {
                address: CONFIG,
                is_signer: true,
            },
        ],
        data: vec![0, 255, 128, 0],
    }
}

#[cfg_attr(test, test)]
pub fn independent_empty_and_nonempty_wire_roundtrips() {
    for data in [vec![], vec![0, 255, 128, 0]] {
        let value = ConfigData {
            keys: vec![],
            data: data.clone(),
        };
        let mut expected = vec![0];
        expected.extend(data);
        assert_eq!(ConfigCodec.encoded_len(&value), Ok(expected.len()));
        assert_eq!(
            ConfigCodec.encode(&value, expected.len()).unwrap(),
            expected
        );
        assert_eq!(ConfigCodec.decode(&expected, LIMITS).unwrap(), value);
    }

    let value = sample();
    let mut expected = vec![3];
    expected.extend([17; 32]);
    expected.push(1);
    expected.extend([42; 32]);
    expected.push(0);
    expected.extend([17; 32]);
    expected.push(1);

    expected.extend([0, 255, 128, 0]);
    assert_eq!(ConfigCodec.encoded_len(&value), Ok(expected.len()));
    assert_eq!(
        ConfigCodec.encode(&value, expected.len()).unwrap(),
        expected
    );
    assert_eq!(ConfigCodec.decode(&expected, LIMITS).unwrap(), value);
}

#[cfg_attr(test, test)]
pub fn both_store_helpers_retain_order_roles_and_program() {
    let value = sample();
    let explicit = store(
        OTHER,
        StoreAccounts {
            config: CONFIG,
            config_is_signer: false,
            signers: &[OTHER, CONFIG, OTHER],
        },
        &value,
        LIMITS,
    )
    .unwrap();
    let derived = store_config(OTHER, CONFIG, true, &value, LIMITS).unwrap();
    assert_eq!(explicit.program_address, OTHER);
    assert_eq!(explicit.accounts[0].address, CONFIG);
    assert_eq!(explicit.accounts[0].role, AccountRole::Writable);
    assert_eq!(derived.accounts[0].role, AccountRole::WritableSigner);
    assert_eq!(
        explicit.accounts[1..]
            .iter()
            .map(|meta| meta.address)
            .collect::<Vec<_>>(),
        [OTHER, CONFIG, OTHER]
    );

    assert_eq!(
        derived.accounts[1..]
            .iter()
            .map(|meta| meta.address)
            .collect::<Vec<_>>(),
        [CONFIG, CONFIG]
    );
    assert!(
        explicit.accounts[1..]
            .iter()
            .all(|meta| meta.role == AccountRole::ReadonlySigner)
    );
    assert_eq!(explicit.data, derived.data);
    assert_eq!(parse_store(&explicit, OTHER, LIMITS).unwrap(), value);
    assert_eq!(parse_store(&derived, OTHER, LIMITS).unwrap(), value);

    assert_small_stores();
}

fn assert_small_stores() {
    let empty = ConfigData {
        keys: vec![],
        data: vec![],
    };
    let instruction = store_config(PROGRAM, CONFIG, false, &empty, Limits::new(1, 1)).unwrap();
    assert_eq!(instruction.data, [0]);
    assert_eq!(instruction.accounts.len(), 1);
    assert_eq!(
        parse_store(&instruction, PROGRAM, Limits::new(1, 1)).unwrap(),
        empty
    );

    let unsigned = ConfigData {
        keys: vec![ConfigKey {
            address: OTHER,
            is_signer: false,
        }],
        data: vec![255],
    };
    let instruction = store_config(PROGRAM, CONFIG, false, &unsigned, LIMITS).unwrap();
    assert_eq!(instruction.accounts.len(), 1);
    assert_eq!(
        parse_store(&instruction, PROGRAM, LIMITS).unwrap(),
        unsigned
    );
}

#[cfg_attr(test, test)]
pub fn writer_preserves_emitted_bytes_cursor_and_truncation() {
    let value = sample();
    let emitted = generated::Config {
        keys: generated::ConfigKeys {
            value: value
                .keys
                .iter()
                .map(|key| (key.address, key.is_signer))
                .collect(),
        },
        data: value.data.clone(),
    };
    let size = ConfigCodec.encoded_len(&value).unwrap();

    for capacity in 0..=size {
        let mut actual = vec![77; capacity + 3];
        let mut expected = actual.clone();
        let mut actual_writer = Writer::new(&mut actual);
        let mut expected_writer = Writer::new(&mut expected);
        actual_writer.seek(3).unwrap();
        expected_writer.seek(3).unwrap();
        let result = ConfigCodec.write(&value, &mut actual_writer);

        let emitted_result = generated::ConfigCodec.write(&emitted, &mut expected_writer);
        assert_eq!(result, emitted_result);
        assert_eq!(actual_writer.position(), expected_writer.position());
        assert_eq!(actual_writer.bytes(), expected_writer.bytes());
    }
}

#[cfg_attr(test, test)]
pub fn malformed_input_keeps_exact_diagnostics() {
    for (bytes, error) in [
        (&[][..], Error::Truncated),
        (&[128][..], Error::Truncated),
        (&[128, 0][..], Error::NonCanonical),
        (&[255, 255, 4][..], Error::Overflow),
        (&[1, 0][..], Error::Truncated),
    ] {
        assert_eq!(ConfigCodec.decode(bytes, LIMITS), Err(error));
    }

    let mut bytes = vec![1];
    bytes.extend([17; 32]);
    bytes.push(255);
    assert_eq!(
        ConfigCodec.decode(&bytes, LIMITS),
        Err(Error::InvalidBoolean { value: 255 })
    );
}

#[cfg_attr(test, test)]
pub fn caller_budgets_and_unrepresentable_counts_keep_error_precedence() {
    let value = sample();
    let size = ConfigCodec.encoded_len(&value).unwrap();

    for limits in [Limits::new(size - 1, 10), Limits::new(size, 9)] {
        assert_eq!(
            store_config(PROGRAM, CONFIG, true, &value, limits),
            Err(Error::LimitExceeded)
        );
        assert_eq!(
            store(
                PROGRAM,
                StoreAccounts {
                    config: CONFIG,
                    config_is_signer: true,
                    signers: &[CONFIG, CONFIG],
                },
                &value,
                limits,
            ),
            Err(Error::LimitExceeded)
        );
    }

    assert!(store_config(PROGRAM, CONFIG, true, &value, Limits::new(size, 10)).is_ok());
    assert_overflowing_count();
}

fn assert_overflowing_count() {
    const UNREPRESENTABLE_KEYS: usize = 65_536;
    const WRITER_CAPACITY: usize = 8;
    let large = ConfigData {
        keys: vec![
            ConfigKey {
                address: CONFIG,
                is_signer: true,
            };
            UNREPRESENTABLE_KEYS
        ],
        data: vec![],
    };
    assert_eq!(ConfigCodec.encoded_len(&large), Err(Error::Overflow));
    assert_eq!(ConfigCodec.encode(&large, 0), Err(Error::Overflow));
    let mut bytes = [77; WRITER_CAPACITY];
    let mut writer = Writer::new(&mut bytes);
    assert_eq!(ConfigCodec.write(&large, &mut writer), Err(Error::Overflow));

    assert_eq!(writer.position(), 0);

    assert_eq!(writer.bytes(), &[77; WRITER_CAPACITY]);

    for (limits, error) in [
        (Limits::new(0, 0), Error::LimitExceeded),
        (Limits::new(0, usize::MAX), Error::Overflow),
    ] {
        assert_eq!(
            store_config(PROGRAM, CONFIG, false, &large, limits),
            Err(error)
        );
        assert_eq!(
            store(
                PROGRAM,
                StoreAccounts {
                    config: CONFIG,
                    config_is_signer: false,
                    signers: &[],
                },
                &large,
                limits,
            ),
            Err(error)
        );
    }
}
