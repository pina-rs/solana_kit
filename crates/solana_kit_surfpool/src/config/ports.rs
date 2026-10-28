//! Port validation is deterministic; probing and reservation belong to the host.

use std::num::NonZeroU16;

/// Three distinct, nonzero ports chosen explicitly by the process owner.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Ports {
    /// HTTP RPC port.
    pub rpc: NonZeroU16,
    /// WebSocket RPC port.
    pub websocket: NonZeroU16,
    /// Studio/scenario port, bound even under `--no-studio`.
    pub studio: NonZeroU16,
}

impl Ports {
    /// Validates the complete assignment before building process arguments.
    pub fn new(
        rpc: NonZeroU16,
        websocket: NonZeroU16,
        studio: NonZeroU16,
    ) -> Result<Self, ConfigError> {
        validate([Some(rpc), Some(websocket), Some(studio)])?;
        Ok(Self {
            rpc,
            websocket,
            studio,
        })
    }

    pub(crate) fn matches(self, config: &super::SurfnetConfig) -> Result<(), ConfigError> {
        validate([Some(self.rpc), Some(self.websocket), Some(self.studio)])?;
        let requested = [config.rpc_port, config.ws_port, config.studio_port];
        let selected = [self.rpc, self.websocket, self.studio];

        if requested
            .into_iter()
            .zip(selected)
            .any(|(fixed, actual)| fixed.is_some_and(|fixed| fixed != actual))
        {
            return Err(ConfigError::PinnedPortChanged);
        }

        Ok(())
    }
}

pub(crate) fn validate(ports: [Option<NonZeroU16>; 3]) -> Result<(), ConfigError> {
    let assigned: Vec<_> = ports.into_iter().flatten().collect();

    if assigned
        .iter()
        .enumerate()
        .any(|(index, port)| assigned[index + 1..].contains(port))
    {
        return Err(ConfigError::PortCollision);
    }

    Ok(())
}

/// Invalid startup settings, before any host resource is acquired.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ConfigError {
    /// Two runtime services would bind the same port.
    PortCollision,
    /// The process owner changed a port explicitly pinned by the caller.
    PinnedPortChanged,
    /// The process owner selected a different payer from configured metadata.
    ConfiguredPayerChanged,
}

impl core::fmt::Display for ConfigError {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        formatter.write_str(match self {
            Self::PortCollision => "Surfpool RPC, WebSocket and Studio ports must differ",
            Self::PinnedPortChanged => "selected port differs from an explicitly configured port",
            Self::ConfiguredPayerChanged => "selected payer differs from the configured payer",
        })
    }
}

impl std::error::Error for ConfigError {}
