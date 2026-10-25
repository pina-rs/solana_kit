//! Source-only until root executes these through the actual selected producer.
use super::*;
use leptoskit_bridge::Bytes;
use solana_kit_core::{Address, Signature, crypto::verify};
use std::{
    fs,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

pub(super) const WASM: &[u8] = br#"(module
 (memory (export "memory") 1)
 (func (export "leptoskit_main"))
 (func (export "leptoskit_plugins_ready"))
 (func (export "leptoskit_event") (param i64 i32 i32 i32) (result i32) i32.const 0))"#;

pub(super) fn interpreter(wasm: &[u8]) -> WasmHost<HeadlessBackend> {
    let binary = wat::parse_bytes(wasm).unwrap();
    WasmHost::headless(&binary).unwrap()
}

pub(super) struct Sandbox(pub(super) std::path::PathBuf);

impl Sandbox {
    pub(super) fn new() -> Self {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let path =
            std::env::temp_dir().join(format!("solana-wallet-{}-{nonce}", std::process::id()));
        fs::create_dir(&path).unwrap();
        Self(path.canonicalize().unwrap())
    }
}

impl Drop for Sandbox {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.0).unwrap();
    }
}

fn submit(
    host: &mut WasmHost<HeadlessBackend>,
    service: &WalletService,
    method: u32,
    bytes: &[u8],
) -> CallId {
    host.plugins()
        .submit(
            service.plugin,
            MethodId(method),
            Bytes::from(bytes.to_vec()),
        )
        .unwrap()
}

async fn event(
    host: &mut WasmHost<HeadlessBackend>,
    service: &mut WalletService,
    call: CallId,
) -> Event {
    let outcome = tokio::time::timeout(
        Duration::from_secs(3),
        std::future::poll_fn(|context| {
            service.poll(host, context).unwrap();

            if let Ok(Some(Err(error))) = host.plugins().result(call) {
                panic!("opening call {call:?} failed: {error:?}");
            }

            match host.plugins().next_event() {
                Some(event) => Poll::Ready(event),
                None => Poll::Pending,
            }
        }),
    )
    .await;

    outcome.unwrap_or_else(|_| {
        panic!(
            "opening call {call:?} timed out: epoch={}, stopped={}, lease={:?}, pending={:?}",
            service.shared.epoch(),
            service.shared.stopped(),
            service.lease,
            service.pending.keys().collect::<Vec<_>>()
        )
    })
}

async fn reply(
    host: &mut WasmHost<HeadlessBackend>,
    service: &mut WalletService,
    call: CallId,
) -> Result<Vec<u8>, Error> {
    tokio::time::timeout(
        Duration::from_secs(3),
        std::future::poll_fn(|context| {
            service.poll(host, context).unwrap();
            match host.plugins().result(call).unwrap() {
                Some(result) => Poll::Ready(
                    result
                        .as_ref()
                        .map(|bytes| bytes.as_ref().to_vec())
                        .map_err(|error| *error),
                ),
                None => Poll::Pending,
            }
        }),
    )
    .await
    .unwrap()
}

#[tokio::test]
async fn selected_local_key_signs_exact_bytes_and_persists_only_on_request() {
    let sandbox = Sandbox::new();
    let mut host = interpreter(WASM);
    let mut service = WalletService::install(&mut host, &sandbox.0).unwrap();
    let failures = service.failures();
    let lease = submit(&mut host, &service, 8, &[]);
    let opened = event(&mut host, &mut service, lease).await;
    assert_eq!(&opened.payload.as_ref()[..8], &lease.0.to_le_bytes());
    let address: Address = std::str::from_utf8(&opened.payload.as_ref()[8..])
        .unwrap()
        .parse()
        .unwrap();

    let saved = sandbox.0.join("solana-wallet/wallet.json");
    assert!(!saved.exists());

    let message = b"explicitly approved source-test message";
    let signing = submit(&mut host, &service, 10, message);
    let signed = reply(&mut host, &mut service, signing).await.unwrap();
    let signature: Signature = std::str::from_utf8(&signed).unwrap().parse().unwrap();
    verify(&address, message, &signature).unwrap();
    assert!(verify(&address, b"different message", &signature).is_err());
    assert!(!saved.exists());

    let save = submit(&mut host, &service, 11, &[0]);
    reply(&mut host, &mut service, save).await.unwrap();
    let original = fs::read(&saved).unwrap();
    let duplicate = submit(&mut host, &service, 11, &[0]);
    assert_eq!(
        reply(&mut host, &mut service, duplicate).await,
        Err(Error::Native)
    );
    assert_eq!(fs::read(&saved).unwrap(), original);

    assert!(matches!(
        failures.take().unwrap(),
        Some(WalletFailure::Storage(_))
    ));
    assert!(matches!(
        host.plugins().result(lease),
        Ok(Some(Err(Error::Cancelled)))
    ));
    Box::new(service).close().await.unwrap();
}

