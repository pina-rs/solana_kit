//! A Surfnet delegates requests to an already configured Kit protocol owner.

pub(crate) mod funding;

use crate::errors::SurfpoolRpcError;
use solana_kit_client::{Client, Transport, methods::Method};

/// Typed Surfpool access with the caller's existing correlation and transport owner.
///
/// This type creates no runtime, process, payer, socket, task, timeout or retry.
/// Dropping an in-flight operation drops its concrete transport future.
pub struct Surfnet<T> {
    rpc: Client<T>,
}

impl<T> Surfnet<T> {
    /// Wraps an existing Kit client without constructing a second network stack.
    pub const fn new(rpc: Client<T>) -> Self {
        Self { rpc }
    }

    /// Borrows the same client for normal Solana RPC operations.
    pub const fn rpc(&self) -> &Client<T> {
        &self.rpc
    }

    /// Returns ownership of the shared client to the caller.
    pub fn into_rpc(self) -> Client<T> {
        self.rpc
    }
}

impl<T: Transport> Surfnet<T> {
    /// Executes one typed request, retaining the original error for explicit inspection.
    pub async fn execute<M: Method>(&self, method: M) -> Result<M::Response, SurfpoolRpcError> {
        self.rpc
            .call(method)
            .await
            .map_err(|cause| SurfpoolRpcError {
                method: M::NAME,
                cause,
            })
    }

    /// Returns the exact minimum rent-exempt balance through Kit's standard method.
    pub async fn minimum_balance(&self, space: u64) -> Result<u64, SurfpoolRpcError> {
        self.execute(
            solana_kit_client::methods::accounts::MinimumBalanceForRentExemption(space, None),
        )
        .await
    }
}

/// Composes typed Surfpool access, caller-owned subscriptions and a funded payer.
///
/// `Payer` can be a Kit signer or another caller-owned signing capability.
/// Ownership and resource shutdown follow those supplied values; no secret is copied.
pub struct SurfpoolClient<T, Subscriptions, Payer> {
    /// Surfpool cheatcodes and the shared normal RPC owner.
    pub surfnet: Surfnet<T>,
    /// Caller-configured subscription owner, retaining its cancellation policy.
    pub rpc_subscriptions: Subscriptions,
    /// Caller-supplied funded payer.
    pub payer: Payer,
}

impl<T, Subscriptions, Payer> SurfpoolClient<T, Subscriptions, Payer> {
    /// Takes explicit ownership of already configured client capabilities.
    pub const fn new(rpc: Client<T>, subscriptions: Subscriptions, payer: Payer) -> Self {
        Self {
            surfnet: Surfnet::new(rpc),
            rpc_subscriptions: subscriptions,
            payer,
        }
    }

    /// Borrows the same normal RPC client used by all cheatcodes.
    pub const fn rpc(&self) -> &Client<T> {
        self.surfnet.rpc()
    }
}
