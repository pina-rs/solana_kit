//! Generic Rust values cover constructors without cloning Dart object wrappers.
use solana_kit_core::{Address, accounts::Account};
use std::hash::{Hash, Hasher};

fn hash<T: Hash>(value: &T) -> u64 {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    value.hash(&mut hasher);
    hasher.finish()
}

#[test]
fn arbitrary_typed_data_and_optional_metadata_construct_compare_hash_and_format() {
    const DATA_VALUE: u32 = 7;
    let absent: (u32, Option<(String, Option<String>)>) = (DATA_VALUE, None);
    let program_only = (DATA_VALUE, Some((String::from("future"), None::<String>)));
    let with_type = (
        DATA_VALUE,
        Some((String::from("future"), Some(String::from("custom")))),
    );
    let address = Address::from_bytes([0; 32]);
    let account = Account {
        address,
        owner: address,
        lamports: 1,
        executable: false,
        space: 3,
        rent_epoch: None,
        data: absent.clone(),
    };

    assert_eq!(account.data.0, DATA_VALUE);
    assert!(account.data.1.is_none());
    assert_eq!(hash(&absent), hash(&absent.clone()));

    assert_eq!(program_only.1.as_ref().unwrap().0, "future");
    assert_eq!(program_only.1.as_ref().unwrap().1, None);

    assert_eq!(with_type.1.as_ref().unwrap().1.as_deref(), Some("custom"));
    assert_ne!(program_only, with_type);
    assert_eq!(hash(&with_type), hash(&with_type.clone()));
    assert!(format!("{with_type:?}").contains("custom"));
    let header = (
        account.executable,
        account.lamports,
        account.owner,
        account.space,
    );
    let equal_header = (false, 1_u64, address, 3_u64);
    assert_eq!(header, equal_header);
    assert_eq!(hash(&header), hash(&equal_header));
    assert!(format!("{header:?}").contains("false"));
}
