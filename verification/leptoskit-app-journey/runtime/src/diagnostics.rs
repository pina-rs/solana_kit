//! Bounded verification receipts contain only fixed categories and numeric metadata.
use leptoskit_bridge::{Error, MethodId, PluginId, Request};
use solana_kit_leptoskit::wallet_service::{WalletFailure, WalletTrace};
use solana_kit_native::keys::{KeyFileError, KeyFileStage};
use std::{
    collections::VecDeque,
    fs::File,
    io::{self, Write},
    path::Path,
};

const LIMIT: usize = 128;
const FAILURE_LIMIT: usize = 16;
const ERROR_DEPTH: usize = 8;
const NODE_BYTES: usize = 8;
const ACTIONS: [&str; 11] = [
    "create",
    "load",
    "restore",
    "save",
    "backup",
    "review",
    "approve",
    "reject",
    "cancel",
    "disconnect",
    "android",
];

pub(super) struct Diagnostics {
    file: File,
    count: usize,
    plugin: PluginId,
    failures: VecDeque<WalletFailure>,
}

#[derive(Default)]
struct ErrorDetails {
    io: Option<(io::ErrorKind, Option<i32>)>,
    published: Option<bool>,
    stage: Option<KeyFileStage>,
}

impl Diagnostics {
    pub(super) fn new(host: &mut crate::Host, data: &Path) -> Result<Self, Error> {
        let file = File::create(data.join("journey-diagnostics.txt")).map_err(crate::native)?;
        let plugin = host.plugins().register(
            "solana-journey-observer",
            [MethodId(0), MethodId(1), MethodId(2)],
        )?;

        for method in [MethodId(0), MethodId(1), MethodId(2)] {
            host.plugins().grant(plugin, method, true)?;
        }

        Ok(Self {
            file,
            count: 0,
            plugin,
            failures: VecDeque::new(),
        })
    }

    pub(super) fn route(
        &mut self,
        host: &mut crate::Host,
        requests: &mut Vec<Request>,
    ) -> Result<(), Error> {
        let mut retained = Vec::with_capacity(requests.len());
        let mut completed = false;

        for request in std::mem::take(requests) {
            if request.plugin != self.plugin {
                retained.push(request);
                continue;
            }

            self.receipt(&request).map_err(crate::native)?;
            host.plugins().complete(request.id, Ok(Vec::new().into()))?;
            completed = true;
        }

        *requests = retained;

        if completed {
            host.wake_plugins().map_err(crate::native)?;
        }

        Ok(())
    }

    fn receipt(&mut self, request: &Request) -> io::Result<()> {
        let payload = request.payload.as_ref();

        match (request.method.0, payload) {
            (0, [node @ .., a, b, c, d]) if node.len() == NODE_BYTES => {
                self.press(node, [*a, *b, *c, *d])
            }

            (1, action) => self.action(action),
            (2, [0]) => self.record("guest result ok"),
            (2, [1]) => self.record("guest result bridge-native"),
            (2, [2]) => self.record("guest result bridge-cancelled"),
            (2, [3]) => self.record("guest result other"),
            _ => self.record("invalid-receipt"),
        }
    }

    fn press(&mut self, node: &[u8], handled: [u8; 4]) -> io::Result<()> {
        let node = i64::from_le_bytes(node.try_into().expect("eight-byte node"));
        let handled = i32::from_le_bytes(handled);

        if !matches!(handled, 0 | 1) {
            return self.record("invalid-receipt");
        }

        self.record(&format!("press node={node} handled={handled}"))
    }

    fn action(&mut self, action: &[u8]) -> io::Result<()> {
        let action = std::str::from_utf8(action).ok();

        match action.filter(|action| ACTIONS.contains(action)) {
            Some(action) => self.record(&format!("action {action}")),
            None => self.record("invalid-receipt"),
        }
    }

    pub(super) fn trace(&mut self, trace: WalletTrace) -> io::Result<()> {
        match trace {
            WalletTrace::SaveReceived => self.record("save received"),
            WalletTrace::SaveAdmitted => self.record("save admitted"),
            WalletTrace::SaveRejected(error) => self.record(&format!("save rejected {error:?}")),
            WalletTrace::SaveCompleted(Ok(())) => self.record("save completed ok"),
            WalletTrace::SaveCompleted(Err(error)) => {
                self.record(&format!("save completed {error:?}"))
            }
            WalletTrace::LeaseEnded => self.record("lease ended"),
            WalletTrace::Overflow => self.record("trace overflow"),
        }
    }

    pub(super) fn failure(&mut self, failure: WalletFailure) -> io::Result<()> {
        let category = match &failure {
            WalletFailure::Bridge(_) => "bridge",
            WalletFailure::Mobile(_) => "mobile",
            WalletFailure::Entropy(_) => "entropy",
            WalletFailure::Storage(_) => "storage",
            WalletFailure::Worker(_) => "worker",
            WalletFailure::Json(_) => "json",
        };

        let details = error_details(&failure);

        self.record(&format!(
            "failure {category} io={:?} published={:?} stage={:?}",
            details.io, details.published, details.stage
        ))?;

        if self.failures.len() == FAILURE_LIMIT {
            self.failures.pop_front();
        }

        self.failures.push_back(failure);
        Ok(())
    }

    fn record(&mut self, receipt: &str) -> io::Result<()> {
        if self.count == LIMIT {
            return Ok(());
        }

        self.count += 1;

        if self.count == LIMIT {
            writeln!(self.file, "receipt overflow")
        } else {
            writeln!(self.file, "{receipt}")
        }
    }
}

/// Traverse a bounded source chain without formatting errors that may contain paths.
fn error_details(failure: &WalletFailure) -> ErrorDetails {
    let mut source: Option<&(dyn std::error::Error + 'static)> = Some(failure);
    let mut details = ErrorDetails::default();

    for _ in 0..ERROR_DEPTH {
        let Some(error) = source else { break };

        if let Some(error) = error.downcast_ref::<io::Error>() {
            details.io = Some((error.kind(), error.raw_os_error()));
        }

        if let Some(error) = error.downcast_ref::<KeyFileError>() {
            details.published = Some(error.published());
            details.stage = error.stage();
        }

        source = error.source();
    }

    details
}
