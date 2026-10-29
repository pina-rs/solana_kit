#![cfg(all(feature = "reactive", not(target_arch = "wasm32")))]
//! A caller-owned native executor can move the concrete stream and its next future.
use futures::{StreamExt, stream};
use solana_kit_client::{
    Presence,
    methods::{Context, Contextual},
    reactive::slot_tracking::SlotTrackingStream,
};

fn response(slot: u64, value: i32) -> Contextual<i32> {
    Contextual {
        context: Context {
            slot,
            api_version: Presence::Missing,
        },
        value,
    }
}

#[tokio::test]
async fn native_slot_stream_and_next_future_are_send() {
    let values = tokio::spawn(async {
        let initial = async { Ok::<_, ()>(response(1, 1)) };
        let subscription = stream::iter([Ok::<_, ()>(response(1, 2)), Ok(response(2, 3))]);
        let mut tracked = SlotTrackingStream::new(initial, subscription, Ok, Ok);
        let first = tracked
            .next()
            .await
            .expect("subscription value")
            .expect("valid response");
        let initial = tracked
            .next()
            .await
            .expect("initial value")
            .expect("valid response");
        let newer = tracked
            .next()
            .await
            .expect("newer value")
            .expect("valid response");
        assert!(tracked.next().await.is_none());
        (first, initial, newer)
    })
    .await
    .expect("native task joined");

    assert_eq!(values, (response(1, 2), response(1, 1), response(2, 3)));
}
