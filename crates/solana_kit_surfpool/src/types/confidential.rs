//! Token-2022 confidential-transfer schemas and explicit authority states.

use serde::{Deserialize, Serialize};
use solana_kit_core::Address;

/// A token authority is unchanged, replaced, or cleared with the literal `"null"`.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum AuthorityUpdate {
    /// Omit the field.
    #[default]
    Unchanged,
    /// Set this address.
    Set(Address),
    /// Send the string `"null"`, as required by Surfpool.
    Clear,
}

impl AuthorityUpdate {
    pub(crate) fn is_unchanged(&self) -> bool {
        matches!(self, Self::Unchanged)
    }
}

impl Serialize for AuthorityUpdate {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Unchanged => serializer.serialize_unit(),
            Self::Set(address) => serializer.collect_str(address),
            Self::Clear => serializer.serialize_str("null"),
        }
    }
}

/// Fields accepted by `surfnet_setTokenAccount`.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetTokenAccountUpdate {
    /// Exact token amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<u64>,
    /// Delegate mutation; invalid set-and-clear combinations cannot be represented.
    #[serde(skip_serializing_if = "AuthorityUpdate::is_unchanged")]
    pub delegate: AuthorityUpdate,
    /// Runtime-defined token account state string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// Exact delegated token amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delegated_amount: Option<u64>,
    /// Close authority mutation.
    #[serde(skip_serializing_if = "AuthorityUpdate::is_unchanged")]
    pub close_authority: AuthorityUpdate,
    /// Token-2022 confidential-transfer setup.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidential: Option<ConfidentialTransferAccountUpdate>,
}

/// Secret string whose explicit serialization retains the exact caller value.
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct SecretString(String);

impl SecretString {
    /// Takes ownership of a base58 or base64 secret without changing its encoding.
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    /// Explicit access for the caller's cryptographic operations.
    pub fn expose(&self) -> &str {
        &self.0
    }
}

impl core::fmt::Debug for SecretString {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        formatter.write_str("SecretString([REDACTED])")
    }
}

impl Drop for SecretString {
    fn drop(&mut self) {
        use zeroize::Zeroize;
        self.0.zeroize();
    }
}

/// Fabricates a configured confidential account for local tests.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConfidentialTransferAccountUpdate {
    /// Owner's encoded 32-byte ElGamal public key.
    pub elgamal_pubkey: String,
    /// Owner's encoded 16-byte AES secret key.
    pub aes_key: SecretString,
    /// Available amount, defaulting to zero in the runtime.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<u64>,
    /// Account approval; omitted uses the runtime default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approved: Option<bool>,
    /// Incoming confidential credit permission.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_confidential_credits: Option<bool>,
    /// Incoming base token credit permission.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_non_confidential_credits: Option<bool>,
    /// Maximum pending credit counter, defaulting to 65536 upstream.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_pending_balance_credit_counter: Option<u64>,
}

/// At least one confidential balance decryption key is always present.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(untagged)]
pub enum ConfidentialBalanceKeys {
    /// Decrypt the available balance only.
    Aes {
        /// Encoded AES key.
        #[serde(rename = "aesKey")]
        aes_key: SecretString,
    },
    /// Decrypt the pending balance only.
    Elgamal {
        /// Encoded ElGamal secret key.
        #[serde(rename = "elgamalSecretKey")]
        elgamal_secret_key: SecretString,
    },
    /// Decrypt both independently encrypted balance halves.
    Both {
        /// Encoded AES key.
        #[serde(rename = "aesKey")]
        aes_key: SecretString,
        /// Encoded ElGamal secret key.
        #[serde(rename = "elgamalSecretKey")]
        elgamal_secret_key: SecretString,
    },
}

object_model! {
/// Decrypted Token-2022 balances; unavailable halves remain null.
pub struct ConfidentialBalance {
    /// Available amount when an AES key was supplied.
    pub available: Option<i128>,
    /// Pending amount when an ElGamal key was supplied.
    pub pending: Option<i128>,
    /// Pending credit count.
    pub pending_balance_credit_counter: i128,
}
}

object_model! {
/// Runtime-derived keys, retaining all three original encoded values.
pub struct ConfidentialKeys {
    /// Encoded ElGamal public key.
    pub elgamal_pubkey: String,
    /// Encoded ElGamal secret key.
    pub elgamal_secret_key: SecretString,
    /// Encoded AES secret key.
    pub aes_key: SecretString,
}
}
