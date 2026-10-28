//! Execute the actual generated guest's controls through production wasmi.
//! This is a protocol integration test, not an installed desktop or mobile journey.
#[path = "support/peer.rs"]
mod peer;
mod support;
#[path = "support/transactions.rs"]
mod transactions;

use leptoskit_bridge_ffi::app::AppService;
use leptoskit_native::{EventOutcome, HeadlessBackend, NativeEvent, NativeKind};
use leptoskit_wasm::WasmHost;
use leptoskit_yoga::NodeId;
use std::{
    fs,
    sync::Arc,
    task::{Context, Wake, Waker},
    thread,
    time::{Duration, Instant},
};
use support::Sandbox;

type Host = WasmHost<HeadlessBackend>;
const WAIT: Duration = Duration::from_secs(30);

struct ThreadWake(thread::Thread);

impl Wake for ThreadWake {
    fn wake(self: Arc<Self>) {
        self.0.unpark();
    }
}

struct Application {
    host: Host,
    service: Option<Box<dyn AppService>>,
    _sandbox: Sandbox,
}

impl Application {
    /// Use the compiled public-generator guest and the same service installer as
    /// its native registrant. Keep production fuel limits and one routing owner.
    fn new(port: u16) -> Self {
        let wasm =
            fs::read(std::env::var_os("GENERATED_WALLET_WASM").expect("explicit guest artifact"))
                .expect("read actual generated guest");
        let sandbox = Sandbox::new();
        sandbox.config(
            &format!("http://127.0.0.1:{port}"),
            &format!("ws://127.0.0.1:{port}"),
        );
        let mut host = Host::headless(&wasm).expect("production interpreter");
        let installer =
            solana_kit_app_journey_runtime::from_app_dirs(&sandbox.0, &sandbox.0).unwrap();
        let service = installer(&mut host).unwrap();
        host.start(390.0, 844.0).expect("actual guest startup");
        Self {
            host,
            service: Some(service),
            _sandbox: sandbox,
        }
    }

    /// Real completion and peer notifications wake this owner. The only timeout
    /// is a failure bound; no fixed sleep fabricates asynchronous progress.
    fn wait(&mut self, predicate: impl Fn(&Host) -> bool) {
        let waker = Waker::from(Arc::new(ThreadWake(thread::current())));
        let mut context = Context::from_waker(&waker);
        let deadline = Instant::now() + WAIT;

        loop {
            let work = self
                .service
                .as_mut()
                .unwrap()
                .poll(&mut self.host, &mut context)
                .unwrap();
            assert!(work.requests.is_empty() && work.cancelled.is_empty());

            if predicate(&self.host) {
                return;
            }

            let remaining = deadline
                .checked_duration_since(Instant::now())
                .expect("guest UI deadline");
            thread::park_timeout(remaining);
        }
    }

    fn text(&mut self, text: &str) {
        self.wait(|host| {
            find(host, |node| {
                host.surface().props(node).text.as_deref() == Some(text)
            })
            .is_some()
        });
    }

    fn tap(&mut self, text: &str) -> NodeId {
        self.text(text);
        let node = find(&self.host, |node| {
            self.host.surface().props(node).text.as_deref() == Some(text)
        })
        .unwrap();
        assert_eq!(self.host.surface().kind(node), NativeKind::Button);
        assert_eq!(
            self.host.dispatch(node, &NativeEvent::Press).unwrap(),
            EventOutcome::Handled
        );
        node
    }

    fn account(&self) -> String {
        let node = find(&self.host, |node| {
            self.host.surface().props(node).id.as_deref() == Some("wallet_local.account")
        })
        .expect("selected wallet address");
        self.host.surface().props(node).text.clone().unwrap()
    }

    fn close(mut self) {
        futures::executor::block_on(self.service.take().unwrap().close()).unwrap();
    }
}

impl Drop for Application {
    fn drop(&mut self) {
        if let Some(service) = self.service.take() {
            match (
                thread::panicking(),
                futures::executor::block_on(service.close()),
            ) {
                (_, Ok(())) => {}
                (true, Err(error)) => eprintln!("Native service cleanup failed: {error}"),
                (false, Err(error)) => panic!("Native service cleanup failed: {error}"),
            }
        }
    }
}

fn find(host: &Host, predicate: impl Fn(NodeId) -> bool) -> Option<NodeId> {
    let mut pending = vec![host.surface().root()?];

    while let Some(node) = pending.pop() {
        if predicate(node) {
            return Some(node);
        }

        pending.extend(host.surface().children(node));
    }

    None
}

