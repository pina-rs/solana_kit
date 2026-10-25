//! Actual guest routing, quiet deadlines and observed native cancellation EOF.
use crate::application::Application as HttpHost;
use crate::fixture::{self, Fixture};
use anyhow::{Context, Result, ensure};
use leptoskit_bridge::{Bridge, Bytes, Error, Event, Limits, MethodId, PluginId};
use leptoskit_wasm::WasmHost;
use solana_kit_leptoskit::HostLimits;
use std::time::Duration;

const VIEWPORT_WIDTH: f32 = 320.0;
const VIEWPORT_HEIGHT: f32 = 480.0;
const RETAIN_WINDOW: Duration = Duration::from_millis(25);

fn trace(app: &mut HttpHost) -> Result<Vec<String>> {
    let host = app.host_mut()?;
    let root = host.surface().root().context("guest report root")?;
    let text = host
        .surface()
        .props(root)
        .text
        .as_deref()
        .context("guest trace")?;
    Ok(serde_json::from_str(text)?)
}

fn require(app: &mut HttpHost, marker: &str) -> Result<()> {
    let actual = trace(app)?;
    ensure!(
        actual.last().map(String::as_str) == Some(marker),
        "expected {marker}, actual {actual:?}"
    );
    Ok(())
}

async fn message(receiver: &mut tokio::sync::mpsc::Receiver<()>) -> Result<()> {
    tokio::time::timeout(Duration::from_secs(2), receiver.recv())
        .await?
        .context("fixture sender closed")
}

async fn progress(app: &mut HttpHost, marker: &str) -> Result<()> {
    let routed = app.drain()?;
    ensure!(
        routed.requests.is_empty() && routed.cancelled.is_empty(),
        "unrouted plugin work"
    );
    let completion = tokio::time::timeout(Duration::from_secs(4), app.next_completion()).await?;
    let completed = completion
        .with_context(|| format!("guest completion before {marker}: {:?}", trace(app)))?;
    ensure!(completed, "missing actual native completion");
    Ok(())
}

async fn setup(wasm: &[u8], borrowed: bool) -> Result<(HttpHost, Vec<Fixture>)> {
    let mut host = WasmHost::headless(wasm)?;
    host.set_fuel_budget(1_000_000_000);
    *host.plugins() = Bridge::new(Limits {
        calls: 4,
        events: 4,
        payload_bytes: 4096,
        buffered_bytes: 16384,
    });
    let mut app = HttpHost::new(
        host,
        HostLimits {
            jobs: 2,
            request_bytes: 1024,
            reply_bytes: 1024,
        },
        borrowed,
    )?;
    let mut fixtures = Vec::new();

    for name in ["secure", "status", "large", "deadline", "quiet"] {
        let mut fixture = fixture::create(name).await?;
        app.install(name, fixture.service.take().context("owned service")?, true)?;
        fixtures.push(fixture);
    }

    app.install(
        "denied",
        solana_kit_native::NativeHttp::new("http://127.0.0.1:1/", Default::default())?,
        false,
    )?;
    app.host_mut()?.plugins().register("application", [])?;
    app.host_mut()?.plugins().register("unrelated", [])?;
    let mut rest = fixture::create("rest").await?;
    app.install_rest(
        "rest",
        solana_kit_native::NativeRest::new(
            vec![((), rest.service.take().context("REST service")?)],
            1024,
        )?,
        true,
    )?;
    fixtures.push(rest);
    app.install_rest(
        "rest-denied",
        solana_kit_native::NativeRest::new(
            vec![(
                (),
                solana_kit_native::NativeHttp::new("http://127.0.0.1:1/", Default::default())?,
            )],
            1024,
        )?,
        false,
    )?;
    denial_controls(&mut app)?;
    Ok((app, fixtures))
}

