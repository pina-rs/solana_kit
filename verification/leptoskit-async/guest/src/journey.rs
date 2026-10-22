//! Actual guest awaits entropy, monotonic time and independently routed cancellation.

use crate::application::record;
use crate::capabilities::{EntropyError, entropy, pending};
use futures::channel::oneshot;
use futures::future::{Either, select};
use leptoskit_bridge::guest::{Client, Events};
use leptoskit_bridge::{Error, MethodId};
use std::rc::Rc;

const DEADLINE: u64 = 10;
const ENTROPY: [u8; 4] = [9, 8, 7, 6];
const SENTINEL: [u8; 4] = [42; 4];

/// Proves Rc remains live across a true suspension, without any Send wrapper.
pub(super) async fn run(client: Client, cancellation: oneshot::Receiver<()>) {
    let local = Rc::new(ENTROPY);
    let mut output = SENTINEL;
    entropy(client, &mut output, "entropy-pending")
        .await
        .expect("complete entropy");
    assert_eq!(output, *local);
    record("entropy-complete");

    output = SENTINEL;
    assert_eq!(
        entropy(client, &mut output, "short-pending").await,
        Err(EntropyError::Length)
    );
    assert_eq!(output, SENTINEL);
    record("short-rejected");
    assert_eq!(
        entropy(client, &mut output, "unavailable-pending").await,
        Err(EntropyError::Bridge(Error::Unavailable))
    );
    assert_eq!(output, SENTINEL);
    record("unavailable-reply");

    timer(client).await;
    cancel_quiet(client, cancellation).await;
    let unload = client.call(MethodId(2), &[]).expect("unload work admitted");
    let _never = pending(unload, "unload-pending").await;
    panic!("unloaded operation must never receive a completion");
}

/// Checks the exact injected monotonic tick, with no ambient clock access.
async fn timer(client: Client) {
    let call = client
        .call(MethodId(1), &DEADLINE.to_le_bytes())
        .expect("timer admitted");
    let bytes = pending(call, "timer-pending")
        .await
        .expect("timer response");
    let tick = u64::from_le_bytes(bytes.as_ref().try_into().expect("complete tick"));
    assert_eq!(tick, DEADLINE);
    record("timer-complete");
}

/// Dropping the losing Call invokes upstream cancellation and removes its waker.
async fn cancel_quiet(client: Client, cancellation: oneshot::Receiver<()>) {
    let call = client
        .call(MethodId(2), &[])
        .expect("quiet operation admitted");
    let waiting = Box::pin(pending(call, "quiet-pending"));

    match select(waiting, cancellation).await {
        Either::Right((Ok(()), work)) => drop(work),
        _ => panic!("quiet operation was not cancelled by application event"),
    }

    record("quiet-cancelled");
    drop(Events::take().expect("application receiver registration released"));
}