#[tokio::test]
async fn association_lease_cancellation_disposes_pending_connect_and_joins_worker() {
    let sandbox = Sandbox::new();
    let mut host = interpreter(WASM);
    let mut service = WalletService::install(&mut host, &sandbox.0).unwrap();
    let lease = submit(&mut host, &service, 0, &[]);
    let first = event(&mut host, &mut service, lease).await;

    let connect = submit(&mut host, &service, 1, &[]);
    std::future::poll_fn(|context| {
        service.poll(&mut host, context).unwrap();
        Poll::Ready(())
    })
    .await;

    host.plugins().cancel(lease).unwrap();
    std::future::poll_fn(|context| {
        service.poll(&mut host, context).unwrap();
        Poll::Ready(())
    })
    .await;

    assert!(matches!(
        host.plugins().result(connect),
        Ok(Some(Err(Error::Cancelled)))
    ));
    let replacement = submit(&mut host, &service, 0, &[]);
    let second = event(&mut host, &mut service, replacement).await;

    assert_ne!(&first.payload.as_ref()[8..], &second.payload.as_ref()[8..]);
    let shared = Arc::clone(&service.shared);
    Box::new(service).close().await.unwrap();
    assert!(shared.stopped());
}

#[tokio::test]
async fn foreign_interpreter_is_refused_and_private_symlink_load_cannot_escape() {
    use std::os::unix::fs::symlink;
    let sandbox = Sandbox::new();
    let mut host = interpreter(WASM);
    let mut other = interpreter(WASM);
    let mut service = WalletService::install(&mut host, &sandbox.0).unwrap();

    std::future::poll_fn(|context| {
        assert!(matches!(
            service.poll(&mut other, context),
            Err(Error::InvalidCall)
        ));
        Poll::Ready(())
    })
    .await;
    let outside = sandbox.0.join("outside.json");
    fs::write(&outside, b"private unrelated data").unwrap();
    symlink(&outside, sandbox.0.join("solana-wallet/wallet.json")).unwrap();
    let load = submit(&mut host, &service, 9, &[0]);
    assert_eq!(
        reply(&mut host, &mut service, load).await,
        Err(Error::Native)
    );
    assert_eq!(fs::read(&outside).unwrap(), b"private unrelated data");
    Box::new(service).close().await.unwrap();
}

#[tokio::test]
async fn intentional_private_copy_can_restore_the_original_public_identity() {
    use solana_kit_core::crypto::ExportableKeypair;
    use solana_kit_native::keys::NativeEntropy;
    let sandbox = Sandbox::new();
    let directory = storage::Directory::new(&sandbox.0).unwrap();
    let key = ExportableKeypair::generate(&mut NativeEntropy::default())
        .await
        .unwrap();
    directory.save(&key, true).unwrap();
    assert!(!sandbox.0.join("solana-wallet/wallet.json").exists());
    let restored = directory.load(true).unwrap();
    assert_eq!(key.signer().address(), restored.signer().address());
    assert!(directory.save(&key, true).is_err());
}

#[test]
fn native_installation_keeps_the_original_directory_failure_redacted() {
    let sandbox = Sandbox::new();
    let mut host = interpreter(WASM);

    let failure = match WalletService::install(&mut host, &sandbox.0.join("missing")) {
        Err(failure) => failure,
        Ok(_) => panic!("missing app directory was accepted"),
    };

    assert!(format!("{failure:?}").contains("REDACTED"));

    match failure {
        WalletInstallError::Directory(error) => {
            assert_eq!(error.kind(), std::io::ErrorKind::NotFound)
        }
        _ => panic!("directory failure lost its original error"),
    }
}

