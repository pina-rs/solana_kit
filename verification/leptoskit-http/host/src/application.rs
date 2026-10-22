//! Runs identical guest assertions through each public ownership choice.
use leptoskit_bridge::PluginId;
use leptoskit_native::HeadlessBackend;
use leptoskit_wasm::WasmHost;
use solana_kit_leptoskit::{HostError, HostLimits, HttpDriver, HttpHost, Routed};
use solana_kit_native::NativeHttp;

type Host = WasmHost<HeadlessBackend>;

pub(super) enum Application {
    Borrowed {
        host: Option<Host>,
        driver: HttpDriver,
    },
    Owned(HttpHost<HeadlessBackend>),
}

impl Application {
    pub(super) fn new(
        mut host: Host,
        limits: HostLimits,
        borrowed: bool,
    ) -> Result<Self, HostError> {
        if borrowed {
            let driver = HttpDriver::new(&mut host, limits)?;
            return Ok(Self::Borrowed {
                host: Some(host),
                driver,
            });
        }

        Ok(Self::Owned(HttpHost::new(host, limits)?))
    }

    pub(super) fn host_mut(&mut self) -> Result<&mut Host, HostError> {
        match self {
            Self::Borrowed { host, .. } => host.as_mut().ok_or(HostError::Unloaded),
            Self::Owned(owner) => owner.host_mut(),
        }
    }

    pub(super) fn start(&mut self, width: f32, height: f32) -> Result<(), HostError> {
        self.host_mut()?.start(width, height)?;
        Ok(())
    }

    pub(super) fn install(
        &mut self,
        name: &str,
        service: NativeHttp,
        granted: bool,
    ) -> Result<PluginId, HostError> {
        match self {
            Self::Borrowed { host, driver } => driver.install(
                host.as_mut().ok_or(HostError::Unloaded)?,
                name,
                service,
                granted,
            ),
            Self::Owned(owner) => owner.install(name, service, granted),
        }
    }

    pub(super) fn drain(&mut self) -> Result<Routed, HostError> {
        match self {
            Self::Borrowed { host, driver } => {
                driver.drain(host.as_mut().ok_or(HostError::Unloaded)?)
            }
            Self::Owned(owner) => owner.drain(),
        }
    }

    pub(super) fn retained_jobs(&self) -> usize {
        match self {
            Self::Borrowed { driver, .. } => driver.retained_jobs(),
            Self::Owned(owner) => owner.retained_jobs(),
        }
    }

    pub(super) async fn next_completion(&mut self) -> Result<bool, HostError> {
        match self {
            Self::Borrowed { host, driver } => {
                let completion = driver.next_completion().await?;
                driver.deliver(host.as_mut().ok_or(HostError::Unloaded)?, completion)?;
                Ok(true)
            }
            Self::Owned(owner) => owner.next_completion().await,
        }
    }

    pub(super) async fn shutdown(&mut self) -> Result<(), HostError> {
        match self {
            Self::Borrowed { host, driver } => {
                let result = driver.shutdown().await;
                host.take();
                result
            }
            Self::Owned(owner) => owner.shutdown().await,
        }
    }
}
