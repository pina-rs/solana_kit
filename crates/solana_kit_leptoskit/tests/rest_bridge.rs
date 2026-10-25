#![cfg(feature = "host")]

use leptoskit_bridge::{Bytes, CallId, MethodId};
use leptoskit_native::HeadlessBackend;
use leptoskit_wasm::WasmHost;
use solana_kit_leptoskit::{HostLimits, HttpDriver};
use solana_kit_native::{NativeHttp, NativeRest};
use std::{error::Error, time::Duration};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
    task::JoinSet,
};

type TestError = Box<dyn Error + Send + Sync>;
const WAIT: Duration = Duration::from_secs(3);

fn host() -> WasmHost<HeadlessBackend> {
    let wasm = wat::parse_str(
        r#"(module
        (memory (export "memory") 1)
        (func (export "leptoskit_main"))
        (func (export "leptoskit_plugins_ready"))
        (func (export "leptoskit_event") (param i64 i32 i32 i32) (result i32) i32.const 0))"#,
    )
    .expect("fixture Wasm");
    WasmHost::headless(&wasm).expect("actual public interpreter")
}

fn limits() -> HostLimits {
    HostLimits {
        jobs: 1,
        request_bytes: 256,
        reply_bytes: 16,
    }
}

fn field(bytes: &mut Vec<u8>, value: &[u8]) {
    bytes.extend(
        u32::try_from(value.len())
            .expect("small fixture field")
            .to_le_bytes(),
    );
    bytes.extend(value);
}

fn frame(method: u8, representation: u8, body: &[u8]) -> Bytes {
    let mut bytes = vec![1, method, representation];
    bytes.extend(16_u32.to_le_bytes());
    bytes.extend(1_u32.to_le_bytes());
    field(&mut bytes, b"a/b");
    bytes.extend(3_u32.to_le_bytes());

    for value in [b"a", b"b", b"a"] {
        field(&mut bytes, b"ids[]");
        field(&mut bytes, value);
    }

    field(&mut bytes, body);
    bytes.into()
}

async fn request(socket: &mut TcpStream) -> Result<(String, Vec<u8>), TestError> {
    let mut bytes = Vec::new();

    while !bytes.ends_with(b"\r\n\r\n") {
        assert!(bytes.len() < 16_384);
        let mut byte = [0];
        socket.read_exact(&mut byte).await?;
        bytes.push(byte[0]);
    }

    let headers = String::from_utf8(bytes)?;
    let length = headers
        .lines()
        .find_map(|line| {
            let (name, value) = line.split_once(':')?;
            name.eq_ignore_ascii_case("content-length")
                .then_some(value.trim())
        })
        .map(str::parse::<usize>)
        .transpose()?
        .unwrap_or(0);
    assert!(length <= 256);
    let mut body = vec![0; length];
    socket.read_exact(&mut body).await?;
    Ok((headers, body))
}

fn reply(
    host: &mut WasmHost<HeadlessBackend>,
    call: CallId,
    reason: &str,
) -> Result<Bytes, TestError> {
    Ok(host.plugins().take_result(call)?.expect(reason)?)
}

async fn respond_to_methods(listener: TcpListener) -> Result<Vec<(String, Vec<u8>)>, TestError> {
    const METHOD_COUNT: usize = 5;
    let mut received = Vec::new();

    for _ in 0..METHOD_COUNT {
        let (mut socket, _) = listener.accept().await?;
        received.push(request(&mut socket).await?);
        socket
            .write_all(b"HTTP/1.1 200 OK\r\nContent-Length: 2\r\nConnection: close\r\n\r\n{}")
            .await?;
        socket.shutdown().await?;
    }

    Ok(received)
}

async fn quiet_peer(
    listener: TcpListener,
    ready: tokio::sync::oneshot::Sender<()>,
) -> Result<(), TestError> {
    let (mut socket, _) = listener.accept().await?;
    request(&mut socket).await?;
    ready.send(()).map_err(|_| "missing fixture observer")?;
    let mut byte = [0];
    assert_eq!(
        tokio::time::timeout(WAIT, socket.read(&mut byte)).await??,
        0
    );

    Ok(())
}

