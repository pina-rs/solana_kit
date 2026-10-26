//! Webhook management uses the independently approved Webhooks service origin.

mod operations;
mod response;

use crate::SensitiveString;

model!(
    "A provider webhook configuration; wire acronyms retain their uppercase spelling.",
    Webhook {
        webhook_id: String => "webhookID",
        wallet: String => "wallet",
        webhook_url: String => "webhookURL",
        transaction_types: Vec<String> => "transactionTypes",
        account_addresses: Vec<String> => "accountAddresses",
        webhook_type: String => "webhookType",
        auth_header: SensitiveString => "authHeader",
    } optional { active: bool => "active" }
);
request!(
    #[serde(deny_unknown_fields)]
    "Create a webhook, leaving optional delivery policy to the provider when omitted.",
    CreateWebhookRequest {
        webhook_url: String => "webhookURL",
        transaction_types: Vec<String> => "transactionTypes",
        account_addresses: Vec<String> => "accountAddresses",
    } optional {
        webhook_type: String => "webhookType",
        auth_header: SensitiveString => "authHeader",
        encoding: String => "encoding",
        txn_status: String => "txnStatus",
    }
);
request!(
    #[derive(Default)]
    #[serde(deny_unknown_fields)]
    "Partial webhook changes. The route identity cannot be included in this body.",
    UpdateWebhookParameters {} optional {
        webhook_url: String => "webhookURL",
        transaction_types: Vec<String> => "transactionTypes",
        account_addresses: Vec<String> => "accountAddresses",
        webhook_type: String => "webhookType",
        auth_header: SensitiveString => "authHeader",
        encoding: String => "encoding",
        txn_status: String => "txnStatus",
    }
);

/// A webhook identity and its separately serialized partial configuration.
#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub struct UpdateWebhookRequest {
    /// One opaque path segment, excluded from the JSON body.
    #[serde(skip)]
    pub webhook_id: String,
    /// Only these fields are sent as the update document.
    #[serde(flatten)]
    pub changes: UpdateWebhookParameters,
}

request!(
    "Read a webhook.",
    GetWebhookRequest { webhook_id: String => "webhookID" } optional {}
);
request!(
    "Read all webhooks for the caller's API key.",
    GetAllWebhooks {} optional {}
);

/// A webhook active-state update; its ID is only a path segment.
#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub struct ToggleWebhook {
    /// Webhook identity, excluded from the JSON body.
    #[serde(skip)]
    pub webhook_id: String,
    /// Requested active state.
    pub active: bool,
}

request!(
    "Delete one webhook. Success acknowledges HTTP completion, including an empty body.",
    DeleteWebhookRequest { webhook_id: String => "webhookID" } optional {}
);
