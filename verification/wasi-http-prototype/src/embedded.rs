//! Wasmtime50 embedding executes the public adapter with host-owned grants.

mod embedded_outgoing;
mod embedded_peer;
mod embedded_policy;

use embedded_policy::{Counts, Policy};
use std::path::Path;
use std::sync::{Arc, atomic::Ordering};
use std::time::Duration;
use wasmtime::component::{Component, Linker, ResourceTable};
use wasmtime::{Engine, Store};
use wasmtime_wasi::{WasiCtx, WasiCtxView, WasiView};
use wasmtime_wasi_http::{WasiHttpCtx, WasiHttpCtxView, WasiHttpView};

struct State {
    table: ResourceTable,
    wasi: WasiCtx,
    http: WasiHttpCtx,
    policy: Policy,
}

const STORE_DROP_LIMIT: Duration = Duration::from_millis(80);
const TASK_SETTLE: Duration = Duration::from_millis(20);

impl State {
    /// Fixtures own explicit endpoints, roots and destination grants.
    fn new(mode: &str, port: u16, counts: Counts) -> Result<Self, Box<dyn std::error::Error>> {
        let secure = matches!(
            mode,
            "success" | "tls-untrusted" | "tls-name" | "tls-deadline"
        );
        let host = if mode == "tls-name" {
            "wrong.test"
        } else {
            "localhost"
        };
        let scheme = if secure { "https" } else { "http" };
        let authority = format!("{host}:{port}");
        let endpoint = format!("{scheme}://{authority}/rpc");
        let guest_mode = if mode == "store-drop" { "hold" } else { mode };

        Ok(Self {
            table: ResourceTable::new(),
            wasi: WasiCtx::builder()
                .inherit_stdio()
                .args(&["embedded", &endpoint, guest_mode])
                .build(),
            http: WasiHttpCtx::new(),
            policy: Policy {
                authority,
                denied: matches!(mode, "denied" | "repeated-denied"),
                denial_delay: mode.starts_with("denied-").then_some(TASK_SETTLE),
                upload_block: mode == "denied-upload",
                delay_dns: mode == "dns-deadline",
                roots: roots(mode)?,
                counts,
            },
        })
    }
}

fn roots(mode: &str) -> Result<Arc<rustls::ClientConfig>, rustls::Error> {
    let certificate: &[u8] = if mode == "tls-untrusted" {
        include_bytes!("../../wasi-sockets/fixtures/wrong-ca.der")
    } else {
        include_bytes!("../../wasi-sockets/fixtures/ca.der")
    };

    let mut roots = rustls::RootCertStore::empty();
    roots.add(rustls::pki_types::CertificateDer::from(
        certificate.to_vec(),
    ))?;

    Ok(Arc::new(
        rustls::ClientConfig::builder()
            .with_root_certificates(roots)
            .with_no_client_auth(),
    ))
}

impl WasiView for State {
    fn ctx(&mut self) -> WasiCtxView<'_> {
        WasiCtxView {
            ctx: &mut self.wasi,
            table: &mut self.table,
        }
    }
}

impl WasiHttpView for State {
    fn http(&mut self) -> WasiHttpCtxView<'_> {
        WasiHttpCtxView {
            ctx: &mut self.http,
            table: &mut self.table,
            hooks: &mut self.policy,
        }
    }
}

#[tokio::main(flavor = "multi_thread", worker_threads = 2)]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    rustls::crypto::ring::default_provider()
        .install_default()
        .expect("one crypto provider");
    let engine = Engine::default();
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("target/wasm32-wasip2/debug/embedded_guest.wasm");
    let component = Component::from_file(&engine, path)?;
    let mut linker = Linker::new(&engine);
    wasmtime_wasi::p2::add_to_linker_async(&mut linker)?;
    assert!(
        linker.instantiate_pre(&component).is_err(),
        "missing HTTP must reject instantiation"
    );
    println!("PASS missing HTTP rejected before guest execution");

    wasmtime_wasi_http::p2::add_only_http_to_linker_async(&mut linker)?;
    if !std::env::args().any(|argument| argument == "--unpatched-host") {
        embedded_outgoing::add(&mut linker)?;
    }
    let selected = std::env::args().nth(1);
    let baseline = run(&engine, &component, &linker, "baseline", None).await?;

    for mode in [
        "denied",
        "denied-upload",
        "denied-response",
        "early-status",
        "malformed-chunk",
        "truncated",
        "upload-deadline",
        "success",
        "tls-untrusted",
        "tls-name",
        "tls-deadline",
        "dns-deadline",
        "redirect",
        "cancel",
        "repeated",
        "repeated-denied",
        "repeated-cancel",
        "store-drop",
        "replacement",
    ] {
        if selected.as_ref().is_some_and(|selected| selected != mode) {
            continue;
        }

        run(&engine, &component, &linker, mode, Some(baseline)).await?;
    }

    Ok(())
}

/// The owner can finish normally or be cancelled with its Store still pending.
async fn run(
    engine: &Engine,
    component: &Component,
    linker: &Linker<State>,
    mode: &str,
    expected_resources: Option<usize>,
) -> Result<usize, Box<dyn std::error::Error>> {
    let peer = embedded_peer::Peer::start(mode);
    let counts = Counts::default();
    let state = State::new(mode, peer.port, counts.clone())?;
    let mut store = Store::new(engine, state);
    let command =
        wasmtime_wasi::p2::bindings::Command::instantiate_async(&mut store, component, linker)
            .await?;
    let baseline = store.data_mut().table.iter_mut().count();
    let task_baseline = tokio::runtime::Handle::current()
        .metrics()
        .num_alive_tasks();
    let limit = if mode == "store-drop" {
        STORE_DROP_LIMIT
    } else {
        Duration::from_secs(4)
    };

    let result = tokio::time::timeout(limit, command.wasi_cli_run().call_run(&mut store)).await;

    let resources = if mode == "store-drop" {
        assert!(result.is_err(), "owner must still be pending at Store drop");
        expected_resources.expect("warm resources")
    } else {
        result??.map_err(|()| "guest assertion failed")?;
        resources(&mut store, expected_resources)
    };

    drop(store);
    tokio::time::sleep(TASK_SETTLE).await;
    assert_eq!(
        counts.active.load(Ordering::SeqCst),
        0,
        "{mode}: retained network flight"
    );
    assert_eq!(
        tokio::runtime::Handle::current()
            .metrics()
            .num_alive_tasks(),
        task_baseline,
        "{mode}: retained Tokio tasks"
    );

    if matches!(mode, "denied" | "dns-deadline") {
        assert_eq!(counts.opened.load(Ordering::SeqCst), 0);
    }

    peer.finish();
    println!(
        "PASS host {mode}: table baseline={baseline}; flights=0; tasks restored; Store dropped"
    );
    Ok(resources)
}

fn resources(store: &mut Store<State>, expected: Option<usize>) -> usize {
    let count = store.data_mut().table.iter_mut().count();
    let options = store
        .data_mut()
        .table
        .iter_mut()
        .filter(|(_, entry)| entry.is::<wasmtime_wasi_http::RequestOptions>())
        .count();
    assert_eq!(
        options, 0,
        "owned request-options leaked from the host table"
    );
    assert!(
        store
            .data_mut()
            .table
            .iter_mut()
            .all(|(_, entry)| entry.is::<wasmtime_wasi::p2::DynOutputStream>()),
        "only CLI output streams may remain"
    );

    if let Some(expected) = expected {
        assert_eq!(count, expected, "table did not return to warm baseline");
    }

    count
}
