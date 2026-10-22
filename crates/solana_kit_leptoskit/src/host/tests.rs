//! Actual upstream memory guards and host admission never turn errors into data.
use super::*;
use leptoskit_native::HeadlessBackend;

fn host() -> HttpHost<HeadlessBackend> {
    let wasm = wat::parse_str(
        r#"(module
        (memory (export "memory") 1)
        (func (export "leptoskit_main"))
        (func (export "leptoskit_event") (param i64 i32 i32 i32) (result i32) i32.const 0)
        (func (export "leptoskit_plugins_ready")))"#,
    )
    .unwrap();
    HttpHost::new(
        WasmHost::headless(&wasm).unwrap(),
        HostLimits {
            jobs: 1,
            request_bytes: 16,
            reply_bytes: 16,
        },
    )
    .unwrap()
}

#[tokio::test]
async fn malformed_and_overbudget_requests_complete_as_typed_failures() {
    for (payload, code) in [
        (vec![], 7),
        (vec![0; 3], 7),
        (vec![0; 4], 5),
        (vec![17, 0, 0, 0, b'{', b'}'], 5),
        (vec![1; 21], 5),
    ] {
        let mut app = host();
        let native = NativeHttp::new("http://127.0.0.1:1/", Default::default()).unwrap();
        let plugin = app.install("fixed", native, true).unwrap();
        let bridge = app.host_mut().unwrap().plugins();
        let call = bridge.submit(plugin, wire::POST, payload.into()).unwrap();
        let routed = app.drain().unwrap();
        assert!(routed.guest_woken);
        assert_eq!(app.retained_jobs(), 0);
        let reply = app
            .host_mut()
            .unwrap()
            .plugins()
            .take_result(call)
            .unwrap()
            .unwrap()
            .unwrap();
        assert_eq!(reply.as_ref(), &[1, code]);
        app.shutdown().await.unwrap();
    }
}

