//! Pure command-line resolution for the publisher CLI.
//!
//! Argument, environment, endpoint and secret parsing live here. File reads,
//! network transport, signing and workflow execution stay caller-owned.

use http::Uri;
use std::{collections::BTreeMap, fmt, string::String, vec::Vec};

mod urls;
use urls::{derive_api_base, is_https, is_localhost, normalize_url};

/// Version of the `solana_kit_dapp_publisher` library package.
///
/// This follows this crate's own Cargo metadata. It is not the version of a
/// dependent executable: the `solana_kit_dapp_publisher_cli` binary reports its
/// own package version so the two packages can be released independently.
pub const CLI_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Documentation shown with CLI usage errors.
pub const DOCS_URL: &str = "https://docs.solanamobile.com/dapp-store/publishing-cli";

/// Default production portal.
pub const DEFAULT_PRODUCTION_PORTAL_URL: &str = "https://publish.solanamobile.com";

/// Default local portal.
pub const DEFAULT_LOCAL_PORTAL_URL: &str = "http://localhost:3333";

/// Default environment variable containing the portal API key.
pub const DEFAULT_API_KEY_ENV: &str = "DAPP_STORE_API_KEY";

/// Environment variable that can override the portal URL.
pub const PORTAL_URL_ENV: &str = "DAPP_STORE_PORTAL_URL";

/// Parsed flag values: `Some(value)` for valued flags, `None` for booleans.
pub type Flags = BTreeMap<String, Option<String>>;

/// One bounded CLI input error.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CliError(String);

/// Parsed options for a new publication.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct NewVersionOptions {
    /// Local APK path.
    pub apk_file: Option<String>,
    /// Remote HTTPS APK URL.
    pub apk_url: Option<String>,
    /// Release notes.
    pub whats_new: Option<String>,
    /// Portal base URL.
    pub portal_url: Option<String>,
    /// Environment variable containing the API key.
    pub api_key_env: Option<String>,
    /// Whether to read the API key from stdin.
    pub api_key_stdin: bool,
    /// Signer keypair path.
    pub keypair: Option<String>,
    /// Optional RPC URL.
    pub rpc_url: Option<String>,
    /// Local-development endpoint policy.
    pub local_dev: bool,
    /// Whether to skip the self-update gate.
    pub skip_self_update: bool,
    /// Optional idempotency key.
    pub idempotency_key: Option<String>,
    /// Whether to print detailed identifiers.
    pub verbose: bool,
}

/// Parsed options for resuming a publication.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ResumeOptions {
    /// Release selector.
    pub release_id: Option<String>,
    /// Session selector.
    pub session_id: Option<String>,
    /// Release selector alias.
    pub resume_release: Option<String>,
    /// Session selector alias.
    pub resume_session: Option<String>,
    /// Portal base URL.
    pub portal_url: Option<String>,
    /// Environment variable containing the API key.
    pub api_key_env: Option<String>,
    /// Whether to read the API key from stdin.
    pub api_key_stdin: bool,
    /// Signer keypair path.
    pub keypair: Option<String>,
    /// Optional RPC URL.
    pub rpc_url: Option<String>,
    /// Local-development endpoint policy.
    pub local_dev: bool,
    /// Whether to skip the self-update gate.
    pub skip_self_update: bool,
    /// Whether to print detailed identifiers.
    pub verbose: bool,
}

/// Resolved portal endpoints.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PortalTargets {
    /// Exact portal base URL without a trailing slash.
    pub portal_url: String,
    /// Derived portal API base URL.
    pub api_base_url: String,
}

/// One publication source selected by the caller.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PublicationSource {
    /// A local APK to read and upload through the caller's uploader.
    LocalApk {
        /// Caller-supplied path.
        path: String,
        /// Public file name taken from the path.
        file_name: String,
    },
    /// An already hosted HTTPS APK.
    RemoteApk {
        /// Caller-supplied URL.
        url: String,
        /// Inferred public file name, when the URL has one.
        file_name: Option<String>,
    },
}

