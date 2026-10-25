//! Exact destination grants and actual embedded Store destruction/replacement.
use anyhow::{Result, ensure};
use std::{
    net::{SocketAddr, TcpListener},
    path::Path,
    thread,
    time::Duration,
};
use wasmtime::{
    Engine, Store,
    component::{Component, Linker, ResourceTable},
};
use wasmtime_wasi::{
    WasiCtx, WasiCtxView, WasiView, p2::pipe::MemoryOutputPipe, sockets::SocketAddrUse,
};

const STORE_DROP_MILLIS: u64 = 120;
const GUEST_SECONDS: u64 = 5;
const OUTPUT_BYTES: usize = 8192;
const JOIN_MILLIS: u64 = 250;
const JOIN_TICK_MILLIS: u64 = 5;

struct State {
    table: ResourceTable,
    wasi: WasiCtx,
}
impl WasiView for State {
    fn ctx(&mut self) -> WasiCtxView<'_> {
        WasiCtxView {
            ctx: &mut self.wasi,
            table: &mut self.table,
        }
    }
}

#[tokio::main(flavor = "multi_thread", worker_threads = 2)]
pub async fn run(guest: &Path) -> Result<()> {
    let engine = Engine::default();
    let component = Component::from_file(&engine, guest).map_err(host_error)?;
    let mut linker = Linker::new(&engine);
    wasmtime_wasi::p2::add_to_linker_async(&mut linker).map_err(host_error)?;
    for mode in [
        "plain",
        "exchange",
        "session",
        "duplex",
        "fragment",
        "wrong-ca",
        "wrong-name",
        "oversized",
        "binary",
        "invalid-utf8",
        "masked",
        "abrupt",
        "cancel",
        "retain",
        "denied",
        "unload",
        "exchange",
    ] {
        execute(&engine, &component, &linker, mode).await?;
    }
    Ok(())
}

fn state(mode: &str, port: u16, output: MemoryOutputPipe) -> State {
    let endpoint = super::endpoint(mode, port);
    let denied = mode == "denied";
    let wasi = WasiCtx::builder()
        .inherit_stderr()
        .stdout(output)
        .args(&["guest", mode, &endpoint])
        .allow_ip_name_lookup(true)
        .allow_tcp(true)
        .allow_udp(false)
        .socket_addr_check(move |address, reason| {
            Box::pin(async move {
                let allowed = !denied && permitted(address, reason, port);
                eprintln!("POLICY {reason:?} {address} allowed={allowed}");
                allowed
            })
        })
        .build();
    State {
        table: ResourceTable::new(),
        wasi,
    }
}

fn permitted(address: SocketAddr, reason: SocketAddrUse, port: u16) -> bool {
    match reason {
        SocketAddrUse::TcpConnect => address.ip().is_loopback() && address.port() == port,
        SocketAddrUse::TcpBind => address.ip().is_unspecified() && address.port() == 0,
        _ => false,
    }
}

async fn execute(
    engine: &Engine,
    component: &Component,
    linker: &Linker<State>,
    mode: &str,
) -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:0")?;
    let port = listener.local_addr()?.port();
    listener.set_nonblocking(true)?;
    let peer_mode = mode.to_owned();
    let peer = thread::spawn(move || super::fixture(listener, &peer_mode));
    let output = MemoryOutputPipe::new(OUTPUT_BYTES);
    let mut store = Store::new(engine, state(mode, port, output.clone()));
    let tasks = tokio::runtime::Handle::current()
        .metrics()
        .num_alive_tasks();
    let outcome = invoke(&mut store, component, linker, mode, &output).await;
    let resources = store.data_mut().table.iter_mut().count();
    drop(store);
    let observed = peer.join().map_err(|_| anyhow::anyhow!("peer panicked"))?;
    let joined = join_tasks(tasks).await;
    outcome?;
    joined?;
    println!(
        "PASS embedded {mode}: resources before Store drop={resources}; tasks restored; {}; stdout={}",
        observed?,
        String::from_utf8_lossy(&output.contents()).trim()
    );
    Ok(())
}

async fn invoke(
    store: &mut Store<State>,
    component: &Component,
    linker: &Linker<State>,
    mode: &str,
    output: &MemoryOutputPipe,
) -> Result<()> {
    let command =
        wasmtime_wasi::p2::bindings::Command::instantiate_async(&mut *store, component, linker)
            .await
            .map_err(host_error)?;
    let limit = if mode == "unload" {
        Duration::from_millis(STORE_DROP_MILLIS)
    } else {
        Duration::from_secs(GUEST_SECONDS)
    };

    let result = tokio::time::timeout(limit, command.wasi_cli_run().call_run(store)).await;
    let stdout = output.contents();
    let stdout = String::from_utf8_lossy(&stdout);

    if mode == "unload" {
        ensure!(
            result.is_err() && stdout.contains("PENDING unload"),
            "Store not actually pending"
        );

        return Ok(());
    }

    result?
        .map_err(host_error)?
        .map_err(|()| anyhow::anyhow!("guest failed {mode}"))?;
    ensure!(
        stdout.contains(&format!("PASS {mode}")),
        "missing guest assertions"
    );
    Ok(())
}

async fn join_tasks(expected: usize) -> Result<()> {
    let limit = tokio::time::Instant::now() + Duration::from_millis(JOIN_MILLIS);

    loop {
        let actual = tokio::runtime::Handle::current()
            .metrics()
            .num_alive_tasks();

        if actual == expected {
            return Ok(());
        }

        ensure!(
            tokio::time::Instant::now() < limit,
            "retained host tasks: {actual}, expected {expected}"
        );
        tokio::time::sleep(Duration::from_millis(JOIN_TICK_MILLIS)).await;
    }
}

fn host_error(error: wasmtime::Error) -> anyhow::Error {
    anyhow::anyhow!("{error:#}")
}
