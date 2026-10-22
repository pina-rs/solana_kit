//! Actual pinned interpreter and real native peers exercise the ownership edge.
#![cfg(feature = "host")]
use leptoskit_bridge::{Bytes, CallId, MethodId};
use leptoskit_bridge_ffi::app::App;
use leptoskit_native::HeadlessBackend;
use leptoskit_wasm::WasmHost;
use solana_kit_leptoskit::{
    HostError as Error, HostLimits as Limits, HttpCompletion as Completion, HttpDriver as Driver,
};
use solana_kit_native::NativeHttp;
use std::time::Duration;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
};

const WASM: &str = r#"(module
    (memory (export "memory") 1)
    (func (export "leptoskit_main"))
    (func (export "leptoskit_plugins_ready"))
    (func (export "leptoskit_event") (param i64 i32 i32 i32) (result i32) i32.const 0))"#;

fn host() -> WasmHost<HeadlessBackend> {
    WasmHost::headless(&wat::parse_str(WASM).unwrap()).unwrap()
}

fn limits() -> Limits {
    Limits {
        jobs: 1,
        request_bytes: 16,
        reply_bytes: 16,
    }
}

fn submit(host: &mut WasmHost<HeadlessBackend>, driver: &mut Driver, address: &str) -> CallId {
    let native = NativeHttp::new(address, Default::default()).unwrap();
    let plugin = driver.install(host, "fixed", native, true).unwrap();
    host.plugins()
        .submit(plugin, MethodId(0), Bytes::from_static(b"\x10\0\0\0{}"))
        .unwrap()
}

async fn peer(listener: TcpListener) {
    let (mut socket, _) = listener.accept().await.unwrap();
    let mut headers = Vec::new();

    while !headers.ends_with(b"\r\n\r\n") {
        headers.push(socket.read_u8().await.unwrap());
        assert!(headers.len() < 1024);
    }

    let mut body = [0; 2];
    socket.read_exact(&mut body).await.unwrap();
    assert_eq!(&body, b"{}");
    socket
        .write_all(b"HTTP/1.1 200 OK\r\nContent-Length: 4\r\nConnection: close\r\n\r\nokay")
        .await
        .unwrap();
    socket.shutdown().await.unwrap();
}

async fn completion(
    host: &mut WasmHost<HeadlessBackend>,
    driver: &mut Driver,
) -> (CallId, Completion) {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = format!("http://{}/", listener.local_addr().unwrap());
    let task = tokio::spawn(peer(listener));
    let call = submit(host, driver, &address);
    driver.drain(host).unwrap();
    let completion = tokio::time::timeout(Duration::from_secs(3), driver.next_completion())
        .await
        .unwrap()
        .unwrap();
    task.await.unwrap();
    assert_eq!(driver.retained_jobs(), 0);

    assert!(host.plugins().result(call).unwrap().is_none());
    (call, completion)
}

#[tokio::test]
async fn cross_host_completion_is_rejected_before_guest_entry() {
    let mut first = host();
    let mut second = host();
    let mut driver = Driver::new(&mut first, limits()).unwrap();
    let (call, completion) = completion(&mut first, &mut driver).await;
    let mut second_driver = Driver::new(&mut second, limits()).unwrap();
    let other = submit(&mut second, &mut second_driver, "http://127.0.0.1:1/");
    assert_eq!(call, other);
    let _request = second.plugins().next_request().unwrap();

    let result = driver.deliver(&mut second, completion);
    assert!(
        matches!(result, Err(Error::Owner)),
        "foreign host accepted completion: {result:?}"
    );
    assert!(second.plugins().result(other).unwrap().is_none());
    driver.shutdown().await.unwrap();
    second_driver.shutdown().await.unwrap();
}

#[tokio::test]
async fn cross_driver_and_replaced_bridge_completions_are_rejected() {
    let mut first = host();
    let mut driver = Driver::new(&mut first, limits()).unwrap();
    let (_, completion) = completion(&mut first, &mut driver).await;
    let mut other = Driver::new(&mut first, limits()).unwrap();
    assert!(matches!(
        other.deliver(&mut first, completion),
        Err(Error::Owner)
    ));
    *first.plugins() = Default::default();
    assert!(matches!(driver.drain(&mut first), Err(Error::Owner)));
    driver.shutdown().await.unwrap();
    other.shutdown().await.unwrap();
}

