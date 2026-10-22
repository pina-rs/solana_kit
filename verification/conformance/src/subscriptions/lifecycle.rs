//! Early delivery, quiet cancellation and generation replacement through public I/O.

use super::*;

fn cycles() -> Check {
    let mut owner = Connection::new(
        GENERATION,
        Budget {
            subscriptions: 1,
            messages: 1,
            queued_bytes: 95,
            ..Default::default()
        },
    );

    for cycle in 0..32u32 {
        cycle_once(&mut owner, cycle * 2 + 1)?;
    }

    Ok(())
}

fn cycle_once(owner: &mut Connection, id: u32) -> Check {
    let handle = owner
        .subscribe(Root, SUBSCRIBE_DEADLINE)
        .map_err(|_| "released registration capacity unavailable")?;
    wire(
        owner,
        &format!(r#"{{"jsonrpc":"2.0","id":{id},"method":"rootSubscribe","params":[]}}"#),
    )?;
    crate::require(
        matches!(
            owner.subscribe(Root, SUBSCRIBE_DEADLINE),
            Err(Failure::Capacity)
        ),
        "registration bound missing",
    )?;
    notify(owner, "rootNotification", SERVER, "9007199254740993")?;
    ack(owner, id, "7")?;
    crate::require(
        value(owner, &handle)? == 9007199254740993,
        "early delivery lost",
    )?;

    owner.cancel(&handle).map_err(|_| "cancel failed")?;
    owner
        .tick(1, CLEANUP_DEADLINE)
        .map_err(|_| "quiet cancellation tick failed")?;
    let unsubscribe = id + 1;
    wire(
        owner,
        &format!(
            r#"{{"jsonrpc":"2.0","id":{unsubscribe},"method":"rootUnsubscribe","params":[7]}}"#
        ),
    )?;
    ack(owner, unsubscribe, "true")?;
    crate::require(
        matches!(owner.next(&handle), Ok(Drain::Closed)),
        "cancel did not close",
    )?;
    owner
        .forget(handle)
        .map_err(|_| "ended registration cleanup failed")?;
    Ok(())
}

fn late_ack() -> Check {
    let mut owner = Connection::new(GENERATION, Budget::default());
    let handle = owner
        .subscribe(Root, SUBSCRIBE_DEADLINE)
        .map_err(|_| "subscribe failed")?;
    wire(
        &mut owner,
        r#"{"jsonrpc":"2.0","id":1,"method":"rootSubscribe","params":[]}"#,
    )?;
    owner.cancel(&handle).map_err(|_| "early cancel failed")?;
    notify(&mut owner, "rootNotification", SERVER, "5")?;
    ack(&mut owner, 1, "7")?;
    crate::require(
        matches!(owner.next(&handle), Ok(Drain::Closed)),
        "cancel delivered late notification",
    )?;
    owner
        .tick(2, CLEANUP_DEADLINE)
        .map_err(|_| "late acknowledgement cleanup failed")?;

    wire(
        &mut owner,
        r#"{"jsonrpc":"2.0","id":2,"method":"rootUnsubscribe","params":[7]}"#,
    )?;
    ack(&mut owner, 2, "false")?;
    crate::require(
        matches!(owner.next(&handle), Err(Failure::UnsubscribeFalse)),
        "unsubscribe false was hidden",
    )?;
    owner
        .forget(handle)
        .map_err(|_| "late acknowledgement cleanup retained state")
}

fn replacement() -> Check {
    let mut owner = Connection::new(GENERATION, Budget::default());
    let old = owner
        .subscribe(Root, SUBSCRIBE_DEADLINE)
        .map_err(|_| "subscribe failed")?;
    owner.outbound().ok_or("subscribe frame absent")?;
    let gap = owner.replace(10).map_err(|_| "replacement failed")?;
    crate::require(
        gap.previous == GENERATION
            && gap.replacement == 10
            && matches!(owner.status(&old), Err(Failure::Closed)),
        "replacement gap missing",
    )?;
    let new = owner
        .subscribe(Root, SUBSCRIBE_DEADLINE)
        .map_err(|_| "replacement subscribe failed")?;
    owner.outbound().ok_or("replacement frame absent")?;
    crate::require(
        matches!(
            owner.ingest(GENERATION, br#"{"jsonrpc":"2.0","id":1,"result":7}"#),
            Err(Failure::Closed)
        ),
        "stale generation accepted",
    )?;

    owner
        .ingest(10, br#"{"jsonrpc":"2.0","id":1,"result":8}"#)
        .map_err(|_| "stale frame closed replacement")?;
    owner.ingest(10, br#"{"jsonrpc":"2.0","method":"rootNotification","params":{"subscription":8,"result":6}}"#).map_err(|_| "replacement notification failed")?;
    crate::require(
        value(&mut owner, &new)? == REPLACEMENT_VALUE && owner.replace(10).is_err(),
        "replacement routing or monotonicity differs",
    )
}

pub(super) fn lifecycle() -> Check {
    cycles()?;
    late_ack()?;
    replacement()
}
