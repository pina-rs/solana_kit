//! Returned links encode opaque values and never resolve ambient host settings.

use super::SignupError;
use crate::{SensitiveString, auth::SignupEndpoints};
use percent_encoding::{AsciiSet, NON_ALPHANUMERIC, utf8_percent_encode};

const COMPONENT: &AsciiSet = &NON_ALPHANUMERIC
    .remove(b'-')
    .remove(b'_')
    .remove(b'.')
    .remove(b'!')
    .remove(b'~')
    .remove(b'*')
    .remove(b'\'')
    .remove(b'(')
    .remove(b')');
const PAYMENT_HOST: &str = "https://dashboard.helius.dev";

/// Builds credential-bearing RPC endpoints without letting a key inject query fields.
pub fn build_endpoints(api_key: &SensitiveString) -> SignupEndpoints {
    let encoded = utf8_percent_encode(api_key.expose(), COMPONENT);
    SignupEndpoints {
        mainnet: SensitiveString::new(format!("https://mainnet.helius-rpc.com/?api-key={encoded}")),
        devnet: SensitiveString::new(format!("https://devnet.helius-rpc.com/?api-key={encoded}")),
    }
}

/// Builds a hosted link from an explicit ASCII HTTP(S) base, without environment reads.
///
/// # Errors
/// Rejects credentials, queries, fragments, invalid authorities and empty intent IDs.
pub fn build_payment_url(intent: &str, host: Option<&str>) -> Result<String, SignupError> {
    let host = payment_host(host)?;

    if intent.is_empty() {
        return Err(SignupError::InvalidPaymentIntent);
    }

    Ok(format!(
        "{host}/pay/{}",
        utf8_percent_encode(intent, COMPONENT)
    ))
}

/// A parsed URI still needs explicit port and credential checks before becoming a payment base.
pub(super) fn payment_host(host: Option<&str>) -> Result<&str, SignupError> {
    let host = host
        .filter(|value| !value.is_empty())
        .unwrap_or(PAYMENT_HOST);
    let host = host.strip_suffix('/').unwrap_or(host);
    let uri = host
        .parse::<http::Uri>()
        .map_err(|_| SignupError::InvalidPaymentHost)?;
    let authority = uri.authority().ok_or(SignupError::InvalidPaymentHost)?;

    if !matches!(uri.scheme_str(), Some("http" | "https"))
        || authority.host().is_empty()
        || authority.as_str().contains('@')
        || uri.query().is_some()
        || host.contains(['#', '\\'])
    {
        return Err(SignupError::InvalidPaymentHost);
    }

    let port = &authority.as_str()[authority.host().len()..];

    if !port.is_empty()
        && !port.strip_prefix(':').is_some_and(|value| {
            !value.is_empty()
                && value.bytes().all(|byte| byte.is_ascii_digit())
                && value.parse::<u16>().is_ok()
        })
    {
        return Err(SignupError::InvalidPaymentHost);
    }

    Ok(host)
}