#[tokio::test]
async fn transplanted_bridge_does_not_authorize_a_different_interpreter() {
    let mut first = host();
    let mut second = host();
    let mut driver = Driver::new(&mut first, limits()).unwrap();
    let (call, completion) = completion(&mut first, &mut driver).await;
    *second.plugins() = std::mem::take(first.plugins());
    assert!(second.plugins().result(call).unwrap().is_none());
    assert!(matches!(
        driver.deliver(&mut second, completion),
        Err(Error::Owner)
    ));
    assert!(second.plugins().result(call).unwrap().is_none());
    driver.shutdown().await.unwrap();
}

#[tokio::test]
async fn moved_owner_delivers_and_abandoned_completion_releases_capacity() {
    let mut first = host();
    let mut driver = Driver::new(&mut first, limits()).unwrap();
    let (call, completion) = completion(&mut first, &mut driver).await;
    let mut moved = Box::new(first);
    driver.deliver(&mut moved, completion).unwrap();
    let bytes = moved.plugins().take_result(call).unwrap().unwrap().unwrap();
    assert_eq!(bytes.as_ref(), b"\0\xc8\0okay");
    let call = moved
        .plugins()
        .submit(
            leptoskit_bridge::PluginId(0),
            MethodId(0),
            Bytes::from_static(b"\x10\0\0\0{}"),
        )
        .unwrap();

    moved.plugins().next_request().unwrap();
    moved.plugins().cancel(call).unwrap();
    driver.shutdown().await.unwrap();
    assert_eq!(driver.retained_jobs(), 0);
}

#[tokio::test]
async fn late_and_dropped_completions_retain_no_native_jobs() {
    let mut first = host();
    let mut driver = Driver::new(&mut first, limits()).unwrap();
    let (_, ready) = completion(&mut first, &mut driver).await;
    drop(ready);
    assert_eq!(driver.retained_jobs(), 0);
    driver.shutdown().await.unwrap();
    let mut second = host();
    let mut driver = Driver::new(&mut second, limits()).unwrap();

    let (_, ready) = completion(&mut second, &mut driver).await;
    driver.shutdown().await.unwrap();
    assert!(matches!(
        driver.deliver(&mut second, ready),
        Err(Error::Closed)
    ));
}

#[tokio::test]
async fn idle_wait_is_pending_and_app_hook_releases_interpreter_before_wait() {
    let mut app = App::new(&wat::parse_str(WASM).unwrap()).unwrap();
    let mut driver = app.with_host(|host| Driver::new(host, limits())).unwrap();
    assert!(
        tokio::time::timeout(Duration::from_millis(20), driver.next_completion())
            .await
            .is_err()
    );
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = format!("http://{}/", listener.local_addr().unwrap());
    let task = tokio::spawn(peer(listener));
    let call = app.with_host(|host| submit(host, &mut driver, &address));
    app.with_host(|host| driver.drain(host)).unwrap();

    let wait = tokio::spawn(async move {
        let completion = driver.next_completion().await.unwrap();
        (driver, completion)
    });
    app.with_host(|host| assert!(host.plugins().result(call).unwrap().is_none()));
    let (mut driver, completion) = wait.await.unwrap();
    app.with_host(|host| driver.deliver(host, completion))
        .unwrap();
    app.with_host(|host| {
        assert_eq!(
            host.plugins()
                .take_result(call)
                .unwrap()
                .unwrap()
                .unwrap()
                .as_ref(),
            b"\0\xc8\0okay"
        )
    });
    driver.shutdown().await.unwrap();
    task.await.unwrap();
    drop(app);
}

#[test]
fn closure_owner_shape_compiles_but_captures_the_interpreter() {
    let mut app = App::new(&wat::parse_str(WASM).unwrap()).unwrap();
    {
        let mut service_owner = || app.with_host(|host| host.commits());
        assert_eq!(service_owner(), 0);
    }
    app.with_host(|host| assert_eq!(host.commits(), 0));
}

