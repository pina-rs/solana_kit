//! Exercises the actual raw-header boundary without relying on an extractor.

use super::{BLOCK, LONG_NAME_LIMIT, NAME_LENGTH, entries, member_path};

/// Builds one ordinary header with a valid checksum and an explicit name.
fn header(name: &[u8], size: usize, kind: u8, gnu: bool) -> [u8; BLOCK] {
    assert!(name.len() <= NAME_LENGTH);
    let mut header = [0; BLOCK];
    header[..name.len()].copy_from_slice(name);
    header[100..108].copy_from_slice(b"0000644\0");
    header[108..116].copy_from_slice(b"0000000\0");
    header[116..124].copy_from_slice(b"0000000\0");
    header[124..136].copy_from_slice(format!("{size:011o}\0").as_bytes());
    header[136..148].copy_from_slice(b"00000000000\0");
    header[156] = kind;
    if gnu {
        header[257..265].copy_from_slice(b"ustar  \0");
    } else {
        header[257..263].copy_from_slice(b"ustar\0");
        header[263..265].copy_from_slice(b"00");
    }
    header[148..156].fill(b' ');
    let checksum: u64 = header.iter().map(|byte| u64::from(*byte)).sum();
    header[148..156].copy_from_slice(format!("{checksum:06o}\0 ").as_bytes());
    header
}

/// Appends one complete record and its zero padding.
fn append(bytes: &mut Vec<u8>, name: &[u8], kind: u8, content: &[u8], gnu: bool) {
    bytes.extend_from_slice(&header(name, content.len(), kind, gnu));
    bytes.extend_from_slice(content);
    let padding = (BLOCK - content.len() % BLOCK) % BLOCK;
    bytes.resize(bytes.len() + padding, 0);
}

/// Appends the two zero blocks required by the tar end marker.
fn finish(bytes: &mut Vec<u8>) {
    bytes.resize(bytes.len() + 2 * BLOCK, 0);
}

/// Builds one ordinary USTAR member with a valid header checksum.
fn fixture(kind: u8) -> Vec<u8> {
    let mut bytes = Vec::new();
    append(&mut bytes, b"core-0.1/lib.rs", kind, b"x", false);
    finish(&mut bytes);
    bytes
}

#[test]
fn accepts_actual_cargo_gnu_layout() {
    let mut bytes = Vec::new();
    append(&mut bytes, b"core-0.1/lib.rs", b'0', b"x", true);
    finish(&mut bytes);
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

    for kind in *b"125xg" {
        assert!(entries(&fixture(kind)).is_err(), "{kind}");
    }

    let error = entries(&fixture(b'K')).unwrap_err().to_string();
    assert!(error.contains("GNU long-link extension"), "{error}");
}

#[test]
fn accepts_interior_spaces_in_actual_member_names() {
    let mut bytes = Vec::new();
    append(
        &mut bytes,
        b"solana_kit_wallet-0.2.0/assets/coinbase wallet.uri",
        b'0',
        b"coinbase",
        true,
    );
    append(
        &mut bytes,
        b"solana_kit_wallet-0.2.0/assets/huobi wallet.uri",
        b'0',
        b"huobi",
        true,
    );
    finish(&mut bytes);

    let members = entries(&bytes).unwrap();
    assert_eq!(members.len(), 2);
    assert_eq!(
        members[0],
        (
            "solana_kit_wallet-0.2.0/assets/coinbase wallet.uri".into(),
            &b"coinbase"[..]
        )
    );
    assert_eq!(
        members[1],
        (
            "solana_kit_wallet-0.2.0/assets/huobi wallet.uri".into(),
            &b"huobi"[..]
        )
    );
}

#[test]
fn accepts_gnu_long_name_metadata_for_the_next_regular_file() {
    let name = concat!(
        "solana_kit_token_2022-0.1.1/src/instructions/",
        "configure_confidential_transfer_account_with_registry.rs"
    );
    assert!(name.len() > NAME_LENGTH);
    let mut long = name.as_bytes().to_vec();
    long.push(0);
    let mut bytes = Vec::new();
    append(&mut bytes, b"././@LongLink", b'L', &long, true);
    append(
        &mut bytes,
        &name.as_bytes()[..NAME_LENGTH],
        b'0',
        b"long",
        true,
    );
    finish(&mut bytes);

    let members = entries(&bytes).unwrap();
    assert_eq!(members.len(), 1);
    assert_eq!(members[0], (name.into(), &b"long"[..]));
}

