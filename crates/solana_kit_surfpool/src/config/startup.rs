//! Exact CLI argument preparation, without process creation or environment reads.

use super::{ConfigError, Ports, SurfnetConfig};
use solana_kit_core::Address;

impl SurfnetConfig {
    /// Produces pinned `surfpool start` arguments using host-selected ports and payer.
    ///
    /// # Errors
    /// Rejects port collisions and any changed caller-pinned port.
    /// The host remains responsible for process lifecycle, readiness and shutdown.
    pub fn start_args(&self, ports: Ports, payer: Address) -> Result<Vec<String>, ConfigError> {
        self.validate()?;
        ports.matches(self)?;

        if self
            .payer
            .as_ref()
            .is_some_and(|configured| configured.address() != payer)
        {
            return Err(ConfigError::ConfiguredPayerChanged);
        }

        let mut arguments = vec![
            "start".into(),
            "--port".into(),
            ports.rpc.to_string(),
            "--ws-port".into(),
            ports.websocket.to_string(),
            "--studio-port".into(),
            ports.studio.to_string(),
            "--host".into(),
            self.host.clone(),
            "--no-tui".into(),
            "--no-studio".into(),
            "--block-production-mode".into(),
            self.block_production_mode.cli_value().into(),
            "--slot-time".into(),
            self.slot_time_ms.to_string(),
            "--airdrop-amount".into(),
            self.airdrop_sol.to_string(),
        ];
        self.upstream_args(&mut arguments);
        arguments.extend(["--airdrop".into(), payer.to_string()]);
        append_addresses(&mut arguments, "--airdrop", &self.airdrop_addresses);
        append_addresses(&mut arguments, "--feature", &self.enable_features);
        append_addresses(&mut arguments, "--disable-feature", &self.disable_features);

        if self.all_features {
            arguments.push("--features-all".into());
        }

        if self.skip_blockhash_check {
            arguments.push("--skip-blockhash-check".into());
        }

        Ok(arguments)
    }

    fn upstream_args(&self, arguments: &mut Vec<String>) {
        if let Some(remote) = &self.remote_rpc_url {
            arguments.extend(["--rpc-url".into(), remote.clone()]);

            return;
        }

        if self.offline {
            arguments.push("--offline".into());
        }
    }
}

fn append_addresses(arguments: &mut Vec<String>, flag: &str, addresses: &[Address]) {
    for address in addresses {
        arguments.extend([flag.into(), address.to_string()]);
    }
}
