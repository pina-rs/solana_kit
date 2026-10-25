//! MIT Dart error names and messages retain exact integer codes and unknown values.
use serde_json::{Value, json};

const ERRORS: &[(&str, &str)] = &[
    (
        "DuplicateMember",
        "Found multiple members with the same pubkey",
    ),
    ("EmptyMembers", "Members array is empty"),
    ("TooManyMembers", "Too many members, can be up to 65535"),
    (
        "InvalidThreshold",
        "Invalid threshold, must be between 1 and number of members with Vote permission",
    ),
    (
        "Unauthorized",
        "Attempted to perform an unauthorized action",
    ),
    ("NotAMember", "Provided pubkey is not a member of multisig"),
    (
        "InvalidTransactionMessage",
        "TransactionMessage is malformed.",
    ),
    ("StaleProposal", "Proposal is stale"),
    ("InvalidProposalStatus", "Invalid proposal status"),
    ("InvalidTransactionIndex", "Invalid transaction index"),
    ("AlreadyApproved", "Member already approved the transaction"),
    ("AlreadyRejected", "Member already rejected the transaction"),
    (
        "AlreadyCancelled",
        "Member already cancelled the transaction",
    ),
    (
        "InvalidNumberOfAccounts",
        "Wrong number of accounts provided",
    ),
    ("InvalidAccount", "Invalid account provided"),
    ("RemoveLastMember", "Cannot remove last member"),
    ("NoVoters", "Members don't include any voters"),
    ("NoProposers", "Members don't include any proposers"),
    ("NoExecutors", "Members don't include any executors"),
    (
        "InvalidStaleTransactionIndex",
        "`stale_transaction_index` must be <= `transaction_index`",
    ),
    (
        "NotSupportedForControlled",
        "Instruction not supported for controlled multisig",
    ),
    (
        "TimeLockNotReleased",
        "Proposal time lock has not been released",
    ),
    (
        "NoActions",
        "Config transaction must have at least one action",
    ),
    ("MissingAccount", "Missing account"),
    ("InvalidMint", "Invalid mint"),
    ("InvalidDestination", "Invalid destination"),
    ("SpendingLimitExceeded", "Spending limit exceeded"),
    ("DecimalsMismatch", "Decimals don't match the mint"),
    ("UnknownPermission", "Member has unknown permission"),
    (
        "ProtectedAccount",
        "Account is protected, it cannot be passed into a CPI as writable",
    ),
    (
        "TimeLockExceedsMaxAllowed",
        "Time lock exceeds the maximum allowed (90 days)",
    ),
    (
        "IllegalAccountOwner",
        "Account is not owned by Multisig program",
    ),
    (
        "RentReclamationDisabled",
        "Rent reclamation is disabled for this multisig",
    ),
    ("InvalidRentCollector", "Invalid rent collector address"),
    (
        "ProposalForAnotherMultisig",
        "Proposal is for another multisig",
    ),
    (
        "TransactionForAnotherMultisig",
        "Transaction is for another multisig",
    ),
    (
        "TransactionNotMatchingProposal",
        "Transaction doesn't match proposal",
    ),
    (
        "TransactionNotLastInBatch",
        "Transaction is not last in batch",
    ),
    ("BatchNotEmpty", "Batch is not empty"),
    ("SpendingLimitInvalidAmount", "Invalid SpendingLimit amount"),
    ("InvalidInstructionArgs", "Invalid Instruction Arguments"),
    (
        "FinalBufferHashMismatch",
        "Final message buffer hash doesnt match the expected hash",
    ),
    (
        "FinalBufferSizeExceeded",
        "Final buffer size cannot exceed 4000 bytes",
    ),
    ("FinalBufferSizeMismatch", "Final buffer size mismatch"),
    (
        "MultisigCreateDeprecated",
        "multisig_create has been deprecated. Use multisig_create_v2 instead.",
    ),
];

pub(super) fn errors() -> Vec<Value> {
    (6000u32..).zip(ERRORS).map(|(code, (name, message))| {
        json!({"kind":"errorNode","code":code,"name":name,"message":message})
    }).collect()
}
