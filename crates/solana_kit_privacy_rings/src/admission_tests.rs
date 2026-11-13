//! Request-bound admission regressions for indexer responses.
//!
//! Subjects here reuse the recorded mainnet fixture subjects so synthetic
//! pages answer the same request the captured examples answer.

use std::future::Ready;

use solana_kit_client::methods::Method;
use solana_kit_client::{
    Client, Error as ClientError, Limits, Reply, Request, Transport, TransportError,
};

use crate::{
    GetEncryptedUtxosByTags, GetMerkleProofs, GetNonInclusionProofs, GetNullifierQueueElements,
    GetShieldedTransactionsByNullifiers, GetShieldedTransactionsBySignature,
    GetShieldedTransactionsByTags, GetUserRecords, HistoryFilters, Page, ShieldedPage, SlotBounds,
    UtxoPage,
};

const TAG: &str = "5Vt1Mnq2b9ynV1EMXS564i5ZDuNudxm8ZG7yCXfH55M";
const TREE: &str = "777pvSGe5LaBd9nCczvE2FXEauiwtfGJ5XRpjKyniGor";
const SIGNATURE: &str =
    "5QVkeFTWxaAiwo1MBrDVvsLuLHoAfsvzXWz9pYDTUyFUkWipfVQ73XaUT74qfLau3HLyxsaoMRKWZGSmF6pdsfJf";
const OWNER: &str = "5Vt1Mnq2b9ynV1EMXS564i5ZDuNudxm8ZG7yCXfH55M";
const FOREIGN_TREE: &str = "Tree1111111111111111111111111111111111111111";
const FOREIGN_SIGNATURE: &str = "ForeignSignature1111111111111111111111111111";
const FOREIGN_OWNER: &str = "Owner1111111111111111111111111111111111111111";
const UTXO_PAGE_SLOT: u64 = 7;
const SHIELDED_PAGE_SLOT: u64 = 8;
const STALE_SLOT: u64 = 499;
const FRESH_SLOT: u64 = 500;
const LEAF_B: u8 = 5;
const FOREIGN_LEAF: u8 = 9;
const NEXT_SEQUENCE: u64 = 5;
const FOLLOWING_SEQUENCE: u64 = 6;
const GAP_SEQUENCE: u64 = 9;

