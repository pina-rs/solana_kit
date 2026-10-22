//! Injected observations are emitted only after the real System decoder accepts.
use solana_kit_client::{
    Presence,
    confirmation::Event,
    methods::{Context, Contextual},
};
use solana_kit_core::{Address, Hash, accounts::EncodedAccount, codecs::Limits};
use solana_kit_system::decode_nonce;
const NONCE_SIZE: usize = 80;
const TOKEN_START: usize = 40;
const TOKEN_END: usize = 72;

fn snapshot() -> Contextual<EncodedAccount> {
    let mut bytes = vec![0; NONCE_SIZE];
    bytes[0] = 1;
    bytes[4] = 1;
    bytes[TOKEN_START..TOKEN_END].fill(2);
    Contextual {
        context: Context {
            slot: u64::MAX,
            api_version: Presence::Missing,
        },
        value: EncodedAccount {
            address: Address::from_bytes([1; 32]),
            owner: Address::from_bytes([0; 32]),
            lamports: 1,
            executable: false,
            space: NONCE_SIZE as u64,
            rent_epoch: None,
            data: bytes,
        },
    }
}
fn validate(snapshot: Contextual<EncodedAccount>) -> Result<Event, &'static str> {
    if snapshot.value.executable {
        return Err("executable nonce account");
    }

    let account = decode_nonce(snapshot.value, Limits::new(NONCE_SIZE, NONCE_SIZE))
        .map_err(|_| "invalid nonce owner or full layout")?;
    let data = account
        .data
        .durable_data()
        .ok_or("not current initialized nonce state")?;
    Ok(Event::Nonce {
        address: account.address,
        slot: snapshot.context.slot,
        token: Some(data.blockhash),
    })
}

#[test]
fn current_initialized_owned_snapshot_preserves_exact_hash_and_context() {
    let Event::Nonce {
        address,
        slot,
        token,
    } = validate(snapshot()).unwrap()
    else {
        panic!("nonce event")
    };
    assert_eq!(address, Address::from_bytes([1; 32]));
    assert_eq!(slot, u64::MAX);
    assert_eq!(token, Some(Hash::from_bytes([2; 32])));
}

#[test]
fn wrong_owner_layout_legacy_uninitialized_and_executable_are_rejected() {
    let mut wrong_owner = snapshot();
    wrong_owner.value.owner = Address::from_bytes([9; 32]);
    assert!(validate(wrong_owner).is_err());
    let mut short = snapshot();
    short.value.data.pop();
    assert!(validate(short).is_err());
    let mut trailing = snapshot();
    trailing.value.data.push(0);
    assert!(validate(trailing).is_err());
    let mut legacy = snapshot();
    legacy.value.data[0] = 0;
    assert!(validate(legacy).is_err());
    let mut uninitialized = snapshot();
    uninitialized.value.data[4] = 0;
    assert!(validate(uninitialized).is_err());
    let mut invalid = snapshot();
    invalid.value.data[4] = 2;
    assert!(validate(invalid).is_err());
    let mut executable = snapshot();
    executable.value.executable = true;
    assert!(validate(executable).is_err());
}
