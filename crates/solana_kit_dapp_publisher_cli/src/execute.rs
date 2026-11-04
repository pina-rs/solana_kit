//! Execution composition for the publisher CLI.

mod complete;
mod funding;

use crate::adapters::{
    FileSource, FileStorage, HttpUploader, PortalTransport, SystemClock, utc_now,
};
use complete::complete;
use funding::funding_preflight;
use solana_kit_client::rest::Budgets;
use solana_kit_core::crypto::Keypair;
use solana_kit_dapp_publisher::{
    cli::{
        NewVersionOptions, PortalTargets, PublicationSource, ResumeOptions,
        build_publication_source, summary_lines,
    },
    portal::{Client, CreateIngestionSession, IngestionSource},
    workflow::{
        HydratedPublication, Limits, LocalApk, Options, ResumeTarget, Stage, Storage, Workflow,
        prepare_local_apk,
    },
};
use std::{
    future::Future,
    path::{Path, PathBuf},
    pin::pin,
    task::{Context, Poll, Waker},
};

const SNAPSHOT_KEY: &str = "solana-kit-publisher";
const TOKEN_BYTES: usize = 16;
const REQUEST_ID_DIGITS: usize = 32;

const WORKFLOW_DEADLINE_MS: u64 = 120_000;
const MAX_POLLS: usize = 60;
const POLL_INTERVAL_MS: u64 = 2_000;
const SOURCE_BYTES: usize = 64 * 1024 * 1024;
const MEDIA_BYTES: usize = 16 * 1024 * 1024;
const MEDIA_ITEMS: usize = 32;

const RANK_PREPARED_FOR_MINT: u8 = 0;
const RANK_MINT_SUBMITTED: u8 = 1;
const RANK_MINT_SAVED: u8 = 2;
const RANK_VERIFICATION_SUBMITTED: u8 = 3;
const RANK_VERIFIED: u8 = 4;
const RANK_ATTESTED: u8 = 5;
const RANK_SUBMITTED: u8 = 6;

/// One successful publication summary.
pub struct Summary {
    pub release_mint: Option<String>,
    pub collection_mint: Option<String>,
    pub ticket: Option<String>,
}

impl Summary {
    /// Human-readable lines for the CLI.
    #[must_use]
    pub fn lines(&self) -> Vec<String> {
        summary_lines(
            self.release_mint.as_deref(),
            self.collection_mint.as_deref(),
            self.ticket.as_deref(),
        )
    }
}

struct Runtime<'a> {
    client: &'a Client<PortalTransport>,
    uploader: &'a HttpUploader,
    source_reader: &'a FileSource,
    clock: &'a SystemClock,
}

/// Runs a new publication from parsed options.
pub fn new_publication(
    options: &NewVersionOptions,
    targets: &PortalTargets,
    api_key: &str,
) -> Result<Summary, String> {
    let keypair = load_keypair(options.keypair.as_deref())?;

    funding_preflight(&keypair, options)?;

    let client = portal_client(targets, api_key)?;
    let uploader = HttpUploader::new();
    let source_reader = FileSource;
    let clock = SystemClock::new();
    let limits = limits()?;
    let mut storage = storage();
    let runtime = Runtime {
        client: &client,
        uploader: &uploader,
        source_reader: &source_reader,
        clock: &clock,
    };

    reject_existing_checkpoint(&mut storage)?;

    let source = prepare_source(options, &runtime, limits)?;
    let input = ingestion_input(options, source)?;
    let mut workflow = workflow(&client, &uploader, &mut storage, &clock, limits)?;
    let hydrated = block_on(workflow.start(&input))
        .map_err(|error| format!("publication start failed: {error:?}"))?;

    complete(
        &mut workflow,
        hydrated,
        &client,
        &keypair,
        options.whats_new.as_deref(),
    )
}

/// Resumes an acknowledged publication.
pub fn resume_publication(
    options: &ResumeOptions,
    targets: &PortalTargets,
    api_key: &str,
) -> Result<Summary, String> {
    let keypair = load_keypair(options.keypair.as_deref())?;
    let client = portal_client(targets, api_key)?;
    let uploader = HttpUploader::new();
    let clock = SystemClock::new();
    let limits = limits()?;
    let mut storage = storage();
    let mut workflow = workflow(&client, &uploader, &mut storage, &clock, limits)?;
    let target = resume_target(options)?;
    let hydrated = block_on(workflow.resume(target))
        .map_err(|error| format!("publication resume failed: {error:?}"))?;

    complete(&mut workflow, hydrated, &client, &keypair, None)
}

fn portal_client(
    targets: &PortalTargets,
    api_key: &str,
) -> Result<Client<PortalTransport>, String> {
    Ok(Client::new(
        PortalTransport::new(&targets.api_base_url, api_key)?,
        Budgets::default(),
    ))
}

