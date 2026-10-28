//! Explicit root-selected paths, fresh challenge and fixed loopback endpoints.

use crate::{Error, Result};
use solana_kit_core::{
    Hash,
    crypto::{Keypair, verify},
};
use solana_kit_surfpool::config::{Ports, SurfnetConfig};
use std::{
    ffi::{OsStr, OsString},
    fs::{self, DirBuilder, OpenOptions},
    io::Write,
    num::NonZeroU16,
    path::{Path, PathBuf},
    process::Command,
};
use zeroize::Zeroizing;

#[cfg(unix)]
use std::os::unix::fs::{DirBuilderExt, OpenOptionsExt};

const ARGUMENTS: usize = 8;
const SEED_BYTES: usize = 32;
const KEYPAIR_BYTES: usize = 64;
const TEST_SEED: u8 = 7;
const NONCE_MIN: usize = 16;
const NONCE_MAX: usize = 64;

/// Only this fresh path may be removed, after the retained child was reaped.
pub(crate) struct Config {
    pub executable: PathBuf,
    pub profile: PathBuf,
    pub directory: PathBuf,
    pub genesis: Hash,
    pub nonce: String,
    pub ports: Ports,
    pub body_probe: bool,
}

impl Config {
    /// Accepts positional OS paths and checked public scalar arguments.
    pub(crate) fn parse() -> Result<Self> {
        let arguments: Vec<_> = std::env::args_os().skip(1).collect();

        if !(arguments.len() == ARGUMENTS || arguments.len() == ARGUMENTS + 1) {
            return Err(Error::Configuration(
                "expected BIN PROFILE NEW_RUNTIME GENESIS NONCE RPC WS STUDIO [--body-cap]",
            ));
        }

        let [
            executable,
            profile,
            directory,
            genesis,
            nonce,
            rpc,
            websocket,
            studio,
            ..,
        ] = arguments.as_slice()
        else {
            return Err(Error::Configuration("missing startup argument"));
        };
        let nonce = checked_nonce(text(nonce)?)?;
        let ports = Ports::new(
            port(text(rpc)?)?,
            port(text(websocket)?)?,
            port(text(studio)?)?,
        )
        .map_err(|_| Error::Configuration("three distinct nonzero ports required"))?;
        let directory = PathBuf::from(directory);

        if !directory.is_absolute() || directory.exists() {
            return Err(Error::Configuration(
                "runtime must be a fresh absolute path",
            ));
        }

        let body_probe = arguments.get(ARGUMENTS).is_some();

        if body_probe && arguments[ARGUMENTS] != "--body-cap" {
            return Err(Error::Configuration("unknown probe option"));
        }

        Ok(Self {
            executable: absolute_file(Path::new(executable))?,
            profile: absolute_file(Path::new(profile))?,
            directory,
            genesis: text(genesis)?
                .parse()
                .map_err(|_| Error::Configuration("invalid genesis hash"))?,
            nonce,
            ports,
            body_probe,
        })
    }

    /// Routes the Surfpool child executable through the supplied profile.
    pub(crate) fn boundary(&self, executable: &OsStr) -> Command {
        let mut command = Command::new("/usr/bin/sandbox-exec");
        command
            .arg("-f")
            .arg(&self.profile)
            .arg(executable)
            .current_dir(&self.directory)
            .env_clear()
            .env("PATH", "/usr/bin:/bin")
            .env("XDG_CONFIG_HOME", self.directory.join("config"))
            .env("TMPDIR", self.directory.join("tmp"));

        command
    }

