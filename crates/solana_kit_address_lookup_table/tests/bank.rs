use solana_kit_address_lookup_table::*;
use solana_kit_core::{
    Address,
    accounts::{Account, AccountError},
    codecs::{Encoder, Error, Limits},
};

const LIMITS: Limits = Limits::new(4096, 4096);
const ADDRESS: Address = Address::from_bytes([7; 32]);
const EXTENSION_SLOT: u64 = 42;
const DEACTIVATION_SLOT: u64 = 45;
const HISTORY_OVER_LIMIT: usize = 513;

fn table() -> AddressLookupTable {
    AddressLookupTable {
        deactivation_slot: u64::MAX,
        last_extended_slot: EXTENSION_SLOT,
        last_extended_slot_start_index: 1,
        authority: Some(ADDRESS),
        padding: 0,
        addresses: vec![ADDRESS, Address::from_bytes([8; 32])],
    }
}

/// Activation and cool-down depend on the exact caller-supplied bank snapshot.
#[cfg_attr(test, test)]
pub fn activation_and_deactivation_use_bounded_bank_history() {
    let mut value = table();
    assert_eq!(
        value.active_addresses(EXTENSION_SLOT, &[]).unwrap(),
        &[ADDRESS]
    );
    assert_eq!(
        value.active_addresses(EXTENSION_SLOT + 1, &[]).unwrap(),
        value.addresses
    );

    value.deactivation_slot = DEACTIVATION_SLOT;
    assert!(value.active_addresses(DEACTIVATION_SLOT, &[]).is_ok());
    assert!(value.active_addresses(46, &[45, 44]).is_ok());
    assert_eq!(value.active_addresses(46, &[44]), Err(Error::InvalidValue));
    assert_eq!(
        value.active_addresses(46, &[45; HISTORY_OVER_LIMIT]),
        Err(Error::LimitExceeded)
    );
    value.deactivation_slot = u64::MAX;
    value.last_extended_slot_start_index = 3;
    assert_eq!(value.active_addresses(42, &[]), Err(Error::InvalidValue));
}

/// Account conversion keeps every metadata field and the strict expected owner.
#[cfg_attr(test, test)]
pub fn fetched_lookup_account_preserves_metadata() {
    let value = table();
    let data = AddressLookupTableCodec.encode(&value, 4096).unwrap();
    let account = Account {
        address: ADDRESS,
        owner: PROGRAM,
        lamports: u64::MAX,
        executable: false,
        rent_epoch: Some(u64::MAX),
        space: data.len() as u64,
        data,
    };
    let decoded = decode_account(account.clone(), LIMITS).unwrap();
    assert_eq!(decoded.data, value);
    assert_eq!(decoded.address, account.address);
    assert_eq!(decoded.owner, account.owner);
    assert_eq!(decoded.lamports, account.lamports);
    assert_eq!(decoded.space, account.space);
    assert_eq!(decoded.rent_epoch, account.rent_epoch);
    assert_eq!(decoded.executable, account.executable);

    let mut foreign = account;
    foreign.owner = ADDRESS;
    assert!(matches!(
        decode_account(foreign, LIMITS),
        Err(AccountError::OwnerMismatch { .. })
    ));
}
