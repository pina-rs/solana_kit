use crate::{WalletAccount, WalletFeature};
use alloc::{string::String, vec::Vec};
use core::future::Future;

/// Signs serialized transactions.
pub const SIGN_TRANSACTION: &str = "solana:signTransaction";
/// Signs and submits serialized transactions.
pub const SIGN_AND_SEND_TRANSACTION: &str = "solana:signAndSendTransaction";
/// Submits a batch with one settled result per input.
pub const SIGN_AND_SEND_ALL_TRANSACTIONS: &str = "solana:signAndSendAllTransactions";
/// Signs arbitrary message bytes.
pub const SIGN_MESSAGE: &str = "solana:signMessage";
/// Domain-bound authentication.
pub const SIGN_IN: &str = "solana:signIn";
/// Signs canonical offchain messages.
pub const SIGN_OFFCHAIN_MESSAGE: &str = "solana:signOffchainMessage";

/// Known advertised transaction versions. Unknown values are never downgraded.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TransactionVersion {
    /// Legacy envelope.
    Legacy,
    /// Version-zero envelope.
    V0,
    /// Version-one envelope.
    V1,
}

impl TransactionVersion {
    /// Exact advertised string.
    pub fn wire_value(self) -> &'static str {
        match self {
            Self::Legacy => "legacy",
            Self::V0 => "0",
            Self::V1 => "1",
        }
    }
    /// Unknown advertised versions remain unsupported.
    pub fn from_wire_value(value: &str) -> Option<Self> {
        match value {
            "legacy" => Some(Self::Legacy),
            "0" => Some(Self::V0),
            "1" => Some(Self::V1),
            _ => None,
        }
    }
}

/// Commitment accepted by signing capabilities.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TransactionCommitment {
    /// Processed by a node.
    Processed,
    /// Confirmed by a supermajority.
    Confirmed,
    /// Finalized by the cluster.
    Finalized,
}

/// Simulation options. Signed integers preserve the source's lack of range checks.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct SignTransactionOptions {
    /// Preflight commitment.
    pub preflight_commitment: Option<TransactionCommitment>,
    /// Minimum evaluation slot. No new validation is imposed here.
    pub min_context_slot: Option<i64>,
}

/// A transaction signing request.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SignTransactionInput {
    /// Requested account.
    pub account: WalletAccount,
    /// Exact transaction bytes. The capability must advertise its version.
    pub transaction: Vec<u8>,
    /// Optional chain. The source does not validate this field as an identifier.
    pub chain: Option<String>,
    /// Optional simulation settings.
    pub options: Option<SignTransactionOptions>,
}

/// Raw signed transaction bytes. No signature verification is implied.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SignTransactionOutput {
    /// Serialized signed transaction.
    pub signed_transaction: Vec<u8>,
}

/// Submission options, composed with the common simulation options.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct SignAndSendTransactionOptions {
    /// Common simulation settings.
    pub simulation: SignTransactionOptions,
    /// Desired confirmation commitment.
    pub commitment: Option<TransactionCommitment>,
    /// Whether to skip preflight.
    pub skip_preflight: Option<bool>,
    /// Maximum retries. No new range restriction is imposed.
    pub max_retries: Option<i64>,
}

/// A signing and submission request with a required chain.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SignAndSendTransactionInput {
    /// Requested account.
    pub account: WalletAccount,
    /// Exact serialized transaction bytes.
    pub transaction: Vec<u8>,
    /// Required target chain. The source accepts an arbitrary string.
    pub chain: String,
    /// Optional submission settings.
    pub options: Option<SignAndSendTransactionOptions>,
}

/// Raw signature returned by a submitting backend. Any byte length is preserved.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SignAndSendTransactionOutput {
    /// Unverified transaction signature bytes.
    pub signature: Vec<u8>,
}

/// Batch prompting and submission order.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum SignAndSendAllMode {
    /// Prompts and submissions may proceed concurrently.
    #[default]
    Parallel,
    /// Prompts and submissions proceed in input order.
    Serial,
}

/// One settled result. Failures retain the backend's concrete error type.
pub type SignAndSendAllResult<E> = Result<SignAndSendTransactionOutput, E>;

/// Arbitrary message signing request.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SignMessageInput {
    /// Requested signer.
    pub account: WalletAccount,
    /// Exact arbitrary bytes.
    pub message: Vec<u8>,
}

/// Raw message signing response. The bytes may differ from the requested message.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SignMessageOutput {
    /// Exact bytes signed by the backend.
    pub signed_message: Vec<u8>,
    /// Raw unverified signature, without an imposed length restriction.
    pub signature: Vec<u8>,
    /// Optional algorithm, normally ed25519.
    pub signature_type: Option<String>,
}

