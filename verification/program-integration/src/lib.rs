//! Owned local-validator harness for real Solana Kit program instructions.
//!
//! Each [`IntegrationEnv`] owns one Agave test validator, one private ledger and
//! one disposable payer. It uses the SDK's native HTTP transport, typed RPC
//! methods, transaction compiler, signer and confirmation reducer; it never
//! reads `~/.config/solana`, a borrowed validator, a remote RPC or a fixed ledger.

mod binary;
mod error;
/// Pinned external program artifacts and exact byte verification.
pub mod fixtures;
mod node;
mod ports;
mod process;
mod reads;
mod rpc;
mod submit;

pub use error::{Error, SubmitError};
pub use node::NodeError;

use fixtures::{FixtureDirectory, LoadedProgram};
use node::Node;
use solana_kit_client::{
    Client, Limits,
    methods::node::{GenesisHash, GetHealth, Identity, Version as NodeVersionRequest},
};
use solana_kit_core::{
    Address, Hash,
    crypto::{Entropy, ExportableKeypair, Keypair},
};
use solana_kit_native::{HttpConfig, NativeHttp, keys::NativeEntropy};
use std::{
    cell::Cell,
    fmt,
    path::{Path, PathBuf},
    time::{Duration, Instant},
};

const LAMPORTS_PER_SOL: u64 = 1_000_000_000;
const AIRDROP_LAMPORTS: u64 = 200 * LAMPORTS_PER_SOL;
const STARTUP_DEADLINE: Duration = Duration::from_secs(120);
const AIRDROP_DEADLINE: Duration = Duration::from_secs(90);
const RPC_POLL: Duration = Duration::from_millis(200);
const RPC_BYTES: usize = 16 << 20;
const RPC_DEPTH: usize = 64;
const EXPECTED_NODE_VERSION: &str = "4.2.";

const CODEC_ACCOUNT_BYTES: usize = 16_384;
const PARSED_ACCOUNT_BYTES: usize = 1 << 20;

/// Caller-owned codec budget for program frames and account snapshots.
///
/// Account tails are charged one item per byte by the core `Bytes` decoder, so
/// the item budget covers a full config/loader allocation rather than a count.
pub const CODEC_LIMITS: solana_kit_core::codecs::Limits =
    solana_kit_core::codecs::Limits::new(CODEC_ACCOUNT_BYTES, CODEC_ACCOUNT_BYTES);

/// Bounded budget for one node `jsonParsed` payload projection.
pub const PARSED_LIMITS: Limits = Limits {
    bytes: PARSED_ACCOUNT_BYTES,
    depth: RPC_DEPTH,
};

type AccountFactory =
    Box<dyn FnOnce(Address) -> Result<Vec<fixtures::LocalAccount>, Error> + Send + 'static>;

/// One private validator with a fresh disposable payer and typed RPC client.
pub struct IntegrationEnv {
    node: Node,
    client: Client<NativeHttp>,
    payer: Keypair,
    payer_address: Address,
    entropy: NativeEntropy,
    loaded: Vec<LoadedProgram>,
    fixture_directory: Option<PathBuf>,
    observed_slot: Cell<u64>,
    /// Exact `solana-core` version reported by the owned node.
    pub validator_version: String,
    /// Identity recorded from the owned ledger and challenged through RPC.
    pub validator_identity: Address,
    /// Genesis hash recorded after readiness and challenged at shutdown.
    pub genesis: Hash,
}

impl fmt::Debug for IntegrationEnv {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("IntegrationEnv")
            .field("rpc_port", &self.node.rpc_port())
            .field("validator_version", &self.validator_version)
            .field("validator_identity", &self.validator_identity)
            .finish_non_exhaustive()
    }
}

impl IntegrationEnv {
    /// Starts one owned validator, proves its identity and funds a fresh payer.
    ///
    /// Missing or wrong binaries fail before any test can appear to pass.
    pub async fn start() -> Result<Self, Error> {
        Self::start_with(&[]).await
    }

    /// Starts the owned validator with exact pinned external program artifacts.
    ///
    /// Each artifact is hashed before spawn, deployed with `--bpf-program` at its
    /// canonical ID, then re-hashed from the node's ProgramData after readiness.
    pub async fn start_with(programs: &[fixtures::ExternalProgram]) -> Result<Self, Error> {
        if programs.is_empty() {
            return Self::start_loaded(Vec::new(), None, None).await;
        }

        let directory = FixtureDirectory::resolve()?;
        let loaded = directory.load(programs)?;

        Self::start_loaded(loaded, Some(directory.path().to_path_buf()), None).await
    }

