//! Actual browser HTTP, WebSocket and Promise progress with local ownership.
use futures::{FutureExt, poll};
use solana_kit_browser::{
    BrowserHttp, BrowserSubscriptionSocket, BrowserWebSocket, HttpConfig, WsConfig,
};
use solana_kit_client::{
    Client, Limits,
    confirmation::{FactoryConfig, Policy, confirm_signature_shared},
    subscriptions::{Budget, Clock, ClockError, Drain, Root, Session},
};
use solana_kit_core::{Hash, Signature, transaction::Lifetime};
use std::{
    cell::RefCell,
    future::{Future, pending},
    num::NonZeroU64,
    pin::Pin,
    rc::Rc,
    time::Duration,
};
use wasm_bindgen_futures::JsFuture;
use wasm_bindgen_test::wasm_bindgen_test;
const DEADLINE: u64 = 2_000;
const GENERATION: u64 = 7;
const LOCAL_MARKER: u32 = 73;
const PROMISE_DELAY: i32 = 5;
const ROOT_VALUE: u64 = 99;
const MILLIS_PER_SECOND: f64 = 1_000.0;
type Owner = RefCell<Session<BrowserSubscriptionSocket>>;

async fn pause(millis: i32) {
    let promise = js_sys::Promise::new(&mut |resolve, _| {
        web_sys::window()
            .unwrap()
            .set_timeout_with_callback_and_timeout_and_arguments_0(&resolve, millis)
            .unwrap();
    });
    JsFuture::from(promise).await.unwrap();
}
struct BrowserClock(f64);
impl Clock for BrowserClock {
    type Sleep<'a> = Pin<Box<dyn Future<Output = Result<(), ClockError>> + 'a>>;
    fn now(&self) -> u64 {
        let elapsed = web_sys::window().unwrap().performance().unwrap().now() - self.0;
        Duration::try_from_secs_f64(elapsed / MILLIS_PER_SECOND)
            .unwrap()
            .as_millis()
            .try_into()
            .unwrap()
    }
    fn sleep_until(&self, deadline: u64) -> Self::Sleep<'_> {
        Box::pin(async move {
            let delay = deadline.saturating_sub(self.now());
            if delay > 0 {
                pause(i32::try_from(delay).unwrap()).await;
            }

            Ok(())
        })
    }
}
#[wasm_bindgen_test]
async fn actual_network_factory_holds_rc_across_pending_promise_and_preserves_root() {
    let socket = BrowserWebSocket::connect(env!("SOLANA_FACTORY_WS"), WsConfig::default())
        .await
        .unwrap();
    let owner = RefCell::new(Session::new(
        BrowserSubscriptionSocket::from(socket),
        GENERATION,
        Budget::default(),
    ));
    let clock = BrowserClock(web_sys::window().unwrap().performance().unwrap().now());
    let root = owner.borrow_mut().subscribe(Root, DEADLINE).unwrap();
    let client = Client::new(
        BrowserHttp::new(env!("SOLANA_FACTORY_HTTP"), HttpConfig::default()).unwrap(),
        Limits::default(),
    );
    let local = Rc::new(LOCAL_MARKER);
    pending_journey(&client, &owner, &clock, local).await;
    let mut session = owner.into_inner();

    assert!(matches!(
        session.next(&root, &clock, DEADLINE).await.unwrap(),
        Drain::Notification(ROOT_VALUE)
    ));
    session.unsubscribe(root, &clock, DEADLINE).await.unwrap();
    session.shutdown(&clock, DEADLINE).await.unwrap();
}
async fn pending_journey(
    client: &Client<BrowserHttp>,
    owner: &Owner,
    clock: &BrowserClock,
    local: Rc<u32>,
) {
    let retained = local.clone();
    let config = FactoryConfig {
        policy: Policy::default(),
        deadline: DEADLINE,
        poll_interval: NonZeroU64::new(10).unwrap(),
        cleanup_millis: NonZeroU64::new(100).unwrap(),
    };

    {
        let operation = async {
            confirm_signature_shared(
                client,
                owner,
                clock,
                Signature::from_bytes([1; 64]),
                Lifetime::Blockhash {
                    blockhash: Hash::from_bytes([0; 32]),
                    last_valid_block_height: 100,
                },
                config,
                pending(),
            )
            .await
            .unwrap();
            assert_eq!(*retained, LOCAL_MARKER);
        }
        .fuse();
        futures::pin_mut!(operation);
        assert!(poll!(operation.as_mut()).is_pending());
        assert!(
            owner.try_borrow_mut().is_ok(),
            "no browser RefCell borrow held across await"
        );
        let promise = pause(PROMISE_DELAY).fuse();
        futures::pin_mut!(promise);
        assert!(poll!(promise.as_mut()).is_pending());

        promise.await;
        assert_eq!(Rc::strong_count(&local), 2);
        operation.await;
    }
}
