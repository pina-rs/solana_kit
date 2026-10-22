//! Explicit framing, queue and deadline failures without hidden loss.

use super::*;
use solana_kit_client::Limits;
use solana_kit_core::{Address, Signature};

fn malformed<R: SubscriptionMethod>(request: R, method: &str, raw: &str) -> Check {
    let mut owner = Connection::new(GENERATION, Budget::default());
    let handle = owner
        .subscribe(request, SUBSCRIBE_DEADLINE)
        .map_err(|_| "malformed fixture subscribe failed")?;
    owner.outbound().ok_or("malformed fixture frame absent")?;
    ack(&mut owner, 1, "7")?;
    notify(&mut owner, method, SERVER, raw)?;
    crate::require(
        matches!(owner.next(&handle), Err(Failure::Protocol(_)))
            && matches!(
                owner.subscribe(Root, SUBSCRIBE_DEADLINE),
                Err(Failure::Closed)
            ),
        "malformed typed value became success",
    )
}

fn schemas() -> Check {
    malformed(
        Account(Address::from_bytes([0; 32]), None),
        "accountNotification",
        &contextual("{}"),
    )?;
    malformed(
        Program(Address::from_bytes([0; 32]), None),
        "programNotification",
        &contextual("{}"),
    )?;
    malformed(
        Logs(LogsFilter::All, None),
        "logsNotification",
        &contextual(r#"{"signature":"bad","err":null,"logs":[]}"#),
    )?;
    malformed(
        SignatureNotifications(Signature::from_bytes([0; 64]), None),
        "signatureNotification",
        &contextual(r#"{"err":{"InstructionError":[256,{"Custom":1}]}}"#),
    )?;
    malformed(Root, "rootNotification", "1.0")?;
    malformed(Slot, "slotNotification", r#"{"slot":1,"parent":0}"#)?;
    malformed(
        Vote,
        "voteNotification",
        r#"{"hash":"bad","signature":"bad","slots":[],"timestamp":null,"votePubkey":"bad"}"#,
    )?;
    malformed(
        SlotsUpdates,
        "slotsUpdatesNotification",
        r#"{"type":"future","slot":1,"timestamp":0}"#,
    )?;

    malformed(
        Block(BlockFilter::All, None),
        "blockNotification",
        &contextual(r#"{"slot":1,"block":null,"err":"FutureError"}"#),
    )
}

fn limits() -> Check {
    let mut owner = Connection::new(
        GENERATION,
        Budget {
            messages: 1,
            queued_bytes: 100,
            ..Default::default()
        },
    );
    let handle = owner
        .subscribe(Root, SUBSCRIBE_DEADLINE)
        .map_err(|_| "bounded subscribe failed")?;
    owner.outbound().ok_or("bounded subscribe frame absent")?;
    notify(&mut owner, "rootNotification", SERVER, "1")?;
    crate::require(matches!(owner.ingest(GENERATION, br#"{"jsonrpc":"2.0","method":"rootNotification","params":{"subscription":7,"result":2}}"#), Err(Failure::Overflow)), "early message overflow hidden")?;
    crate::require(
        matches!(owner.next(&handle), Err(Failure::Closed)),
        "overflow retained registration",
    )?;
    let mut owner = Connection::new(
        GENERATION,
        Budget {
            queued_bytes: 80,
            ..Default::default()
        },
    );
    let _handle = owner
        .subscribe(Root, SUBSCRIBE_DEADLINE)
        .map_err(|_| "byte fixture subscribe failed")?;

    owner.outbound().ok_or("byte fixture frame absent")?;
    let raw = format!(
        r#"{{"jsonrpc":"2.0","method":"rootNotification","params":{{"subscription":7,"result":"{}"}}}}"#,
        "x".repeat(OVERSIZED_RECORD)
    );
    crate::require(
        matches!(
            owner.ingest(GENERATION, raw.as_bytes()),
            Err(Failure::Overflow)
        ),
        "retained byte overflow hidden",
    )?;
    let mut owner = Connection::new(
        GENERATION,
        Budget {
            frame: Limits {
                bytes: 80,
                depth: 4,
            },
            ..Default::default()
        },
    );
    let _handle = owner
        .subscribe(Root, SUBSCRIBE_DEADLINE)
        .map_err(|_| "frame fixture subscribe failed")?;
    owner.outbound().ok_or("frame fixture absent")?;
    crate::require(
        matches!(
            owner.ingest(GENERATION, raw.as_bytes()),
            Err(Failure::Protocol(_))
        ),
        "frame byte bound missing",
    )
}

fn framing() -> Check {
    for raw in [
        r#"{"jsonrpc":"2.0","id":1,"id":1,"result":7}"#,
        r#"{"jsonrpc":"2.0","id":1,"result":[[[[[[0]]]]]]}"#,
        r#"{"jsonrpc":"2.0","id":1,"result":null}"#,
    ] {
        let mut owner = Connection::new(
            GENERATION,
            Budget {
                frame: Limits {
                    bytes: 1024,
                    depth: 4,
                },
                ..Default::default()
            },
        );
        let _handle = owner
            .subscribe(Root, SUBSCRIBE_DEADLINE)
            .map_err(|_| "framing subscribe failed")?;
        owner.outbound().ok_or("framing subscribe frame absent")?;
        crate::require(
            owner.ingest(GENERATION, raw.as_bytes()).is_err(),
            "malformed acknowledgement accepted",
        )?;
    }

    Ok(())
}

fn deadlines() -> Check {
    let mut owner = Connection::new(GENERATION, Budget::default());
    let handle = owner
        .subscribe(Root, SUBSCRIBE_DEADLINE)
        .map_err(|_| "deadline subscribe failed")?;
    owner.outbound().ok_or("deadline frame absent")?;
    owner
        .tick(SUBSCRIBE_DEADLINE, CLEANUP_DEADLINE)
        .map_err(|_| "deadline tick failed")?;
    crate::require(
        matches!(owner.next(&handle), Err(Failure::Deadline)),
        "quiet subscribe deadline hidden",
    )?;
    owner
        .forget(handle)
        .map_err(|_| "deadline cleanup failed")?;
    let handle = owner
        .subscribe(Root, CLEANUP_DEADLINE)
        .map_err(|_| "deadline capacity not released")?;
    owner.outbound().ok_or("second deadline frame absent")?;

    ack(&mut owner, 2, "7")?;
    owner
        .unsubscribe(&handle, UNSUBSCRIBE_DEADLINE)
        .map_err(|_| "unsubscribe deadline setup failed")?;
    wire(
        &mut owner,
        r#"{"jsonrpc":"2.0","id":3,"method":"rootUnsubscribe","params":[7]}"#,
    )?;
    owner
        .tick(UNSUBSCRIBE_DEADLINE, NEXT_DEADLINE)
        .map_err(|_| "unsubscribe deadline tick failed")?;
    crate::require(
        matches!(owner.next(&handle), Err(Failure::Deadline)),
        "quiet unsubscribe deadline hidden",
    )
}

pub(super) fn bounds() -> Check {
    schemas()?;
    limits()?;
    framing()?;
    deadlines()
}