    /// Starts with exact local genesis accounts built from the fresh payer.
    pub async fn start_with_accounts<F>(
        programs: &[fixtures::ExternalProgram],
        accounts: F,
    ) -> Result<Self, Error>
    where
        F: FnOnce(Address) -> Result<Vec<fixtures::LocalAccount>, Error> + Send + 'static,
    {
        let (loaded, directory_path) = if programs.is_empty() {
            (Vec::new(), None)
        } else {
            let directory = FixtureDirectory::resolve()?;
            let loaded = directory.load(programs)?;

            (loaded, Some(directory.path().to_path_buf()))
        };

        Self::start_loaded(
            loaded,
            directory_path,
            Some(Box::new(accounts) as AccountFactory),
        )
        .await
    }

    /// Starts with verified artifacts deployed at caller-selected addresses.
    ///
    /// Hashes and canonical IDs still come from the pinned fixture; only the
    /// genesis deployment address is overridden, which is used to prove
    /// program-ID guards without rebuilding or editing any artifact.
    pub async fn start_with_overrides(
        overrides: &[(fixtures::ExternalProgram, Address)],
    ) -> Result<Self, Error> {
        if overrides.is_empty() {
            return Self::start().await;
        }

        let directory = FixtureDirectory::resolve()?;
        let programs: Vec<_> = overrides.iter().map(|(program, _)| *program).collect();
        let mut loaded = directory.load(&programs)?;

        for (loaded, (_, address)) in loaded.iter_mut().zip(overrides) {
            loaded.deployed_at = *address;
        }

        Self::start_loaded(loaded, Some(directory.path().to_path_buf()), None).await
    }

    async fn start_loaded(
        loaded: Vec<LoadedProgram>,
        fixture_directory: Option<PathBuf>,
        account_factory: Option<AccountFactory>,
    ) -> Result<Self, Error> {
        let bpf_programs: Vec<(Address, PathBuf)> = loaded
            .iter()
            .map(|program| (program.deployed_at, program.path.clone()))
            .collect();
        let mut entropy = NativeEntropy::default();
        let payer = new_signer(&mut entropy).await?;

        let accounts = match account_factory {
            Some(factory) => factory(payer.address())?,
            None => Vec::new(),
        };

        let mint = new_signer(&mut entropy).await?;
        let (directory_nonce, port_nonce) = entropy_nonces(&mut entropy).await?;
        let node = Node::spawn(
            mint.address(),
            &directory_nonce,
            &port_nonce,
            &bpf_programs,
            &accounts,
        )?;
        let client = connect(&node)?;
        let payer_address = payer.address();
        let mut env = Self {
            node,
            client,
            payer,
            payer_address,
            entropy,
            loaded,
            fixture_directory,
            observed_slot: Cell::new(0),
            validator_version: String::new(),
            validator_identity: Address::from_bytes([0; 32]),
            genesis: Hash::from_bytes([0; 32]),
        };

        if let Err(error) = env.bootstrap().await {
            let cleanup = env.node.shutdown().await.err();

            return Err(Error::Startup {
                error: Box::new(error),
                cleanup,
            });
        }

        Ok(env)
    }

    /// Disposable payer that signs every submitted transaction.
    pub fn payer(&self) -> &Keypair {
        &self.payer
    }

    /// Payer address funded by this node's faucet.
    pub const fn payer_address(&self) -> Address {
        self.payer_address
    }

    /// Numeric loopback endpoint owned by this environment.
    pub fn rpc_endpoint(&self) -> String {
        format!("http://127.0.0.1:{}", self.node.rpc_port())
    }

    /// Exact fixture directory used by this environment, when it loaded any.
    pub fn fixture_dir(&self) -> Option<&Path> {
        self.fixture_directory.as_deref()
    }

    /// Verified external artifacts loaded into this node.
    pub fn loaded_programs(&self) -> &[LoadedProgram] {
        &self.loaded
    }

    pub(crate) const fn client(&self) -> &Client<NativeHttp> {
        &self.client
    }

    pub(crate) fn read_config(&self) -> solana_kit_client::methods::ReadConfig {
        let mut config = rpc::confirmed();
        config.min_context_slot = Some(self.observed_slot.get()).filter(|slot| *slot > 0);
        config
    }

    pub(crate) const fn observed_slot(&self) -> &Cell<u64> {
        &self.observed_slot
    }

    /// Creates one fresh disposable signer from caller-owned OS entropy.
    pub async fn new_key(&mut self) -> Result<Keypair, Error> {
        new_signer(&mut self.entropy).await
    }

    /// Awaits termination, final identity/genesis challenge and owned-data removal.
    pub async fn close(mut self) -> Result<(), Error> {
        let mut failures = Vec::new();

        match self.node.alive() {
            Ok(()) => self.challenge_endpoint(&mut failures).await,
            Err(error) => failures.push(format!("validator exited before cleanup: {error}")),
        }

        if let Err(error) = self.node.shutdown().await {
            failures.push(error.to_string());
        }

        if failures.is_empty() {
            return Ok(());
        }

        Err(Error::Cleanup(failures.join("; ")))
    }

