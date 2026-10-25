//! Cancellation retains capacity until native socket ownership really ends.
use super::*;
const PENDING_WINDOW: Duration = Duration::from_millis(20);
const MALFORMED: u8 = 5;
const OTHER: MethodId = MethodId(7);
const LEASE_BYTES: usize = core::mem::size_of::<u64>();

fn reply(host: &mut Host, call: CallId) -> Bytes {
    host.plugins().take_result(call).unwrap().unwrap().unwrap()
}

#[tokio::test]
async fn idle_lease_drop_cancels_pending_wait_and_joins_before_reuse() {
    let (endpoint, peer, _signal) = fixture(false).await;
    let mut host = host();
    let mut driver = driver(&mut host);
    let plugin = install(&mut host, &mut driver, endpoint);
    let lease = open(&mut host, &mut driver, plugin).await;
    assert!(
        tokio::time::timeout(PENDING_WINDOW, driver.next_completion())
            .await
            .is_err()
    );
    host.plugins().cancel(lease).unwrap();
    route(&mut host, &mut driver);

    assert_eq!(driver.retained_sockets(), 1);
    let blocked = submit(&mut host, plugin, 1, Vec::new());
    route(&mut host, &mut driver);
    assert!(matches!(
        host.plugins().take_result(blocked).unwrap(),
        Some(Err(leptoskit_bridge::Error::Busy))
    ));
    let completion = next(&mut driver).await;
    driver.deliver(&mut host, completion).unwrap();
    assert_eq!(driver.retained_sockets(), 0);
    assert!(host.plugins().event().is_none());

    joined(peer).await;
    driver.shutdown().await.unwrap();
    driver.shutdown().await.unwrap();
}

#[tokio::test]
async fn cancelled_owner_discards_a_retained_late_frame() {
    let (endpoint, peer, signal) = fixture(true).await;
    let mut host = host();
    let mut driver = driver(&mut host);
    let plugin = install(&mut host, &mut driver, endpoint);
    let lease = open(&mut host, &mut driver, plugin).await;
    signal.send(()).unwrap();
    let late = next(&mut driver).await;
    host.plugins().cancel(lease).unwrap();

    route(&mut host, &mut driver);
    driver.deliver(&mut host, late).unwrap();
    assert!(host.plugins().event().is_none());
    let joined_completion = next(&mut driver).await;
    driver.deliver(&mut host, joined_completion).unwrap();
    assert_eq!(driver.retained_sockets(), 0);
    driver.shutdown().await.unwrap();
    joined(peer).await;
}

#[tokio::test]
async fn invalid_and_oversize_commands_are_explicit_and_shutdown_closes_idle_io() {
    let (endpoint, peer, _signal) = fixture(false).await;
    let mut host = host();
    let mut driver = driver(&mut host);
    let plugin = install(&mut host, &mut driver, endpoint);
    let lease = open(&mut host, &mut driver, plugin).await;
    let malformed = submit(&mut host, plugin, 2, vec![0]);
    let oversized = submit(&mut host, plugin, 2, command(lease, &[b'x'; 65]));
    route(&mut host, &mut driver);

    assert_eq!(reply(&mut host, malformed).as_ref(), &[MALFORMED]);
    assert_eq!(reply(&mut host, oversized).as_ref(), &[2]);
    driver.shutdown().await.unwrap();
    assert_eq!(driver.retained_sockets(), 0);
    joined(peer).await;
}

#[tokio::test]
async fn unrelated_requests_cancellation_and_wake_flags_survive_routing() {
    let mut host = host();
    let mut driver = driver(&mut host);
    let plugin = host.plugins().register("other", [OTHER]).unwrap();
    host.plugins().grant(plugin, OTHER, true).unwrap();
    let call = host
        .plugins()
        .submit(plugin, OTHER, Bytes::from_static(b"other"))
        .unwrap();
    let request = host.plugins().next_request().unwrap();
    host.plugins().cancel(call).unwrap();
    let cancelled = host.plugins().next_cancelled().unwrap();

    let routed = solana_kit_leptoskit::Routed {
        requests: vec![request],
        cancelled: vec![cancelled],
        guest_woken: true,
    };
    let result = driver.route(&mut host, routed).unwrap();
    assert_eq!(result.requests[0].payload.as_ref(), b"other");
    assert_eq!(result.cancelled, vec![call]);
    assert!(result.guest_woken);
    driver.shutdown().await.unwrap();
}

#[tokio::test]
async fn cancelled_bridge_lease_blocks_delivery_before_cancellation_drain() {
    let (endpoint, peer, signal) = fixture(true).await;
    let mut host = host();
    let mut driver = driver(&mut host);
    let plugin = install(&mut host, &mut driver, endpoint);
    let lease = open(&mut host, &mut driver, plugin).await;
    signal.send(()).unwrap();
    let late = next(&mut driver).await;

    host.plugins().cancel(lease).unwrap();
    driver.deliver(&mut host, late).unwrap();
    assert!(host.plugins().event().is_none());
    route(&mut host, &mut driver);
    let completion = next(&mut driver).await;
    driver.deliver(&mut host, completion).unwrap();
    assert_eq!(driver.retained_sockets(), 0);
    driver.shutdown().await.unwrap();

    joined(peer).await;
}

