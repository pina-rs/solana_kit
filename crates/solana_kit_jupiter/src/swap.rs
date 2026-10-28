//! The managed and self-landed paths share the adapter-owned REST exchange boundary.

use crate::{
    BuildResponse, Client, Error, ExecutionResponse, Integer, OrderRequest, OrderResponse, Service,
    object,
};
use serde::Serialize;
use solana_kit_client::rest::{self, Budgets, Transport};

impl<T: Transport<Service>> Client<T> {
    /// Requests a quote and an optional assembled transaction from Swap API v2.
    ///
    /// # Errors
    /// Returns adapter, HTTP, budget or schema failures without retrying the exchange.
    pub async fn get_order(&self, order: &OrderRequest) -> Result<OrderResponse, Error> {
        let body = self
            .get(
                &["swap", "v2", "order"],
                order.bounded_query(self.budgets.bytes)?,
            )
            .await?;
        decode_order(&body, self.budgets)
    }

    /// Requests raw instructions for a caller-assembled and self-landed transaction.
    ///
    /// # Errors
    /// Returns adapter, HTTP, budget or schema failures without retrying the exchange.
    pub async fn build_swap(&self, order: &OrderRequest) -> Result<BuildResponse, Error> {
        let body = self
            .get(
                &["swap", "v2", "build"],
                order.bounded_query(self.budgets.bytes)?,
            )
            .await?;
        decode_build(&body, self.budgets)
    }

    /// Submits an already signed base64 transaction once for an accepted order.
    ///
    /// Requires a nonblank request ID. Expiry remains a decimal string on wire.
    /// The caller inspects and signs the assembled transaction before this call;
    /// execution status and code must be checked even when a signature is present.
    ///
    /// # Errors
    /// Rejects a missing or blank identifier before I/O. Also returns request-budget,
    /// adapter, HTTP, response-budget and response-schema failures without retrying.
    pub async fn execute_order(
        &self,
        order: &OrderResponse,
        signed_transaction: &str,
    ) -> Result<ExecutionResponse, Error> {
        let request_id = order
            .request_id
            .as_deref()
            .filter(|id| !id.trim().is_empty())
            .ok_or(Error::MissingRequestId)?;
        self.budgets
            .check_bytes(request_id.as_bytes())
            .map_err(Error::Json)?;
        self.budgets
            .check_bytes(signed_transaction.as_bytes())
            .map_err(Error::Json)?;
        if let Some(expiry) = &order.last_valid_block_height {
            self.budgets
                .check_bytes(expiry.as_str().as_bytes())
                .map_err(Error::Json)?;
        }

        let payload = ExecuteBody {
            request_id,
            signed_transaction,
            last_valid_block_height: order.last_valid_block_height.as_ref(),
        };
        let body = self.budgets.serialize(&payload).map_err(Error::Json)?;
        let response = self
            .request(
                rest::Method::Post,
                &["swap", "v2", "execute"],
                Vec::new(),
                rest::Body::Json(body),
            )
            .await?;
        decode_execution(&response, self.budgets)
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ExecuteBody<'a> {
    request_id: &'a str,
    signed_transaction: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    last_valid_block_height: Option<&'a Integer>,
}

/// Decodes an order object after shared budget, syntax and duplicate-key admission.
///
/// # Errors
/// Rejects admission failures, nonobject responses and malformed known fields.
pub fn decode_order(bytes: &[u8], budgets: Budgets) -> Result<OrderResponse, Error> {
    budgets.validate_json(bytes).map_err(Error::Json)?;
    let mut response: OrderResponse = decode_object(bytes)?;
    response.raw_response = bytes.to_vec();
    Ok(response)
}

/// Decodes a build object, preserving response map order for lookup-address fallback.
///
/// # Errors
/// Rejects admission failures, nonobject payloads and malformed known fields.
pub fn decode_build(bytes: &[u8], budgets: Budgets) -> Result<BuildResponse, Error> {
    budgets.validate_json(bytes).map_err(Error::Json)?;
    let mut response: BuildResponse = decode_object(bytes)?;

    if response.address_lookup_table_addresses.is_none() {
        response.address_lookup_table_addresses = response
            .addresses_by_lookup_table_address
            .as_ref()
            .map(|tables| {
                tables
                    .entries
                    .iter()
                    .map(|table| table.address.clone())
                    .collect()
            });
    }

    response.raw_response = bytes.to_vec();
    Ok(response)
}

/// Decodes execution status without inferring success from signature presence.
///
/// # Errors
/// Rejects admission failures, nonobject responses and malformed known fields.
pub fn decode_execution(bytes: &[u8], budgets: Budgets) -> Result<ExecutionResponse, Error> {
    budgets.validate_json(bytes).map_err(Error::Json)?;
    let mut response: ExecutionResponse = decode_object(bytes)?;
    response.raw_response = bytes.to_vec();
    Ok(response)
}

fn decode_object<T: serde::de::DeserializeOwned>(bytes: &[u8]) -> Result<T, Error> {
    let text = std::str::from_utf8(bytes).map_err(|_| Error::Payload("invalid JSON UTF-8"))?;
    object::parse(text).map_err(Error::Schema)
}
