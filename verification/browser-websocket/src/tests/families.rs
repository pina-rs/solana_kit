use super::{config, connect, endpoint};
use solana_kit_browser::BrowserWebSocket;
use solana_kit_client::{methods, subscriptions::*};
use wasm_bindgen_test::wasm_bindgen_test;
const ADDRESS: &str = "11111111111111111111111111111111";
const GENERATION: u64 = 7;
const SIGNATURE: &str = concat!(
    "11111111111111111111111111111111",
    "11111111111111111111111111111111"
);
fn send(owner: &mut Connection, socket: &mut BrowserWebSocket) {
    socket
        .send(std::str::from_utf8(&owner.outbound().unwrap()).unwrap())
        .unwrap();
}
async fn ingest(owner: &mut Connection, socket: &mut BrowserWebSocket) {
    owner
        .ingest(GENERATION, socket.receive().await.unwrap().as_bytes())
        .unwrap();
}
async fn delivered<R: SubscriptionMethod>(request: R, params: &str) -> R::Response {
    let mut socket = connect("/family").await;
    let mut owner = Connection::new(GENERATION, Budget::default());
    let handle = owner.subscribe(request, 100).unwrap();
    let body = owner.outbound().unwrap();

    assert_eq!(
        std::str::from_utf8(&body).unwrap(),
        format!(
            r#"{{"jsonrpc":"2.0","id":1,"method":"{}","params":{params}}}"#,
            R::NAME
        )
    );

    socket.send(std::str::from_utf8(&body).unwrap()).unwrap();
    ingest(&mut owner, &mut socket).await;
    assert_eq!(owner.status(&handle).unwrap(), Status::Subscribing);
    assert!(matches!(owner.next(&handle).unwrap(), Drain::Pending));

    ingest(&mut owner, &mut socket).await;
    let Drain::Notification(value) = owner.next(&handle).unwrap() else {
        panic!("typed notification required");
    };

    if R::NAME != "signatureSubscribe" {
        owner.unsubscribe(&handle, 100).unwrap();
        send(&mut owner, &mut socket);
        ingest(&mut owner, &mut socket).await;
    }

    assert_eq!(owner.status(&handle).unwrap(), Status::Closed);
    socket.close().await.unwrap();
    value
}
#[wasm_bindgen_test]
async fn account_and_program_travel_over_actual_browser_socket() {
    let config = || {
        Some(AccountConfig {
            encoding: Some(methods::accounts::AccountEncoding::Base64),
            ..Default::default()
        })
    };
    let params = format!(r#"["{ADDRESS}",{{"encoding":"base64"}}]"#);
    let account = delivered(Account(ADDRESS.parse().unwrap(), config()), &params).await;
    assert_eq!(account.context.slot, 1);

    let program = delivered(
        Program(
            ADDRESS.parse().unwrap(),
            Some(ProgramConfig {
                account: config().unwrap(),
                filters: None,
            }),
        ),
        &params,
    )
    .await;
    assert_eq!(program.value.lamports, 9007199254740993);
}
#[wasm_bindgen_test]
async fn logs_signature_and_block_travel_over_actual_browser_socket() {
    assert_eq!(
        delivered(Logs(LogsFilter::All, None), r#"["all"]"#)
            .await
            .value
            .logs,
        ["hello"]
    );

    let signature = delivered(
        SignatureNotifications(SIGNATURE.parse().unwrap(), None),
        &format!(r#"["{SIGNATURE}"]"#),
    )
    .await;
    assert!(matches!(
        signature.value,
        SignatureInfo::Processed { err: Some(_) }
    ));

    assert!(
        delivered(Block(BlockFilter::All, None), r#"["all"]"#)
            .await
            .value
            .block
            .is_none()
    );
}
#[wasm_bindgen_test]
async fn slot_root_vote_and_updates_travel_over_actual_browser_socket() {
    assert_eq!(delivered(Slot, "[]").await.slot, 2);
    assert_eq!(delivered(Root, "[]").await, u64::MAX);
    assert_eq!(delivered(Vote, "[]").await.slots, [1, 2]);
    assert!(matches!(
        delivered(SlotsUpdates, "[]").await,
        SlotUpdate::Dead { slot: 2, .. }
    ));
}
#[wasm_bindgen_test]
async fn signature_terminal_preserves_root_on_same_socket() {
    let mut socket = BrowserWebSocket::connect(&endpoint("/survive"), config())
        .await
        .unwrap();
    let mut owner = Connection::new(GENERATION, Budget::default());

    let signature = owner
        .subscribe(
            SignatureNotifications(SIGNATURE.parse().unwrap(), None),
            100,
        )
        .unwrap();
    let root = owner.subscribe(Root, 100).unwrap();

    for _ in 0..2 {
        send(&mut owner, &mut socket);
        ingest(&mut owner, &mut socket).await;
    }

    for _ in 0..3 {
        ingest(&mut owner, &mut socket).await;
    }

    assert!(matches!(
        owner.next(&signature).unwrap(),
        Drain::Notification(_)
    ));
    assert!(matches!(owner.next(&signature).unwrap(), Drain::Closed));
    assert!(matches!(
        owner.next(&root).unwrap(),
        Drain::Notification(u64::MAX)
    ));
    assert!(matches!(owner.next(&root).unwrap(), Drain::Notification(7)));
    assert_eq!(owner.status(&root).unwrap(), Status::Active);
    socket.close().await.unwrap();
}
