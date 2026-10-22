//! Independent fixtures exercise all nine typed families through public I/O.
use solana_kit_client::{
    methods::{self, Method},
    subscriptions::*,
};
use solana_kit_core::accounts::MaybeAccount;

const ADDRESS: &str = "11111111111111111111111111111111";
const SIGNATURE: &str = concat!(
    "11111111111111111111111111111111",
    "11111111111111111111111111111111"
);
const ACCOUNT: &str = r#"{"executable":false,"lamports":9007199254740993,"owner":"11111111111111111111111111111111","rentEpoch":18446744073709551615,"space":1,"data":["AQ==","base64"]}"#;
const DEADLINE: u64 = 100;

fn delivered<R: SubscriptionMethod>(request: R, raw: &str) -> R::Response {
    let mut owner = Connection::new(1, Budget::default());
    let subscription = owner.subscribe(request, DEADLINE).unwrap();
    let sent: serde_json::Value = serde_json::from_slice(&owner.outbound().unwrap()).unwrap();
    assert_eq!(sent["method"], R::NAME);
    owner
        .ingest(1, br#"{"jsonrpc":"2.0","id":1,"result":0}"#)
        .unwrap();
    let notification = format!(
        r#"{{"jsonrpc":"2.0","method":"{}","params":{{"subscription":0,"result":{raw}}}}}"#,
        R::NOTIFICATION
    );
    owner.ingest(1, notification.as_bytes()).unwrap();

    match owner.next(&subscription).unwrap() {
        Drain::Notification(value) => value,
        other => panic!(
            "expected notification, got {}",
            matches!(other, Drain::Closed)
        ),
    }
}

fn account_config() -> AccountConfig {
    AccountConfig {
        encoding: Some(methods::accounts::AccountEncoding::Base64),
        ..Default::default()
    }
}

#[test]
fn account_and_program_reuse_owned_account_validation() {
    let raw = format!(r#"{{"context":{{"slot":1}},"value":{ACCOUNT}}}"#);
    let result = delivered(
        Account(ADDRESS.parse().unwrap(), Some(account_config())),
        &raw,
    );
    let MaybeAccount::Present(account) = result.value else {
        panic!("present account");
    };
    assert_eq!(account.lamports, 9007199254740993);
    assert_eq!(account.rent_epoch, Some(u64::MAX));

    let raw = format!(
        r#"{{"context":{{"slot":1}},"value":{{"pubkey":"{ADDRESS}","account":{ACCOUNT}}}}}"#
    );
    let result = delivered(
        Program(
            ADDRESS.parse().unwrap(),
            Some(ProgramConfig {
                account: account_config(),
                filters: None,
            }),
        ),
        &raw,
    );
    assert_eq!(result.value.address.to_string(), ADDRESS);
    assert_eq!(result.value.owner.to_string(), ADDRESS);
}

#[test]
fn logs_reuse_validated_signature_and_transaction_failure() {
    let raw = format!(
        r#"{{"context":{{"slot":1}},"value":{{"signature":"{SIGNATURE}","err":"AccountInUse","logs":["failed"]}}}}"#
    );
    let result = delivered(Logs(LogsFilter::All, None), &raw);
    assert_eq!(result.value.signature.to_string(), SIGNATURE);
    assert_eq!(result.value.logs, ["failed"]);
    assert!(matches!(
        result.value.err,
        Some(methods::submission::TransactionFailure::Known(
            methods::submission::KnownTransactionFailure::AccountInUse
        ))
    ));
}

#[test]
fn signature_progress_distinguishes_receipt_and_completes_execution() {
    let received = r#"{"context":{"slot":1},"value":"receivedSignature"}"#;
    let request = || {
        SignatureNotifications(
            SIGNATURE.parse().unwrap(),
            Some(SignatureConfig {
                enable_received_notification: Some(true),
                ..Default::default()
            }),
        )
    };
    assert!(matches!(
        delivered(request(), received).value,
        SignatureInfo::Received(_)
    ));
    assert!(
        SignatureNotifications(SIGNATURE.parse().unwrap(), None)
            .decode(received)
            .is_err()
    );
    let mut owner = Connection::new(1, Budget::default());
    let subscription = owner.subscribe(request(), DEADLINE).unwrap();
    owner.outbound().unwrap();
    owner
        .ingest(1, br#"{"jsonrpc":"2.0","id":1,"result":0}"#)
        .unwrap();

    owner.ingest(1, br#"{"jsonrpc":"2.0","method":"signatureNotification","params":{"subscription":0,"result":{"context":{"slot":1},"value":{"err":null}}}}"#).unwrap();
    assert!(matches!(
        owner.next(&subscription).unwrap(),
        Drain::Notification(_)
    ));
    assert!(matches!(owner.next(&subscription).unwrap(), Drain::Closed));
    owner.forget(subscription).unwrap();
    assert!(owner.outbound().is_none());
}

#[test]
fn slot_root_vote_and_slot_update_have_typed_results() {
    assert_eq!(delivered(Root, "18446744073709551615"), u64::MAX);
    assert_eq!(delivered(Slot, r#"{"slot":2,"parent":1,"root":0}"#).slot, 2);
    let vote = format!(
        r#"{{"hash":"{ADDRESS}","signature":"{SIGNATURE}","slots":[1,2],"timestamp":null,"votePubkey":"{ADDRESS}"}}"#
    );
    assert_eq!(delivered(Vote, &vote).slots, [1, 2]);
    let update = delivered(
        SlotsUpdates,
        r#"{"type":"dead","slot":2,"timestamp":3,"err":"bank failure"}"#,
    );
    assert!(matches!(update, SlotUpdate::Dead { slot: 2, .. }));
    assert!(
        SlotsUpdates
            .decode(r#"{"type":"future","slot":2,"timestamp":3}"#)
            .is_err()
    );
}

#[test]
fn block_reuses_request_selected_rpc_block_decoder() {
    let config = methods::transaction_results::BlockConfig {
        rewards: Some(false),
        transaction_details: Some(methods::transaction_results::TransactionDetails::None),
        ..Default::default()
    };
    let raw = format!(
        r#"{{"context":{{"slot":1}},"value":{{"slot":1,"err":null,"block":{{"blockhash":"{ADDRESS}","previousBlockhash":"{ADDRESS}","parentSlot":0,"blockTime":null,"blockHeight":null}}}}}}"#
    );
    let result = delivered(Block(BlockFilter::All, Some(BlockConfig(config))), &raw);
    let block = result.value.block.expect("reported block");
    assert_eq!(block.parent_slot, 0);
    assert!(matches!(
        block.transactions,
        methods::transaction_results::BlockTransactions::None
    ));
}

#[test]
fn malformed_family_values_never_become_successful_empty_results() {
    assert!(Root.decode("1.0").is_err());
    assert!(Slot.decode(r#"{"slot":1,"parent":0}"#).is_err());
    assert!(Vote.decode(r#"{"hash":"broken","signature":"broken","slots":[],"timestamp":null,"votePubkey":"broken"}"#).is_err());
    let raw = format!(r#"{{"context":{{"slot":1}},"value":{ACCOUNT}}}"#).replace("AQ==", "AAE=");
    assert!(
        Account(ADDRESS.parse().unwrap(), Some(account_config()))
            .decode(&raw)
            .is_err()
    );
    assert!(
        SignatureNotifications(SIGNATURE.parse().unwrap(), None)
            .decode(r#"{"context":{"slot":1},"value":{}}"#)
            .is_err()
    );
}
