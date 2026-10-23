//! Complete original Anchor test bodies, adapted only for host-neutral reporting.
use alloc::{
    format,
    string::{String, ToString},
    vec::Vec,
};
use serde_json::json;
use solana_kit_anchor::{
    Idl, Limits, STANDARD_ERRORS, account_discriminator, anchor_sighash, event_discriminator,
    has_discriminator, instruction_discriminator, program_error, sha256,
};

pub(super) fn all_77_standard_codes_names_and_messages_match_fresh_pinned_source()
-> Result<(), &'static str> {
    let rows: Vec<_> =
        include_str!("../../../../crates/solana_kit_anchor/tests/fixtures/errors.tsv")
            .lines()
            .collect();

    super::assert_eq!(rows.len(), 77);

    super::assert_eq!(STANDARD_ERRORS.len(), 77);

    for (entry, row) in STANDARD_ERRORS.iter().zip(rows) {
        let columns: Vec<_> = row.split('\t').collect();
        super::assert_eq!(entry.code.to_string(), columns[0]);
        super::assert_eq!(entry.name, columns[1]);
        super::assert_eq!(entry.message, columns[2]);
        let resolved = program_error(entry.code, None);
        super::assert_eq!(resolved.name, entry.name);
        super::assert_eq!(resolved.message, entry.message);
    }

    Ok(())
}

pub(super) fn custom_codes_override_standards_and_preserve_empty_messages()
-> Result<(), &'static str> {
    const OMITTED_MESSAGE_CODE: i64 = 7000;
    const UNKNOWN_CODE: i64 = 987654;
    let idl = Idl::from_value(
        &json!({"errors": [
            {"code": 100, "name": "Override", "msg": ""},
            {"code": OMITTED_MESSAGE_CODE, "name": "NoMessage"},
        ]}),
        Limits::new(1000, 100, 10),
    )
    .unwrap();

    let custom = program_error(100, Some(&idl));

    super::assert_eq!(custom.name, "Override");

    super::assert_eq!(custom.message, "");

    super::assert_eq!(
        program_error(OMITTED_MESSAGE_CODE, Some(&idl)).message,
        "Custom program error: 7000"
    );

    let unknown = program_error(UNKNOWN_CODE, None);

    super::assert_eq!(unknown.name, "Unknown");

    super::assert!(unknown.message.contains("not a known Anchor error"));

    super::assert_eq!(custom.to_string(), "AnchorProgramError(100, Override): ");
    super::assert_eq!(program_error(-1, None).code, -1);
    Ok(())
}

pub(super) fn sha256_matches_nist_and_original_padding_boundary_literals()
-> Result<(), &'static str> {
    fn hex(bytes: [u8; 32]) -> String {
        bytes.iter().map(|byte| format!("{byte:02x}")).collect()
    }

    super::assert_eq!(
        hex(sha256(b"")),
        "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
    );

    super::assert_eq!(
        hex(sha256(b"abc")),
        "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
    );

    super::assert_eq!(
        hex(sha256(&[b'a'; 55])),
        "9f4390f8d30c2dd92ec9f095b65e2b9ae9b0a925a5258e241c9f1e910f734318"
    );

    super::assert_eq!(
        hex(sha256(&[b'a'; 64])),
        "ffe054fe7ae0cb6dc65c3af9b61d5209f439851db43d0ba5997337df154668eb"
    );
    Ok(())
}

pub(super) fn every_namespace_and_arbitrary_explicit_prefix_matches_original_literals()
-> Result<(), &'static str> {
    super::assert_eq!(
        instruction_discriminator("initialize"),
        [175, 175, 109, 31, 13, 152, 155, 237]
    );

    super::assert_eq!(
        anchor_sighash("global", "account_and_event_arg_and_field"),
        [42, 50, 168, 74, 160, 93, 226, 83]
    );

    super::assert_eq!(
        account_discriminator("MyAccount"),
        [246, 28, 6, 87, 251, 45, 50, 42]
    );

    super::assert_eq!(
        event_discriminator("MyEvent"),
        [96, 184, 197, 243, 139, 2, 90, 148]
    );

    super::assert!(has_discriminator(&[1, 2, 3], &[1, 2]));

    super::assert!(has_discriminator(&[], &[]));

    super::assert!(!has_discriminator(&[1, 2, 3], &[1, 3]));

    super::assert!(!has_discriminator(&[1], &[1, 2]));
    Ok(())
}