#[test]
fn upstream_rejects_foreign_guest_ranges_before_reading() {
    for (pointer, length) in [(65535, 4), (i32::MAX, 4), (-1, 1), (0, -1)] {
        let wasm = wat::parse_str(format!(r#"(module
            (import "leptoskit_plugins_v1" "submit" (func $submit (param i32 i32 i32 i32) (result i64)))
            (memory (export "memory") 1)
            (func (export "leptoskit_main")
                i32.const 0 i32.const 0 i32.const {pointer} i32.const {length}
                call $submit drop)
            (func (export "leptoskit_event") (param i64 i32 i32 i32) (result i32) i32.const 0))"#)).unwrap();
        let mut host = WasmHost::headless(&wasm).unwrap();
        let plugin = host.plugins().register("fixed", [wire::POST]).unwrap();
        host.plugins().grant(plugin, wire::POST, true).unwrap();
        let result = host.start(1.0, 1.0);

        if length < 0 {
            assert!(result.is_ok());
        } else {
            assert!(
                result
                    .unwrap_err()
                    .to_string()
                    .contains("plugin payload outside guest memory")
            );
        }

        assert!(host.plugins().next_request().is_none());
        assert_eq!(host.plugins().buffered_bytes(), 0);
    }
}

#[tokio::test]
async fn unrelated_requests_and_cancellations_remain_application_owned() {
    let mut app = host();
    let plugin = app
        .host_mut()
        .unwrap()
        .plugins()
        .register("other", [wire::POST])
        .unwrap();
    app.host_mut()
        .unwrap()
        .plugins()
        .grant(plugin, wire::POST, true)
        .unwrap();
    let call = app
        .host_mut()
        .unwrap()
        .plugins()
        .submit(plugin, wire::POST, Bytes::from_static(b"other"))
        .unwrap();
    let routed = app.drain().unwrap();
    assert_eq!(routed.requests.len(), 1);
    assert_eq!(routed.requests[0].payload.as_ref(), b"other");
    app.host_mut().unwrap().plugins().cancel(call).unwrap();

    assert_eq!(app.drain().unwrap().cancelled, [call]);
    assert_eq!(app.retained_jobs(), 0);
    app.shutdown().await.unwrap();
}

#[tokio::test]
async fn stale_generation_never_wakes_a_replacement() {
    let mut old = host();
    let mut replacement = host();
    assert_ne!(old.instance, replacement.instance);
    let token = Token {
        instance: old.instance,
        call: CallId(1),
    };
    let abort = replacement
        .tasks
        .spawn(async move { (token, vec![0, 200, 0]) });
    replacement.jobs.insert(abort.id(), Job { token, abort });
    assert!(matches!(
        replacement.next_completion().await,
        Err(HostError::Task)
    ));
    assert_eq!(replacement.retained_jobs(), 0);

    assert_eq!(replacement.host_mut().unwrap().commits(), 0);
    old.shutdown().await.unwrap();
    replacement.shutdown().await.unwrap();
}

#[tokio::test]
async fn completed_native_failure_after_cancellation_is_joined_without_guest_entry() {
    let mut app = host();
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    drop(listener);
    let native = NativeHttp::new(&format!("http://{address}/"), Default::default()).unwrap();
    let plugin = app.install("fixed", native, true).unwrap();
    let call = app
        .host_mut()
        .unwrap()
        .plugins()
        .submit(plugin, wire::POST, Bytes::from_static(b"\x10\0\0\0{}"))
        .unwrap();
    app.drain().unwrap();

    tokio::time::timeout(std::time::Duration::from_secs(2), async {
        while !app.jobs.values().next().unwrap().abort.is_finished() {
            tokio::task::yield_now().await;
        }
    })
    .await
    .unwrap();
    assert_eq!(app.retained_jobs(), 1);
    app.host_mut().unwrap().plugins().cancel(call).unwrap();
    app.drain().unwrap();
    assert_eq!(app.retained_jobs(), 1);
    assert!(app.next_completion().await.unwrap());
    assert_eq!(app.retained_jobs(), 0);
    assert_eq!(app.host_mut().unwrap().commits(), 0);

    assert_eq!(app.host_mut().unwrap().plugins().buffered_bytes(), 0);
    app.shutdown().await.unwrap();
}

#[test]
fn upstream_rejects_response_destination_without_consuming_ready_bytes() {
    let wasm = wat::parse_str(
        r#"(module
        (import "leptoskit_plugins_v1" "submit" (func $submit (param i32 i32 i32 i32) (result i64)))
        (import "leptoskit_plugins_v1" "take_result" (func $take (param i64 i32 i32) (result i64)))
        (memory (export "memory") 1)
        (data (i32.const 0) "{}")
        (func (export "leptoskit_main")
            i32.const 0 i32.const 0 i32.const 0 i32.const 2 call $submit drop)
        (func (export "leptoskit_event") (param i64 i32 i32 i32) (result i32) i32.const 0)
        (func (export "leptoskit_plugins_ready")
            i64.const 1 i32.const 65535 i32.const 4 call $take drop))"#,
    )
    .unwrap();
    let mut host = WasmHost::headless(&wasm).unwrap();
    let plugin = host.plugins().register("fixed", [wire::POST]).unwrap();
    host.plugins().grant(plugin, wire::POST, true).unwrap();
    host.start(1.0, 1.0).unwrap();
    let request = host.plugins().next_request().unwrap();
    host.plugins()
        .complete(request.id, Ok(Bytes::from_static(b"okay")))
        .unwrap();
    let error = host.wake_plugins().unwrap_err();

    assert!(
        error
            .to_string()
            .contains("plugin response destination outside guest memory")
    );
    let ready = host
        .plugins()
        .result(request.id)
        .unwrap()
        .unwrap()
        .as_ref()
        .unwrap();
    assert_eq!(ready.as_ref(), b"okay");
    assert_eq!(host.plugins().buffered_bytes(), 4);
}
