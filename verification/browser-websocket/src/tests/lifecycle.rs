use super::*;

struct DropTerminal {
    drops: Arc<AtomicUsize>,
    wakes: Arc<AtomicUsize>,
}

impl Wake for DropTerminal {
    fn wake(self: Arc<Self>) {
        self.wakes.fetch_add(1, Ordering::SeqCst);
    }
}

impl Drop for DropTerminal {
    fn drop(&mut self) {
        self.drops.fetch_add(1, Ordering::SeqCst);
        drop_terminal();
    }
}

fn register(
    future: std::pin::Pin<&mut (impl std::future::Future + ?Sized)>,
    drops: &Arc<AtomicUsize>,
    wakes: &Arc<AtomicUsize>,
) {
    let waker = Waker::from(Arc::new(DropTerminal {
        drops: drops.clone(),
        wakes: wakes.clone(),
    }));
    let mut context = Context::from_waker(&waker);
    assert!(future.poll(&mut context).is_pending());
}

#[wasm_bindgen_test]
async fn lifecycle_cancel_drops_waker_outside_state_borrow() {
    let mut socket = connect("/quiet").await;
    capture_terminal();
    let drops = Arc::new(AtomicUsize::new(0));
    let wakes = Arc::new(AtomicUsize::new(0));
    let mut receive = socket.receive().boxed_local();
    register(receive.as_mut(), &drops, &wakes);
    assert_eq!(drops.load(Ordering::SeqCst), 0);

    drop(receive);
    assert_eq!(drops.load(Ordering::SeqCst), 1);
    assert_eq!(wakes.load(Ordering::SeqCst), 0);
    assert_eq!(handlers(), 0);
    assert_eq!(socket.receive().await, Err(WsError::Browser));
    JsFuture::from(closed()).await.unwrap();
}

#[wasm_bindgen_test]
async fn lifecycle_replacement_drops_waker_outside_state_borrow() {
    let mut socket = connect("/quiet").await;
    capture_terminal();
    let drops = Arc::new(AtomicUsize::new(0));
    let wakes = Arc::new(AtomicUsize::new(0));
    let mut receive = socket.receive().boxed_local();
    register(receive.as_mut(), &drops, &wakes);
    let counter = Arc::new(Counter(AtomicUsize::new(0)));
    let waker = Waker::from(counter.clone());
    let mut context = Context::from_waker(&waker);

    assert!(receive.as_mut().poll(&mut context).is_pending());
    assert_eq!(drops.load(Ordering::SeqCst), 1);
    assert_eq!(wakes.load(Ordering::SeqCst), 0);
    assert_eq!(counter.0.load(Ordering::SeqCst), 1);
    assert_eq!(receive.await, Err(WsError::Browser));
    assert_eq!(handlers(), 0);
    JsFuture::from(closed()).await.unwrap();
}

#[wasm_bindgen_test]
async fn lifecycle_ready_result_does_not_retain_waker() {
    let mut socket = connect("/echo").await;
    capture_terminal();
    let message = message_event();
    socket.send("go").unwrap();
    JsFuture::from(message).await.unwrap();
    let drops = Arc::new(AtomicUsize::new(0));
    let wakes = Arc::new(AtomicUsize::new(0));
    let waker = Waker::from(Arc::new(DropTerminal {
        drops: drops.clone(),
        wakes: wakes.clone(),
    }));
    let mut context = Context::from_waker(&waker);
    let mut receive = socket.receive().boxed_local();

    assert_eq!(
        receive.as_mut().poll(&mut context),
        Poll::Ready(Ok("resumed".into()))
    );
    drop(receive);
    drop(waker);

    assert_eq!(drops.load(Ordering::SeqCst), 1);
    assert_eq!(wakes.load(Ordering::SeqCst), 0);
    assert_eq!(socket.receive().await, Err(WsError::Browser));
    assert_eq!(handlers(), 0);
    JsFuture::from(closed()).await.unwrap();
}

#[wasm_bindgen_test]
async fn lifecycle_terminal_wakes_once_and_preserves_first_cause() {
    let mut socket = connect("/quiet").await;
    capture_terminal();
    let drops = Arc::new(AtomicUsize::new(0));
    let wakes = Arc::new(AtomicUsize::new(0));
    let mut receive = socket.receive().boxed_local();
    register(receive.as_mut(), &drops, &wakes);

    blob_event();
    assert_eq!(drops.load(Ordering::SeqCst), 1);
    assert_eq!(wakes.load(Ordering::SeqCst), 1);
    assert_eq!(receive.await, Err(WsError::Binary));
    assert_eq!(socket.receive().await, Err(WsError::Binary));
    assert_eq!(handlers(), 0);
    JsFuture::from(closed()).await.unwrap();
}

#[wasm_bindgen_test]
async fn lifecycle_connect_cancellation_drops_registered_waker() {
    capture();
    let endpoint = endpoint("/stall");
    let mut connecting = BrowserWebSocket::connect(&endpoint, config()).boxed_local();
    let drops = Arc::new(AtomicUsize::new(0));
    let wakes = Arc::new(AtomicUsize::new(0));
    register(connecting.as_mut(), &drops, &wakes);
    capture_terminal();
    super::handshake_observed().await;

    drop(connecting);
    assert_eq!(drops.load(Ordering::SeqCst), 1);
    assert_eq!(wakes.load(Ordering::SeqCst), 0);
    assert_eq!(handlers(), 0);
    JsFuture::from(closed()).await.unwrap();
}

#[wasm_bindgen_test]
async fn lifecycle_close_replacement_drops_registered_waker() {
    let socket = connect("/quiet").await;
    capture_terminal();
    let drops = Arc::new(AtomicUsize::new(0));
    let wakes = Arc::new(AtomicUsize::new(0));
    let mut closing = socket.close().boxed_local();
    register(closing.as_mut(), &drops, &wakes);
    let counter = Arc::new(Counter(AtomicUsize::new(0)));
    let waker = Waker::from(counter.clone());
    let mut context = Context::from_waker(&waker);

    assert!(closing.as_mut().poll(&mut context).is_pending());
    assert_eq!(drops.load(Ordering::SeqCst), 1);
    assert_eq!(wakes.load(Ordering::SeqCst), 0);
    assert_eq!(counter.0.load(Ordering::SeqCst), 1);
    assert_eq!(closing.await, Err(WsError::Browser));
    assert_eq!(handlers(), 0);
    JsFuture::from(closed()).await.unwrap();
}
