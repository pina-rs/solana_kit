//! Generated from the pinned Dart error oracle.
use super::{CodeDefinition, ErrorCode};
impl ErrorCode {
    /// Pinned `addressesInvalidSeedsPointOnCurve` code.
    pub const ADDRESSES_INVALID_SEEDS_POINT_ON_CURVE: Self = Self(2800008);
    /// Pinned `addressesFailedToFindViablePdaBumpSeed` code.
    pub const ADDRESSES_FAILED_TO_FIND_VIABLE_PDA_BUMP_SEED: Self = Self(2800009);
    /// Pinned `addressesPdaEndsWithPdaMarker` code.
    pub const ADDRESSES_PDA_ENDS_WITH_PDA_MARKER: Self = Self(2800010);
    /// Pinned `addressesInvalidOffCurveAddress` code.
    pub const ADDRESSES_INVALID_OFF_CURVE_ADDRESS: Self = Self(2800011);
    /// Pinned `accountsAccountNotFound` code.
    pub const ACCOUNTS_ACCOUNT_NOT_FOUND: Self = Self(3230000);
    /// Pinned `accountsOneOrMoreAccountsNotFound` code.
    pub const ACCOUNTS_ONE_OR_MORE_ACCOUNTS_NOT_FOUND: Self = Self(32300001);
    /// Pinned `accountsFailedToDecodeAccount` code.
    pub const ACCOUNTS_FAILED_TO_DECODE_ACCOUNT: Self = Self(3230002);
    /// Pinned `accountsExpectedDecodedAccount` code.
    pub const ACCOUNTS_EXPECTED_DECODED_ACCOUNT: Self = Self(3230003);
    /// Pinned `accountsExpectedAllAccountsToBeDecoded` code.
    pub const ACCOUNTS_EXPECTED_ALL_ACCOUNTS_TO_BE_DECODED: Self = Self(3230004);
    /// Pinned `subtleCryptoDisallowedInInsecureContext` code.
    pub const SUBTLE_CRYPTO_DISALLOWED_IN_INSECURE_CONTEXT: Self = Self(3610000);
    /// Pinned `subtleCryptoDigestUnimplemented` code.
    pub const SUBTLE_CRYPTO_DIGEST_UNIMPLEMENTED: Self = Self(3610001);
    /// Pinned `subtleCryptoEd25519AlgorithmUnimplemented` code.
    pub const SUBTLE_CRYPTO_ED25519_ALGORITHM_UNIMPLEMENTED: Self = Self(3610002);
    /// Pinned `subtleCryptoExportFunctionUnimplemented` code.
    pub const SUBTLE_CRYPTO_EXPORT_FUNCTION_UNIMPLEMENTED: Self = Self(3610003);
    /// Pinned `subtleCryptoGenerateFunctionUnimplemented` code.
    pub const SUBTLE_CRYPTO_GENERATE_FUNCTION_UNIMPLEMENTED: Self = Self(3610004);
    /// Pinned `subtleCryptoSignFunctionUnimplemented` code.
    pub const SUBTLE_CRYPTO_SIGN_FUNCTION_UNIMPLEMENTED: Self = Self(3610005);
    /// Pinned `subtleCryptoVerifyFunctionUnimplemented` code.
    pub const SUBTLE_CRYPTO_VERIFY_FUNCTION_UNIMPLEMENTED: Self = Self(3610006);
    /// Pinned `subtleCryptoCannotExportNonExtractableKey` code.
    pub const SUBTLE_CRYPTO_CANNOT_EXPORT_NON_EXTRACTABLE_KEY: Self = Self(3610007);
    /// Pinned `cryptoRandomValuesFunctionUnimplemented` code.
    pub const CRYPTO_RANDOM_VALUES_FUNCTION_UNIMPLEMENTED: Self = Self(3611000);
    /// Pinned `keysInvalidKeyPairByteLength` code.
    pub const KEYS_INVALID_KEY_PAIR_BYTE_LENGTH: Self = Self(3704000);
    /// Pinned `keysInvalidPrivateKeyByteLength` code.
    pub const KEYS_INVALID_PRIVATE_KEY_BYTE_LENGTH: Self = Self(3704001);
    /// Pinned `keysInvalidSignatureByteLength` code.
    pub const KEYS_INVALID_SIGNATURE_BYTE_LENGTH: Self = Self(3704002);
    /// Pinned `keysSignatureStringLengthOutOfRange` code.
    pub const KEYS_SIGNATURE_STRING_LENGTH_OUT_OF_RANGE: Self = Self(3704003);
    /// Pinned `keysPublicKeyMustMatchPrivateKey` code.
    pub const KEYS_PUBLIC_KEY_MUST_MATCH_PRIVATE_KEY: Self = Self(3704004);
    /// Pinned `keysInvalidBase58InGrindRegex` code.
    pub const KEYS_INVALID_BASE58_IN_GRIND_REGEX: Self = Self(3704005);
}
pub(super) const DEFINITIONS: &[CodeDefinition] = &[
    CodeDefinition {
        code: ErrorCode(2800008),
        name: "addressesInvalidSeedsPointOnCurve",
        template: "Invalid seeds; point must fall off the Ed25519 curve.",
    },
    CodeDefinition {
        code: ErrorCode(2800009),
        name: "addressesFailedToFindViablePdaBumpSeed",
        template: "Unable to find a viable program address bump seed.",
    },
    CodeDefinition {
        code: ErrorCode(2800010),
        name: "addressesPdaEndsWithPdaMarker",
        template: "Program address cannot end with PDA marker.",
    },
    CodeDefinition {
        code: ErrorCode(2800011),
        name: "addressesInvalidOffCurveAddress",
        template: "$putativeOffCurveAddress is not a base58-encoded off-curve address.",
    },
    CodeDefinition {
        code: ErrorCode(3230000),
        name: "accountsAccountNotFound",
        template: "Account not found at address: $address",
    },
    CodeDefinition {
        code: ErrorCode(32300001),
        name: "accountsOneOrMoreAccountsNotFound",
        template: "Accounts not found at addresses: $addresses",
    },
    CodeDefinition {
        code: ErrorCode(3230002),
        name: "accountsFailedToDecodeAccount",
        template: "Failed to decode account data at address: $address",
    },
    CodeDefinition {
        code: ErrorCode(3230003),
        name: "accountsExpectedDecodedAccount",
        template: "Expected decoded account at address: $address",
    },
    CodeDefinition {
        code: ErrorCode(3230004),
        name: "accountsExpectedAllAccountsToBeDecoded",
        template: "Not all accounts were decoded. Encoded accounts found at addresses: $addresses.",
    },
    CodeDefinition {
        code: ErrorCode(3610000),
        name: "subtleCryptoDisallowedInInsecureContext",
        template: "Cryptographic operations are only allowed in secure contexts.",
    },
    CodeDefinition {
        code: ErrorCode(3610001),
        name: "subtleCryptoDigestUnimplemented",
        template: "No digest implementation could be found.",
    },
    CodeDefinition {
        code: ErrorCode(3610002),
        name: "subtleCryptoEd25519AlgorithmUnimplemented",
        template: "This runtime does not support the generation of Ed25519 key pairs.",
    },
    CodeDefinition {
        code: ErrorCode(3610003),
        name: "subtleCryptoExportFunctionUnimplemented",
        template: "No signature verification implementation could be found.",
    },
    CodeDefinition {
        code: ErrorCode(3610004),
        name: "subtleCryptoGenerateFunctionUnimplemented",
        template: "No key generation implementation could be found.",
    },
    CodeDefinition {
        code: ErrorCode(3610005),
        name: "subtleCryptoSignFunctionUnimplemented",
        template: "No signing implementation could be found.",
    },
    CodeDefinition {
        code: ErrorCode(3610006),
        name: "subtleCryptoVerifyFunctionUnimplemented",
        template: "No key export implementation could be found.",
    },
    CodeDefinition {
        code: ErrorCode(3610007),
        name: "subtleCryptoCannotExportNonExtractableKey",
        template: "Cannot export a non-extractable key.",
    },
    CodeDefinition {
        code: ErrorCode(3611000),
        name: "cryptoRandomValuesFunctionUnimplemented",
        template: "No random values implementation could be found.",
    },
    CodeDefinition {
        code: ErrorCode(3704000),
        name: "keysInvalidKeyPairByteLength",
        template: "Key pair bytes must be of length 64, got $byteLength.",
    },
    CodeDefinition {
        code: ErrorCode(3704001),
        name: "keysInvalidPrivateKeyByteLength",
        template: "Expected private key bytes with length 32. Actual length: $actualLength.",
    },
    CodeDefinition {
        code: ErrorCode(3704002),
        name: "keysInvalidSignatureByteLength",
        template: "Expected base58-encoded signature to decode to a byte array of length 64. Actual length: $actualLength.",
    },
    CodeDefinition {
        code: ErrorCode(3704003),
        name: "keysSignatureStringLengthOutOfRange",
        template: "Expected base58-encoded signature string of length in the range [64, 88]. Actual length: $actualLength.",
    },
    CodeDefinition {
        code: ErrorCode(3704004),
        name: "keysPublicKeyMustMatchPrivateKey",
        template: "The provided private key does not match the provided public key.",
    },
    CodeDefinition {
        code: ErrorCode(3704005),
        name: "keysInvalidBase58InGrindRegex",
        template: "The grind regex `/$source/` contains the character `$character`, which is not in the base58 alphabet and can never match a Solana address.",
    },
];
