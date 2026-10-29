//! Borrowed, resumable shutdown never signals a reaped direct child.

use super::{ChildState, CloseError, OwnedSurfpool, POLL, ShutdownError, deadline};
use serde::{Serialize, Serializer};
use solana_kit_client::methods::{Method, node::GenesisHash};
use std::{fmt, mem, process::ExitStatus, time::Duration};
use tokio::time::Instant;

/// Positive finite budgets for one explicit close attempt.
#[derive(Clone, Copy, Debug)]
pub struct ClosePolicy {
    /// Identity and private exit RPC share this bounded request budget.
    pub request_timeout: Duration,
    /// Total graceful phase including identity/exit RPC.
    pub graceful_timeout: Duration,
    /// Bounded nonblocking exit observation after handle-based force termination.
    pub reap_timeout: Duration,
}

impl Default for ClosePolicy {
    fn default() -> Self {
        Self {
            request_timeout: Duration::from_secs(2),
            graceful_timeout: Duration::from_secs(3),
            reap_timeout: Duration::from_secs(2),
        }
    }
}

/// Private exit request outcome, independent of confirmed child cleanup.
#[derive(Default)]
pub enum ShutdownRequest {
    /// No verified endpoint was available, or the child already exited.
    #[default]
    NotRequested,
    /// Cancellation may have occurred during identity checking or sending exit.
    /// A later close never automatically resends this uncertain request.
    Unconfirmed,
    /// Exact-genesis endpoint acknowledged the private exit method.
    Acknowledged,
    /// Original request/identity/observation failure remains deliberately inspectable.
    Failed(ShutdownError),
}

impl fmt::Debug for ShutdownRequest {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::NotRequested => "NotRequested",
            Self::Unconfirmed => "Unconfirmed",
            Self::Acknowledged => "Acknowledged",
            Self::Failed(_) => "Failed(redacted)",
        });
    }
}

/// Cached cleanup confirmation; nonzero exit status is not application success.
#[derive(Debug)]
pub struct CloseReceipt {
    /// Actual direct-child status, observed/reaped through the retained handle.
    pub status: ExitStatus,
    /// True if this owner successfully requested handle-based force termination.
    pub forced: bool,
    /// Shutdown request outcome, including original errors even when reaping succeeds.
    pub request: ShutdownRequest,
}

impl OwnedSurfpool {
    /// Confirms reaping, retaining the owner/runtime on unconfirmed cleanup.
    ///
    /// Only a previously verified endpoint with a fresh matching genesis recheck
    /// receives private `exit`. Request errors remain in the receipt. A graceful
    /// deadline is followed by handle-based kill and bounded nonblocking reaping.
    /// Cancelled attempts retain all state; a new explicit call receives fresh
    /// finite wait budgets but never resends uncertain exit or successful kill.
    /// Cancellation during identity checking can therefore lead to force-kill on
    /// a later close even when no exit request was actually sent.
    ///
    /// Cached receipts make repeated close idempotent without signaling after
    /// reaping. The borrowed receipt preserves non-Clone typed causes. Caller
    /// directory cleanup is always separate. Drop alone cannot confirm cleanup.
    ///
    /// # Errors
    /// Invalid policy, original observation/kill failure or bounded reap timeout.
    /// These retain the live handle/runtime; no cleanup runtime/task is spawned.
    pub async fn close(&mut self, policy: ClosePolicy) -> Result<&CloseReceipt, CloseError> {
        if self.receipt.is_none() {
            self.close_live(policy).await?;
        }

        Ok(self
            .receipt
            .as_ref()
            .expect("successful close stores a receipt"))
    }

    async fn close_live(&mut self, policy: ClosePolicy) -> Result<(), CloseError> {
        if let Some(status) = self.observe().map_err(CloseError::Observe)? {
            self.complete(status);

            return Ok(());
        }

        let graceful = deadline(policy.graceful_timeout).ok_or(CloseError::InvalidPolicy)?;
        let request = deadline(policy.request_timeout).ok_or(CloseError::InvalidPolicy)?;
        // Validate now; the actual reap budget begins after force termination.
        let _ = deadline(policy.reap_timeout).ok_or(CloseError::InvalidPolicy)?;
        self.closing = true;

        self.attempt_shutdown(request.min(graceful)).await;

        if let Some(status) = self.graceful_phase(graceful).await? {
            self.complete(status);

            return Ok(());
        }

        let reap = deadline(policy.reap_timeout).ok_or(CloseError::InvalidPolicy)?;
        let status = self.drain(reap).await?.ok_or(CloseError::Deadline)?;

        self.complete(status);

        Ok(())
    }

    async fn attempt_shutdown(&mut self, deadline: Instant) {
        if !self.verified || self.forced || !matches!(self.request, ShutdownRequest::NotRequested) {
            return;
        }

        self.request = ShutdownRequest::Unconfirmed;

        self.request = match tokio::time::timeout_at(deadline, self.request_exit()).await {
            Ok(Ok(())) => ShutdownRequest::Acknowledged,
            Ok(Err(error)) => ShutdownRequest::Failed(error),
            Err(_) => ShutdownRequest::Failed(ShutdownError::Deadline),
        };
    }

    async fn graceful_phase(
        &mut self,
        deadline: Instant,
    ) -> Result<Option<ExitStatus>, CloseError> {
        if self.forced {
            return Ok(None);
        }

        if let Some(status) = self.drain(deadline).await? {
            return Ok(Some(status));
        }

        // An exited unreaped child retains its identity until try_wait reaps it.
        if let Some(status) = self.observe().map_err(CloseError::Observe)? {
            return Ok(Some(status));
        }

        if let ChildState::Live(child) = &mut self.child {
            child.kill().map_err(CloseError::Kill)?;
            self.forced = true;
        }

        Ok(None)
    }

    fn complete(&mut self, status: ExitStatus) {
        self.receipt = Some(CloseReceipt {
            status,
            forced: self.forced,
            request: mem::take(&mut self.request),
        })
    }

    async fn drain(&mut self, deadline: Instant) -> Result<Option<ExitStatus>, CloseError> {
        loop {
            if let Some(status) = self.observe().map_err(CloseError::Observe)? {
                return Ok(Some(status));
            }

            if Instant::now() >= deadline {
                return Ok(None);
            }

            tokio::time::sleep_until((Instant::now() + POLL).min(deadline)).await;
        }
    }

    async fn request_exit(&mut self) -> Result<(), ShutdownError> {
        self.shutdown_live()?;
        let genesis = self
            .surfnet
            .rpc()
            .call(GenesisHash)
            .await
            .map_err(ShutdownError::Rpc)?;
        self.shutdown_live()?;

        if genesis != self.genesis {
            return Err(ShutdownError::Identity);
        }

        self.surfnet
            .rpc()
            .call(Exit)
            .await
            .map_err(ShutdownError::Rpc)?;
        self.observe().map_err(ShutdownError::Observe)?;

        Ok(())
    }

    fn shutdown_live(&mut self) -> Result<(), ShutdownError> {
        if let Some(status) = self.observe().map_err(ShutdownError::Observe)? {
            return Err(ShutdownError::Exited(status));
        }

        Ok(())
    }
}

struct Exit;
impl Serialize for Exit {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        <[u8; 0]>::default().serialize(serializer)
    }
}
impl Method for Exit {
    type Response = ();
    const NAME: &'static str = "exit";
    fn decode(&self, value: &str) -> Result<(), solana_kit_client::Error> {
        serde_json::from_str(value).map_err(solana_kit_client::Error::Schema)
    }
}