#[test]
fn rejects_orphan_repeated_and_ambiguous_long_name_metadata() {
    let long = format!("core-0.1/{}.rs", "a".repeat(NAME_LENGTH));
    let mut payload = long.as_bytes().to_vec();
    payload.push(0);

    let mut orphan = Vec::new();
    append(&mut orphan, b"././@LongLink", b'L', &payload, true);
    finish(&mut orphan);
    assert!(
        entries(&orphan)
            .unwrap_err()
            .to_string()
            .contains("orphan GNU long-name")
    );

    let mut repeated = Vec::new();
    append(&mut repeated, b"././@LongLink", b'L', &payload, true);
    append(&mut repeated, b"././@LongLink", b'L', &payload, true);
    finish(&mut repeated);
    assert!(
        entries(&repeated)
            .unwrap_err()
            .to_string()
            .contains("repeated GNU long-name")
    );

    let mut mismatched = Vec::new();
    append(&mut mismatched, b"././@LongLink", b'L', &payload, true);
    append(&mut mismatched, &[b'x'; NAME_LENGTH], b'0', b"x", true);
    finish(&mut mismatched);
    assert!(
        entries(&mismatched)
            .unwrap_err()
            .to_string()
            .contains("ambiguous GNU long-name")
    );

    let mut duplicate = Vec::new();
    for _ in 0..2 {
        append(&mut duplicate, b"././@LongLink", b'L', &payload, true);
        append(
            &mut duplicate,
            &long.as_bytes()[..NAME_LENGTH],
            b'0',
            b"x",
            true,
        );
    }
    finish(&mut duplicate);
    assert!(
        entries(&duplicate)
            .unwrap_err()
            .to_string()
            .contains("duplicate archive member")
    );

    let mut directory = Vec::new();
    append(&mut directory, b"././@LongLink", b'L', &payload, true);
    append(
        &mut directory,
        &long.as_bytes()[..NAME_LENGTH],
        b'5',
        b"",
        true,
    );
    finish(&mut directory);
    assert!(entries(&directory).is_err());
}

#[test]
fn rejects_invalid_long_name_payloads_and_bounds() {
    let valid = format!("core-0.1/{}.rs", "a".repeat(NAME_LENGTH));
    let mut cases = vec![valid.as_bytes().to_vec()];
    let mut interior = valid.as_bytes().to_vec();
    interior.insert(NAME_LENGTH / 2, 0);
    interior.push(0);
    cases.push(interior);
    let mut control = valid.as_bytes().to_vec();
    control[NAME_LENGTH / 2] = b'\n';
    control.push(0);
    cases.push(control);
    cases.push(b"core-0.1/short.rs\0".to_vec());
    let mut oversized = vec![b'a'; LONG_NAME_LIMIT + 1];
    oversized.push(0);
    cases.push(oversized);

    for payload in cases {
        let mut bytes = Vec::new();
        append(&mut bytes, b"././@LongLink", b'L', &payload, true);
        let name = valid.as_bytes()[..NAME_LENGTH].to_vec();
        append(&mut bytes, &name, b'0', b"x", true);
        finish(&mut bytes);
        assert!(entries(&bytes).is_err(), "{payload:?}");
    }

    let mut truncated = Vec::new();
    append(
        &mut truncated,
        b"././@LongLink",
        b'L',
        b"unterminated",
        true,
    );
    assert!(entries(&truncated).is_err());
}

#[test]
fn rejects_long_name_header_checksum_and_conflicting_root() {
    let long = format!("other-0.1/{}.rs", "a".repeat(NAME_LENGTH));
    let mut payload = long.as_bytes().to_vec();
    payload.push(0);

    let mut bytes = Vec::new();
    append(&mut bytes, b"core-0.1/lib.rs", b'0', b"x", true);
    append(&mut bytes, b"././@LongLink", b'L', &payload, true);
    append(
        &mut bytes,
        &long.as_bytes()[..NAME_LENGTH],
        b'0',
        b"x",
        true,
    );
    finish(&mut bytes);
    assert!(
        entries(&bytes)
            .unwrap_err()
            .to_string()
            .contains("conflicting archive root")
    );

    let mut checksum = Vec::new();
    append(&mut checksum, b"././@LongLink", b'L', &payload, true);
    append(
        &mut checksum,
        &long.as_bytes()[..NAME_LENGTH],
        b'0',
        b"x",
        true,
    );
    finish(&mut checksum);
    checksum[0] ^= 1;
    assert!(
        entries(&checksum)
            .unwrap_err()
            .to_string()
            .contains("checksum")
    );
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
    let mut bytes = Vec::new();
    append(&mut bytes, b"core-0.1/lib.rs", b'0', b"x", false);
    append(&mut bytes, b"core-0.1/lib.rs", b'0', b"y", false);
    finish(&mut bytes);
    assert!(entries(&bytes).is_err());
}

#[test]
fn requires_one_normalized_root() {
    assert_eq!(
        member_path("core-0.1/src/lib.rs", "core-0.1").unwrap(),
        "src/lib.rs"
    );
    assert_eq!(
        member_path("core-0.1/assets/coinbase wallet.uri", "core-0.1").unwrap(),
        "assets/coinbase wallet.uri"
    );

    for path in [
        "other/src/lib.rs",
        "core-0.1/../lib.rs",
        "core-0.1/a//b",
        "core-0.1/a\nb",
        "core-0.1/ leading.uri",
        "core-0.1/trailing.uri ",
        "core-0.1/../assets/space name.uri",
    ] {
        assert!(member_path(path, "core-0.1").is_err(), "{path}");
    }
}
