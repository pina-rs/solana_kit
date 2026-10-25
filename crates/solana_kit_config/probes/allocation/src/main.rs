mod allocation;

use solana_kit_config::{
    ConfigCodec, ConfigData, ConfigKey, PROGRAM, StoreAccounts, store, store_config,
};
use solana_kit_core::{
    Address,
    codecs::{Encoder, Error, Limits, Writer},
};
use std::hint::black_box;

const ADDRESS: Address = Address::from_bytes([17; 32]);
const KEY_COUNT: usize = 4096;
const DATA_SIZE: usize = 2 * 1024 * 1024;
const WIRE_SIZE: usize = 2 + KEY_COUNT * 33 + DATA_SIZE;
const KEY_WIRE_BYTES: usize = 33;
const ADDRESS_BYTES: usize = 32;

fn main() {
    let operation = std::env::args()
        .nth(1)
        .expect("supply one public operation");
    let value = ConfigData {
        keys: vec![
            ConfigKey {
                address: ADDRESS,
                is_signer: true,
            };
            KEY_COUNT
        ],
        data: vec![255; DATA_SIZE],
    };
    let limits = Limits::new(1, usize::MAX);

    match operation.as_str() {
        "length" => length(&value, &operation),
        "write" => write(&value, &operation),
        "encode-reject" => encode_reject(&value, &operation),
        "store-reject" => store_reject(&value, &operation, limits),
        "derived-byte-reject" | "derived-item-reject" => derived_reject(&value, &operation, limits),
        _ => panic!("unknown public operation"),
    }
}

fn length(value: &ConfigData, operation: &str) {
    let (result, requested) = allocation::measure(|| ConfigCodec.encoded_len(black_box(value)));
    assert_eq!(result, Ok(WIRE_SIZE));
    requested.assert_bounded(operation);
}

fn write(value: &ConfigData, operation: &str) {
    let mut output = vec![0; WIRE_SIZE];
    let mut writer = Writer::new(&mut output);
    let (result, requested) =
        allocation::measure(|| ConfigCodec.write(black_box(value), &mut writer));
    assert_eq!(result, Ok(()));
    assert_eq!(writer.position(), WIRE_SIZE);
    assert_eq!(&output[..2], &[128, 32]);

    for key in output[2..WIRE_SIZE - DATA_SIZE].chunks_exact(KEY_WIRE_BYTES) {
        assert_eq!(&key[..ADDRESS_BYTES], &[17; 32]);
        assert_eq!(key[ADDRESS_BYTES], 1);
    }

    assert_eq!(&output[WIRE_SIZE - DATA_SIZE..], value.data.as_slice());
    requested.assert_bounded(operation);
}

fn encode_reject(value: &ConfigData, operation: &str) {
    let (result, requested) = allocation::measure(|| ConfigCodec.encode(black_box(value), 1));
    assert_eq!(result, Err(Error::LimitExceeded));
    requested.assert_bounded(operation);
}

fn store_reject(value: &ConfigData, operation: &str, limits: Limits) {
    let accounts = StoreAccounts {
        config: ADDRESS,
        config_is_signer: true,
        signers: &[ADDRESS],
    };
    let (result, requested) =
        allocation::measure(|| store(PROGRAM, accounts, black_box(value), limits));
    assert_eq!(result, Err(Error::LimitExceeded));
    requested.assert_bounded(operation);
}

fn derived_reject(value: &ConfigData, operation: &str, limits: Limits) {
    let limits = if operation == "derived-item-reject" {
        Limits::new(usize::MAX, KEY_COUNT + DATA_SIZE)
    } else {
        limits
    };

    let (result, requested) =
        allocation::measure(|| store_config(PROGRAM, ADDRESS, true, black_box(value), limits));
    assert_eq!(result, Err(Error::LimitExceeded));
    requested.assert_bounded(operation);
}
