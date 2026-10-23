//! Distinct failing account identities collapse to the same public error fact.
use solana_kit_client::{Error, accounts::assert_account_decoded, methods::accounts::AccountData};
use solana_kit_core::{Address, accounts::Account};

pub fn decoded_assertion_preserves_failing_account_address() {
    for address in [Address::from_bytes([1; 32]), Address::from_bytes([2; 32])] {
        let account = Account {
            address,
            owner: address,
            lamports: 0,
            executable: false,
            space: 0,
            rent_epoch: None,
            data: AccountData::Bytes {
                bytes: vec![],
                slice: None,
            },
        };
        let error = assert_account_decoded(&account).unwrap_err();

        assert!(matches!(error, Error::AccountNotDecoded(actual) if actual == address));
        println!("address {address} preserved in {error:?}");
    }
}