fn workflow<'a>(
    client: &'a Client<PortalTransport>,
    uploader: &'a HttpUploader,
    storage: &'a mut FileStorage,
    clock: &'a SystemClock,
    limits: Limits,
) -> Result<Workflow<'a, PortalTransport, HttpUploader, FileStorage, SystemClock>, String> {
    Workflow::new(client, uploader, storage, clock, workflow_options(limits)?)
        .map_err(|error| format!("workflow configuration failed: {error:?}"))
}

fn reject_existing_checkpoint(storage: &mut FileStorage) -> Result<(), String> {
    let existing = block_on(storage.load(SNAPSHOT_KEY, 1024 * 1024))
        .map_err(|error| format!("checkpoint read failed: {error:?}"))?;

    if existing.is_some() {
        return Err("existing checkpoint requires resume".into());
    }

    Ok(())
}

fn prepare_source(
    options: &NewVersionOptions,
    runtime: &Runtime<'_>,
    limits: Limits,
) -> Result<IngestionSource, String> {
    let source = build_publication_source(options).map_err(|error| error.message().to_owned())?;

    match source {
        PublicationSource::LocalApk { path, file_name } => block_on(prepare_local_apk(
            runtime.client,
            runtime.uploader,
            runtime.source_reader,
            runtime.clock,
            &LocalApk {
                token: path,
                file_name,
                content_type: None,
            },
            limits,
        ))
        .map_err(|error| format!("local APK preparation failed: {error:?}")),
        PublicationSource::RemoteApk { url, file_name } => Ok(IngestionSource::ExternalUrl {
            apk_url: url,
            release_file_name: file_name,
        }),
    }
}

fn ingestion_input(
    options: &NewVersionOptions,
    source: IngestionSource,
) -> Result<CreateIngestionSession, String> {
    let explicit_key = options
        .idempotency_key
        .clone()
        .filter(|value| !value.trim().is_empty());
    let idempotency_key = match explicit_key {
        Some(key) => key,
        None => random_token(TOKEN_BYTES)?,
    };

    Ok(CreateIngestionSession {
        source,
        whats_new: options.whats_new.clone().unwrap_or_default(),
        idempotency_key,
        dapp_id: None,
    })
}

fn resume_target(options: &ResumeOptions) -> Result<ResumeTarget, String> {
    let release = options
        .release_id
        .as_deref()
        .or(options.resume_release.as_deref());
    let session = options
        .session_id
        .as_deref()
        .or(options.resume_session.as_deref());

    match (release, session) {
        (_, Some(session)) => Ok(ResumeTarget::Session(session.trim().to_owned())),
        (Some(release), None) => Ok(ResumeTarget::Release(release.trim().to_owned())),
        _ => Err("a release or session selector is required".into()),
    }
}

fn load_keypair(path: Option<&str>) -> Result<Keypair, String> {
    let path = path.ok_or("`--keypair` is required")?;
    let bytes = std::fs::read(path).map_err(|_| "failed to read the signer keypair")?;
    let values: Vec<u8> =
        serde_json::from_slice(&bytes).map_err(|_| "signer keypair is not a JSON byte array")?;
    Keypair::from_bytes(&values)
        .map_err(|_| "signer keypair is not a valid 64-byte keypair".to_owned())
}

fn limits() -> Result<Limits, String> {
    let budgets = Budgets::default();
    let limits = Limits {
        deadline: WORKFLOW_DEADLINE_MS,
        max_polls: MAX_POLLS,
        poll_interval_ms: POLL_INTERVAL_MS,
        source_bytes: SOURCE_BYTES,
        media_bytes: MEDIA_BYTES,
        media_items: MEDIA_ITEMS,
        snapshot: budgets,
        metadata: budgets,
    };
    Ok(limits)
}

fn workflow_options(limits: Limits) -> Result<Options, String> {
    Ok(Options {
        snapshot_key: SNAPSHOT_KEY.into(),
        updated_on: utc_now()?,
        limits,
    })
}

fn storage() -> FileStorage {
    FileStorage::new(PathBuf::from(".solana-kit-publisher/checkpoint.json"))
}

pub(super) fn mint_of(hydrated: &HydratedPublication) -> Option<String> {
    text(&hydrated.backend_session().expected_mint_address)
        .or_else(|| {
            option(&hydrated.backend_bundle().release)
                .and_then(|release| text(&release.nft_mint_address))
        })
        .map(str::to_owned)
}

pub(super) fn option<T>(value: &solana_kit_client::Presence<Option<T>>) -> Option<&T> {
    match value {
        solana_kit_client::Presence::Present(Some(value)) => Some(value),
        _ => None,
    }
}

pub(super) fn text(value: &solana_kit_client::Presence<Option<String>>) -> Option<&str> {
    option(value)
        .map(String::as_str)
        .filter(|value| !value.trim().is_empty())
}

