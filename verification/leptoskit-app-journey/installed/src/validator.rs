//! The responding RPC identity must match this live child's newly created ledger.
use crate::{Error, Result, ports::Ports, require, rpc::Rpc};
use serde_json::json;
use std::{
    ffi::OsStr,
    fs::{self, File},
    path::{Path, PathBuf},
    process::{Child, Command, Stdio},
    thread,
    time::{Duration, Instant},
};

const STARTUP: Duration = Duration::from_secs(120);
const OBSERVATION_INTERVAL: Duration = Duration::from_millis(100);
const PUBLIC_KEY_BYTES: usize = 32;
const KEYPAIR_BYTES: usize = 64;

pub(crate) struct Validator {
    child: Option<Child>,
    ports: Ports,
    ledger: PathBuf,
    pub rpc: Rpc,
}

impl Validator {
    pub(crate) fn start(directory: &Path, executable: &OsStr) -> Result<Self> {
        let mut ports = Ports::acquire()?;
        let ledger = directory.join("ledger");
        fs::create_dir(&ledger)?;
        let endpoint = format!("http://127.0.0.1:{}", ports.rpc);
        let rpc = Rpc::new(&endpoint)?;
        let config = configuration(directory, &ports)?;

        // This public genesis destination is never used as a signing wallet.
        // The validator owns its separate fresh faucet key; no client key is read.
        let mut mint = [0_u8; PUBLIC_KEY_BYTES];
        getrandom::fill(&mut mint)
            .map_err(|error| Error::Configuration(format!("native entropy: {error}")))?;
        let mint = bs58::encode(mint).into_string();
        let log = File::create(directory.join("validator.log"))?;
        ports.release_listeners();

        let child = Command::new(executable)
            .arg("--config")
            .arg(config)
            .arg("--ledger")
            .arg(&ledger)
            .args(["--bind-address", "127.0.0.1", "--rpc-port"])
            .arg(ports.rpc.to_string())
            .arg("--faucet-port")
            .arg(ports.faucet.to_string())
            .arg("--gossip-port")
            .arg(ports.gossip.to_string())
            .arg("--dynamic-port-range")
            .arg(&ports.dynamic)
            .args(["--mint", &mint, "--quiet"])
            .stdin(Stdio::null())
            .stdout(log.try_clone()?)
            .stderr(log)
            .spawn()?;
        let mut node = Self {
            child: Some(child),
            ports,
            ledger,
            rpc,
        };
        node.ready()?;
        Ok(node)
    }

    pub(crate) fn guest_endpoints(&self, host: &str) -> serde_json::Value {
        json!({
            "http":format!("http://{host}:{}", self.ports.rpc),
            "socket":format!("ws://{host}:{}", self.ports.rpc+1),
        })
    }

    pub(crate) fn alive(&mut self) -> Result<()> {
        let child = self
            .child
            .as_mut()
            .ok_or_else(|| Error::Response("owned validator already closed".into()))?;
        require(child.try_wait()?.is_none(), "owned validator exited")
    }

    /// Health can succeed while the ledger is still on slot zero. The UI starts
    /// only after identity matches this child and a processed slot exists.
    fn ready(&mut self) -> Result<()> {
        let deadline = Instant::now() + STARTUP;

        loop {
            if self.poll_ready()? {
                return Ok(());
            }

            require(Instant::now() < deadline, "validator startup deadline")?;
            thread::sleep(OBSERVATION_INTERVAL);
        }
    }

    fn poll_ready(&mut self) -> Result<bool> {
        self.alive()?;

        match self.rpc.healthy() {
            Ok(true) => {}
            Ok(false) => return Ok(false),
            Err(Error::Transport(error)) if rpc_stalled(&error) => return Ok(false),
            Err(error) => return Err(error),
        }

        self.same_identity()?;
        self.alive()?;
        processed_slot_open(&self.rpc)
    }

    fn same_identity(&self) -> Result<()> {
        let keypair: Vec<u8> =
            serde_json::from_slice(&fs::read(self.ledger.join("validator-keypair.json"))?)?;
        require(
            keypair.len() == KEYPAIR_BYTES,
            "fresh node identity keypair length",
        )?;
        let expected = bs58::encode(&keypair[PUBLIC_KEY_BYTES..]).into_string();
        require(
            self.rpc.call("getIdentity", json!([]))?["identity"] == expected,
            "RPC identity differs from owned ledger",
        )
    }

    pub(crate) fn close(&mut self) -> Result<()> {
        if let Some(child) = self.child.as_mut() {
            if child.try_wait()?.is_none() {
                child.kill()?;
            }

            child.wait()?;
            self.child = None;
        }

        if self.ledger.exists() {
            fs::remove_dir_all(&self.ledger)?;
        }

        Ok(())
    }
}

impl Drop for Validator {
    fn drop(&mut self) {
        if let Err(error) = self.close() {
            eprintln!("Owned validator cleanup failed: {error}");
        }
    }
}

/// `getSlot` answers `0` before the ledger moves. Confirmed reads then block
/// inside the client deadline, so startup keeps polling until a slot exists.
fn processed_slot_open(rpc: &Rpc) -> Result<bool> {
    let slot = match rpc.call("getSlot", json!([{"commitment":"processed"}])) {
        Ok(slot) => slot,
        Err(Error::Transport(error)) if rpc_stalled(&error) => return Ok(false),
        Err(error) => return Err(error),
    };

    open_slot(&slot)
}

fn open_slot(slot: &serde_json::Value) -> Result<bool> {
    match slot.as_u64() {
        Some(slot) => Ok(slot > 0),
        None => Err(Error::Response("slot is not an unsigned integer".into())),
    }
}

fn rpc_stalled(error: &reqwest::Error) -> bool {
    error.is_connect() || error.is_timeout()
}

fn configuration(directory: &Path, ports: &Ports) -> Result<PathBuf> {
    let config = directory.join("solana-config.json");
    fs::write(
        &config,
        serde_json::to_vec(&json!({
            "json_rpc_url":format!("http://127.0.0.1:{}",ports.rpc),
            "websocket_url":format!("ws://127.0.0.1:{}",ports.rpc+1),
            "keypair_path":directory.join("unused-client-keypair.json"),
            "address_labels":{},"commitment":"confirmed"
        }))?,
    )?;
    Ok(config)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn startup_waits_while_the_processed_slot_is_still_zero() {
        assert!(!open_slot(&json!(0)).unwrap());
        assert!(open_slot(&json!(1)).unwrap());
        assert!(open_slot(&json!("none")).is_err());
    }
}
