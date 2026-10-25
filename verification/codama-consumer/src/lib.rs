extern crate alloc;

#[path = "../generated/counter.rs"]
pub mod counter;
#[path = "../generated/empty_pda_contracts.rs"]
pub mod empty_pda_contracts;
#[path = "../generated/literal_contracts.rs"]
pub mod literal_contracts;
#[path = "../generated/record_contracts.rs"]
pub mod record_contracts;
#[path = "../generated/resolver_contracts.rs"]
pub mod resolver_contracts;

mod contracts;

use counter::{SetValue, SetValueCodec};
use solana_kit_core::codecs::{Decoder, Encoder, Error, Limits, Number, PairError, Paired, Size};
use solana_kit_core::crypto::{Keypair, KeypairImportError, verify};

const VALUE: u64 = 0x0102_0304_0506_0708;
const WIRE: [u8; 9] = [7, 8, 7, 6, 5, 4, 3, 2, 1];
const LIMITS: Limits = Limits::new(128, 128);

/// Number of assertion groups shared by every generated-client host.
pub const CONTRACT_GROUPS: usize = 8;

fn literal_wire() {
    let value = SetValue { value: VALUE };
    let codec = SetValueCodec;
    let bytes = codec.encode(&value, WIRE.len()).unwrap();
    assert_eq!(bytes, WIRE);
    assert_eq!(codec.decode(&WIRE, LIMITS).unwrap().value, VALUE);
    assert_eq!(codec.encode(&value, 8), Err(Error::LimitExceeded));

    let invocation = value.instruction(LIMITS).unwrap();
    assert_eq!(invocation.program_address, counter::PROGRAM);
    assert_eq!(invocation.data, WIRE);
    assert!(invocation.accounts.is_empty());
    assert_eq!(SetValue::parse(&invocation, LIMITS).unwrap().value, VALUE);

    let mut trailing = WIRE.to_vec();
    trailing.push(0);
    assert!(matches!(
        codec.decode(&trailing, LIMITS),
        Err(Error::TrailingBytes)
    ));
    assert!(codec.decode(&WIRE[..8], LIMITS).is_err());
    let mut wrong_tag = WIRE;
    wrong_tag[0] = 8;
    assert!(codec.decode(&wrong_tag, LIMITS).is_err());
}

fn metadata_composition() {
    let codec = SetValueCodec;
    assert_eq!(codec.encode_size().unwrap(), Size::Unknown);
    assert_eq!(codec.decode_size().unwrap(), Size::Unknown);
    let unclassified = Paired::<_, _, SetValue, SetValue>::try_new(&codec, &codec);
    assert!(matches!(unclassified, Err(PairError::UnknownSize)));

    let number = Number::<u64>::LE;
    let paired = Paired::<_, _, u64, u64>::try_new(&number, &number).unwrap();
    assert_eq!(paired.encode_size().unwrap(), Size::Fixed(8));
    assert_eq!(paired.encode(&VALUE, 8).unwrap(), WIRE[1..]);
    assert_eq!(paired.decode(&WIRE[1..], LIMITS).unwrap(), VALUE);
}

fn imported_signer() {
    let seed = [7; 32];
    let expected = Keypair::from_seed(&seed);
    let mut input = [0; 64];
    input[..32].copy_from_slice(&seed);
    input[32..].copy_from_slice(expected.address().as_bytes());
    let unchanged = input;
    let keypair = Keypair::from_bytes(&input).unwrap();

    let signature = keypair.sign(&WIRE);
    verify(&keypair.address(), &WIRE, &signature).unwrap();
    assert_eq!(input, unchanged);

    let mut changed = WIRE;
    changed[1] ^= 1;
    assert!(verify(&keypair.address(), &changed, &signature).is_err());
    assert!(matches!(
        Keypair::from_bytes(&seed),
        Err(KeypairImportError::InvalidLength { actual: 32 })
    ));
    input[63] ^= 1;
    assert!(matches!(
        Keypair::from_bytes(&input),
        Err(KeypairImportError::PublicKeyMismatch)
    ));
}

pub fn run() -> usize {
    literal_wire();
    metadata_composition();
    imported_signer();
    contracts::run();

    CONTRACT_GROUPS
}

#[cfg(all(test, target_arch = "wasm32", target_os = "unknown"))]
mod browser {
    use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn packed_generated_client_contracts() {
        assert_eq!(super::run(), super::CONTRACT_GROUPS);
    }
}
