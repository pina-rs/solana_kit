//! Absolute URL parsing cannot select an authentication namespace or follow redirects.

use crate::workflow::Error;
use http::{Uri, uri::Scheme};

const HTTPS_PORT: u16 = 443;

/// Normalize a missing scheme while preserving query bytes and denying credentials or downgrade.
pub(crate) fn https(value: &str) -> Result<String, Error> {
    let value = value.trim();

    let explicit_scheme = value
        .split_once("://")
        .is_some_and(|(prefix, _)| !prefix.contains(['/', '?']));
    let normalized = if explicit_scheme {
        value.to_owned()
    } else {
        format!("https://{value}")
    };

    let uri = absolute(&normalized)?;

    if uri.query().is_some() {
        return Ok(normalized);
    }

    Ok(normalized
        .strip_suffix('/')
        .unwrap_or(&normalized)
        .to_owned())
}

/// Presigned PUT targets must already be absolute; approval never rewrites their signed bytes.
pub(crate) fn signed_target(value: &str) -> Result<(), Error> {
    if value != value.trim() {
        return Err(Error::Invalid("invalid presigned target whitespace"));
    }

    absolute(value).map(|_| ())
}

/// Durable artifact locations are query-free; this cannot prove arbitrary path bytes are public.
pub(crate) fn public(value: &str) -> Result<String, Error> {
    public_uri(value)?;
    Ok(value.to_owned())
}

fn public_uri(value: &str) -> Result<Uri, Error> {
    if value != value.trim() {
        return Err(Error::Invalid("invalid public URL whitespace"));
    }

    let uri = absolute(value)?;

    if uri.query().is_some() {
        return Err(Error::Invalid("public artifact URL must be query-free"));
    }

    Ok(uri)
}

fn absolute(value: &str) -> Result<Uri, Error> {
    if value.is_empty()
        || value.contains(['#', '\\'])
        || value.bytes().any(|byte| byte.is_ascii_control())
    {
        return Err(Error::Invalid("invalid media URL"));
    }

    let uri: Uri = value
        .parse()
        .map_err(|_| Error::Invalid("invalid media URL"))?;
    approve(&uri)?;
    Ok(uri)
}

fn approve(uri: &Uri) -> Result<(), Error> {
    let authority = uri
        .authority()
        .ok_or(Error::Invalid("missing media authority"))?;

    if uri.scheme() != Some(&Scheme::HTTPS)
        || authority.as_str().contains('@')
        || authority.host().is_empty()
    {
        return Err(Error::Invalid("media URL requires credential-free HTTPS"));
    }

    Ok(())
}

pub(in crate::workflow) fn public_r2(value: &str) -> bool {
    let Ok(uri) = public_uri(value) else {
        return false;
    };
    let Some(authority) = uri.authority() else {
        return false;
    };
    let host = authority.host();
    let ordinary_port =
        authority.as_str().eq_ignore_ascii_case(host) || authority.port_u16() == Some(HTTPS_PORT);
    (host.eq_ignore_ascii_case("r2.solanamobiledappstore.com")
        || host.eq_ignore_ascii_case("r2-staging.solanamobiledappstore.com"))
        && ordinary_port
}

pub(super) fn file_name(value: &str) -> Option<&str> {
    value
        .split_once("://")?
        .1
        .split_once('/')?
        .1
        .split('?')
        .next()?
        .rsplit('/')
        .find(|part| !part.is_empty())
}