    // Two independent challenges are retained: the first failure must not hide
    // a subsequent identity/genesis change, and both are reported with cleanup.
    async fn challenge_endpoint(&self, failures: &mut Vec<String>) {
        match self.client.call(Identity).await {
            Ok(identity) if identity.identity != self.validator_identity => failures.push(format!(
                "identity changed from {} to {}",
                self.validator_identity, identity.identity
            )),
            Ok(_) => {}
            Err(error) => failures.push(format!("identity challenge: {error}")),
        }

        match self.client.call(GenesisHash).await {
            Ok(genesis) if genesis != self.genesis => failures.push(format!(
                "genesis changed from {} to {genesis}",
                self.genesis
            )),
            Ok(_) => {}
            Err(error) => failures.push(format!("genesis challenge: {error}")),
        }
    }

    async fn bootstrap(&mut self) -> Result<(), Error> {
        self.await_ready().await?;
        let identity = self.client.call(Identity).await?.identity;
        let ledger_identity = self.node.ledger_identity()?;

        if identity != ledger_identity {
            return Err(NodeError::Identity {
                expected: ledger_identity,
                actual: identity,
            }
            .into());
        }

        let genesis = self.client.call(GenesisHash).await?;
        self.validator_identity = identity;
        self.genesis = genesis;
        self.airdrop().await?;
        reads::verify_loaded_programs(self).await
    }

    async fn await_ready(&mut self) -> Result<(), Error> {
        self.await_health().await?;
        let version = self.client.call(NodeVersionRequest).await?.solana_core;

        if !version.starts_with(EXPECTED_NODE_VERSION) {
            return Err(NodeError::WrongBinary(version).into());
        }

        self.validator_version = version;

        Ok(())
    }

    async fn await_health(&mut self) -> Result<(), Error> {
        let deadline = Instant::now() + STARTUP_DEADLINE;
        let mut last = "no RPC exchange completed".to_owned();

        loop {
            self.node.alive()?;

            if Instant::now() >= deadline {
                return Err(NodeError::Readiness(format!(
                    "deadline elapsed before health; last: {last}"
                ))
                .into());
            }

            match self.client.call(GetHealth).await {
                Ok(_) => return Ok(()),
                Err(error) => last = rpc::rpc_category(&error),
            }

            tokio::time::sleep(RPC_POLL).await;
        }
    }

    async fn airdrop(&self) -> Result<(), Error> {
        self.client
            .call(solana_kit_client::methods::submission::RequestAirdrop(
                self.payer_address,
                AIRDROP_LAMPORTS,
                None,
            ))
            .await?;

        let deadline = Instant::now() + AIRDROP_DEADLINE;

        loop {
            let balance = self.balance(self.payer_address).await?;

            if balance >= AIRDROP_LAMPORTS {
                return Ok(());
            }

            if Instant::now() >= deadline {
                return Err(Error::Airdrop(format!(
                    "payer balance remained {balance} below {AIRDROP_LAMPORTS} lamports"
                )));
            }

            tokio::time::sleep(RPC_POLL).await;
        }
    }
}

async fn new_signer(entropy: &mut NativeEntropy) -> Result<Keypair, Error> {
    let key = ExportableKeypair::generate(entropy)
        .await
        .map_err(Error::Entropy)?;

    Ok(key.into_signer())
}

async fn entropy_nonces(entropy: &mut NativeEntropy) -> Result<([u8; 8], [u8; 2]), Error> {
    let mut directory = [0_u8; 8];
    let mut port = [0_u8; 2];
    entropy.fill(&mut directory).await.map_err(Error::Entropy)?;
    entropy.fill(&mut port).await.map_err(Error::Entropy)?;

    Ok((directory, port))
}

fn connect(node: &Node) -> Result<Client<NativeHttp>, Error> {
    let config = HttpConfig {
        exchange_timeout: Duration::from_secs(60),
        ..HttpConfig::default()
    };
    let transport = NativeHttp::new(&node.endpoint(), config)?;

    Ok(Client::new(
        transport,
        Limits {
            bytes: RPC_BYTES,
            depth: RPC_DEPTH,
        },
    ))
}

/// Asserts the exact custom program error and failing instruction position.
pub fn expect_custom_error(error: &SubmitError, instruction: u8, code: u32) {
    match error.instruction_failure() {
        Some((
            index,
            solana_kit_client::methods::submission::InstructionFailure::Custom(actual),
        )) if index == instruction && *actual == code => {}
        other => panic!(
            "expected custom program error {code} at instruction {instruction}, got {other:?} ({error:?})"
        ),
    }
}

/// Asserts a known top-level transaction failure.
pub fn expect_transaction_failure(
    error: &SubmitError,
    expected: solana_kit_client::methods::submission::KnownTransactionFailure,
) {
    match error.known_failure() {
        Some(actual) if *actual == expected => {}
        other => {
            panic!("expected top-level transaction failure {expected:?}, got {other:?} ({error:?})")
        }
    }
}
