//! Inspect existing typed RPC results without reconstructing signed messages.
//!
//! Instruction traces describe attempted execution. Inspect
//! [`DecodedRpcTransaction::metadata`](crate::introspection::DecodedRpcTransaction::metadata)
//! before treating a trace as successful execution. Server-parsed payloads remain
//! borrowed opaque JSON and cannot become raw program bytes.

mod accounts;
mod resolve;
mod value;

use std::fmt;

use crate::methods::transaction_results::{self as rpc, TransactionResult, TransactionView};
use solana_kit_core::{AccountMeta, Instruction, Signature, transaction};

/// A structural failure at the RPC introspection boundary.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    /// Core wire validation failed.
    Wire(transaction::Error),
    /// Account-only results contain no instruction message.
    AccountsOnly,
    /// A wire-only operation requires a binary RPC representation.
    BinaryRequired,
    /// Reported version contradicts the message or is unsupported.
    Version,
    /// Signer/readonly counts or parsed account privileges are inconsistent.
    Header,
    /// Signature count does not equal the required signer count.
    Signatures,
    /// A required lookup resolution was absent or explicitly null.
    MissingLoadedAddresses,
    /// Lookup counts, version, or loaded writable/readonly partitions disagree.
    LoadedAddresses,
    /// Program or positional account index violates its message bounds.
    Index,
    /// An address-based instruction references an account absent from the message.
    Address,
    /// A server-parsed instruction did not carry an object payload.
    ParsedData,
    /// Inner instruction groups repeat an outer index or refer to no outer call.
    InnerGroup,
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "transaction introspection error: {self:?}")
    }
}

impl std::error::Error for Error {}

/// Exact instruction position; inner positions retain node-reported depth presence.
#[derive(Debug, PartialEq, Eq)]
pub enum InstructionTrace {
    /// Top-level message instruction position.
    Outer {
        /// Zero-based position in the message.
        index: usize,
        /// Missing, null and a reported top-level depth remain distinct.
        stack_height: crate::Presence<Option<u32>>,
    },
    /// Ordered cross-program invocation within one top-level instruction.
    Inner {
        /// Zero-based top-level instruction position.
        outer_index: usize,
        /// Zero-based position within the node's inner group.
        inner_index: usize,
        /// Missing, null and a reported depth remain distinct.
        stack_height: crate::Presence<Option<u32>>,
    },
}

/// Raw bytes or a node-owned parsed representation, without an invented conversion.
#[derive(Debug)]
pub enum ResolvedInstruction<'a> {
    /// Exact raw program bytes and ordered Kit account roles.
    Raw(Instruction),
    /// Program-defined parsed JSON borrowed from the original typed RPC result.
    Parsed(&'a rpc::ParsedInstruction),
}

/// One resolved invocation with its original execution position.
#[derive(Debug)]
pub struct TracedInstruction<'a> {
    /// Raw or parsed instruction content.
    pub instruction: ResolvedInstruction<'a>,
    /// Position in the attempted execution trace.
    pub trace: InstructionTrace,
}

/// Validated introspection over one existing Kit-owned RPC result.
///
/// Binary responses retain the exact core transaction and immutable signed message.
/// JSON responses retain the original typed DTO and never synthesize wire bytes.
#[derive(Debug)]
pub struct DecodedRpcTransaction<'a> {
    source: &'a TransactionResult,
    transaction: Option<transaction::Transaction>,
    accounts: Vec<AccountMeta>,
    version: transaction::Version,
    lifetime: transaction::Lifetime,
}

/// Validates a typed transaction result for account and instruction inspection.
///
/// # Errors
/// Rejects malformed wire, conflicting versions, signatures, header partitions,
/// missing or inconsistent lookup resolutions, unknown addresses and inner groups.
pub fn decode_rpc_transaction(
    source: &TransactionResult,
) -> Result<DecodedRpcTransaction<'_>, Error> {
    let transaction = match &source.transaction {
        TransactionView::Binary(_) => Some(decode_rpc_wire(source)?),
        TransactionView::Accounts(_) => return Err(Error::AccountsOnly),
        _ => None,
    };

    let version = accounts::version(source, transaction.as_ref())?;
    let accounts = accounts::resolve(source, transaction.as_ref(), version)?;

    let lifetime = match (&source.transaction, &transaction) {
        (TransactionView::Json(value), _) => {
            transaction::Lifetime::Unresolved(value.message.recent_blockhash)
        }
        (TransactionView::JsonParsed(value), _) => {
            transaction::Lifetime::Unresolved(value.message.recent_blockhash)
        }
        (_, Some(value)) => value.message().lifetime(),
        _ => return Err(Error::AccountsOnly),
    };

    let decoded = DecodedRpcTransaction {
        source,
        transaction,
        accounts,
        version,
        lifetime,
    };
    decoded.get_instructions()?;
    decoded.get_inner_instructions()?;

    Ok(decoded)
}

