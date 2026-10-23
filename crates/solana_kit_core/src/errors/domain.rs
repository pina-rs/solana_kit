//! Source-compatible classification, including reserved unknown numbers.
/// High-level category of a stable Kit error code.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ErrorDomain {
    /// General core failures.
    General,
    /// JSON-RPC protocol and server failures.
    JsonRpc,
    /// Address validation and derivation.
    Addresses,
    /// Account parsing and decoding.
    Accounts,
    /// SubtleCrypto adapters.
    SubtleCrypto,
    /// Randomness capabilities.
    Crypto,
    /// Keys and signatures.
    Keys,
    /// Instruction shapes.
    Instruction,
    /// Instruction execution.
    InstructionError,
    /// Signer capabilities.
    Signer,
    /// Off-chain messages.
    OffchainMessage,
    /// Transaction construction.
    Transaction,
    /// Transaction execution.
    TransactionError,
    /// Transaction introspection.
    TransactionIntrospection,
    /// Instruction planning.
    InstructionPlans,
    /// Binary codecs.
    Codecs,
    /// RPC and HTTP transports.
    Rpc,
    /// RPC subscriptions.
    RpcSubscriptions,
    /// Reactive stores.
    Subscribable,
    /// Program client resolution.
    ProgramClients,
    /// Mobile wallet sessions and protocols.
    MobileWalletAdapter,
    /// Helius integration.
    Helius,
    /// Wallet availability.
    Wallet,
    /// Internal invariants.
    InvariantViolation,
    /// Unclassified by the pinned source, even if a named code exists.
    Unknown,
}

struct DomainRange {
    minimum: i64,
    maximum: i64,
    domain: ErrorDomain,
}
const RANGES: &[DomainRange] = &[
    DomainRange {
        minimum: -32768,
        maximum: -32000,
        domain: ErrorDomain::JsonRpc,
    },
    DomainRange {
        minimum: 1,
        maximum: 10,
        domain: ErrorDomain::General,
    },
    DomainRange {
        minimum: 2800000,
        maximum: 2800999,
        domain: ErrorDomain::Addresses,
    },
    DomainRange {
        minimum: 3230000,
        maximum: 3230999,
        domain: ErrorDomain::Accounts,
    },
    DomainRange {
        minimum: 32300001,
        maximum: 32300001,
        domain: ErrorDomain::Accounts,
    },
    DomainRange {
        minimum: 3610000,
        maximum: 3610999,
        domain: ErrorDomain::SubtleCrypto,
    },
    DomainRange {
        minimum: 3611000,
        maximum: 3611050,
        domain: ErrorDomain::Crypto,
    },
    DomainRange {
        minimum: 3704000,
        maximum: 3704999,
        domain: ErrorDomain::Keys,
    },
    DomainRange {
        minimum: 4128000,
        maximum: 4128999,
        domain: ErrorDomain::Instruction,
    },
    DomainRange {
        minimum: 4615000,
        maximum: 4615999,
        domain: ErrorDomain::InstructionError,
    },
    DomainRange {
        minimum: 5508000,
        maximum: 5508999,
        domain: ErrorDomain::Signer,
    },
    DomainRange {
        minimum: 5607000,
        maximum: 5607999,
        domain: ErrorDomain::OffchainMessage,
    },
    DomainRange {
        minimum: 5663000,
        maximum: 5663999,
        domain: ErrorDomain::Transaction,
    },
    DomainRange {
        minimum: 5664000,
        maximum: 5664999,
        domain: ErrorDomain::TransactionIntrospection,
    },
    DomainRange {
        minimum: 7050000,
        maximum: 7050999,
        domain: ErrorDomain::TransactionError,
    },
    DomainRange {
        minimum: 7618000,
        maximum: 7618999,
        domain: ErrorDomain::InstructionPlans,
    },
    DomainRange {
        minimum: 8078000,
        maximum: 8078999,
        domain: ErrorDomain::Codecs,
    },
    DomainRange {
        minimum: 8100000,
        maximum: 8100999,
        domain: ErrorDomain::Rpc,
    },
    DomainRange {
        minimum: 8190000,
        maximum: 8190999,
        domain: ErrorDomain::RpcSubscriptions,
    },
    DomainRange {
        minimum: 8195000,
        maximum: 8195999,
        domain: ErrorDomain::Subscribable,
    },
    DomainRange {
        minimum: 8400000,
        maximum: 8400199,
        domain: ErrorDomain::MobileWalletAdapter,
    },
    DomainRange {
        minimum: 8500000,
        maximum: 8500999,
        domain: ErrorDomain::ProgramClients,
    },
    DomainRange {
        minimum: 8600000,
        maximum: 8600099,
        domain: ErrorDomain::Helius,
    },
    DomainRange {
        minimum: 8900000,
        maximum: 8900999,
        domain: ErrorDomain::Wallet,
    },
    DomainRange {
        minimum: 9900000,
        maximum: 9900999,
        domain: ErrorDomain::InvariantViolation,
    },
];
pub(super) fn classify(value: i64) -> ErrorDomain {
    RANGES
        .iter()
        .find(|range| (range.minimum..=range.maximum).contains(&value))
        .map_or(ErrorDomain::Unknown, |range| range.domain)
}
