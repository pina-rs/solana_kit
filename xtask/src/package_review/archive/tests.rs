//! Exercises the actual raw-header boundary without relying on an extractor.

use super::{BLOCK, entries, member_path};

/// Builds one ordinary USTAR member with a valid header checksum.
fn fixture(kind: u8) -> Vec<u8> {
    let mut bytes = vec![0; 4 * BLOCK];
    bytes[..15].copy_from_slice(b"core-0.1/lib.rs");
    bytes[124..136].copy_from_slice(b"00000000001\0");
    bytes[156] = kind;
    bytes[257..263].copy_from_slice(b"ustar\0");
    bytes[263..265].copy_from_slice(b"00");
    bytes[148..156].fill(b' ');
    let checksum: u64 = bytes[..BLOCK].iter().map(|byte| u64::from(*byte)).sum();
    bytes[148..156].copy_from_slice(format!("{checksum:06o}\0 ").as_bytes());
    bytes[BLOCK] = b'x';
    bytes
}

#[test]
fn accepts_actual_cargo_gnu_layout() {
    let mut bytes = fixture(b'0');
    bytes[257..265].copy_from_slice(b"ustar  \0");
    bytes[148..156].fill(b' ');
    let checksum: u64 = bytes[..BLOCK].iter().map(|byte| u64::from(*byte)).sum();
    bytes[148..156].copy_from_slice(format!("{checksum:06o}\0 ").as_bytes());
    assert_eq!(entries(&bytes).unwrap().len(), 1);
    bytes[345] = 1;
    assert!(entries(&bytes).is_err());
}

#[test]
fn reads_actual_member_bytes_and_rejects_links_and_extensions() {
    let bytes = fixture(b'0');
    let members = entries(&bytes).unwrap();
    assert_eq!(members.len(), 1);
    assert_eq!(members[0], ("core-0.1/lib.rs".into(), &b"x"[..]));

    for kind in *b"125xgL" {
        assert!(entries(&fixture(kind)).is_err(), "{kind}");
    }
}

#[test]
fn rejects_invalid_checksum_and_truncated_end() {
    let mut bytes = fixture(b'0');
    bytes[10] ^= 1;
    assert!(entries(&bytes).is_err());
    assert!(entries(&fixture(b'0')[..3 * BLOCK]).is_err());
}

#[test]
fn rejects_actual_duplicate_headers() {
    let mut bytes = fixture(b'0');
    bytes.truncate(2 * BLOCK);
    bytes.extend(fixture(b'0'));
    assert!(entries(&bytes).is_err());
}

#[test]
fn requires_one_normalized_root() {
    assert_eq!(
        member_path("core-0.1/src/lib.rs", "core-0.1").unwrap(),
        "src/lib.rs"
    );

    for path in [
        "other/src/lib.rs",
        "core-0.1/../lib.rs",
        "core-0.1/a//b",
        "core-0.1/a\nb",
    ] {
        assert!(member_path(path, "core-0.1").is_err(), "{path}");
    }
}