#[tokio::test]
async fn cancellation_and_shutdown_join_real_quiet_io_before_owner_destruction() {
    for cancelled in [true, false] {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address = format!("http://{}/", listener.local_addr().unwrap());
        let (accepted, received) = tokio::sync::oneshot::channel();
        let peer = tokio::spawn(async move {
            let (mut socket, _) = listener.accept().await.unwrap();
            let mut headers = Vec::new();

            while !headers.ends_with(b"\r\n\r\n") {
                headers.push(socket.read_u8().await.unwrap());
                assert!(headers.len() < 1024);
            }

            let mut body = [0; 2];
            socket.read_exact(&mut body).await.unwrap();
            accepted.send(()).unwrap();
            assert_eq!(socket.read(&mut body).await.unwrap(), 0);
        });
        let mut host = host();
        let mut driver = Driver::new(&mut host, limits()).unwrap();
        let call = submit(&mut host, &mut driver, &address);
        driver.drain(&mut host).unwrap();
        received.await.unwrap();
        assert_eq!(driver.retained_jobs(), 1);

        if cancelled {
            host.plugins().cancel(call).unwrap();
            driver.drain(&mut host).unwrap();
            assert_eq!(driver.retained_jobs(), 1);
            let joined = driver.next_completion().await.unwrap();
            assert_eq!(driver.retained_jobs(), 0);
            driver.deliver(&mut host, joined).unwrap();
        }

        driver.shutdown().await.unwrap();
        assert_eq!(driver.retained_jobs(), 0);
        tokio::time::timeout(Duration::from_secs(2), peer)
            .await
            .unwrap()
            .unwrap();
        drop(host);
    }
}

#[tokio::test]
async fn denial_and_unrelated_work_preserve_application_dispatch() {
    let mut host = host();
    let mut driver = Driver::new(&mut host, limits()).unwrap();
    let native = NativeHttp::new("http://127.0.0.1:1/", Default::default()).unwrap();
    let denied = driver.install(&mut host, "denied", native, false).unwrap();
    assert_eq!(
        host.plugins().submit(denied, MethodId(0), Bytes::new()),
        Err(leptoskit_bridge::Error::PermissionDenied)
    );
    let plugin = host.plugins().register("unrelated", [MethodId(7)]).unwrap();
    host.plugins().grant(plugin, MethodId(7), true).unwrap();
    let call = host
        .plugins()
        .submit(plugin, MethodId(7), Bytes::from_static(b"other"))
        .unwrap();

    let routed = driver.drain(&mut host).unwrap();
    assert_eq!(routed.requests.len(), 1);
    assert_eq!(routed.requests[0].id, call);
    assert_eq!(routed.requests[0].payload.as_ref(), b"other");
    host.plugins().cancel(call).unwrap();
    assert_eq!(driver.drain(&mut host).unwrap().cancelled, [call]);
    driver.shutdown().await.unwrap();
}

#[tokio::test]
async fn out_of_order_native_results_keep_their_actual_call_identity() {
    let slow = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let fast = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let slow_address = format!("http://{}/", slow.local_addr().unwrap());
    let fast_address = format!("http://{}/", fast.local_addr().unwrap());
    let (release, gate) = tokio::sync::oneshot::channel();
    let slow_peer = tokio::spawn(async move {
        gate.await.unwrap();
        peer(slow).await;
    });
    let fast_peer = tokio::spawn(peer(fast));
    let mut host = host();

    let mut limits = limits();
    limits.jobs = 2;
    let mut driver = Driver::new(&mut host, limits).unwrap();
    let slow_call = submit(&mut host, &mut driver, &slow_address);
    let native = NativeHttp::new(&fast_address, Default::default()).unwrap();
    let plugin = driver.install(&mut host, "fast", native, true).unwrap();
    let fast_call = host
        .plugins()
        .submit(plugin, MethodId(0), Bytes::from_static(b"\x10\0\0\0{}"))
        .unwrap();
    driver.drain(&mut host).unwrap();
    let ready = driver.next_completion().await.unwrap();
    assert_eq!(driver.retained_jobs(), 1);
    driver.deliver(&mut host, ready).unwrap();
    assert!(host.plugins().result(slow_call).unwrap().is_none());
    assert_eq!(
        host.plugins()
            .take_result(fast_call)
            .unwrap()
            .unwrap()
            .unwrap()
            .as_ref(),
        b"\0\xc8\0okay"
    );
    release.send(()).unwrap();
    let ready = driver.next_completion().await.unwrap();
    driver.deliver(&mut host, ready).unwrap();
    assert_eq!(
        host.plugins()
            .take_result(slow_call)
            .unwrap()
            .unwrap()
            .unwrap()
            .as_ref(),
        b"\0\xc8\0okay"
    );
    driver.shutdown().await.unwrap();
    slow_peer.await.unwrap();
    fast_peer.await.unwrap();
}
