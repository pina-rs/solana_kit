//! Independent program limits, display, edition bits and payload ownership cases.
use solana_kit_core::codecs::{Error, Limits};
use solana_kit_mpl_token_metadata::*;

#[cfg_attr(test, test)]
pub fn string_limits_and_puffed_view() {
    let mut data = Data {
        name: "é".repeat(16),
        symbol: "S".repeat(10),
        uri: "U".repeat(200),
        seller_fee_basis_points: 0,
        creators: None,
    };
    assert!(validate_metadata_strings(&data).is_ok());
    data.name.push('x');
    assert_eq!(validate_metadata_strings(&data), Err(Error::LimitExceeded));
    data.name.clear();
    data.symbol.push('x');
    assert!(validate_metadata_strings(&data).is_err());
    data.symbol.clear();

    data.uri.push('x');
    assert!(validate_metadata_strings(&data).is_err());
    assert_eq!(unpuffed("A\0B\0\0"), "A\0B");
    assert_eq!(unpuffed("\0\0"), "");
}

#[cfg_attr(test, test)]
pub fn edition_bitmap_is_most_significant_bit_first() {
    let mut marker = EditionMarker { ledger: [0; 31] };
    marker.ledger[0] = 0x80;
    marker.ledger[30] = 1;
    assert!(edition_is_marked(&marker, 0));
    assert!(!edition_is_marked(&marker, 1));
    assert!(edition_is_marked(&marker, 247));
    assert!(edition_is_marked(&marker, 248));
    assert!(!edition_is_marked(&marker, 249));

    marker.ledger = [0; 31];
    marker.ledger[1] = 1;
    assert!(edition_is_marked(&marker, u64::MAX));
}

#[cfg_attr(test, test)]
pub fn payload_keys_preserve_order_and_reject_duplicates() {
    const FIRST_NUMBER: u64 = 7;
    const SECOND_NUMBER: u64 = 9;
    let first = ("B".into(), PayloadType::Number(FIRST_NUMBER));
    let second = ("A".into(), PayloadType::Number(SECOND_NUMBER));
    let payload =
        authorization_payload(vec![first.clone(), second.clone()], Limits::new(2, 2)).unwrap();
    assert_eq!(payload.map, [first.clone(), second]);
    assert!(authorization_payload(vec![first.clone(), first.clone()], Limits::new(9, 9)).is_err());

    assert!(authorization_payload(vec![first.clone()], Limits::new(9, 0)).is_err());
    assert!(authorization_payload(vec![first], Limits::new(0, 9)).is_err());
}
