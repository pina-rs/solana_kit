//! Caller-visible SOL funding preflight warnings for the CLI boundary.

use solana_kit_core::crypto::Keypair;
use solana_kit_dapp_publisher::{
    FundingCheck, check_signer_balance, cli::NewVersionOptions, format_sol_amount,
    resolve_funding_rpc_url,
};
use std::time::Duration;

const RPC_TIMEOUT_SECONDS: u64 = 15;

pub(super) fn funding_preflight(
    keypair: &Keypair,
    options: &NewVersionOptions,
) -> Result<(), String> {
    let Some(url) = resolve_funding_rpc_url(options.local_dev, options.rpc_url.as_deref()) else {
        return Ok(());
    };
    let address = keypair.address().to_string();

    match fetch_balance(url, &address) {
        Ok(balance) => match check_signer_balance(&address, balance) {
            Ok(FundingCheck::Sufficient { .. }) => Ok(()),
            Ok(FundingCheck::Insufficient { balance, required }) => Err(format!(
                "signer {address} has {} SOL, but publishing needs at least {} SOL before it starts",
                format_sol_amount(balance),
                format_sol_amount(required)
            )),
            Err(_) => {
                warn_address();

                Ok(())
            }
        },

        Err(_) => {
            warn_balance();

            Ok(())
        }
    }
}

fn warn_address() {
    eprintln!("Warning: unable to validate the signer address for funding preflight.");
}

fn warn_balance() {
    eprintln!(
        "Warning: unable to confirm the signer balance; continuing without a SOL preflight check."
    );
}

fn fetch_balance(url: &str, address: &str) -> Result<u64, String> {
    let body = serde_json::json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "getBalance",
        "params": [address],
    })
    .to_string();
    let agent = ureq::Agent::config_builder()
        .timeout_global(Some(Duration::from_secs(RPC_TIMEOUT_SECONDS)))
        .http_status_as_error(false)
        .build()
        .new_agent();
    let response = agent
        .post(url)
        .header("content-type", "application/json")
        .header("accept", "application/json")
        .send(body.as_str())
        .map_err(|_| "RPC request failed".to_owned())?;

    if !(200..300).contains(&response.status().as_u16()) {
        return Err("RPC returned a non-success status".into());
    }

    let mut response = response;
    let bytes = response
        .body_mut()
        .read_to_vec()
        .map_err(|_| "RPC response failed".to_owned())?;
    let value: serde_json::Value =
        serde_json::from_slice(&bytes).map_err(|_| "RPC response was not JSON".to_owned())?;

    if value.get("error").is_some() {
        return Err("RPC returned an error".into());
    }

    value
        .get("result")
        .and_then(|result| result.get("value"))
        .and_then(serde_json::Value::as_u64)
        .ok_or_else(|| "RPC balance was missing or invalid".into())
}