/// Decodes exact binary RPC wire independently of execution or lookup metadata.
///
/// This preserves signatures and immutable message bytes even when loaded account
/// addresses are unavailable. The lifetime remains unresolved. Use
/// [`decode_rpc_transaction`] when complete resolved instructions are required.
///
/// # Errors
/// Rejects non-binary representations, malformed wire and a contradictory version.
pub fn decode_rpc_wire(source: &TransactionResult) -> Result<transaction::Transaction, Error> {
    let TransactionView::Binary(bytes) = &source.transaction else {
        return Err(Error::BinaryRequired);
    };
    let transaction = transaction::Transaction::decode(bytes).map_err(Error::Wire)?;
    accounts::version(source, Some(&transaction))?;

    Ok(transaction)
}

/// Resolves the full account list of an already validated core message.
///
/// # Errors
/// Rejects missing or inconsistent loaded writable and readonly partitions.
pub fn get_account_metas(
    message: &transaction::Message,
    loaded: Option<&rpc::LoadedAddresses>,
) -> Result<Vec<AccountMeta>, Error> {
    accounts::from_message(message, loaded)
}

/// Resolves raw instructions of a core legacy, v0 or v1 message in message order.
///
/// # Errors
/// Rejects missing or inconsistent loaded address partitions or account indices.
pub fn get_instructions(
    message: &transaction::Message,
    loaded: Option<&rpc::LoadedAddresses>,
) -> Result<Vec<Instruction>, Error> {
    let accounts = get_account_metas(message, loaded)?;
    message
        .instructions()
        .iter()
        .map(|instruction| {
            let ResolvedInstruction::Raw(instruction) = resolve::compiled(
                instruction.program_index(),
                instruction.accounts(),
                instruction.data(),
                &accounts,
            )?
            else {
                unreachable!("compiled instructions always contain raw bytes");
            };

            Ok(instruction)
        })
        .collect()
}

/// Resolves recorded inner invocations against a caller-supplied account table.
///
/// This standalone helper cannot associate groups with a message. Use
/// [`DecodedRpcTransaction::walk_instructions`] to reject orphan outer indices.
/// Missing and null metadata remain available on the original typed input.
///
/// # Errors
/// Rejects duplicate groups and unresolved index or address references.
pub fn get_inner_instructions<'a>(
    meta: Option<&'a rpc::TransactionMeta>,
    accounts: &[AccountMeta],
) -> Result<Vec<TracedInstruction<'a>>, Error> {
    resolve::inner_groups(meta, accounts, None)
}

impl<'a> DecodedRpcTransaction<'a> {
    /// Original typed result, including all absent/null metadata distinctions.
    #[must_use]
    pub const fn source(&self) -> &'a TransactionResult {
        self.source
    }

    /// Original execution metadata; absent metadata does not establish success.
    #[must_use]
    pub fn metadata(&self) -> Option<&'a rpc::TransactionMeta> {
        self.source.meta.as_ref()
    }

    /// Exact validated binary transaction, absent for JSON responses.
    #[must_use]
    pub const fn transaction(&self) -> Option<&transaction::Transaction> {
        self.transaction.as_ref()
    }

    /// Validated legacy, version-zero or version-one message version.
    #[must_use]
    pub const fn version(&self) -> transaction::Version {
        self.version
    }

    /// Ordered static accounts followed by writable and readonly lookup accounts.
    #[must_use]
    pub fn account_metas(&self) -> &[AccountMeta] {
        &self.accounts
    }

    /// Required signer addresses in their exact signature-slot order.
    pub fn signers(&self) -> impl Iterator<Item = solana_kit_core::Address> + '_ {
        self.accounts
            .iter()
            .filter(|meta| meta.role.is_signer())
            .map(|meta| meta.address)
    }

    /// Original JSON signatures, or binary slots including explicit unsigned slots.
    #[must_use]
    pub fn signatures(&self) -> Vec<Option<Signature>> {
        if let Some(transaction) = &self.transaction {
            return transaction.signatures().to_vec();
        }

        match &self.source.transaction {
            TransactionView::Json(value) => value.signatures.iter().copied().map(Some).collect(),
            TransactionView::JsonParsed(value) => {
                value.signatures.iter().copied().map(Some).collect()
            }
            _ => unreachable!("binary and account-only views were handled at construction"),
        }
    }

    /// Received hash/nonce token with no inferred expiry or durable nonce status.
    #[must_use]
    pub const fn lifetime(&self) -> transaction::Lifetime {
        self.lifetime
    }

    /// Resolves all top-level invocations in message order.
    ///
    /// # Errors
    /// Rejects unresolved account references.
    pub fn get_instructions(&self) -> Result<Vec<TracedInstruction<'a>>, Error> {
        resolve::outer(self)
    }

    /// Resolves inner groups in node order, retaining each group's call positions.
    ///
    /// # Errors
    /// Rejects duplicate/orphan groups and unresolved account references.
    pub fn get_inner_instructions(&self) -> Result<Vec<TracedInstruction<'a>>, Error> {
        resolve::inner(self)
    }

    /// Interleaves each top-level invocation with its ordered inner invocations.
    ///
    /// # Errors
    /// Rejects malformed groups and unresolved account references.
    pub fn walk_instructions(&self) -> Result<Vec<TracedInstruction<'a>>, Error> {
        resolve::walk(self)
    }
}