fn context(slot: u64) -> String {
    format!(r#"{{"blockTime":1,"slot":{slot}}}"#)
}

/// A real 32-byte base58 nullifier hash.
fn nullifier(byte: u8) -> String {
    bs58::encode([byte; 32]).into_string()
}

fn utxo_page(slot: u64, groups: usize, outputs: usize, token: &str) -> String {
    let output = format!(
        r#"{{"slot":1,"txSignature":"{SIGNATURE}","outputSlot":{{"viewTag":"tv","outputContext":{{"hash":"h","tree":"{TREE}","treeId":0,"leafIndex":0}},"payload":""}}}}"#
    );

    let data: Vec<String> = (0..groups)
        .map(|_| {
            let repeated = vec![output.as_str(); outputs.max(1)];
            format!(
                r#"{{"slot":1,"signature":"{SIGNATURE}","outputs":[{}]}}"#,
                repeated.join(",")
            )
        })
        .collect();

    format!(
        r#"{{"context":{},"outputTreeId":0,"data":[{}],"paginationToken":{token}}}"#,
        context(slot),
        data.join(",")
    )
}

fn shielded_event(signature: &str) -> String {
    format!(
        r#"{{"slot":1,"txSignature":"{signature}","outputSlots":[],"messages":[],"nullifiers":[],"proofless":false}}"#
    )
}

fn shielded_page(slot: u64, groups: usize, events: usize, token: &str) -> String {
    let event = shielded_event(SIGNATURE);
    let group = format!(
        r#"{{"slot":1,"signature":"{SIGNATURE}","events":[{}]}}"#,
        vec![event.as_str(); events.max(1)].join(",")
    );
    let data: Vec<&str> = vec![group.as_str(); groups];

    format!(
        r#"{{"context":{},"outputTreeId":0,"data":[{}],"paginationToken":{token}}}"#,
        context(slot),
        data.join(",")
    )
}

fn signature_page(signatures: &[&str]) -> String {
    let transactions: Vec<String> = signatures
        .iter()
        .enumerate()
        .map(|(event_index, signature)| {
            format!(
                r#"{{"eventIndex":{event_index},"transaction":{}}}"#,
                shielded_event(signature)
            )
        })
        .collect();

    format!(
        r#"{{"context":{},"outputTreeId":0,"transactions":[{}]}}"#,
        context(2),
        transactions.join(",")
    )
}

fn inclusion_proof(leaf: &str, tree: &str) -> String {
    format!(
        r#"{{"leaf":"{leaf}","merkleContext":{{"treeType":1,"tree":"{tree}"}},"path":[],"leafIndex":0,"root":"{leaf}","rootSeq":1,"rootIndex":0}}"#
    )
}

fn non_inclusion_proof(leaf: &str, tree: &str) -> String {
    format!(
        r#"{{"leaf":"{leaf}","merkleContext":{{"treeType":2,"tree":"{tree}"}},"path":[],"lowElement":"{leaf}","lowElementIndex":0,"highElement":"{leaf}","highElementIndex":1,"root":"{leaf}","rootSeq":1,"rootIndex":0}}"#
    )
}

fn proofs_page(proofs: &[String]) -> String {
    format!(
        r#"{{"context":{},"proofs":[{}]}}"#,
        context(2),
        proofs.join(",")
    )
}

fn record(owner: &str) -> String {
    format!(
        r#"{{"owner":"{owner}","nullifierPubkey":"npk","viewingPubkey":"vpk","mergeEncryption":0}}"#
    )
}

fn records_page(records: &[Option<&str>]) -> String {
    let entries: Vec<String> = records
        .iter()
        .map(|entry| match *entry {
            None => "null".to_owned(),
            Some(owner) => record(owner),
        })
        .collect();

    format!(
        r#"{{"context":{},"records":[{}]}}"#,
        context(2),
        entries.join(",")
    )
}

fn queue_element(seq: u64, value: &str) -> String {
    format!(r#"{{"seq":{seq},"value":"{value}"}}"#)
}

fn queue_page(slot: u64, elements: &[String]) -> String {
    format!(
        r#"{{"context":{},"elements":[{}]}}"#,
        context(slot),
        elements.join(",")
    )
}

fn utxo_request(min_context_slot: Option<u64>, limit: Option<u64>) -> GetEncryptedUtxosByTags {
    GetEncryptedUtxosByTags {
        tags: vec![TAG.to_owned()],
        filters: None,
        limit,
        min_context_slot,
        pagination_token: None,
        ring_program_id: None,
    }
}

fn shielded_request(
    min_context_slot: Option<u64>,
    limit: Option<u64>,
) -> GetShieldedTransactionsByTags {
    GetShieldedTransactionsByTags {
        tags: vec![TAG.to_owned()],
        filters: None,
        limit,
        min_context_slot,
        pagination_token: None,
        ring_program_id: None,
    }
}

fn spent_request(
    min_context_slot: Option<u64>,
    limit: Option<u64>,
) -> GetShieldedTransactionsByNullifiers {
    GetShieldedTransactionsByNullifiers {
        nullifiers: vec![nullifier(1)],
        filters: None,
        limit,
        min_context_slot,
        pagination_token: None,
    }
}

fn queue_request(start_seq: u64, limit: u64) -> GetNullifierQueueElements {
    GetNullifierQueueElements {
        tree_account: TREE.to_owned(),
        start_seq,
        limit,
    }
}

fn schema_message<T: std::fmt::Debug>(decoded: Result<T, ClientError>) -> String {
    match decoded {
        Err(ClientError::Schema(error)) => error.to_string(),
        other => panic!("expected a schema error, got {other:?}"),
    }
}

struct Scripted {
    result: String,
}

impl Transport for Scripted {
    type Exchange<'a> = Ready<Result<Reply, TransportError>>;

    fn exchange(&self, request: Request) -> Self::Exchange<'_> {
        let value: serde_json::Value = serde_json::from_slice(&request.body).expect("frame");
        let id = value["id"].as_u64().expect("numeric id");
        std::future::ready(Ok(Reply {
            status: 200,
            body: format!(r#"{{"jsonrpc":"2.0","id":{id},"result":{}}}"#, self.result).into_bytes(),
        }))
    }
}

#[test]
fn a_missing_pagination_token_key_is_a_schema_error() {
    let body = format!(r#"{{"context":{},"outputTreeId":0,"data":[]}}"#, context(2));

    // Direct typed deserialization and the request's decode must agree.
    assert!(serde_json::from_str::<UtxoPage>(&body).is_err());
    let error = schema_message(utxo_request(None, None).decode(&body));
    assert!(error.contains("paginationToken"), "{error}");

    assert!(serde_json::from_str::<ShieldedPage>(&body).is_err());
    assert!(shielded_request(None, None).decode(&body).is_err());
    assert!(spent_request(None, None).decode(&body).is_err());
}

#[test]
fn an_explicit_null_token_ends_the_page_and_a_nonempty_token_continues() {
    let terminal = utxo_request(None, None)
        .decode(&utxo_page(2, 0, 0, "null"))
        .expect("terminal page");
    assert_eq!(terminal.pagination_token, None);

    let continuing = utxo_request(None, None)
        .decode(&utxo_page(2, 0, 0, r#""next""#))
        .expect("continuation page");
    assert_eq!(continuing.pagination_token.as_deref(), Some("next"));

    // The empty-string convention survives decode as an explicit token.
    let empty = shielded_request(None, None)
        .decode(&shielded_page(2, 0, 0, r#""""#))
        .expect("empty token");
    assert_eq!(empty.pagination_token.as_deref(), Some(""));
}

#[test]
fn grouped_transaction_identities_must_agree() {
    let utxo: serde_json::Value =
        serde_json::from_str(&utxo_page(2, 1, 1, "null")).expect("UTXO page");
    assert!(utxo_request(None, None).decode(&utxo.to_string()).is_ok());

    for (field, value) in [
        ("txSignature", serde_json::Value::from(FOREIGN_SIGNATURE)),
        ("slot", serde_json::Value::from(2u64)),
    ] {
        let mut conflicting = utxo.clone();
        conflicting["data"][0]["outputs"][0][field] = value;
        schema_message(utxo_request(None, None).decode(&conflicting.to_string()));
    }

    let shielded: serde_json::Value =
        serde_json::from_str(&shielded_page(2, 1, 1, "null")).expect("shielded page");
    assert!(
        shielded_request(None, None)
            .decode(&shielded.to_string())
            .is_ok()
    );

    for (field, value) in [
        ("txSignature", serde_json::Value::from(FOREIGN_SIGNATURE)),
        ("slot", serde_json::Value::from(2u64)),
    ] {
        let mut conflicting = shielded.clone();
        conflicting["data"][0]["events"][0][field] = value;
        let body = conflicting.to_string();
        schema_message(shielded_request(None, None).decode(&body));
        schema_message(spent_request(None, None).decode(&body));
    }
}

#[test]
fn signature_event_indices_and_slots_must_agree() {
    let request = GetShieldedTransactionsBySignature {
        tx_signature: SIGNATURE.to_owned(),
    };
    let mut valid: serde_json::Value =
        serde_json::from_str(&signature_page(&[SIGNATURE, SIGNATURE])).expect("signature page");
    valid["transactions"][1]["transaction"]["eventIndex"] = 1u64.into();
    assert!(request.decode(&valid.to_string()).is_ok());

    for (field, value) in [
        ("eventIndex", serde_json::Value::from(0u64)),
        ("slot", serde_json::Value::from(2u64)),
    ] {
        let mut conflicting = valid.clone();
        conflicting["transactions"][1]["transaction"][field] = value;
        schema_message(request.decode(&conflicting.to_string()));
    }
}

#[test]
fn queue_response_admission_rejects_invalid_request_bounds() {
    let body = queue_page(2, &[]);
    let page = serde_json::from_str::<crate::NullifierQueue>(&body).expect("empty page");

    for request in [
        queue_request(0, 0),
        queue_request(0, 1_001),
        GetNullifierQueueElements {
            tree_account: String::new(),
            ..queue_request(0, 1)
        },
    ] {
        schema_message(request.decode(&body));
        assert!(matches!(
            request.next_start_seq(&page),
            Err(crate::Error::Limit("queue request"))
        ));
    }
}

#[test]
fn indexer_pages_convert_into_walkable_pages_with_their_context() {
    let page: Page<_> = utxo_request(None, None)
        .decode(&utxo_page(UTXO_PAGE_SLOT, 1, 2, r#""next""#))
        .expect("page")
        .into();
    assert_eq!(page.context_slot, UTXO_PAGE_SLOT);
    assert_eq!(page.pagination_token.as_deref(), Some("next"));
    assert_eq!(page.items.len(), 1);
    assert_eq!(page.items[0].outputs.len(), 2);

    let page: Page<_> = shielded_request(None, None)
        .decode(&shielded_page(SHIELDED_PAGE_SLOT, 1, 2, "null"))
        .expect("page")
        .into();
    assert_eq!(page.context_slot, SHIELDED_PAGE_SLOT);
    assert_eq!(page.pagination_token, None);
    assert_eq!(page.items[0].events.len(), 2);
}

#[test]
fn a_page_below_the_requested_min_context_slot_is_rejected() {
    let stale = utxo_page(STALE_SLOT, 0, 0, "null");
    let error = schema_message(utxo_request(Some(FRESH_SLOT), None).decode(&stale));
    assert!(error.contains("499"), "{error}");
    assert!(utxo_request(Some(STALE_SLOT), None).decode(&stale).is_ok());

    let shielded = shielded_page(STALE_SLOT, 0, 0, "null");
    assert!(
        shielded_request(Some(FRESH_SLOT), None)
            .decode(&shielded)
            .is_err()
    );
    assert!(
        shielded_request(Some(STALE_SLOT), None)
            .decode(&shielded)
            .is_ok()
    );

    assert!(
        spent_request(Some(FRESH_SLOT), None)
            .decode(&shielded)
            .is_err()
    );
    assert!(
        spent_request(Some(STALE_SLOT), None)
            .decode(&shielded)
            .is_ok()
    );
}

#[test]
fn history_slot_filters_do_not_bound_the_context_fence() {
    let request = GetShieldedTransactionsByTags {
        tags: vec![TAG.to_owned()],
        filters: Some(HistoryFilters {
            slot: Some(SlotBounds {
                gte: Some(9_000),
                ..SlotBounds::default()
            }),
        }),
        limit: None,
        min_context_slot: None,
        pagination_token: None,
        ring_program_id: None,
    };

    // The filter window describes matched rows; decode fences only the
    // requested minContextSlot, which this request omits.
    assert!(request.decode(&shielded_page(2, 0, 0, "null")).is_ok());
    assert!(request.decode(&shielded_page(9_000, 0, 0, "null")).is_ok());
}

#[test]
fn the_requested_limit_bounds_transaction_groups_not_nested_items() {
    let error = schema_message(utxo_request(None, Some(1)).decode(&utxo_page(2, 2, 1, "null")));
    assert!(error.contains("2 transaction groups"), "{error}");

    // One group carrying several outputs stays inside the limit.
    assert!(
        utxo_request(None, Some(1))
            .decode(&utxo_page(2, 1, 2, "null"))
            .is_ok()
    );

    let error =
        schema_message(shielded_request(None, Some(1)).decode(&shielded_page(2, 2, 1, "null")));
    assert!(error.contains("2 transaction groups"), "{error}");

    // One group carrying several events stays inside the limit.
    assert!(
        shielded_request(None, Some(1))
            .decode(&shielded_page(2, 1, 2, "null"))
            .is_ok()
    );
}

#[test]
fn signature_events_must_name_the_requested_signature() {
    let request = GetShieldedTransactionsBySignature {
        tx_signature: SIGNATURE.to_owned(),
    };

    // The recorded mainnet example binds every event to its signature.
    let recorded = request
        .decode(include_str!("../tests/fixtures/mainnet_by_signature.json"))
        .expect("recorded");
    assert_eq!(recorded.transactions.len(), 1);
    assert_eq!(recorded.transactions[0].event_index, 0);

    // Several legitimate events for one signature decode together.
    let double = request
        .decode(&signature_page(&[SIGNATURE, SIGNATURE]))
        .expect("events");
    assert_eq!(double.transactions.len(), 2);

    // One foreign event among matching ones is refused.
    let error = schema_message(request.decode(&signature_page(&[SIGNATURE, FOREIGN_SIGNATURE])));
    assert!(error.contains("names signature"), "{error}");
    assert!(error.contains("event 1"), "{error}");
}

#[test]
fn merkle_proofs_bind_count_order_tree_and_leaf_to_the_request() {
    let leaf_a = nullifier(1);
    let leaf_b = nullifier(2);
    let leaf_c = nullifier(3);
    let request = GetMerkleProofs {
        tree_account: TREE.to_owned(),
        leaves: vec![leaf_a.clone(), leaf_b.clone()],
    };

    let both = request
        .decode(&proofs_page(&[
            inclusion_proof(&leaf_a, TREE),
            inclusion_proof(&leaf_b, TREE),
        ]))
        .expect("both");
    assert_eq!(both.proofs.len(), 2);
    assert_eq!(both.proofs[1].leaf, leaf_b);

    // Repeated requested leaves keep repeated proofs.
    let repeated = GetMerkleProofs {
        tree_account: TREE.to_owned(),
        leaves: vec![leaf_a.clone(), leaf_a.clone()],
    };

    assert!(
        repeated
            .decode(&proofs_page(&[
                inclusion_proof(&leaf_a, TREE),
                inclusion_proof(&leaf_a, TREE),
            ]))
            .is_ok()
    );

    // Each invalid condition varies alone.
    let extra = proofs_page(&[
        inclusion_proof(&leaf_a, TREE),
        inclusion_proof(&leaf_b, TREE),
        inclusion_proof(&leaf_c, TREE),
    ]);
    let error = schema_message(request.decode(&extra));
    assert!(error.contains("3 proofs for 2 requested leaves"), "{error}");

    let missing = proofs_page(&[inclusion_proof(&leaf_a, TREE)]);
    let error = schema_message(request.decode(&missing));
    assert!(error.contains("1 proofs for 2 requested leaves"), "{error}");

    let tree = proofs_page(&[
        inclusion_proof(&leaf_a, TREE),
        inclusion_proof(&leaf_b, FOREIGN_TREE),
    ]);
    let error = schema_message(request.decode(&tree));
    assert!(error.contains("proof 1 names tree"), "{error}");

    let swapped = proofs_page(&[
        inclusion_proof(&leaf_b, TREE),
        inclusion_proof(&leaf_a, TREE),
    ]);
    let error = schema_message(request.decode(&swapped));
    assert!(error.contains("proof 0 names leaf"), "{error}");
}

#[test]
fn non_inclusion_proofs_bind_count_order_tree_and_leaf_to_the_request() {
    let leaf_a = nullifier(4);
    let leaf_b = nullifier(LEAF_B);
    let request = GetNonInclusionProofs {
        tree_account: TREE.to_owned(),
        leaves: vec![leaf_a.clone(), leaf_b.clone()],
    };

    let both = request
        .decode(&proofs_page(&[
            non_inclusion_proof(&leaf_a, TREE),
            non_inclusion_proof(&leaf_b, TREE),
        ]))
        .expect("both");
    assert_eq!(both.proofs.len(), 2);

    let missing = proofs_page(&[non_inclusion_proof(&leaf_a, TREE)]);
    let error = schema_message(request.decode(&missing));
    assert!(error.contains("1 proofs for 2 requested leaves"), "{error}");

    let tree = proofs_page(&[
        non_inclusion_proof(&leaf_a, FOREIGN_TREE),
        non_inclusion_proof(&leaf_b, TREE),
    ]);
    let error = schema_message(request.decode(&tree));
    assert!(error.contains("proof 0 names tree"), "{error}");

    let foreign_leaf = proofs_page(&[
        non_inclusion_proof(&leaf_a, TREE),
        non_inclusion_proof(&nullifier(FOREIGN_LEAF), TREE),
    ]);
    let error = schema_message(request.decode(&foreign_leaf));
    assert!(error.contains("proof 1 names leaf"), "{error}");
}

#[test]
fn user_records_bind_count_and_owner_to_the_request() {
    let request = GetUserRecords {
        owners: vec![OWNER.to_owned(), OWNER.to_owned()],
    };

    // A None record stays a legitimate not-found answer at its position.
    let page = request
        .decode(&records_page(&[Some(OWNER), None]))
        .expect("found and missing");
    assert!(page.records[0].is_some());
    assert!(page.records[1].is_none());

    // Repeated owners each answer their own position.
    let doubled = request
        .decode(&records_page(&[Some(OWNER), Some(OWNER)]))
        .expect("repeated");
    assert_eq!(doubled.records.len(), 2);

    // A short registry is refused.
    let error = schema_message(request.decode(&records_page(&[Some(OWNER)])));
    assert!(
        error.contains("1 records for 2 requested owners"),
        "{error}"
    );

    // An over-long registry is refused.
    let error = schema_message(
        GetUserRecords {
            owners: vec![OWNER.to_owned()],
        }
        .decode(&records_page(&[Some(OWNER), None])),
    );
    assert!(
        error.contains("2 records for 1 requested owners"),
        "{error}"
    );

    // A foreign owner at any position is refused.
    let error = schema_message(request.decode(&records_page(&[Some(OWNER), Some(FOREIGN_OWNER)])));
    assert!(error.contains("record 1 names owner"), "{error}");
}

#[test]
fn queue_pages_admit_contract_conforming_elements() {
    let request = queue_request(4, 2);

    // Pruned sequences leave legitimate gaps.
    let gapped = request
        .decode(&queue_page(
            2,
            &[
                queue_element(4, &nullifier(1)),
                queue_element(GAP_SEQUENCE, &nullifier(2)),
            ],
        ))
        .expect("gapped");
    assert_eq!(gapped.elements[1].seq, GAP_SEQUENCE);

    let empty = request.decode(&queue_page(2, &[])).expect("empty");
    assert!(empty.elements.is_empty());

    let full = request
        .decode(&queue_page(
            2,
            &[
                queue_element(4, &nullifier(1)),
                queue_element(NEXT_SEQUENCE, &nullifier(2)),
            ],
        ))
        .expect("full");
    assert_eq!(full.elements.len(), 2);
}

#[test]
fn queue_pages_reject_each_contract_violation_on_its_own() {
    let request = queue_request(4, 2);
    let leaf_a = nullifier(1);
    let leaf_b = nullifier(2);
    let leaf_c = nullifier(3);

    let cases: [(&str, String); 7] = [
        (
            "queue sequence bound",
            queue_page(2, &[queue_element(3, &leaf_a)]),
        ),
        (
            "queue order",
            queue_page(2, &[queue_element(4, &leaf_a), queue_element(4, &leaf_b)]),
        ),
        (
            "queue order",
            queue_page(
                2,
                &[
                    queue_element(FOLLOWING_SEQUENCE, &leaf_a),
                    queue_element(NEXT_SEQUENCE, &leaf_b),
                ],
            ),
        ),
        (
            "queue page bound",
            queue_page(
                2,
                &[
                    queue_element(4, &leaf_a),
                    queue_element(NEXT_SEQUENCE, &leaf_b),
                    queue_element(FOLLOWING_SEQUENCE, &leaf_c),
                ],
            ),
        ),
        ("queue value", queue_page(2, &[queue_element(4, "nf")])),
        ("queue value", queue_page(2, &[queue_element(4, "0")])),
        (
            "queue value",
            queue_page(
                2,
                &[queue_element(4, &bs58::encode([1u8, 2, 3]).into_string())],
            ),
        ),
    ];

    for (rule, body) in cases {
        let error = schema_message(request.decode(&body));
        assert!(error.contains(rule), "{rule}: {error}");
    }
}

#[test]
fn client_call_surfaces_indexer_schema_errors() {
    let client = Client::new(
        Scripted {
            result: format!(r#"{{"context":{},"data":[]}}"#, context(2)),
        },
        Limits::default(),
    );
    let error = futures::executor::block_on(client.call(shielded_request(None, None)))
        .expect_err("missing paginationToken");
    assert!(matches!(error, ClientError::Schema(_)));
    assert!(error.to_string().contains("paginationToken"));

    let client = Client::new(
        Scripted {
            result: records_page(&[Some(FOREIGN_OWNER)]),
        },
        Limits::default(),
    );
    let error = futures::executor::block_on(client.call(GetUserRecords {
        owners: vec![OWNER.to_owned()],
    }))
    .expect_err("foreign owner");
    assert!(matches!(error, ClientError::Schema(_)));
    assert!(error.to_string().contains("names owner"), "{error}");
}