impl CliError {
    /// Human-readable message without any captured secret.
    pub fn message(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for CliError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl std::error::Error for CliError {}

const VALUED: &[&str] = &[
    "apk-file",
    "apk-url",
    "whats-new",
    "portal-url",
    "api-key-env",
    "keypair",
    "rpc-url",
    "idempotency-key",
    "release-id",
    "session-id",
    "resume-release",
    "resume-session",
];

const BOOLEAN: &[&str] = &["api-key-stdin", "local-dev", "skip-self-update", "verbose"];

/// Parses the root command flags.
pub fn parse_flags(arguments: &[String]) -> Result<Flags, CliError> {
    let mut values = BTreeMap::new();
    let mut index = 0;

    while index < arguments.len() {
        let argument = &arguments[index];
        let (name, inline) = split_flag(argument)?;

        if !VALUED.contains(&name) && !BOOLEAN.contains(&name) {
            return Err(CliError(format!("Unknown argument: {argument}")));
        }

        let value = parse_flag(arguments, &mut index, name, inline)?;

        values.insert(name.to_owned(), value);
        index += 1;
    }

    Ok(values)
}

fn split_flag(argument: &str) -> Result<(&str, Option<&str>), CliError> {
    let Some(without_prefix) = argument.strip_prefix("--") else {
        return Err(CliError(format!("Unexpected argument: {argument}")));
    };

    Ok(without_prefix
        .split_once('=')
        .map_or((without_prefix, None), |(name, value)| (name, Some(value))))
}

fn parse_flag(
    arguments: &[String],
    index: &mut usize,
    name: &str,
    inline: Option<&str>,
) -> Result<Option<String>, CliError> {
    if VALUED.contains(&name) {
        return parse_value(arguments, index, name, inline).map(Some);
    }

    reject_boolean_value(name, inline)?;

    Ok(None)
}

fn parse_value(
    arguments: &[String],
    index: &mut usize,
    name: &str,
    inline: Option<&str>,
) -> Result<String, CliError> {
    if let Some(value) = inline {
        return Ok(value.to_owned());
    }

    let next = arguments.get(*index + 1).ok_or_else(|| {
        CliError(format!(
            "Missing value for --{name}. Provide it as --{name} <value>."
        ))
    })?;
    *index += 1;

    Ok(next.clone())
}

fn reject_boolean_value(name: &str, inline: Option<&str>) -> Result<(), CliError> {
    if inline.is_some() {
        return Err(CliError(format!(
            "The --{name} flag does not accept a value."
        )));
    }

    Ok(())
}

/// Builds new-version options from parsed flags.
pub fn new_version_options(flags: &Flags) -> NewVersionOptions {
    NewVersionOptions {
        apk_file: value(flags, "apk-file"),
        apk_url: value(flags, "apk-url"),
        whats_new: value(flags, "whats-new"),
        portal_url: value(flags, "portal-url"),
        api_key_env: value(flags, "api-key-env"),
        api_key_stdin: flags.contains_key("api-key-stdin"),
        keypair: value(flags, "keypair"),
        rpc_url: value(flags, "rpc-url"),
        local_dev: flags.contains_key("local-dev"),
        skip_self_update: flags.contains_key("skip-self-update"),
        idempotency_key: value(flags, "idempotency-key"),
        verbose: flags.contains_key("verbose"),
    }
}

/// Builds resume options from parsed flags.
pub fn resume_options(flags: &Flags) -> ResumeOptions {
    ResumeOptions {
        release_id: value(flags, "release-id"),
        session_id: value(flags, "session-id"),
        resume_release: value(flags, "resume-release"),
        resume_session: value(flags, "resume-session"),
        portal_url: value(flags, "portal-url"),
        api_key_env: value(flags, "api-key-env"),
        api_key_stdin: flags.contains_key("api-key-stdin"),
        keypair: value(flags, "keypair"),
        rpc_url: value(flags, "rpc-url"),
        local_dev: flags.contains_key("local-dev"),
        skip_self_update: flags.contains_key("skip-self-update"),
        verbose: flags.contains_key("verbose"),
    }
}

/// Validates the new-version inputs without reading the APK.
pub fn validate_new_version(
    options: &NewVersionOptions,
    file_exists: impl Fn(&str) -> bool,
) -> Result<(), CliError> {
    validate_one_source(options)?;
    validate_required(options.whats_new.as_deref(), "`--whats-new` is required.")?;
    validate_required(options.keypair.as_deref(), "`--keypair` is required.")?;
    validate_apk_file(options.apk_file.as_deref(), &file_exists)?;
    validate_apk_url(options.apk_url.as_deref())?;

    Ok(())
}

fn validate_one_source(options: &NewVersionOptions) -> Result<(), CliError> {
    let sources = usize::from(options.apk_file.is_some()) + usize::from(options.apk_url.is_some());

    if sources == 1 {
        return Ok(());
    }

    Err(CliError(
        "Specify exactly one of `--apk-file` or `--apk-url`.".into(),
    ))
}

fn validate_required(value: Option<&str>, message: &'static str) -> Result<(), CliError> {
    if value.is_some_and(|value| !value.trim().is_empty()) {
        return Ok(());
    }

    Err(CliError(message.into()))
}

fn validate_apk_file(
    path: Option<&str>,
    file_exists: &impl Fn(&str) -> bool,
) -> Result<(), CliError> {
    let Some(path) = path else {
        return Ok(());
    };

    if file_exists(path) {
        return Ok(());
    }

    Err(CliError(format!("APK file not found: {path}")))
}

fn validate_apk_url(url: Option<&str>) -> Result<(), CliError> {
    let Some(url) = url else {
        return Ok(());
    };

    let parsed = url
        .parse::<Uri>()
        .map_err(|_| CliError("`--apk-url` must be a valid HTTPS URL.".into()))?;

    if parsed.scheme_str() != Some("https") || parsed.host().is_none() {
        return Err(CliError("`--apk-url` must be a valid HTTPS URL.".into()));
    }

    Ok(())
}

/// Validates the resume inputs.
pub fn validate_resume(options: &ResumeOptions) -> Result<(), CliError> {
    let release = resolve_resume_target(
        options.release_id.as_deref(),
        options.resume_release.as_deref(),
        "--release-id",
        "--resume-release",
    )?;
    let session = resolve_resume_target(
        options.session_id.as_deref(),
        options.resume_session.as_deref(),
        "--session-id",
        "--resume-session",
    )?;

    if usize::from(release.is_some()) + usize::from(session.is_some()) != 1 {
        return Err(CliError(
            "Specify exactly one of `--release-id` or `--session-id`.".into(),
        ));
    }

    if options
        .keypair
        .as_deref()
        .is_none_or(|value| value.trim().is_empty())
    {
        return Err(CliError("`--keypair` is required.".into()));
    }

    Ok(())
}

/// Resolves a primary selector and its alias.
pub fn resolve_resume_target(
    primary: Option<&str>,
    alias: Option<&str>,
    primary_label: &str,
    alias_label: &str,
) -> Result<Option<String>, CliError> {
    let primary = trim_to_none(primary);
    let alias = trim_to_none(alias);

    if let (Some(primary), Some(alias)) = (primary, alias)
        && primary != alias
    {
        return Err(CliError(format!(
            "Conflicting values were provided for {primary_label} and {alias_label}."
        )));
    }

    Ok(primary.or(alias).map(str::to_owned))
}

/// Rejects `--skip-self-update` outside local development.
pub fn enforce_self_update_policy(skip_self_update: bool, local_dev: bool) -> Result<(), CliError> {
    if skip_self_update && !local_dev {
        return Err(CliError(
            "`--skip-self-update` is only allowed together with `--local-dev`.".into(),
        ));
    }

    Ok(())
}

/// Resolves the portal and derived API endpoints.
pub fn resolve_portal_targets(
    portal_url: Option<&str>,
    local_dev: bool,
    environment: &BTreeMap<String, String>,
) -> Result<PortalTargets, CliError> {
    let selected = select_portal_url(portal_url, local_dev, environment);
    let portal_url = normalize_url(selected, "portal URL")?;
    let api_base_url = normalize_url(&derive_api_base(&portal_url)?, "portal API base URL")?;
    enforce_endpoint_policy(&portal_url, &api_base_url, local_dev)?;

    Ok(PortalTargets {
        portal_url,
        api_base_url,
    })
}

fn select_portal_url<'a>(
    portal_url: Option<&'a str>,
    local_dev: bool,
    environment: &'a BTreeMap<String, String>,
) -> &'a str {
    let environment_url = environment.get(PORTAL_URL_ENV).map(String::as_str);

