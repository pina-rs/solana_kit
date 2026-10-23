//! Generated from the pinned Dart error oracle.
use super::{CodeDefinition, ErrorCode};
impl ErrorCode {
    /// Pinned `mwaEncryptionFailed` code.
    pub const MWA_ENCRYPTION_FAILED: Self = Self(8400011);
    /// Pinned `mwaHandshakeFailed` code.
    pub const MWA_HANDSHAKE_FAILED: Self = Self(8400012);
    /// Pinned `mwaInvalidHelloResponse` code.
    pub const MWA_INVALID_HELLO_RESPONSE: Self = Self(8400013);
    /// Pinned `mwaProtocolAuthorizationFailed` code.
    pub const MWA_PROTOCOL_AUTHORIZATION_FAILED: Self = Self(8400100);
    /// Pinned `mwaProtocolInvalidPayloads` code.
    pub const MWA_PROTOCOL_INVALID_PAYLOADS: Self = Self(8400101);
    /// Pinned `mwaProtocolNotSigned` code.
    pub const MWA_PROTOCOL_NOT_SIGNED: Self = Self(8400102);
    /// Pinned `mwaProtocolNotSubmitted` code.
    pub const MWA_PROTOCOL_NOT_SUBMITTED: Self = Self(8400103);
    /// Pinned `mwaProtocolTooManyPayloads` code.
    pub const MWA_PROTOCOL_TOO_MANY_PAYLOADS: Self = Self(8400104);
    /// Pinned `mwaProtocolAttestOriginAndroid` code.
    pub const MWA_PROTOCOL_ATTEST_ORIGIN_ANDROID: Self = Self(8400105);
    /// Pinned `heliusRpcError` code.
    pub const HELIUS_RPC_ERROR: Self = Self(8600000);
    /// Pinned `heliusRestError` code.
    pub const HELIUS_REST_ERROR: Self = Self(8600001);
    /// Pinned `heliusApiKeyRequired` code.
    pub const HELIUS_API_KEY_REQUIRED: Self = Self(8600002);
    /// Pinned `heliusWebSocketError` code.
    pub const HELIUS_WEB_SOCKET_ERROR: Self = Self(8600003);
    /// Pinned `heliusTransactionConfirmationTimeout` code.
    pub const HELIUS_TRANSACTION_CONFIRMATION_TIMEOUT: Self = Self(8600004);
    /// Pinned `heliusTransactionSimulationFailed` code.
    pub const HELIUS_TRANSACTION_SIMULATION_FAILED: Self = Self(8600005);
    /// Pinned `walletNotConnected` code.
    pub const WALLET_NOT_CONNECTED: Self = Self(8900000);
    /// Pinned `walletNoSignerConnected` code.
    pub const WALLET_NO_SIGNER_CONNECTED: Self = Self(8900001);
    /// Pinned `walletSignerNotAvailable` code.
    pub const WALLET_SIGNER_NOT_AVAILABLE: Self = Self(8900002);
    /// Pinned `walletAccountNotAvailable` code.
    pub const WALLET_ACCOUNT_NOT_AVAILABLE: Self = Self(8900003);
    /// Pinned `walletInvalidMessageSignature` code.
    pub const WALLET_INVALID_MESSAGE_SIGNATURE: Self = Self(8900004);
    /// Pinned `walletInvalidSignInOutput` code.
    pub const WALLET_INVALID_SIGN_IN_OUTPUT: Self = Self(8900005);
    /// Pinned `walletInvalidNumberOfOutputs` code.
    pub const WALLET_INVALID_NUMBER_OF_OUTPUTS: Self = Self(8900006);
    /// Pinned `invariantViolationSubscriptionIteratorStateMissing` code.
    pub const INVARIANT_VIOLATION_SUBSCRIPTION_ITERATOR_STATE_MISSING: Self = Self(9900000);
    /// Pinned `invariantViolationSubscriptionIteratorMustNotPollBeforeResolvingExistingMessagePromise` code.
    pub const INVARIANT_VIOLATION_SUBSCRIPTION_ITERATOR_MUST_NOT_POLL_BEFORE_RESOLVING_EXISTING_MESSAGE_PROMISE: Self = Self(9900001);
}
pub(super) const DEFINITIONS: &[CodeDefinition] = &[
    CodeDefinition {
        code: ErrorCode(8400011),
        name: "mwaEncryptionFailed",
        template: "Failed to encrypt mobile wallet adapter message.",
    },
    CodeDefinition {
        code: ErrorCode(8400012),
        name: "mwaHandshakeFailed",
        template: "Mobile wallet adapter handshake failed.",
    },
    CodeDefinition {
        code: ErrorCode(8400013),
        name: "mwaInvalidHelloResponse",
        template: "Invalid HELLO_RSP: expected at least 65 bytes, got $actualLength.",
    },
    CodeDefinition {
        code: ErrorCode(8400100),
        name: "mwaProtocolAuthorizationFailed",
        template: "Wallet authorization failed.",
    },
    CodeDefinition {
        code: ErrorCode(8400101),
        name: "mwaProtocolInvalidPayloads",
        template: "The payloads provided to the wallet were invalid.",
    },
    CodeDefinition {
        code: ErrorCode(8400102),
        name: "mwaProtocolNotSigned",
        template: "The wallet did not sign the payloads.",
    },
    CodeDefinition {
        code: ErrorCode(8400103),
        name: "mwaProtocolNotSubmitted",
        template: "The wallet did not submit the transactions.",
    },
    CodeDefinition {
        code: ErrorCode(8400104),
        name: "mwaProtocolTooManyPayloads",
        template: "Too many payloads were provided. Maximum: $maxPayloads.",
    },
    CodeDefinition {
        code: ErrorCode(8400105),
        name: "mwaProtocolAttestOriginAndroid",
        template: "Android origin attestation is required by the wallet.",
    },
    CodeDefinition {
        code: ErrorCode(8600000),
        name: "heliusRpcError",
        template: "Helius RPC error: $message",
    },
    CodeDefinition {
        code: ErrorCode(8600001),
        name: "heliusRestError",
        template: "Helius REST API error ($statusCode): $message",
    },
    CodeDefinition {
        code: ErrorCode(8600002),
        name: "heliusApiKeyRequired",
        template: "A Helius API key is required for this operation.",
    },
    CodeDefinition {
        code: ErrorCode(8600003),
        name: "heliusWebSocketError",
        template: "Helius WebSocket error: $message",
    },
    CodeDefinition {
        code: ErrorCode(8600004),
        name: "heliusTransactionConfirmationTimeout",
        template: "Transaction confirmation timed out after $timeoutMs ms for signature: $signature.",
    },
    CodeDefinition {
        code: ErrorCode(8600005),
        name: "heliusTransactionSimulationFailed",
        template: "Transaction simulation failed: $message",
    },
    CodeDefinition {
        code: ErrorCode(8900000),
        name: "walletNotConnected",
        template: "Wallet not connected",
    },
    CodeDefinition {
        code: ErrorCode(8900001),
        name: "walletNoSignerConnected",
        template: "No signing wallet connected (status: $status)",
    },
    CodeDefinition {
        code: ErrorCode(8900002),
        name: "walletSignerNotAvailable",
        template: "Connected wallet does not support signing",
    },
    CodeDefinition {
        code: ErrorCode(8900003),
        name: "walletAccountNotAvailable",
        template: "Wallet account $address is not available in wallet $walletName.",
    },
    CodeDefinition {
        code: ErrorCode(8900004),
        name: "walletInvalidMessageSignature",
        template: "The signature returned by wallet $walletName for the message failed verification against the account with address $address.",
    },
    CodeDefinition {
        code: ErrorCode(8900005),
        name: "walletInvalidSignInOutput",
        template: "The sign-in output returned by wallet $walletName failed verification against the account with address $address.",
    },
    CodeDefinition {
        code: ErrorCode(8900006),
        name: "walletInvalidNumberOfOutputs",
        template: "Wallet $walletName returned $actual outputs for the `$featureName` feature, but exactly $expected were expected.",
    },
    CodeDefinition {
        code: ErrorCode(9900000),
        name: "invariantViolationSubscriptionIteratorStateMissing",
        template: "Invariant violation: WebSocket message iterator is missing state storage. Please file an issue at https://sola.na/web3invariant",
    },
    CodeDefinition {
        code: ErrorCode(9900001),
        name: "invariantViolationSubscriptionIteratorMustNotPollBeforeResolvingExistingMessagePromise",
        template: "Invariant violation: WebSocket message iterator state is corrupt; iterated without first resolving existing message promise. Please file an issue at https://sola.na/web3invariant",
    },
];
