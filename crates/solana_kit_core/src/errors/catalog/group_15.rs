//! Generated from the pinned Dart error oracle.
use super::{CodeDefinition, ErrorCode};
impl ErrorCode {
    /// Pinned `invariantViolationCachedAbortableIterableCacheEntryMissing` code.
    pub const INVARIANT_VIOLATION_CACHED_ABORTABLE_ITERABLE_CACHE_ENTRY_MISSING: Self =
        Self(9900002);
    /// Pinned `invariantViolationSwitchMustBeExhaustive` code.
    pub const INVARIANT_VIOLATION_SWITCH_MUST_BE_EXHAUSTIVE: Self = Self(9900003);
    /// Pinned `invariantViolationDataPublisherChannelUnimplemented` code.
    pub const INVARIANT_VIOLATION_DATA_PUBLISHER_CHANNEL_UNIMPLEMENTED: Self = Self(9900004);
    /// Pinned `invariantViolationInvalidInstructionPlanKind` code.
    pub const INVARIANT_VIOLATION_INVALID_INSTRUCTION_PLAN_KIND: Self = Self(9900005);
    /// Pinned `invariantViolationInvalidTransactionPlanKind` code.
    pub const INVARIANT_VIOLATION_INVALID_TRANSACTION_PLAN_KIND: Self = Self(9900006);
}
pub(super) const DEFINITIONS: &[CodeDefinition] = &[
    CodeDefinition {
        code: ErrorCode(9900002),
        name: "invariantViolationCachedAbortableIterableCacheEntryMissing",
        template: "Invariant violation: Found no abortable iterable cache entry for key $cacheKey. Please file an issue at https://sola.na/web3invariant",
    },
    CodeDefinition {
        code: ErrorCode(9900003),
        name: "invariantViolationSwitchMustBeExhaustive",
        template: "Invariant violation: Switch statement non-exhaustive. Received unexpected value $unexpectedValue. Please file an issue at https://sola.na/web3invariant",
    },
    CodeDefinition {
        code: ErrorCode(9900004),
        name: "invariantViolationDataPublisherChannelUnimplemented",
        template: "Invariant violation: This data publisher does not publish to the channel named $channelName. Supported channels include $supportedChannelNames.",
    },
    CodeDefinition {
        code: ErrorCode(9900005),
        name: "invariantViolationInvalidInstructionPlanKind",
        template: "Invalid instruction plan kind: $kind.",
    },
    CodeDefinition {
        code: ErrorCode(9900006),
        name: "invariantViolationInvalidTransactionPlanKind",
        template: "Invalid transaction plan kind: $kind.",
    },
];