/// Sign In With Solana fields. Formatting and authorization belong to the backend.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct SignInInput {
    /// Requesting domain.
    pub domain: Option<String>,
    /// Requested address.
    pub address: Option<String>,
    /// Human-readable purpose.
    pub statement: Option<String>,
    /// Requesting URI.
    pub uri: Option<String>,
    /// Message version.
    pub version: Option<String>,
    /// Chain identifier.
    pub chain_id: Option<String>,
    /// Replay nonce.
    pub nonce: Option<String>,
    /// ISO-8601 issue time.
    pub issued_at: Option<String>,
    /// ISO-8601 expiry time.
    pub expiration_time: Option<String>,
    /// ISO-8601 earliest validity time.
    pub not_before: Option<String>,
    /// Application request identifier.
    pub request_id: Option<String>,
    /// Authorized resources. None differs from an empty list.
    pub resources: Option<Vec<String>>,
}

/// Backend-selected account and raw signed message.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SignInOutput {
    /// Account selected by the wallet.
    pub account: WalletAccount,
    /// Raw signing response, without a verification claim.
    pub message: SignMessageOutput,
}

/// Version-one offchain signing input.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SignOffchainMessageInput {
    /// Requested signer.
    pub account: WalletAccount,
    /// UTF-8 body.
    pub message: String,
    /// Claimed required keys. The pinned source copies but does not validate lengths.
    pub required_signers: Vec<Vec<u8>>,
}

/// Canonical bytes returned by the backend, without a verification claim.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SignOffchainMessageOutput {
    /// Exact preamble and message bytes signed.
    pub signed_offchain_message: Vec<u8>,
    /// Raw unverified signature.
    pub signature: Vec<u8>,
    /// Optional signature algorithm.
    pub signature_type: Option<String>,
}

/// Transaction signing capability.
pub trait SignTransaction: WalletFeature {
    /// Concrete backend failure.
    type Error;
    /// Concrete future with no universal Send requirement.
    type Future<'a>: Future<Output = Result<Vec<SignTransactionOutput>, Self::Error>> + 'a
    where
        Self: 'a;
    /// Versions the backend supports. No automatic downgrade exists.
    fn supported_transaction_versions(&self) -> &[TransactionVersion];
    /// Signs each request in input order.
    fn sign_transaction(&self, inputs: Vec<SignTransactionInput>) -> Self::Future<'_>;
}

/// Signing and submission capability.
pub trait SignAndSendTransaction: WalletFeature {
    /// Concrete backend failure.
    type Error;
    /// Concrete future.
    type Future<'a>: Future<Output = Result<Vec<SignAndSendTransactionOutput>, Self::Error>> + 'a
    where
        Self: 'a;
    /// Supported transaction versions.
    fn supported_transaction_versions(&self) -> &[TransactionVersion];
    /// Signs and submits each request in order.
    fn sign_and_send_transaction(
        &self,
        inputs: Vec<SignAndSendTransactionInput>,
    ) -> Self::Future<'_>;
}

/// Batch signing and submission with one settled result for every input.
pub trait SignAndSendAllTransactions: WalletFeature {
    /// Per-item or operation failure.
    type Error;
    /// Concrete future preserving partial failures.
    type Future<'a>: Future<Output = Result<Vec<SignAndSendAllResult<Self::Error>>, Self::Error>>
        + 'a
    where
        Self: 'a;
    /// Supported transaction versions.
    fn supported_transaction_versions(&self) -> &[TransactionVersion];
    /// Submits in the requested mode, preserving input result order.
    fn sign_and_send_all_transactions(
        &self,
        inputs: Vec<SignAndSendTransactionInput>,
        mode: SignAndSendAllMode,
    ) -> Self::Future<'_>;
}

/// Arbitrary message signing capability.
pub trait SignMessage: WalletFeature {
    /// Concrete backend failure.
    type Error;
    /// Concrete future.
    type Future<'a>: Future<Output = Result<Vec<SignMessageOutput>, Self::Error>> + 'a
    where
        Self: 'a;
    /// Signs in input order. Returned signed bytes remain authoritative.
    fn sign_message(&self, inputs: Vec<SignMessageInput>) -> Self::Future<'_>;
}

/// Domain-bound authentication capability.
pub trait SignIn: WalletFeature {
    /// Concrete backend failure.
    type Error;
    /// Concrete future.
    type Future<'a>: Future<Output = Result<Vec<SignInOutput>, Self::Error>> + 'a
    where
        Self: 'a;
    /// Requests user authorization and signs in input order.
    fn sign_in(&self, inputs: Vec<SignInInput>) -> Self::Future<'_>;
}

/// Canonical offchain message signing capability.
pub trait SignOffchainMessage: WalletFeature {
    /// Concrete backend failure.
    type Error;
    /// Concrete future.
    type Future<'a>: Future<Output = Result<Vec<SignOffchainMessageOutput>, Self::Error>> + 'a
    where
        Self: 'a;
    /// Exact message versions advertised by the backend, including future values.
    fn supported_message_versions(&self) -> &[i64];
    /// Signs canonical messages in input order.
    fn sign_offchain_message(&self, inputs: Vec<SignOffchainMessageInput>) -> Self::Future<'_>;
}
