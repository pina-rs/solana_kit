//! Typed Connection requests and results travel through real loopback WebSockets.
use futures_util::{SinkExt, StreamExt};
use solana_kit_client::{
    methods::{
        self,
        submission::{KnownTransactionFailure, TransactionFailure},
    },
    subscriptions::*,
};
use solana_kit_core::accounts::MaybeAccount;
use solana_kit_native::{NativeSubscriptionSocket, NativeWebSocket, WsConfig};
use std::pin::Pin;
use tokio::net::TcpListener;
use tokio_tungstenite::tungstenite::Message;

const ADDRESS: &str = "11111111111111111111111111111111";
const GENERATION: u64 = 7;
const ACK_DEADLINE: u64 = 100;
const NETWORK_DEADLINE: u64 = 2_000;
struct Monotonic(tokio::time::Instant);
impl Clock for Monotonic {
    type Sleep<'a> =
        Pin<Box<dyn core::future::Future<Output = Result<(), ClockError>> + Send + 'a>>;
    fn now(&self) -> u64 {
        u64::try_from(self.0.elapsed().as_millis()).unwrap()
    }
    fn sleep_until(&self, deadline: u64) -> Self::Sleep<'_> {
        Box::pin(async move {
            tokio::time::sleep_until(self.0 + std::time::Duration::from_millis(deadline)).await;
            Ok(())
        })
    }
}
const SIGNATURE: &str = concat!(
    "11111111111111111111111111111111",
    "11111111111111111111111111111111"
);
const ACCOUNT: &str = r#"{"executable":false,"lamports":9007199254740993,"owner":"11111111111111111111111111111111","rentEpoch":18446744073709551615,"space":1,"data":["AQ==","base64"]}"#;

async fn delivered<R: SubscriptionMethod>(request: R, params: &str, raw: &str) -> R::Response {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let endpoint = format!("ws://{}/rpc", listener.local_addr().unwrap());
    let expected = format!(
        r#"{{"jsonrpc":"2.0","id":1,"method":"{}","params":{params}}}"#,
        R::NAME
    );
    let notification = format!(
        r#"{{"jsonrpc":"2.0","method":"{}","params":{{"subscription":17,"result":{raw}}}}}"#,
        R::NOTIFICATION
    );
    let one_shot = R::NAME == "signatureSubscribe";
    let unsubscribe = format!(
        r#"{{"jsonrpc":"2.0","id":2,"method":"{}","params":[17]}}"#,
        R::UNSUBSCRIBE
    );
    let peer = tokio::spawn(async move {
        let (stream, _) = listener.accept().await.unwrap();
        let mut peer = tokio_tungstenite::accept_async(stream).await.unwrap();
        assert_eq!(
            peer.next().await.unwrap().unwrap(),
            Message::Text(expected.into())
        );
        peer.send(Message::Text(
            r#"{"jsonrpc":"2.0","id":1,"result":17}"#.into(),
        ))
        .await
        .unwrap();
        peer.send(Message::Text(notification.into())).await.unwrap();
        if !one_shot {
            assert_eq!(
                peer.next().await.unwrap().unwrap(),
                Message::Text(unsubscribe.into())
            );
            peer.send(Message::Text(
                r#"{"jsonrpc":"2.0","id":2,"result":true}"#.into(),
            ))
            .await
            .unwrap();
        }

        assert!(matches!(
            peer.next().await.unwrap().unwrap(),
            Message::Close(_)
        ));
        peer.flush().await.unwrap();
    });
    let socket = NativeWebSocket::connect(&endpoint, WsConfig::default())
        .await
        .unwrap();
    let clock = Monotonic(tokio::time::Instant::now());
    let mut owner = Session::new(
        NativeSubscriptionSocket::from(socket),
        GENERATION,
        Budget::default(),
    );
    let handle = owner.subscribe(request, NETWORK_DEADLINE).unwrap();
    let Drain::Notification(result) = owner.next(&handle, &clock, NETWORK_DEADLINE).await.unwrap()
    else {
        panic!("typed notification required");
    };

    if !one_shot {
        owner
            .unsubscribe(handle, &clock, NETWORK_DEADLINE)
            .await
            .unwrap();
    } else {
        owner.forget(handle).unwrap();
    }

    owner.shutdown(&clock, NETWORK_DEADLINE).await.unwrap();
    peer.await.unwrap();
    result
}

async fn send(owner: &mut Connection, socket: &mut NativeWebSocket) {
    socket
        .send(std::str::from_utf8(&owner.outbound().unwrap()).unwrap())
        .await
        .unwrap();
}
async fn ingest(owner: &mut Connection, socket: &mut NativeWebSocket) {
    owner
        .ingest(
            GENERATION,
            socket.receive().await.unwrap().unwrap().as_bytes(),
        )
        .unwrap();
}

#[tokio::test]
async fn account_and_program_roundtrip_exact_requests_and_typed_values() {
    let config = || {
        Some(AccountConfig {
            encoding: Some(methods::accounts::AccountEncoding::Base64),
            ..Default::default()
        })
    };
    let params = format!(r#"["{ADDRESS}",{{"encoding":"base64"}}]"#);
    let raw = format!(r#"{{"context":{{"slot":1}},"value":{ACCOUNT}}}"#);
    let result = delivered(Account(ADDRESS.parse().unwrap(), config()), &params, &raw).await;
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
                account: config().unwrap(),
                filters: None,
            }),
        ),
        &params,
        &raw,
    )
    .await;
    assert_eq!(result.value.address.to_string(), ADDRESS);
    let methods::accounts::AccountData::Bytes { bytes, .. } = result.value.data else {
        panic!("binary data required");
    };
    assert_eq!(bytes, [1]);
}

