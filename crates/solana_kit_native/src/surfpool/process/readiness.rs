//! One deadline bounds both RPC probes and sleeps while the owner remains borrowed.

use super::{OwnedSurfpool, POLL, StartError, deadline, status};
use solana_kit_client::{Error, TransportFailureKind, methods::node::GenesisHash};
use solana_kit_surfpool::cheatcodes::GetSurfnetInfo;
use std::time::Duration;

impl OwnedSurfpool {
    /// Waits for this live child's fresh-genesis RPC and sealed completed startup.
    ///
    /// Readiness proves neither WebSocket delivery nor Studio listener ownership.
    /// Genesis freshness is a caller obligation and collision protection, not an
    /// attestation against a malicious local peer. Only typed connection/deadline
    /// failures and known pending phases are retried within the one total budget.
    /// Dropping the future drops its RPC/timer work and retains the child owner.
    ///
    /// # Errors
    /// Returns the cached child exit, exact RPC cause, identity/status failure or
    /// total timeout. The caller must subsequently close the retained owner.
    pub async fn wait_rpc_ready(&mut self, timeout: Duration) -> Result<(), StartError> {
        if self.closing {
            return Err(StartError::Closing);
        }

        let deadline = deadline(timeout).ok_or(StartError::InvalidTimeout)?;

        tokio::time::timeout_at(deadline, self.ready_loop())
            .await
            .map_err(|_| StartError::Deadline)?
    }

    async fn ready_loop(&mut self) -> Result<(), StartError> {
        loop {
            self.live()?;

            if self.probe().await? {
                self.live()?;
                self.verified = true;

                return Ok(());
            }

            tokio::time::sleep(POLL).await;
        }
    }

    fn live(&mut self) -> Result<(), StartError> {
        if let Some(status) = self.observe().map_err(StartError::Observe)? {
            return Err(StartError::Exited(status));
        }

        Ok(())
    }

    async fn probe(&mut self) -> Result<bool, StartError> {
        if !self.matching_genesis().await? {
            return Ok(false);
        }

        let info = self.surfnet.execute(GetSurfnetInfo).await;
        self.live()?;

        match info {
            Ok(info) => status::ready(&info.value).map_err(StartError::Startup),

            Err(error) if pending(&error.cause) => Ok(false),
            Err(error) => Err(StartError::Surfpool(error)),
        }
    }

    /// Only typed connection/deadline errors may delay identity verification.
    async fn matching_genesis(&mut self) -> Result<bool, StartError> {
        let genesis = self.surfnet.rpc().call(GenesisHash).await;
        self.live()?;

        let genesis = match genesis {
            Ok(genesis) => genesis,

            Err(error) if pending(&error) => return Ok(false),
            Err(error) => return Err(StartError::Rpc(error)),
        };

        if genesis != self.genesis {
            return Err(StartError::Identity);
        }

        Ok(true)
    }
}

fn pending(error: &Error) -> bool {
    matches!(error, Error::Transport(error)
        if matches!(error.kind, TransportFailureKind::Io | TransportFailureKind::Deadline))
}