    trim_to_none(portal_url)
        .or_else(|| trim_to_none(environment_url))
        .or_else(|| local_dev.then_some(DEFAULT_LOCAL_PORTAL_URL))
        .unwrap_or(DEFAULT_PRODUCTION_PORTAL_URL)
}

fn enforce_endpoint_policy(
    portal_url: &str,
    api_base_url: &str,
    local_dev: bool,
) -> Result<(), CliError> {
    if local_dev {
        return enforce_local_endpoints(portal_url, api_base_url);
    }

    if is_https(portal_url) {
        return Ok(());
    }

    Err(CliError(format!(
        "Portal endpoints must use HTTPS unless --local-dev is set. Received portal URL: {portal_url}"
    )))
}

fn enforce_local_endpoints(portal_url: &str, api_base_url: &str) -> Result<(), CliError> {
    if is_localhost(portal_url) && is_localhost(api_base_url) {
        return Ok(());
    }

    let target = if is_localhost(portal_url) {
        format!("portal API base URL: {api_base_url}")
    } else {
        format!("portal URL: {portal_url}")
    };

    Err(CliError(format!(
        "--local-dev only allows localhost portal endpoints. Received {target}"
    )))
}

/// Resolves the portal API key from stdin or the selected environment variable.
///
/// The returned value is plain text; callers must keep it out of logs and argv.
pub fn resolve_api_key(
    api_key_stdin: bool,
    api_key_env: Option<&str>,
    environment: &BTreeMap<String, String>,
    stdin: Option<&[u8]>,
    stdin_is_tty: bool,
) -> Result<String, CliError> {
    if api_key_stdin {
        if stdin_is_tty {
            return Err(CliError(with_docs("No API key was piped into stdin.")));
        }

        let value = std::str::from_utf8(stdin.unwrap_or_default())
            .map_err(|_| CliError(with_docs("No API key was provided on stdin.")))?
            .trim();

        return if value.is_empty() {
            Err(CliError(with_docs("No API key was provided on stdin.")))
        } else {
            Ok(value.to_owned())
        };
    }

    let variable = api_key_env
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .unwrap_or(DEFAULT_API_KEY_ENV);
    let value = environment.get(variable).map(String::as_str);
    trim_to_none(value).map(str::to_owned).ok_or_else(|| {
        CliError(with_docs(&format!(
            "Portal API key is required. Set {variable} or pass --api-key-stdin."
        )))
    })
}

