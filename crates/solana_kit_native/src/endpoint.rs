//! Absolute HTTP endpoint parsing without retaining diagnostic secret values.
use crate::ConfigError;
use hyper::http::Uri;
use tokio_rustls::rustls::pki_types::ServerName;

const HTTP_PORT: u16 = 80;
const HTTPS_PORT: u16 = 443;

pub(crate) struct Endpoint {
    pub(crate) host: String,
    pub(crate) port: u16,
    pub(crate) secure: bool,
    pub(crate) authority: String,
    pub(crate) path: String,
}

impl Endpoint {
    pub(crate) fn parse(value: &str) -> Result<Self, ConfigError> {
        validate_text(value)?;
        let uri: Uri = value
            .parse()
            .map_err(|_| ConfigError("invalid HTTP endpoint"))?;
        let secure = scheme(&uri)?;
        let authority = uri
            .authority()
            .ok_or(ConfigError("endpoint requires a host"))?;

        if authority.as_str().contains('@') || authority.host().is_empty() {
            return Err(ConfigError("URL credentials are forbidden"));
        }

        let host = authority
            .host()
            .trim_start_matches('[')
            .trim_end_matches(']')
            .to_owned();
        ServerName::try_from(host.clone()).map_err(|_| ConfigError("invalid endpoint host"))?;
        let suffix = &authority.as_str()[authority.host().len()..];
        let port = port(suffix, secure)?;
        let path = uri
            .path_and_query()
            .map_or("/", |path| path.as_str())
            .to_owned();

        Ok(Self {
            host,
            port,
            secure,
            authority: authority.as_str().to_owned(),
            path,
        })
    }
}

fn validate_text(value: &str) -> Result<(), ConfigError> {
    if value.contains('#') || value.chars().any(char::is_whitespace) {
        return Err(ConfigError("invalid HTTP endpoint"));
    }

    Ok(())
}

fn scheme(uri: &Uri) -> Result<bool, ConfigError> {
    match uri.scheme_str() {
        Some("https") => Ok(true),
        Some("http") => Ok(false),
        _ => Err(ConfigError("endpoint must use http or https")),
    }
}

fn port(suffix: &str, secure: bool) -> Result<u16, ConfigError> {
    let Some(text) = suffix.strip_prefix(':') else {
        return Ok(if secure { HTTPS_PORT } else { HTTP_PORT });
    };

    if text.is_empty() || !text.bytes().all(|byte| byte.is_ascii_digit()) {
        return Err(ConfigError("invalid endpoint port"));
    }

    // http::Uri::port() returns None for overflow, which would hide invalid input.
    text.parse::<u16>()
        .ok()
        .filter(|port| *port != 0)
        .ok_or(ConfigError("invalid endpoint port"))
}