pub(super) fn rank(stage: Stage) -> u8 {
    match stage {
        Stage::PreparedForMint => RANK_PREPARED_FOR_MINT,
        Stage::MintSubmitted => RANK_MINT_SUBMITTED,
        Stage::MintSaved => RANK_MINT_SAVED,
        Stage::VerificationSubmitted => RANK_VERIFICATION_SUBMITTED,
        Stage::Verified => RANK_VERIFIED,
        Stage::Attested => RANK_ATTESTED,
        Stage::Submitted => RANK_SUBMITTED,
        Stage::Failed => u8::MAX,
    }
}

fn random_token(length: usize) -> Result<String, String> {
    let bytes = random_bytes_with(length, getrandom::fill)?;

    Ok(bytes.iter().map(|byte| format!("{byte:02x}")).collect())
}

fn random_bytes_with(
    length: usize,
    fill: impl FnOnce(&mut [u8]) -> Result<(), getrandom::Error>,
) -> Result<Vec<u8>, String> {
    let mut bytes = vec![0; length];

    fill(&mut bytes).map_err(|_| "system entropy is unavailable".to_owned())?;

    Ok(bytes)
}

pub(super) fn random_digits(length: usize) -> Result<String, String> {
    let bytes = random_bytes_with(length, getrandom::fill)?;

    Ok(bytes
        .iter()
        .map(|byte| char::from(b'0' + (byte % 10)))
        .collect())
}

pub(super) fn block_on<F: Future>(future: F) -> F::Output {
    let mut future = pin!(future);
    let waker = Waker::noop();
    let mut context = Context::from_waker(waker);

    loop {
        if let Poll::Ready(value) = future.as_mut().poll(&mut context) {
            return value;
        }

        std::thread::yield_now();
    }
}

/// Absolute path exists check used by the CLI validator.
pub fn path_exists(path: &str) -> bool {
    Path::new(path).exists()
}

#[cfg(test)]
mod tests {
    use super::*;
    use solana_kit_core::crypto::ExportableKeypair;

    #[test]
    fn loads_a_solana_cli_keypair_json() {
        let key = ExportableKeypair::from_seed(&[7; 32]);
        let bytes = key.backup().expose_secret().to_vec();
        let path = std::env::temp_dir().join(format!(
            "solana-kit-publisher-keypair-{}.json",
            std::process::id()
        ));
        std::fs::write(&path, serde_json::to_vec(&bytes).unwrap()).unwrap();

        let loaded = load_keypair(path.to_str()).expect("valid keypair");
        assert_eq!(loaded.address(), key.signer().address());
        assert!(load_keypair(Some(path.to_str().unwrap())).is_ok());
        let _ = std::fs::remove_file(path);
    }

    #[test]
    fn rejects_short_and_non_array_keypairs() {
        let path = std::env::temp_dir().join(format!(
            "solana-kit-publisher-bad-keypair-{}.json",
            std::process::id()
        ));
        std::fs::write(&path, b"[1,2,3]").unwrap();
        assert!(load_keypair(path.to_str()).is_err());
        std::fs::write(&path, b"{\"secret\":true}").unwrap();
        assert!(load_keypair(path.to_str()).is_err());
        let _ = std::fs::remove_file(path);
    }

    #[test]
    fn utc_timestamp_is_the_workflow_accepted_form() {
        let value = utc_now().expect("valid time");
        assert!(value.ends_with('Z'));
        assert!(value.contains('T'));
    }

    #[test]
    fn random_outputs_have_the_expected_shape() {
        let token = random_token(8).expect("system entropy");
        let digits = random_digits(32).expect("system entropy");

        assert_eq!(token.len(), 16);
        assert!(token.bytes().all(|byte| byte.is_ascii_hexdigit()));
        assert_eq!(digits.len(), 32);
        assert!(digits.bytes().all(|byte| byte.is_ascii_digit()));
    }

    #[test]
    fn entropy_failure_never_produces_a_publication_identity() {
        let result = random_bytes_with(TOKEN_BYTES, |bytes| {
            bytes.fill(7);
            Err(getrandom::Error::UNSUPPORTED)
        });

        assert_eq!(result, Err("system entropy is unavailable".to_owned()));
    }

    #[test]
    fn stage_rank_orders_publication_progress() {
        assert!(rank(Stage::PreparedForMint) < rank(Stage::MintSubmitted));
        assert!(rank(Stage::MintSubmitted) < rank(Stage::MintSaved));
        assert!(rank(Stage::MintSaved) < rank(Stage::VerificationSubmitted));
        assert!(rank(Stage::VerificationSubmitted) < rank(Stage::Verified));
        assert!(rank(Stage::Verified) < rank(Stage::Attested));
        assert!(rank(Stage::Attested) < rank(Stage::Submitted));
        assert_eq!(rank(Stage::Failed), u8::MAX);
    }
}
