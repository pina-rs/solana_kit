//! An optional interpreter owner delegates all native task state to its driver.
use crate::{HostError, HostLimits, HttpDriver, Routed};
use leptoskit_bridge::PluginId;
use leptoskit_native::NativeBackend;
use leptoskit_wasm::WasmHost;
use solana_kit_native::{NativeHttp, NativeRest};

/// Actual interpreter and its single HTTP driver for application-owned routing.
///
/// Applications that already own their interpreter use [`HttpDriver`] directly.
/// Plugin Events remain application-owned in either ownership model.
pub struct HttpHost<B: NativeBackend> {
    host: Option<WasmHost<B>>,
    driver: HttpDriver,
}

impl<B: NativeBackend> HttpHost<B> {
    /// Takes ownership of an actual interpreter with explicit admission limits.
    pub fn new(mut host: WasmHost<B>, limits: HostLimits) -> Result<Self, HostError> {
        let driver = HttpDriver::new(&mut host, limits)?;
        Ok(Self {
            host: Some(host),
            driver,
        })
    }

    /// Accesses the owned interpreter for application events.
    ///
    /// Replacing its Bridge invalidates this driver's binding.
    pub fn host_mut(&mut self) -> Result<&mut WasmHost<B>, HostError> {
        self.host.as_mut().ok_or(HostError::Unloaded)
    }

    /// Starts the guest; drain after this or any application guest entry.
    pub fn start(&mut self, width: f32, height: f32) -> Result<(), HostError> {
        self.host_mut()?.start(width, height)?;
        Ok(())
    }

    /// Registers fixed endpoint, authentication, TLS and deadline policy.
    pub fn install(
        &mut self,
        name: &str,
        service: NativeHttp,
        granted: bool,
    ) -> Result<PluginId, HostError> {
        self.driver.install(
            self.host.as_mut().ok_or(HostError::Unloaded)?,
            name,
            service,
            granted,
        )
    }

    /// Drains cancellation and admits HTTP, returning unrelated work intact.
    pub fn drain(&mut self) -> Result<Routed, HostError> {
        self.driver
            .drain(self.host.as_mut().ok_or(HostError::Unloaded)?)
    }

    /// Registers one fixed named REST service through this same HTTP driver.
    pub fn install_rest(
        &mut self,
        name: &str,
        service: NativeRest<()>,
        granted: bool,
    ) -> Result<PluginId, HostError> {
        self.driver.install_rest(
            self.host.as_mut().ok_or(HostError::Unloaded)?,
            name,
            service,
            granted,
        )
    }

    /// Counts running and aborting native tasks until their join is observed.
    pub fn retained_jobs(&self) -> usize {
        self.driver.retained_jobs()
    }

    /// Joins and delivers one completion, or returns false when there are no jobs.
    ///
    /// Use [`HttpDriver::next_completion`] for a Pending idle wait that releases
    /// the interpreter for other application work.
    pub async fn next_completion(&mut self) -> Result<bool, HostError> {
        if self.driver.retained_jobs() == 0 {
            return Ok(false);
        }

        let completion = self.driver.next_completion().await?;
        self.driver
            .deliver(self.host.as_mut().ok_or(HostError::Unloaded)?, completion)?;
        Ok(true)
    }

    /// Closes admission, aborts and joins I/O, then unloads the owned interpreter.
    ///
    /// Dropping the owner aborts remaining tasks but cannot await their cleanup.
    pub async fn shutdown(&mut self) -> Result<(), HostError> {
        let result = self.driver.shutdown().await;
        self.host.take();
        result
    }
}

#[cfg(test)]
mod tests;