    /// Uses the portable CLI builder, adding only verified 1.7 host flags.
    pub(crate) fn arguments(&self, payer: &Keypair) -> Result<Vec<OsString>> {
        let prepared = SurfnetConfig::default()
            .start_args(self.ports, payer.address())
            .map_err(|_| Error::Configuration("portable startup configuration rejected"))?;
        let mut args: Vec<OsString> = prepared.into_iter().map(OsString::from).collect();
        args.extend([
            "--no-deploy".into(),
            "--db".into(),
            ":memory:".into(),
            "--log-level".into(),
            "none".into(),
            "--disable-instruction-profiling".into(),
            "--genesis-hash".into(),
            self.genesis.to_string().into(),
            "--surfnet-id".into(),
            self.nonce.clone().into(),
            "--airdrop-keypair-path".into(),
            self.directory.join("payer.json").into_os_string(),
            "--log-path".into(),
            self.directory.join("logs").into_os_string(),
        ]);

        Ok(args)
    }

    /// Creates explicit private directories and a known test key, never an ambient wallet.
    pub(crate) fn prepare(&self) -> Result<Keypair> {
        private_directory(&self.directory)?;

        for name in ["config", "tmp", "logs"] {
            private_directory(&self.directory.join(name))?;
        }

        payer(&self.directory)
    }

    /// Selects only an IP-literal loopback endpoint, with no DNS or auth data.
    pub(crate) fn endpoint(&self) -> String {
        format!("http://127.0.0.1:{}", self.ports.rpc)
    }
}

/// Rejects encoding loss for public scalar values while paths remain OS strings.
fn text(value: &OsStr) -> Result<&str> {
    value
        .to_str()
        .ok_or(Error::Configuration("non-UTF8 scalar"))
}

/// The root must provide freshness; this checks only the nonce's bounded syntax.
fn checked_nonce(value: &str) -> Result<String> {
    if !(NONCE_MIN..=NONCE_MAX).contains(&value.len())
        || !value.bytes().all(|byte| byte.is_ascii_alphanumeric())
    {
        return Err(Error::Configuration(
            "nonce must be fresh alphanumeric text",
        ));
    }

    Ok(value.to_owned())
}

/// Rejects relative or non-regular executable/profile selections before launch.
fn absolute_file(path: &Path) -> Result<PathBuf> {
    if !path.is_absolute() || !fs::metadata(path)?.is_file() {
        return Err(Error::Configuration(
            "explicit regular executable/profile required",
        ));
    }

    Ok(fs::canonicalize(path)?)
}

/// Keeps zero a rejected port rather than guessing a server-selected address.
fn port(value: &str) -> Result<NonZeroU16> {
    value
        .parse()
        .ok()
        .and_then(NonZeroU16::new)
        .ok_or(Error::Configuration("invalid port"))
}

/// Validates both key halves and a real Kit signature; all secret buffers erase on drop.
fn payer(directory: &Path) -> Result<Keypair> {
    let seed = Zeroizing::new([TEST_SEED; SEED_BYTES]);
    let derived = Keypair::from_seed(&seed);
    let mut material = Zeroizing::new([0; KEYPAIR_BYTES]);
    material[..SEED_BYTES].copy_from_slice(&seed[..]);
    material[SEED_BYTES..].copy_from_slice(derived.address().as_bytes());

    let imported =
        Keypair::from_bytes(&material[..]).map_err(|_| Error::Check("test keypair import"))?;
    let challenge = b"surfpool-native verification test payer";
    verify(&imported.address(), challenge, &imported.sign(challenge))
        .map_err(|_| Error::Check("test payer signature validation"))?;
    let encoded = Zeroizing::new(
        serde_json::to_vec(&material[..]).map_err(|_| Error::Check("test key serialization"))?,
    );
    let mut options = OpenOptions::new();
    options.write(true).create_new(true);
    #[cfg(unix)]
    options.mode(0o600);
    let mut file = options.open(directory.join("payer.json"))?;
    file.write_all(&encoded)?;

    Ok(imported)
}

fn private_directory(path: &Path) -> std::io::Result<()> {
    let mut builder = DirBuilder::new();
    builder.recursive(false);
    #[cfg(unix)]
    builder.mode(0o700);
    builder.create(path)
}
