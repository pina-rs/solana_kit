//! Portal URL normalization and localhost policy used by CLI resolution.

use super::CliError;
use http::Uri;

pub(super) fn normalize_url(value: &str, label: &str) -> Result<String, CliError> {
    if value.contains('#') {
        return Err(CliError(format!("Invalid {label}: {value}")));
    }

    let parsed = value
        .parse::<Uri>()
        .map_err(|_| CliError(format!("Invalid {label}: {value}")))?;
    let scheme = parsed
        .scheme_str()
        .ok_or_else(|| CliError(format!("Invalid {label}: {value}")))?;
    let authority = parsed
        .authority()
        .ok_or_else(|| CliError(format!("Invalid {label}: {value}")))?;

    if authority.host().is_empty() || parsed.query().is_some() {
        return Err(CliError(format!("Invalid {label}: {value}")));
    }

    let path = parsed.path().trim_end_matches('/');

    Ok(format!("{scheme}://{authority}{path}"))
}

pub(super) fn derive_api_base(portal_url: &str) -> Result<String, CliError> {
    let parsed = portal_url
        .parse::<Uri>()
        .map_err(|_| CliError(format!("Invalid portal URL: {portal_url}")))?;
    let scheme = parsed
        .scheme_str()
        .ok_or_else(|| CliError(format!("Invalid portal URL: {portal_url}")))?;
    let authority = parsed
        .authority()
        .ok_or_else(|| CliError(format!("Invalid portal URL: {portal_url}")))?;
    let path = parsed.path().trim_end_matches('/');

    if path.is_empty() {
        return Ok(format!("{scheme}://{authority}/api"));
    }

    Ok(format!("{scheme}://{authority}{path}/api"))
}

pub(super) fn is_https(url: &str) -> bool {
    url.parse::<Uri>()
        .ok()
        .and_then(|uri| uri.scheme_str().map(|scheme| scheme == "https"))
        .unwrap_or(false)
}

pub(super) fn is_localhost(url: &str) -> bool {
    let Ok(parsed) = url.parse::<Uri>() else {
        return false;
    };
    let Some(host) = parsed.host() else {
        return false;
    };
    let host = host.trim_matches(['[', ']']);

    matches!(host, "localhost" | "127.0.0.1" | "::1")
}