#[test]
fn actual_guest_reads_rejects_stale_approval_and_cancels_real_pending_http() {
    let mut peer = peer::Peer::new();
    let mut app = Application::new(peer.port());
    app.tap("Create local wallet");
    app.text("Local wallet connected; not saved automatically");
    let account = app.account();
    assert_eq!(
        app.host.surface().kind(app.host.surface().root().unwrap()),
        NativeKind::ScrollView
    );
    app.tap("Read balance");
    app.text("Balance: 42 lamports");
    app.tap("Review test SOL transfer");
    app.text("Approve local wallet signature");
    let old_approval = find(&app.host, |node| {
        app.host.surface().props(node).text.as_deref() == Some("Approve signature")
    })
    .unwrap();
    app.tap("Reject signature");
    app.text("Transfer rejected; nothing submitted");
    assert_eq!(
        app.host
            .dispatch(old_approval, &NativeEvent::Press)
            .unwrap(),
        EventOutcome::Unhandled
    );
    assert_eq!(
        peer.methods(),
        ["getBalance", "getBalance", "getLatestBlockhash"]
    );
    assert_eq!(peer.requests()[0]["params"][0], account);
    peer.withhold();
    app.tap("Read balance");
    app.wait(|_| peer.methods().len() == 4);
    app.tap("Cancel request");
    app.text("Request cancelled; wallet disconnected");
    app.text("Create local wallet");
    app.wait(|_| peer.cancelled());
    app.tap("Create local wallet");
    app.text("Local wallet connected; not saved automatically");
    assert_ne!(app.account(), account);
    app.tap("Review signature");
    app.text("Approve local wallet signature");
    assert_eq!(
        app.host
            .dispatch(old_approval, &NativeEvent::Press)
            .unwrap(),
        EventOutcome::Unhandled
    );
    app.tap("Approve signature");
    app.text("Signature independently verified");
    app.close();
    peer.close().unwrap();
}

#[test]
fn failed_private_copy_reaches_the_guest_without_becoming_cancellation() {
    let mut peer = peer::Peer::new();
    let mut app = Application::new(peer.port());
    app.tap("Create local wallet");
    app.text("Local wallet connected; not saved automatically");
    let destination = app._sandbox.0.join("solana-wallet/wallet-backup.json");
    fs::write(&destination, b"existing private copy").unwrap();

    app.tap("Create private copy");
    app.text("wallet session request failed");
    app.text("Create local wallet");
    assert_eq!(fs::read(destination).unwrap(), b"existing private copy");
    let receipt = fs::read_to_string(app._sandbox.0.join("journey-diagnostics.txt")).unwrap();
    let backup = receipt.find("action backup\n").unwrap();
    let received = receipt.find("save received\n").unwrap();
    let admitted = receipt.find("save admitted\n").unwrap();
    let completed = receipt.find("save completed Native\n").unwrap();
    let ended = receipt.find("lease ended\n").unwrap();
    let guest = receipt.find("guest result bridge-native\n").unwrap();
    let failure = receipt
        .find("failure storage io=Some((AlreadyExists,")
        .unwrap();
    assert!(backup < received && received < admitted && admitted < completed && completed < ended);
    assert!(ended < guest);
    assert!(backup < failure);
    assert!(receipt.contains("published=Some(false)"));
    assert!(receipt.contains("stage=Some(PublishDestination)"));
    assert!(!receipt.contains("existing private copy"));
    app.close();
    peer.close().unwrap();
}

#[test]
fn approval_submits_once_and_matches_an_independent_signed_wire_oracle() {
    let mut peer = peer::Peer::new();
    let mut app = Application::new(peer.port());
    app.tap("Create local wallet");
    app.text("Local wallet connected; not saved automatically");
    let account = app.account();
    peer.allow_transfer(&account);
    app.tap("Review test SOL transfer");
    app.text("Approve local wallet signature");
    let recipient = bs58::encode([98; 32]).into_string();
    app.text(&format!(
        "Transfer 1000000 lamports from {account} to {recipient}. Network fees also apply."
    ));
    app.tap("Approve signature");
    app.wait(|_| peer.signature().is_some());
    let signature = peer.signature().unwrap();
    app.text(&format!(
        "Transfer confirmed: {signature}. Recipient balance: 1000042 lamports"
    ));
    assert_eq!(
        peer.methods(),
        [
            "getBalance",
            "getLatestBlockhash",
            "sendTransaction",
            "getSignatureStatuses",
            "getBalance"
        ]
    );
    let requests = peer.requests();
    assert_eq!(requests[0]["params"][0], recipient);
    assert_eq!(requests[4]["params"][0], recipient);
    app.close();
    peer.close().unwrap();
}

#[test]
fn rpc_rejection_preserves_the_selected_key_and_its_signing_authority() {
    let mut peer = peer::Peer::new();
    let mut app = Application::new(peer.port());
    app.tap("Create local wallet");
    app.text("Local wallet connected; not saved automatically");
    let account = app.account();
    peer.reject_reads(true);
    app.tap("Read balance");
    app.wait(|host| {
        find(host, |node| {
            let props = host.surface().props(node);
            props.id.as_deref() == Some("wallet_fixture.status")
                && props
                    .text
                    .as_deref()
                    .is_some_and(|text| text.contains("-32000"))
        })
        .is_some()
    });
    assert_eq!(app.account(), account);
    peer.reject_reads(false);
    app.tap("Read balance");
    app.text("Balance: 42 lamports");
    app.tap("Review signature");
    app.text("Approve local wallet signature");
    app.tap("Approve signature");
    app.text("Signature independently verified");
    assert_eq!(app.account(), account);
    assert_eq!(peer.methods(), ["getBalance", "getBalance"]);
    app.close();
    peer.close().unwrap();
}