#[tokio::test]
async fn additive_rest_method_uses_actual_native_requests_and_existing_joined_completions()
-> Result<(), TestError> {
    let listener = TcpListener::bind("127.0.0.1:0").await?;
    let endpoint = format!("http://{}/base?fixed=1", listener.local_addr()?);
    let mut peers = JoinSet::new();
    peers.spawn(respond_to_methods(listener));
    let mut host = host();
    let mut driver = HttpDriver::new(&mut host, limits())?;
    let service = NativeRest::new(
        vec![((), NativeHttp::new(&endpoint, Default::default())?)],
        8192,
    )?;
    let plugin = driver.install_rest(&mut host, "rest", service, true)?;

    for (method, representation, body) in [
        (0, 0, b"".as_slice()),
        (1, 1, b"null"),
        (2, 2, b"code=a%2Bb"),
        (3, 1, b"null"),
        (4, 0, b""),
    ] {
        let call =
            host.plugins()
                .submit(plugin, MethodId(1), frame(method, representation, body))?;
        let routed = driver.drain(&mut host)?;
        assert!(routed.requests.is_empty() && routed.cancelled.is_empty());
        assert_eq!(driver.retained_jobs(), 1);
        let completion = tokio::time::timeout(WAIT, driver.next_completion()).await??;
        assert_eq!(driver.retained_jobs(), 0);
        driver.deliver(&mut host, completion)?;
        assert_eq!(
            reply(&mut host, call, "actual call result")?.as_ref(),
            b"\0\xc8\0{}"
        );
    }

    driver.shutdown().await?;
    let received = tokio::time::timeout(WAIT, peers.join_next())
        .await?
        .expect("owned peer task")??;

    for ((headers, _), method) in received
        .iter()
        .zip(["GET", "POST", "PUT", "PATCH", "DELETE"])
    {
        assert!(headers.starts_with(&format!(
            "{method} /base/a%2Fb?fixed=1&ids%5B%5D=a&ids%5B%5D=b&ids%5B%5D=a HTTP/1.1\r\n"
        )));
    }

    assert!(received[0].1.is_empty());
    assert_eq!(received[1].1, b"null");
    assert_eq!(received[2].1, b"code=a%2Bb");
    assert!(!received[0].0.to_ascii_lowercase().contains("content-type:"));
    assert!(
        received[2]
            .0
            .to_ascii_lowercase()
            .contains("content-type: application/x-www-form-urlencoded\r\n")
    );
    assert!(peers.is_empty());
    Ok(())
}

#[tokio::test]
async fn rest_wire_and_permission_denials_happen_before_starting_native_jobs()
-> Result<(), TestError> {
    let mut host = host();
    let mut driver = HttpDriver::new(&mut host, limits())?;
    let service = NativeRest::new(
        vec![(
            (),
            NativeHttp::new("http://127.0.0.1:1/", Default::default())?,
        )],
        8192,
    )?;
    let plugin = driver.install_rest(&mut host, "rest", service, true)?;

    for bytes in [b"\x02".as_slice(), b"\x01\xff\0", b"\x01\0\xff"] {
        let call = host
            .plugins()
            .submit(plugin, MethodId(1), Bytes::from(bytes.to_vec()))?;
        assert!(driver.drain(&mut host)?.guest_woken);
        assert_eq!(driver.retained_jobs(), 0);
        assert_eq!(
            reply(&mut host, call, "typed denial")?.as_ref(),
            b"\x01\x07"
        );
    }

    let call = host
        .plugins()
        .submit(plugin, MethodId(1), Bytes::from(vec![0; 257]))?;
    driver.drain(&mut host)?;
    assert_eq!(
        reply(&mut host, call, "typed byte denial")?.as_ref(),
        b"\x01\x05"
    );
    assert_eq!(driver.retained_jobs(), 0);
    let service = NativeRest::new(
        vec![(
            (),
            NativeHttp::new("http://127.0.0.1:1/", Default::default())?,
        )],
        8192,
    )?;
    let denied = driver.install_rest(&mut host, "denied", service, false)?;
    assert!(matches!(
        host.plugins().submit(denied, MethodId(1), frame(0, 0, b"")),
        Err(leptoskit_bridge::Error::PermissionDenied)
    ));
    driver.shutdown().await?;
    Ok(())
}

#[tokio::test]
async fn rest_cancellation_retains_capacity_until_join_and_shutdown_joins_quiet_io()
-> Result<(), TestError> {
    for cancel in [true, false] {
        let listener = TcpListener::bind("127.0.0.1:0").await?;
        let endpoint = format!("http://{}", listener.local_addr()?);
        let (ready, observed) = tokio::sync::oneshot::channel();
        let mut peers = JoinSet::new();
        peers.spawn(quiet_peer(listener, ready));
        let mut host = host();
        let mut driver = HttpDriver::new(&mut host, limits())?;

        let service = NativeRest::new(
            vec![((), NativeHttp::new(&endpoint, Default::default())?)],
            8192,
        )?;
        let plugin = driver.install_rest(&mut host, "quiet", service, true)?;
        let call = host
            .plugins()
            .submit(plugin, MethodId(1), frame(0, 0, b""))?;
        driver.drain(&mut host)?;
        tokio::time::timeout(WAIT, observed).await??;

        if cancel {
            host.plugins().cancel(call)?;
            assert!(driver.drain(&mut host)?.cancelled.is_empty());
            assert_eq!(driver.retained_jobs(), 1);
            let completion = tokio::time::timeout(WAIT, driver.next_completion()).await??;
            assert_eq!(driver.retained_jobs(), 0);
            driver.deliver(&mut host, completion)?;
        }

        driver.shutdown().await?;
        assert_eq!(driver.retained_jobs(), 0);
        tokio::time::timeout(WAIT, peers.join_next())
            .await?
            .expect("owned peer task")??;
        assert!(peers.is_empty());
    }

    Ok(())
}
