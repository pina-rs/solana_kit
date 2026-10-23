extern crate std;
use std::prelude::v1::*;
use std::vec;
// Exact independent envelopes exercise the existing core decoder.
use super::fixtures::*;
use base64::{Engine, engine::general_purpose::STANDARD};
use serde_json::json;
use solana_kit_client::introspection::{Error, ResolvedInstruction, decode_rpc_transaction};
use solana_kit_client::methods::transaction_results::TransactionEncoding;
use solana_kit_core::transaction::{Lifetime, Version};

const LEGACY_TAG: u8 = 255;
const TRUNCATED_HEADER_END: usize = 66;
const SIGNATURE_PREFIX_LENGTH: usize = 65;

fn reported_version(version: u8) -> serde_json::Value {
    if version == LEGACY_TAG {
        json!("legacy")
    } else {
        json!(version)
    }
}

fn version_kind(version: u8) -> Version {
    match version {
        0 => Version::V0,
        1 => Version::V1,
        _ => Version::Legacy,
    }
}

pub(super) fn exact_legacy_v0_v1_envelopes_roundtrip_in_both_rpc_binary_encodings()
-> Result<(), &'static str> {
    for version in [255, 0, 1] {
        let bytes = literal_wire(version);

        for (encoding, tag, text) in [
            (
                TransactionEncoding::Base64,
                "base64",
                STANDARD.encode(&bytes),
            ),
            (
                TransactionEncoding::Base58,
                "base58",
                bs58::encode(&bytes).into_string(),
            ),
        ] {
            let mut value = raw();
            value["version"] = reported_version(version);

            value["transaction"] = json!([text, tag]);
            let source = parse(value, encoding);
            let decoded = decode_rpc_transaction(&source).unwrap();
            check_eq!(decoded.transaction().unwrap().encode(false).unwrap(), bytes);
            check_eq!(decoded.signers().collect::<Vec<_>>(), [address(1)]);
            check_eq!(decoded.version(), version_kind(version));
            check!(matches!(decoded.lifetime(), Lifetime::Unresolved(_)));
            let instructions = decoded.get_instructions().unwrap();
            let ResolvedInstruction::Raw(instruction) = &instructions[0].instruction else {
                return Err("raw wire instruction");
            };

            check_eq!(instruction.data, [0, 1, 255]);
            check_eq!(instruction.accounts.len(), 2);
        }
    }

    Ok(())
}

pub(super) fn malformed_truncated_trailing_header_index_and_signature_counts_fail_closed()
-> Result<(), &'static str> {
    let original = literal_wire(255);
    let mut trailing = original.clone();
    trailing.push(0);
    let mut header = original.clone();
    header[65] = 0;
    let mut count = original.clone();
    count[0] = 2;
    let mut index = original.clone();

    index[166] = 99;

    for bytes in [
        vec![],
        original[..TRUNCATED_HEADER_END].to_vec(),
        trailing,
        header,
        count,
        index,
    ] {
        let mut value = raw();
        value["transaction"] = json!([STANDARD.encode(bytes), "base64"]);
        let source = parse(value, TransactionEncoding::Base64);
        check!(matches!(
            decode_rpc_transaction(&source),
            Err(Error::Wire(_))
        ));
    }
    Ok(())
}

pub(super) fn binary_reported_version_must_equal_the_actual_signed_envelope()
-> Result<(), &'static str> {
    let mut value = raw();
    value["transaction"] = json!([STANDARD.encode(literal_wire(0)), "base64"]);
    let source = parse(value, TransactionEncoding::Base64);
    check_eq!(decode_rpc_transaction(&source).unwrap_err(), Error::Version);
    Ok(())
}

pub(super) fn direct_message_helpers_and_wire_lookup_resolution_share_the_same_account_table()
-> Result<(), &'static str> {
    let mut bytes = literal_wire(0);
    bytes[169] = 2;
    bytes[170] = 3;
    bytes.pop();
    bytes.push(1);
    bytes.extend([10; 32]);
    bytes.extend([1, 9, 1, 11]);
    let mut value = with_lookup();

    value["transaction"] = json!([STANDARD.encode(&bytes), "base64"]);
    let source = parse(value, TransactionEncoding::Base64);
    let decoded = decode_rpc_transaction(&source).unwrap();
    let transaction = decoded.transaction().unwrap();
    let loaded = solana_kit_client::methods::transaction_results::LoadedAddresses {
        writable: vec![address(LOADED_WRITABLE)],
        readonly: vec![address(LOADED_READONLY)],
    };
    use solana_kit_client::introspection::{
        get_account_metas, get_inner_instructions, get_instructions,
    };
    check_eq!(
        get_account_metas(transaction.message(), Some(&loaded)).unwrap(),
        decoded.account_metas()
    );
    let instructions = get_instructions(transaction.message(), Some(&loaded)).unwrap();
    check_eq!(
        instructions[0]
            .accounts
            .iter()
            .map(|account| account.address)
            .collect::<Vec<_>>(),
        [address(LOADED_WRITABLE), address(LOADED_READONLY)]
    );

    check!(
        get_inner_instructions(decoded.metadata(), decoded.account_metas())
            .unwrap()
            .is_empty()
    );
    check_eq!(
        get_account_metas(transaction.message(), None).unwrap_err(),
        Error::MissingLoadedAddresses
    );
    check_eq!(transaction.encode(false).unwrap(), bytes);
    Ok(())
}

pub(super) fn metadata_independent_wire_decode_preserves_v0_while_resolution_requires_loaded_addresses()
-> Result<(), &'static str> {
    use solana_kit_client::introspection::decode_rpc_wire;
    let mut bytes = literal_wire(0);
    bytes.pop();
    bytes.push(1);
    bytes.extend([10; 32]);
    bytes.extend([1, 9, 1, 11]);

    for metadata in [serde_json::Value::Null, meta(), {
        let mut metadata = meta();
        metadata["loadedAddresses"] = json!(null);
        metadata
    }] {
        let mut value = with_lookup();
        value["transaction"] = json!([STANDARD.encode(&bytes), "base64"]);
        value["meta"] = metadata;
        let source = parse(value, TransactionEncoding::Base64);
        let transaction = decode_rpc_wire(&source).unwrap();
        check_eq!(transaction.encode(false).unwrap(), bytes);
        check_eq!(
            transaction.message().bytes(),
            &bytes[SIGNATURE_PREFIX_LENGTH..]
        );

        check_eq!(
            transaction.signatures(),
            [Some(solana_kit_core::Signature::from_bytes([7; 64]))]
        );
        check_eq!(
            transaction.message().static_accounts(),
            [address(1), address(4)]
        );
        check_eq!(transaction.message().lookups().len(), 1);
        check!(matches!(
            transaction.message().lifetime(),
            Lifetime::Unresolved(_)
        ));
        check_eq!(
            decode_rpc_transaction(&source).unwrap_err(),
            Error::MissingLoadedAddresses
        );
    }

    let source = parse(raw(), TransactionEncoding::Json);
    check_eq!(decode_rpc_wire(&source).unwrap_err(), Error::BinaryRequired);
    Ok(())
}