#[tokio::test]
async fn cancelled_send_aborts_socket_instead_of_retrying_uncertain_work() {
    let (endpoint, peer, _signal) = fixture(false).await;
    let mut host = host();
    let mut driver = driver(&mut host);
    let plugin = install(&mut host, &mut driver, endpoint);
    let lease = open(&mut host, &mut driver, plugin).await;
    let call = submit(&mut host, plugin, 2, command(lease, b"never retry"));
    let queued = submit(&mut host, plugin, 2, command(lease, b"also pending"));
    route(&mut host, &mut driver);

    host.plugins().cancel(call).unwrap();
    route(&mut host, &mut driver);
    assert_eq!(driver.retained_sockets(), 1);
    let completion = next(&mut driver).await;
    driver.deliver(&mut host, completion).unwrap();
    assert_eq!(driver.retained_sockets(), 0);
    assert_eq!(reply(&mut host, lease).as_ref(), &[4]);
    assert_eq!(reply(&mut host, queued).as_ref(), &[4]);

    driver.shutdown().await.unwrap();

    joined(peer).await;
}

#[tokio::test]
async fn full_event_queue_still_completes_real_close_and_terminal_lease() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let endpoint = format!("ws://{}/", listener.local_addr().unwrap());
    let peer = tokio::spawn(async move {
        let mut socket = accept_async(listener.accept().await.unwrap().0)
            .await
            .unwrap();
        assert!(matches!(socket.next().await, Some(Ok(Message::Close(_)))));

        match socket.flush().await {
            Ok(()) | Err(tokio_tungstenite::tungstenite::Error::ConnectionClosed) => {}
            Err(error) => panic!("close response failed: {error}"),
        }
    });
    let mut host = host();
    *host.plugins() = leptoskit_bridge::Bridge::new(leptoskit_bridge::Limits {
        calls: 4,
        events: 1,
        payload_bytes: 1024,
        buffered_bytes: 4096,
    });
    let mut driver = driver(&mut host);
    let plugin = install(&mut host, &mut driver, endpoint);
    let lease = open(&mut host, &mut driver, plugin).await;
    let close = submit(&mut host, plugin, 3, command(lease, &[]));
    host.plugins()
        .emit(leptoskit_bridge::Event {
            plugin,
            kind: 99,
            payload: Bytes::new(),
        })
        .unwrap();
    route(&mut host, &mut driver);
    let completion = next(&mut driver).await;

    assert!(driver.deliver(&mut host, completion).is_err());
    assert_eq!(reply(&mut host, close).as_ref(), &[0]);
    assert_eq!(reply(&mut host, lease).as_ref(), &[2]);
    assert_eq!(host.plugins().next_event().unwrap().kind, 99);
    assert_eq!(driver.retained_sockets(), 0);
    driver.shutdown().await.unwrap();
    joined(peer).await;
}

#[tokio::test]
async fn actual_binary_notice_preserves_wire_identity_and_awaited_close() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let endpoint = format!("ws://{}/", listener.local_addr().unwrap());
    let peer = tokio::spawn(async move {
        let mut socket = accept_async(listener.accept().await.unwrap().0)
            .await
            .unwrap();
        socket
            .send(Message::Binary(vec![0, 255, 7].into()))
            .await
            .unwrap();
        assert!(matches!(socket.next().await, Some(Ok(Message::Close(_)))));

        match socket.flush().await {
            Ok(()) | Err(tokio_tungstenite::tungstenite::Error::ConnectionClosed) => (),
            Err(error) => panic!("actual close failed: {error}"),
        }
    });
    let mut host = host();
    let mut driver = driver(&mut host);
    let plugin = install(&mut host, &mut driver, endpoint);
    let lease = open(&mut host, &mut driver, plugin).await;
    let completion = next(&mut driver).await;
    driver.deliver(&mut host, completion).unwrap();
    let event = host.plugins().next_event().unwrap();
    assert_eq!(&event.payload[..LEASE_BYTES], &lease.0.to_le_bytes());

    assert_eq!(event.payload[LEASE_BYTES], 4, "additive binary opcode");
    assert_eq!(&event.payload[LEASE_BYTES + 1..], &[0, 255, 7]);
    let close = submit(&mut host, plugin, 3, command(lease, &[]));
    route(&mut host, &mut driver);
    let completion = next(&mut driver).await;
    driver.deliver(&mut host, completion).unwrap();
    assert_eq!(driver.retained_sockets(), 0);
    assert_eq!(reply(&mut host, close).as_ref(), &[0]);

    assert_eq!(
        host.plugins().next_event().unwrap().payload[LEASE_BYTES],
        3,
        "existing terminal opcode"
    );
    driver.shutdown().await.unwrap();
    joined(peer).await;
}