/// Registered-but-denied and undefined methods remain distinct before any native work.
fn denial_controls(app: &mut HttpHost) -> Result<()> {
    let bridge = app.host_mut()?.plugins();
    let rpc = bridge.resolve("denied").context("RPC denial capability")?;
    let rest = bridge
        .resolve("rest-denied")
        .context("REST denial capability")?;

    for (plugin, method, expected) in [
        (rpc, MethodId(0), Error::PermissionDenied),
        (rpc, MethodId(1), Error::Unavailable),
        (rest, MethodId(1), Error::PermissionDenied),
    ] {
        let actual = bridge.submit(plugin, method, Bytes::new());
        ensure!(
            actual == Err(expected),
            "capability {plugin:?}/{method:?}: {actual:?}"
        );
        println!("capability {plugin:?}/{method:?}: {expected:?}; no native job");
    }

    ensure!(
        bridge.next_request().is_none(),
        "denied method queued native work"
    );
    ensure!(
        app.retained_jobs() == 0,
        "denied method retained a native job"
    );
    Ok(())
}

pub(super) async fn run(wasm: &[u8]) -> Result<()> {
    for borrowed in [true, false] {
        instances(wasm, borrowed).await?;
    }

    println!(
        "replacement uses a distinct owner; no callback or guest pointer escapes a task owner"
    );
    Ok(())
}

async fn instances(wasm: &[u8], borrowed: bool) -> Result<()> {
    for generation in 0..2 {
        let (mut app, mut fixtures) = setup(wasm, borrowed).await?;
        app.start(VIEWPORT_WIDTH, VIEWPORT_HEIGHT)?;
        require(&mut app, "pending")?;
        app.drain()?;
        message(&mut fixtures[0].accepted).await?;
        message(&mut fixtures[0].closed).await?;
        require(&mut app, "pending")?;

        for marker in [
            "https-max",
            "status-rejected",
            "body-bounded",
            "quiet-deadline",
            "rest-wire",
            "rest-bounded",
        ] {
            progress(&mut app, marker).await?;
            let actual = trace(&mut app)?;
            ensure!(
                actual.contains(&marker.to_owned()),
                "missing {marker}: {actual:?}"
            );
        }
        ensure!(
            trace(&mut app)?.contains(&"rest-denied".to_owned()),
            "REST grant denial"
        );
        cancel_and_unload(&mut app, &mut fixtures[4]).await?;

        for fixture in fixtures {
            fixture.task.await??;
        }

        println!(
            "borrowed {borrowed}, instance {generation}: actual HTTPS/u64MAX, status, body budget, quiet deadline, local Rc Pending/wake, retained/cancel EOF, unrelated event routing, abort+join unload PASS"
        );
    }
    Ok(())
}

async fn cancel_and_unload(app: &mut HttpHost, quiet: &mut Fixture) -> Result<()> {
    app.drain()?;
    message(&mut quiet.accepted).await?;
    ensure!(
        tokio::time::timeout(RETAIN_WINDOW, quiet.closed.recv())
            .await
            .is_err(),
        "retained future closed peer"
    );

    ensure!(app.retained_jobs() == 1, "quiet request not owned");
    app.host_mut()?.plugins().emit(Event {
        plugin: PluginId(7),
        kind: 0,
        payload: Bytes::new(),
    })?;
    app.host_mut()?.plugins().emit(Event {
        plugin: PluginId(6),
        kind: 7,
        payload: Bytes::new(),
    })?;
    app.host_mut()?.wake_plugins()?;
    let routed = app.drain()?;
    ensure!(
        routed.requests.is_empty() && routed.cancelled.is_empty(),
        "cancel routing lost work"
    );
    ensure!(
        app.retained_jobs() == 2,
        "aborting task accounting released before join"
    );
    ensure!(
        app.next_completion().await?,
        "cancelled native task missing"
    );

    message(&mut quiet.closed).await?;
    ensure!(
        app.retained_jobs() == 1,
        "cancelled task not removed after join"
    );
    message(&mut quiet.accepted).await?;
    let before = trace(app)?;
    ensure!(
        before.contains(&"unrelated-routed".into()) && before.contains(&"quiet-cancelled".into()),
        "application event ownership"
    );
    app.shutdown().await?;
    ensure!(
        app.retained_jobs() == 0 && app.host_mut().is_err(),
        "unload retained host resources"
    );
    message(&mut quiet.closed).await?;

    Ok(())
}
