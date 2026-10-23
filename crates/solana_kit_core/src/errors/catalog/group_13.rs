//! Generated from the pinned Dart error oracle.
use super::{CodeDefinition, ErrorCode};
impl ErrorCode {
    /// Pinned `rpcSubscriptionsExpectedServerSubscriptionId` code.
    pub const RPC_SUBSCRIPTIONS_EXPECTED_SERVER_SUBSCRIPTION_ID: Self = Self(8190001);
    /// Pinned `rpcSubscriptionsChannelClosedBeforeMessageBuffered` code.
    pub const RPC_SUBSCRIPTIONS_CHANNEL_CLOSED_BEFORE_MESSAGE_BUFFERED: Self = Self(8190002);
    /// Pinned `rpcSubscriptionsChannelConnectionClosed` code.
    pub const RPC_SUBSCRIPTIONS_CHANNEL_CONNECTION_CLOSED: Self = Self(8190003);
    /// Pinned `rpcSubscriptionsChannelFailedToConnect` code.
    pub const RPC_SUBSCRIPTIONS_CHANNEL_FAILED_TO_CONNECT: Self = Self(8190004);
    /// Pinned `subscribableRetryNotSupported` code.
    pub const SUBSCRIBABLE_RETRY_NOT_SUPPORTED: Self = Self(8195000);
    /// Pinned `subscribableStreamClosedWithoutError` code.
    pub const SUBSCRIBABLE_STREAM_CLOSED_WITHOUT_ERROR: Self = Self(8195001);
    /// Pinned `programClientsInsufficientAccountMetas` code.
    pub const PROGRAM_CLIENTS_INSUFFICIENT_ACCOUNT_METAS: Self = Self(8500000);
    /// Pinned `programClientsUnrecognizedInstructionType` code.
    pub const PROGRAM_CLIENTS_UNRECOGNIZED_INSTRUCTION_TYPE: Self = Self(8500001);
    /// Pinned `programClientsFailedToIdentifyInstruction` code.
    pub const PROGRAM_CLIENTS_FAILED_TO_IDENTIFY_INSTRUCTION: Self = Self(8500002);
    /// Pinned `programClientsUnexpectedResolvedInstructionInputType` code.
    pub const PROGRAM_CLIENTS_UNEXPECTED_RESOLVED_INSTRUCTION_INPUT_TYPE: Self = Self(8500003);
    /// Pinned `programClientsResolvedInstructionInputMustBeNonNull` code.
    pub const PROGRAM_CLIENTS_RESOLVED_INSTRUCTION_INPUT_MUST_BE_NON_NULL: Self = Self(8500004);
    /// Pinned `programClientsUnrecognizedAccountType` code.
    pub const PROGRAM_CLIENTS_UNRECOGNIZED_ACCOUNT_TYPE: Self = Self(8500005);
    /// Pinned `programClientsFailedToIdentifyAccount` code.
    pub const PROGRAM_CLIENTS_FAILED_TO_IDENTIFY_ACCOUNT: Self = Self(8500006);
    /// Pinned `mwaAssociationPortOutOfRange` code.
    pub const MWA_ASSOCIATION_PORT_OUT_OF_RANGE: Self = Self(8400000);
    /// Pinned `mwaReflectorIdOutOfRange` code.
    pub const MWA_REFLECTOR_ID_OUT_OF_RANGE: Self = Self(8400001);
    /// Pinned `mwaForbiddenWalletBaseUrl` code.
    pub const MWA_FORBIDDEN_WALLET_BASE_URL: Self = Self(8400002);
    /// Pinned `mwaSessionClosed` code.
    pub const MWA_SESSION_CLOSED: Self = Self(8400003);
    /// Pinned `mwaSessionTimeout` code.
    pub const MWA_SESSION_TIMEOUT: Self = Self(8400004);
    /// Pinned `mwaWalletNotFound` code.
    pub const MWA_WALLET_NOT_FOUND: Self = Self(8400005);
    /// Pinned `mwaInvalidProtocolVersion` code.
    pub const MWA_INVALID_PROTOCOL_VERSION: Self = Self(8400006);
    /// Pinned `mwaPlatformNotSupported` code.
    pub const MWA_PLATFORM_NOT_SUPPORTED: Self = Self(8400007);
    /// Pinned `mwaSequenceNumberOverflow` code.
    pub const MWA_SEQUENCE_NUMBER_OVERFLOW: Self = Self(8400008);
    /// Pinned `mwaInvalidSequenceNumber` code.
    pub const MWA_INVALID_SEQUENCE_NUMBER: Self = Self(8400009);
    /// Pinned `mwaDecryptionFailed` code.
    pub const MWA_DECRYPTION_FAILED: Self = Self(8400010);
}
pub(super) const DEFINITIONS: &[CodeDefinition] = &[
    CodeDefinition {
        code: ErrorCode(8190001),
        name: "rpcSubscriptionsExpectedServerSubscriptionId",
        template: "Failed to obtain a subscription id from the server",
    },
    CodeDefinition {
        code: ErrorCode(8190002),
        name: "rpcSubscriptionsChannelClosedBeforeMessageBuffered",
        template: "WebSocket was closed before payload could be added to the send buffer",
    },
    CodeDefinition {
        code: ErrorCode(8190003),
        name: "rpcSubscriptionsChannelConnectionClosed",
        template: "WebSocket connection closed",
    },
    CodeDefinition {
        code: ErrorCode(8190004),
        name: "rpcSubscriptionsChannelFailedToConnect",
        template: "WebSocket failed to connect",
    },
    CodeDefinition {
        code: ErrorCode(8195000),
        name: "subscribableRetryNotSupported",
        template: "This data publisher does not support retrying.",
    },
    CodeDefinition {
        code: ErrorCode(8195001),
        name: "subscribableStreamClosedWithoutError",
        template: "The stream store closed in an error state but did not report an error.",
    },
    CodeDefinition {
        code: ErrorCode(8500000),
        name: "programClientsInsufficientAccountMetas",
        template: "The provided instruction is missing some accounts. Expected at least $expectedAccountMetas account(s), got $actualAccountMetas.",
    },
    CodeDefinition {
        code: ErrorCode(8500001),
        name: "programClientsUnrecognizedInstructionType",
        template: "Unrecognized instruction type '$instructionType' for the $programName program.",
    },
    CodeDefinition {
        code: ErrorCode(8500002),
        name: "programClientsFailedToIdentifyInstruction",
        template: "The provided instruction could not be identified as an instruction from the $programName program.",
    },
    CodeDefinition {
        code: ErrorCode(8500003),
        name: "programClientsUnexpectedResolvedInstructionInputType",
        template: "Expected resolved instruction input '$inputName' to be of type $expectedType.",
    },
    CodeDefinition {
        code: ErrorCode(8500004),
        name: "programClientsResolvedInstructionInputMustBeNonNull",
        template: "Expected resolved instruction input '$inputName' to be non-null.",
    },
    CodeDefinition {
        code: ErrorCode(8500005),
        name: "programClientsUnrecognizedAccountType",
        template: "Unrecognized account type '$accountType' for the $programName program.",
    },
    CodeDefinition {
        code: ErrorCode(8500006),
        name: "programClientsFailedToIdentifyAccount",
        template: "The provided account could not be identified as an account from the $programName program.",
    },
    CodeDefinition {
        code: ErrorCode(8400000),
        name: "mwaAssociationPortOutOfRange",
        template: "Association port number must be between 49152 and 65535. $port given.",
    },
    CodeDefinition {
        code: ErrorCode(8400001),
        name: "mwaReflectorIdOutOfRange",
        template: "Reflector ID must be between 0 and 2^53 - 1. $id given.",
    },
    CodeDefinition {
        code: ErrorCode(8400002),
        name: "mwaForbiddenWalletBaseUrl",
        template: "Wallet base URL must use HTTPS. $url given.",
    },
    CodeDefinition {
        code: ErrorCode(8400003),
        name: "mwaSessionClosed",
        template: "Mobile wallet adapter session was closed.",
    },
    CodeDefinition {
        code: ErrorCode(8400004),
        name: "mwaSessionTimeout",
        template: "Mobile wallet adapter session timed out.",
    },
    CodeDefinition {
        code: ErrorCode(8400005),
        name: "mwaWalletNotFound",
        template: "No compatible wallet app was found on this device.",
    },
    CodeDefinition {
        code: ErrorCode(8400006),
        name: "mwaInvalidProtocolVersion",
        template: "Invalid MWA protocol version: $version.",
    },
    CodeDefinition {
        code: ErrorCode(8400007),
        name: "mwaPlatformNotSupported",
        template: "Mobile Wallet Adapter is only supported on Android.",
    },
    CodeDefinition {
        code: ErrorCode(8400008),
        name: "mwaSequenceNumberOverflow",
        template: "Sequence number $sequenceNumber exceeds maximum value of 2^32 - 1.",
    },
    CodeDefinition {
        code: ErrorCode(8400009),
        name: "mwaInvalidSequenceNumber",
        template: "Expected sequence number $expected, got $actual.",
    },
    CodeDefinition {
        code: ErrorCode(8400010),
        name: "mwaDecryptionFailed",
        template: "Failed to decrypt mobile wallet adapter message.",
    },
];