/// Builds the publication source from validated options.
pub fn build_publication_source(
    options: &NewVersionOptions,
) -> Result<PublicationSource, CliError> {
    if let Some(path) = options.apk_file.as_deref() {
        return Ok(PublicationSource::LocalApk {
            path: path.to_owned(),
            file_name: file_name_of(path),
        });
    }

    let url = options
        .apk_url
        .as_deref()
        .ok_or_else(|| CliError("`--apk-file` or `--apk-url` is required.".into()))?;
    Ok(PublicationSource::RemoteApk {
        url: url.to_owned(),
        file_name: infer_file_name_from_url(url),
    })
}

/// Returns the final path segment, accepting both slash styles.
pub fn file_name_of(path: &str) -> String {
    let normalized = path.replace('\\', "/");
    normalized.rsplit('/').next().unwrap_or(path).to_owned()
}

/// Infers a nonempty final URL path segment.
pub fn infer_file_name_from_url(url: &str) -> Option<String> {
    let parsed = url.parse::<Uri>().ok()?;
    parsed
        .path()
        .rsplit('/')
        .find(|segment| !segment.is_empty())
        .map(str::to_owned)
}

/// Publication summary lines shown after a successful run.
pub fn summary_lines(
    release_mint: Option<&str>,
    collection_mint: Option<&str>,
    ticket: Option<&str>,
) -> Vec<String> {
    let mut lines = vec![String::from("This app is now in review.")];

    for (value, label) in [
        (release_mint, "Release mint address"),
        (collection_mint, "Collection mint address"),
        (ticket, "Ticket ID"),
    ] {
        if let Some(value) = value.map(str::trim).filter(|value| !value.is_empty()) {
            lines.push(format!("{label}: {value}"));
        }
    }

    lines
}

