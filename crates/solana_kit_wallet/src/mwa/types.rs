use super::Error;
use alloc::{string::String, vec::Vec};
use serde::{Deserialize, Serialize};

/// Negotiated MWA application protocol. The association wire paths remain v1.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProtocolVersion {
    V1,
    Legacy,
}

/// Authenticated session properties received after key agreement.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SessionProperties {
    pub protocol_version: ProtocolVersion,
}

impl SessionProperties {
    /// Parses the pinned numeric, string, and missing version representations.
    pub fn from_json(input: &str) -> Result<Self, Error> {
        let object: serde_json::Map<String, serde_json::Value> =
            serde_json::from_str(input).map_err(|_| Error::Json)?;

        let protocol_version = match object.get("v") {
            None => ProtocolVersion::Legacy,

            Some(serde_json::Value::String(value)) if value == "legacy" => ProtocolVersion::Legacy,

            Some(serde_json::Value::String(value)) if value == "1" || value == "v1" => {
                ProtocolVersion::V1
            }

            Some(serde_json::Value::Number(value)) if value.as_u64() == Some(1) => {
                ProtocolVersion::V1
            }
            _ => return Err(Error::Association),
        };

        Ok(Self { protocol_version })
    }
}

/// Application identity. Absent fields are omitted from JSON.
#[derive(Clone, Default, Serialize, Deserialize, Eq, PartialEq)]
pub struct AppIdentity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// Wallet account metadata. The address is base64 on the MWA wire.
#[derive(Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct MwaAccount {
    pub address: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chains: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub features: Option<Vec<String>>,
}

impl MwaAccount {
    /// Decodes a Solana account into the existing Kit address type.
    pub fn solana_address(&self) -> Result<solana_kit_core::Address, Error> {
        use base64::Engine;
        let bytes = base64::engine::general_purpose::STANDARD
            .decode(&self.address)
            .map_err(|_| Error::Json)?;
        Ok(solana_kit_core::Address::from_bytes(
            bytes.try_into().map_err(|_| Error::Json)?,
        ))
    }
}

/// Base64 fields returned by the wallet for a SIWS signature.
#[derive(Serialize, Deserialize)]
pub struct SignInResult {
    pub address: String,
    pub signed_message: String,
    pub signature: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature_type: Option<String>,
}

/// Authorization response. Tokens and signatures have no Debug implementation.
#[derive(Serialize, Deserialize)]
pub struct AuthorizationResult {
    pub accounts: Vec<MwaAccount>,
    pub auth_token: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wallet_uri_base: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sign_in_result: Option<SignInResult>,
}

/// SIWS signing fields. Text validation occurs before message serialization.
#[derive(Clone, Default, Serialize, Deserialize, Eq, PartialEq)]
pub struct SignInPayload {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chain_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nonce: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issued_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<Vec<String>>,
}

/// Supported transaction version, with no arbitrary JSON values.
#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
#[serde(untagged)]
pub enum TransactionVersion {
    Number(u8),
    Named(String),
}

/// Wallet signing limits and advertised features.
#[derive(Clone, Default, Serialize, Deserialize, Eq, PartialEq)]
pub struct WalletCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_transactions_per_request: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_messages_per_request: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_transaction_versions: Option<Vec<TransactionVersion>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub features: Option<Vec<String>>,
}

/// Options forwarded to the wallet for transaction submission.
#[derive(Clone, Default, Serialize, Deserialize, Eq, PartialEq)]
pub struct SignAndSendOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_context_slot: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commitment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_preflight: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_retries: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wait_for_commitment_to_send_next_transaction: Option<bool>,
}