#[tokio::test]
async fn composed_routing_returns_actual_foreign_requests_and_cancellations_intact() {
    let sandbox = Sandbox::new();
    let mut host = interpreter(WASM);
    let foreign = host.plugins().register("foreign", [MethodId(0)]).unwrap();
    host.plugins().grant(foreign, MethodId(0), true).unwrap();
    let first = host
        .plugins()
        .submit(foreign, MethodId(0), Bytes::default())
        .unwrap();
    assert_eq!(host.plugins().next_request().unwrap().id, first);
    host.plugins().cancel(first).unwrap();
    let cancelled = host.plugins().next_cancelled().unwrap();

    let second = host
        .plugins()
        .submit(foreign, MethodId(0), Bytes::from_static(b"unchanged"))
        .unwrap();
    let request = host.plugins().next_request().unwrap();
    let mut service = WalletService::install(&mut host, &sandbox.0).unwrap();
    let waker = futures_util::task::noop_waker();
    let mut context = Context::from_waker(&waker);
    let incoming = ServiceWork {
        requests: vec![request],
        cancelled: vec![cancelled],
    };
    let routed = service.route(&mut host, &mut context, incoming).unwrap();

    assert_eq!(routed.cancelled, [first]);
    assert_eq!(routed.requests.len(), 1);
    assert_eq!(routed.requests[0].id, second);
    assert_eq!(routed.requests[0].plugin, foreign);
    assert_eq!(routed.requests[0].payload.as_ref(), b"unchanged");
    Box::new(service).close().await.unwrap();
}

#[test]
fn decoding_erases_the_actual_bounded_owner_on_success_and_every_failure() {
    use solana_kit_core::crypto::ExportableKeypair;
    use zeroize::Zeroizing;
    let key = ExportableKeypair::from_seed(&[7; 32]);
    let exported = key.backup();
    let valid = serde_json::to_vec(&exported.expose_secret().as_slice()).unwrap();
    let prefix = format!("[{}", vec!["7"; 32].join(","));
    let inputs = [
        valid,
        format!("{prefix},false]").into_bytes(),
        prefix.into_bytes(),
        format!("[{}]", vec!["7"; 65].join(",")).into_bytes(),
        b"[256]".to_vec(),
        b"[-1]".to_vec(),
        b"[1.5]".to_vec(),
        b"[]".to_vec(),
    ];

    for (index, input) in inputs.iter().enumerate() {
        let mut owner = Zeroizing::new([0; 64]);
        let result = storage::decode_key(input, &mut owner);
        assert_eq!(result.is_ok(), index == 0);
        assert_eq!(*owner, [0; 64]);
    }
}

#[tokio::test]
async fn public_load_rejects_malformed_truncated_and_oversized_private_files() {
    use std::os::unix::fs::PermissionsExt;
    let sandbox = Sandbox::new();
    let saved = sandbox.0.join("solana-wallet/wallet.json");
    let prefix = format!("[{}", vec!["7"; 32].join(","));
    let inputs = [
        format!("{prefix},false]"),
        prefix,
        format!("[{}]", vec!["7"; 65].join(",")),
        " ".repeat(1025),
    ];

    for input in inputs {
        let mut host = interpreter(WASM);
        let mut service = WalletService::install(&mut host, &sandbox.0).unwrap();
        fs::write(&saved, input).unwrap();
        fs::set_permissions(&saved, fs::Permissions::from_mode(0o600)).unwrap();
        let failures = service.failures();
        let load = submit(&mut host, &service, wire::LOAD_LOCAL, &[0]);
        assert_eq!(
            reply(&mut host, &mut service, load).await,
            Err(Error::Native)
        );
        assert!(matches!(
            failures.take().unwrap(),
            Some(WalletFailure::Storage(_))
        ));
        Box::new(service).close().await.unwrap();
    }
}