fn value(flags: &Flags, name: &str) -> Option<String> {
    flags.get(name).and_then(Clone::clone)
}

fn trim_to_none(value: Option<&str>) -> Option<&str> {
    value.map(str::trim).filter(|value| !value.is_empty())
}

fn with_docs(message: &str) -> String {
    format!("{message}\n\nThe publishing CLI has changed. See the updated usage guide:\n{DOCS_URL}")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn flags(arguments: &[&str]) -> Flags {
        parse_flags(
            &arguments
                .iter()
                .map(|value| value.to_string())
                .collect::<Vec<_>>(),
        )
        .unwrap()
    }

    #[test]
    fn parses_valued_boolean_and_equals_forms() {
        let parsed = flags(&[
            "--apk-file=app.apk",
            "--whats-new",
            "New",
            "--local-dev",
            "--verbose",
        ]);
        assert_eq!(parsed["apk-file"], Some("app.apk".into()));
        assert_eq!(parsed["whats-new"], Some("New".into()));
        assert_eq!(parsed["local-dev"], None);
        assert_eq!(parsed["verbose"], None);
    }

    #[test]
    fn rejects_unknown_missing_and_boolean_values() {
        assert!(parse_flags(&["--unknown".into()]).is_err());
        assert!(parse_flags(&["--whats-new".into()]).is_err());
        assert!(parse_flags(&["--verbose=true".into()]).is_err());
        assert!(parse_flags(&["positional".into()]).is_err());
    }

    #[test]
    fn validates_new_version_inputs_without_reading_files() {
        let mut options = NewVersionOptions {
            apk_file: Some("missing.apk".into()),
            whats_new: Some("New".into()),
            keypair: Some("key.json".into()),
            ..Default::default()
        };
        assert!(validate_new_version(&options, |_| false).is_err());
        assert!(validate_new_version(&options, |_| true).is_ok());

        options.apk_url = Some("https://example.invalid/app.apk".into());
        assert!(validate_new_version(&options, |_| true).is_err());

        options.apk_file = None;
        assert!(validate_new_version(&options, |_| true).is_ok());

        options.apk_url = Some("http://example.invalid/app.apk".into());
        assert!(validate_new_version(&options, |_| true).is_err());
    }

    #[test]
    fn resume_aliases_must_agree_and_select_exactly_one_target() {
        assert_eq!(
            resolve_resume_target(Some("rel"), Some("rel"), "--release-id", "--resume-release"),
            Ok(Some("rel".into()))
        );
        assert!(
            resolve_resume_target(
                Some("rel"),
                Some("other"),
                "--release-id",
                "--resume-release"
            )
            .is_err()
        );
        let options = ResumeOptions {
            release_id: Some("rel".into()),
            keypair: Some("key.json".into()),
            ..Default::default()
        };
        assert!(validate_resume(&options).is_ok());
        let conflicting = ResumeOptions {
            release_id: Some("rel".into()),
            session_id: Some("session".into()),
            keypair: Some("key.json".into()),
            ..Default::default()
        };
        assert!(validate_resume(&conflicting).is_err());
    }

    #[test]
    fn self_update_requires_local_development() {
        assert!(enforce_self_update_policy(true, true).is_ok());
        assert!(enforce_self_update_policy(false, false).is_ok());
        assert!(enforce_self_update_policy(true, false).is_err());
    }

    #[test]
    fn portal_targets_default_derive_and_enforce_security() {
        let environment = BTreeMap::new();
        let targets = resolve_portal_targets(None, false, &environment).unwrap();
        let local = resolve_portal_targets(None, true, &environment).unwrap();

        assert_eq!(targets.portal_url, DEFAULT_PRODUCTION_PORTAL_URL);
        assert_eq!(targets.api_base_url, "https://publish.solanamobile.com/api");
        assert_eq!(local.api_base_url, "http://localhost:3333/api");
        assert!(
            resolve_portal_targets(Some("https://example.invalid/base/"), false, &environment)
                .unwrap()
                .api_base_url
                .ends_with("/base/api")
        );
        assert!(
            resolve_portal_targets(Some("http://example.invalid"), false, &environment).is_err()
        );

        assert!(resolve_portal_targets(Some("http://localhost:3333"), true, &environment).is_ok());
        assert!(resolve_portal_targets(Some("http://127.0.0.1:3333"), true, &environment).is_ok());
        assert!(resolve_portal_targets(Some("http://[::1]:3333"), true, &environment).is_ok());
        assert!(
            resolve_portal_targets(Some("https://example.invalid"), true, &environment).is_err()
        );

        let environment =
            BTreeMap::from([(PORTAL_URL_ENV.to_owned(), "https://env.invalid".to_owned())]);
        assert_eq!(
            resolve_portal_targets(None, false, &environment)
                .unwrap()
                .portal_url,
            "https://env.invalid"
        );
    }

    #[test]
    fn api_keys_resolve_from_env_or_stdin_without_logging_values() {
        let environment = BTreeMap::from([(DEFAULT_API_KEY_ENV.to_owned(), "secret".to_owned())]);
        assert_eq!(
            resolve_api_key(false, None, &environment, None, false),
            Ok("secret".into())
        );
        assert_eq!(
            resolve_api_key(true, None, &environment, Some(b"stdin-secret\n"), false),
            Ok("stdin-secret".into())
        );
        assert!(resolve_api_key(false, None, &BTreeMap::new(), None, false).is_err());
        assert!(resolve_api_key(true, None, &environment, None, true).is_err());
        assert!(resolve_api_key(true, None, &environment, Some(b"  "), false).is_err());
        let named = BTreeMap::from([("CUSTOM".to_owned(), "value".to_owned())]);
        assert_eq!(
            resolve_api_key(false, Some("CUSTOM"), &named, None, false),
            Ok("value".into())
        );
    }

    #[test]
    fn sources_and_summaries_preserve_caller_values() {
        let options = NewVersionOptions {
            apk_file: Some("C:\\builds\\app.apk".into()),
            ..Default::default()
        };
        assert_eq!(
            build_publication_source(&options).unwrap(),
            PublicationSource::LocalApk {
                path: "C:\\builds\\app.apk".into(),
                file_name: "app.apk".into(),
            }
        );
        let options = NewVersionOptions {
            apk_url: Some("https://example.invalid/builds/app.apk?token=1".into()),
            ..Default::default()
        };
        assert_eq!(
            build_publication_source(&options).unwrap(),
            PublicationSource::RemoteApk {
                url: "https://example.invalid/builds/app.apk?token=1".into(),
                file_name: Some("app.apk".into()),
            }
        );
        assert_eq!(file_name_of("/tmp/app.apk"), "app.apk");
        assert_eq!(infer_file_name_from_url("https://example.invalid"), None);
        assert_eq!(
            summary_lines(Some("mint"), Some("collection"), Some("ticket")),
            vec![
                "This app is now in review.".to_owned(),
                "Release mint address: mint".to_owned(),
                "Collection mint address: collection".to_owned(),
                "Ticket ID: ticket".to_owned(),
            ]
        );
    }
}
