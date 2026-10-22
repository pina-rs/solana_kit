//! Wasmtime50 embedding executes the public adapter with host-owned grants.

mod embedded_outgoing;
mod embedded_peer;
mod embedded_policy;
mod embedded_resources;

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
    cli_streams: embedded_resources::Snapshot,
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
            cli_streams: embedded_resources::Snapshot::new(),
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
    let cli_only = std::env::args().any(|argument| argument == "--cli-only");
    let guest = if cli_only {
        "cli_guest"
    } else {
        "embedded_guest"
    };

    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join(format!("target/wasm32-wasip2/debug/{guest}.wasm"));
    let component = Component::from_file(&engine, path)?;
    let linker = linker(&engine, &component, cli_only)?;
    let selected = std::env::args().nth(1);
    let baseline = run(&engine, &component, &linker, "baseline", None).await?;

    if cli_only {
        return Ok(());
    }

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

        run(&engine, &component, &linker, mode, Some(&baseline)).await?;
    }

    Ok(())
}

/// The CLI-only probe has no HTTP import to reject before installing grants.
fn linker(
    engine: &Engine,
    component: &Component,
    cli_only: bool,
) -> wasmtime::Result<Linker<State>> {
    let mut linker = Linker::new(engine);
    wasmtime_wasi::p2::add_to_linker_async(&mut linker)?;

    if !cli_only {
        assert!(
            linker.instantiate_pre(component).is_err(),
            "missing HTTP must reject instantiation"
        );
        println!("PASS missing HTTP rejected before guest execution");
    }

    wasmtime_wasi_http::p2::add_only_http_to_linker_async(&mut linker)?;
    embedded_resources::add(&mut linker)?;

    if !std::env::args().any(|argument| argument == "--unpatched-host") {
        embedded_outgoing::add(&mut linker)?;
    }

    Ok(linker)
}

/// The owner can finish normally or be cancelled with its Store still pending.
async fn run(
    engine: &Engine,
    component: &Component,
    linker: &Linker<State>,
    mode: &str,
    expected_resources: Option<&embedded_resources::Snapshot>,
) -> Result<embedded_resources::Snapshot, Box<dyn std::error::Error>> {
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
        expected_resources.expect("warm resources").clone()
    } else {
        result??.map_err(|()| "guest assertion failed")?;
        resources(&mut store, expected_resources)
    };

    drop(store);
    teardown(&counts, task_baseline, mode).await;
    peer.finish();
    println!(
        "PASS host {mode}: table baseline={baseline}; flights=0; tasks restored; Store dropped"
    );
    Ok(resources)
}

/// Store destruction must terminate actual network work and restore task counts.
async fn teardown(counts: &Counts, task_baseline: usize, mode: &str) {
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
}

fn resources(
    store: &mut Store<State>,
    expected: Option<&embedded_resources::Snapshot>,
) -> embedded_resources::Snapshot {
    let state = store.data_mut();
    let actual = embedded_resources::snapshot(&mut state.table, &state.cli_streams, expected);

    println!("CLI warm resources {actual:?}");
    actual
}