#[tokio::test]
async fn logs_signature_and_block_roundtrip_typed_execution_results() {
    let raw = format!(
        r#"{{"context":{{"slot":1}},"value":{{"signature":"{SIGNATURE}","err":"AccountInUse","logs":["failed"]}}}}"#
    );
    let result = delivered(Logs(LogsFilter::All, None), r#"["all"]"#, &raw).await;
    assert_eq!(result.value.signature.to_string(), SIGNATURE);
    assert!(matches!(
        result.value.err,
        Some(TransactionFailure::Known(
            KnownTransactionFailure::AccountInUse
        ))
    ));
    let result = delivered(
        SignatureNotifications(SIGNATURE.parse().unwrap(), None),
        &format!(r#"["{SIGNATURE}"]"#),
        r#"{"context":{"slot":1},"value":{"err":"AccountInUse"}}"#,
    )
    .await;
    assert!(matches!(
        result.value,
        SignatureInfo::Processed {
            err: Some(TransactionFailure::Known(
                KnownTransactionFailure::AccountInUse
            ))
        }
    ));
    let request = Block(
        BlockFilter::All,
        Some(BlockConfig(methods::transaction_results::BlockConfig {
            rewards: Some(false),
            transaction_details: Some(methods::transaction_results::TransactionDetails::None),
            ..Default::default()
        })),
    );
    let raw = format!(
        r#"{{"context":{{"slot":1}},"value":{{"slot":1,"err":null,"block":{{"blockhash":"{ADDRESS}","previousBlockhash":"{ADDRESS}","parentSlot":0,"blockTime":null,"blockHeight":null}}}}}}"#
    );

    let result = delivered(
        request,
        r#"["all",{"showRewards":false,"transactionDetails":"none"}]"#,
        &raw,
    )
    .await;
    assert_eq!(result.value.block.unwrap().parent_slot, 0);
}

#[tokio::test]
async fn slot_root_vote_and_updates_roundtrip_full_width_values() {
    assert_eq!(
        delivered(Root, "[]", "18446744073709551615").await,
        u64::MAX
    );
    assert_eq!(
        delivered(Slot, "[]", r#"{"slot":2,"parent":1,"root":0}"#)
            .await
            .slot,
        2
    );
    let vote = format!(
        r#"{{"hash":"{ADDRESS}","signature":"{SIGNATURE}","slots":[1,2],"timestamp":null,"votePubkey":"{ADDRESS}"}}"#
    );
    assert_eq!(delivered(Vote, "[]", &vote).await.slots, [1, 2]);
    assert!(matches!(
        delivered(
            SlotsUpdates,
            "[]",
            r#"{"type":"dead","slot":2,"timestamp":3,"err":"bank failure"}"#
        )
        .await,
        SlotUpdate::Dead { slot: 2, .. }
    ));
}

#[tokio::test]
async fn failed_signature_ends_once_and_root_peer_survives_on_same_socket() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let endpoint = format!("ws://{}/rpc", listener.local_addr().unwrap());
    let peer = tokio::spawn(signature_peer(listener));
    let mut socket = NativeWebSocket::connect(&endpoint, WsConfig::default())
        .await
        .unwrap();
    let mut owner = Connection::new(GENERATION, Budget::default());
    let signature = owner
        .subscribe(
            SignatureNotifications(SIGNATURE.parse().unwrap(), None),
            ACK_DEADLINE,
        )
        .unwrap();
    let root = owner.subscribe(Root, ACK_DEADLINE).unwrap();

    for _ in 0..2 {
        send(&mut owner, &mut socket).await;
        ingest(&mut owner, &mut socket).await;
    }

    for _ in 0..2 {
        ingest(&mut owner, &mut socket).await;
    }

    let Drain::Notification(value) = owner.next(&signature).unwrap() else {
        panic!("signature result required");
    };
    assert!(matches!(value.value, SignatureInfo::Processed { .. }));
    assert!(matches!(owner.next(&signature).unwrap(), Drain::Closed));
    assert!(matches!(
        owner.next(&root).unwrap(),
        Drain::Notification(u64::MAX)
    ));
    assert_eq!(owner.status(&root).unwrap(), Status::Active);
    socket.close().await.unwrap();
    peer.await.unwrap();
}

async fn signature_peer(listener: TcpListener) {
    let (stream, _) = listener.accept().await.unwrap();
    let mut peer = tokio_tungstenite::accept_async(stream).await.unwrap();

    for id in 1..=2 {
        assert!(matches!(
            peer.next().await.unwrap().unwrap(),
            Message::Text(_)
        ));
        peer.send(Message::Text(
            format!(r#"{{"jsonrpc":"2.0","id":{id},"result":{id}}}"#).into(),
        ))
        .await
        .unwrap();
    }
    for text in [
        r#"{"jsonrpc":"2.0","method":"signatureNotification","params":{"subscription":1,"result":{"context":{"slot":1},"value":{"err":"AccountInUse"}}}}"#,
        r#"{"jsonrpc":"2.0","method":"rootNotification","params":{"subscription":2,"result":18446744073709551615}}"#,
    ] {
        peer.send(Message::Text(text.into())).await.unwrap();
    }
    assert!(matches!(
        peer.next().await.unwrap().unwrap(),
        Message::Close(_)
    ));
    peer.flush().await.unwrap();
}
