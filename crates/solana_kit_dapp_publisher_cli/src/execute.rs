//! Execution composition for the publisher CLI.

use crate::adapters::{
    FileSource, FileStorage, HttpUploader, PortalTransport, SystemClock, utc_now,
};
use base64::{Engine as _, engine::general_purpose::STANDARD};
use solana_kit_client::rest::Budgets;
use solana_kit_core::{Address, Hash, codecs::Limits as CodecLimits, crypto::Keypair};
use solana_kit_dapp_publisher::{
    AttestationMessage, ExpectedTransaction, FundingCheck, PublicationPolicy, RequestId,
    check_signer_balance,
    cli::{
        NewVersionOptions, PortalTargets, PublicationSource, ResumeOptions,
        build_publication_source, summary_lines,
    },
    format_sol_amount,
    portal::{
        Client, CreateIngestionSession, GetBlockData, IngestionSource, PreparedReleaseTransaction,
        PreparedVerifyCollectionTransaction, StoreAttestation,
    },
    resolve_funding_rpc_url, validate_prepared_transaction,
    workflow::{
        HydratedPublication, Limits, LocalApk, MintPreparation, MintSave, Options, ResumeTarget,
        Stage, Storage, StoreSubmission, TransactionPurpose, Workflow, prepare_local_apk,
    },
};
use std::{
    future::Future,
    path::{Path, PathBuf},
    pin::pin,
    task::{Context, Poll, Waker},
    time::Duration,
};

const SNAPSHOT_KEY: &str = "solana-kit-publisher";
const TRANSACTION_BYTES: usize = 4096;
const TRANSACTION_ITEMS: usize = 12_288;
const ATTESTATION_BYTES: usize = 4096;
const ATTESTATION_ITEMS: usize = 8192;
const STAGE_ATTEMPTS: usize = 30;

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

/// Runs a new publication from parsed options.
pub fn new_publication(
    options: &NewVersionOptions,
    targets: &PortalTargets,
    api_key: &str,
) -> Result<Summary, String> {
    let keypair = load_keypair(options.keypair.as_deref())?;
    funding_preflight(&keypair, options)?;
    let client = Client::new(
        PortalTransport::new(&targets.api_base_url, api_key)?,
        Budgets::default(),
    );
    let uploader = HttpUploader::new();
    let source_reader = FileSource;
    let clock = SystemClock::new();
    let limits = limits()?;
    let mut storage = storage();

    if block_on(storage.load(SNAPSHOT_KEY, 1024 * 1024))
        .map_err(|error| format!("checkpoint read failed: {error:?}"))?
        .is_some()
    {
        return Err("existing checkpoint requires resume".into());
    }

    let source =
        match build_publication_source(options).map_err(|error| error.message().to_owned())? {
            PublicationSource::LocalApk { path, file_name } => block_on(prepare_local_apk(
                &client,
                &uploader,
                &source_reader,
                &clock,
                &LocalApk {
                    token: path,
                    file_name,
                    content_type: None,
                },
                limits,
            ))
            .map_err(|error| format!("local APK preparation failed: {error:?}"))?,
            PublicationSource::RemoteApk { url, file_name } => IngestionSource::ExternalUrl {
                apk_url: url,
                release_file_name: file_name,
            },
        };
    let idempotency_key = options
        .idempotency_key
        .clone()
        .filter(|value| !value.trim().is_empty())
        .unwrap_or_else(|| random_token(16));
    let input = CreateIngestionSession {
        source,
        whats_new: options.whats_new.clone().unwrap_or_default(),
        idempotency_key,
        dapp_id: None,
    };
    let mut workflow = Workflow::new(
        &client,
        &uploader,
        &mut storage,
        &clock,
        workflow_options(limits)?,
    )
    .map_err(|error| format!("workflow configuration failed: {error:?}"))?;
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
    let client = Client::new(
        PortalTransport::new(&targets.api_base_url, api_key)?,
        Budgets::default(),
    );
    let uploader = HttpUploader::new();
    let clock = SystemClock::new();
    let limits = limits()?;
    let mut storage = storage();
    let mut workflow = Workflow::new(
        &client,
        &uploader,
        &mut storage,
        &clock,
        workflow_options(limits)?,
    )
    .map_err(|error| format!("workflow configuration failed: {error:?}"))?;
    let target = match (
        options
            .release_id
            .as_deref()
            .or(options.resume_release.as_deref()),
        options
            .session_id
            .as_deref()
            .or(options.resume_session.as_deref()),
    ) {
        (_, Some(session)) => ResumeTarget::Session(session.trim().to_owned()),
        (Some(release), None) => ResumeTarget::Release(release.trim().to_owned()),
        _ => return Err("a release or session selector is required".into()),
    };
    let hydrated = block_on(workflow.resume(target))
        .map_err(|error| format!("publication resume failed: {error:?}"))?;

    complete(&mut workflow, hydrated, &client, &keypair, None)
}

fn complete(
    workflow: &mut Workflow<'_, PortalTransport, HttpUploader, FileStorage, SystemClock>,
    mut hydrated: HydratedPublication,
    client: &Client<PortalTransport>,
    keypair: &Keypair,
    whats_new: Option<&str>,
) -> Result<Summary, String> {
    let (app_mint_text, publisher_text, payer_text, collection_authority_text) = {
        let bundle = hydrated.backend_bundle();
        let authority = option(&bundle.signer_authority)
            .ok_or("publication bundle lacks a signer authority")?;
        let app_mint = text(&authority.app_mint_address)
            .ok_or("publication bundle lacks an app mint")?
            .to_owned();
        let publisher = text(&authority.required_signer)
            .or_else(|| text(&authority.dapp_wallet_address))
            .ok_or("publication bundle lacks a publisher")?
            .to_owned();
        let payer = text(&authority.fee_payer).unwrap_or(&publisher).to_owned();
        let collection_authority = text(&authority.collection_authority).map(str::to_owned);
        (app_mint, publisher, payer, collection_authority)
    };
    let app_mint = app_mint_text
        .parse::<Address>()
        .map_err(|_| "publication bundle has an invalid app mint")?;
    let publisher = publisher_text
        .parse::<Address>()
        .map_err(|_| "publication bundle has an invalid publisher")?;
    let payer = payer_text
        .parse::<Address>()
        .map_err(|_| "publication bundle has an invalid payer")?;
    let (release_name, release_version, android_package, release_notes) = {
        let bundle = hydrated.backend_bundle();
        let release = option(&bundle.release).ok_or("publication bundle lacks a release")?;
        (
            option(&bundle.dapp)
                .and_then(|dapp| text(&dapp.dapp_name))
                .unwrap_or("Release update")
                .to_owned(),
            text(&release.version_name).unwrap_or("1.0").to_owned(),
            text(&release.android_package)
                .unwrap_or_default()
                .to_owned(),
            text(&release.new_in_version).map(str::to_owned),
        )
    };
    let mut release_mint = mint_of(&hydrated);
    let mut ticket = None;

    if rank(hydrated.stage()) < rank(Stage::MintSubmitted) {
        let prepared = block_on(workflow.prepare_release_mint(
            &hydrated,
            MintPreparation {
                release_name: &release_name,
                app_mint_address: &app_mint_text,
                publisher_address: &publisher_text,
                payer_address: &payer_text,
            },
        ))
        .map_err(|error| format!("release mint preparation failed: {error:?}"))?;
        release_mint = Some(prepared.mint_address.clone());
        let signed = sign_release_mint(&prepared, keypair, payer, publisher, app_mint)?;
        block_on(workflow.submit_signed_transaction(
            &hydrated,
            TransactionPurpose::ReleaseMint,
            &signed,
        ))
        .map_err(|error| format!("release mint submission failed: {error:?}"))?;
        hydrated = wait_for(workflow, Stage::MintSubmitted)?;
    }

    if rank(hydrated.stage()) < rank(Stage::MintSaved) {
        let mint = release_mint
            .as_deref()
            .ok_or("publication did not resolve a release mint")?;
        block_on(
            workflow.save_release_nft_data(
                &hydrated,
                MintSave {
                    mint_address: mint,
                    transaction_signature: text(
                        &hydrated.backend_session().mint_transaction_signature,
                    )
                    .ok_or("publication session lacks the mint signature")?,
                    owner_address: &publisher_text,
                    release_name: &release_name,
                    release_version: &release_version,
                    android_package: &android_package,
                    app_mint_address: &app_mint_text,
                },
            ),
        )
        .map_err(|error| format!("release data save failed: {error:?}"))?;
        hydrated = wait_for(workflow, Stage::MintSaved)?;
    }

    if rank(hydrated.stage()) < rank(Stage::VerificationSubmitted) {
        let prepared = block_on(workflow.prepare_collection_verification(&hydrated, &payer_text))
            .map_err(|error| {
            format!("collection verification preparation failed: {error:?}")
        })?;
        let mint = release_mint
            .as_deref()
            .ok_or("publication did not resolve a release mint")?
            .parse::<Address>()
            .map_err(|_| "publication has an invalid release mint")?;
        let authority_address = collection_authority_text
            .as_deref()
            .ok_or("publication lacks a collection authority")?
            .parse::<Address>()
            .map_err(|_| "publication has an invalid collection authority")?;
        let signed = sign_verification(
            &prepared,
            keypair,
            payer,
            publisher,
            mint,
            app_mint,
            authority_address,
        )?;
        block_on(workflow.submit_signed_transaction(
            &hydrated,
            TransactionPurpose::CollectionVerification,
            &signed,
        ))
        .map_err(|error| format!("collection verification submission failed: {error:?}"))?;
        hydrated = wait_for(workflow, Stage::VerificationSubmitted)?;
    }

    if rank(hydrated.stage()) < rank(Stage::Verified) {
        block_on(workflow.mark_collection_verified(&hydrated)).map_err(|error| {
            format!("collection verification acknowledgement failed: {error:?}")
        })?;
        hydrated = wait_for(workflow, Stage::Verified)?;
    }

    if rank(hydrated.stage()) < rank(Stage::Submitted) {
        let attestation = create_attestation(client, keypair)?;
        let result = block_on(
            workflow.submit_to_store(
                &hydrated,
                StoreSubmission {
                    whats_new: whats_new
                        .or(release_notes.as_deref())
                        .unwrap_or("New version"),
                    critical_update: false,
                    testing_instructions: None,
                    is_resubmission: None,
                    attestation,
                },
            ),
        )
        .map_err(|error| format!("store submission failed: {error:?}"))?;
        ticket = match result.hubspot_ticket_id {
            solana_kit_client::Presence::Present(Some(value)) => Some(value),
            _ => None,
        };
    }

    Ok(Summary {
        release_mint,
        collection_mint: Some(app_mint.to_string()),
        ticket,
    })
}

fn wait_for(
    workflow: &mut Workflow<'_, PortalTransport, HttpUploader, FileStorage, SystemClock>,
    minimum: Stage,
) -> Result<HydratedPublication, String> {
    for _ in 0..STAGE_ATTEMPTS {
        let hydrated = block_on(workflow.resume(ResumeTarget::Snapshot))
            .map_err(|error| format!("publication refresh failed: {error:?}"))?;

        if hydrated.stage() == Stage::Failed {
            return Err("publication reached the failed stage".into());
        }

        if rank(hydrated.stage()) >= rank(minimum) {
            return Ok(hydrated);
        }

        std::thread::sleep(Duration::from_secs(2));
    }

    Err("publication did not advance to the next stage".into())
}

fn sign_release_mint(
    prepared: &PreparedReleaseTransaction,
    keypair: &Keypair,
    payer: Address,
    publisher: Address,
    collection: Address,
) -> Result<String, String> {
    let blockhash = prepared
        .blockhash
        .parse::<Hash>()
        .map_err(|_| "prepared mint has an invalid blockhash")?;
    let mint = prepared
        .mint_address
        .parse::<Address>()
        .map_err(|_| "prepared mint has an invalid address")?;
    sign_prepared(
        &prepared.transaction,
        PublicationPolicy::ReleaseMint {
            expected: ExpectedTransaction {
                blockhash,
                payer,
                signer: publisher,
            },
            mint,
            collection,
        },
        keypair,
    )
}

fn sign_verification(
    prepared: &PreparedVerifyCollectionTransaction,
    keypair: &Keypair,
    payer: Address,
    publisher: Address,
    nft_mint: Address,
    collection: Address,
    authority: Address,
) -> Result<String, String> {
    let blockhash = prepared
        .blockhash
        .parse::<Hash>()
        .map_err(|_| "prepared verification has an invalid blockhash")?;
    sign_prepared(
        &prepared.transaction,
        PublicationPolicy::VerifyCollection {
            expected: ExpectedTransaction {
                blockhash,
                payer,
                signer: publisher,
            },
            nft_mint,
            collection,
            authority,
        },
        keypair,
    )
}

fn sign_prepared(
    prepared: &str,
    policy: PublicationPolicy,
    keypair: &Keypair,
) -> Result<String, String> {
    let bytes = STANDARD
        .decode(prepared)
        .map_err(|_| "prepared transaction is not valid base64")?;
    let sealed = validate_prepared_transaction(
        &bytes,
        policy,
        CodecLimits::new(TRANSACTION_BYTES, TRANSACTION_ITEMS),
    )
    .map_err(|error| format!("prepared transaction policy failed: {error:?}"))?;
    let signature = {
        let request = sealed.signing_request();
        keypair.sign(request.message)
    };
    let signed = sealed
        .attach_signature(signature)
        .map_err(|error| format!("prepared transaction signing failed: {error:?}"))?;
    let wire = signed
        .encode(true)
        .map_err(|error| format!("signed transaction encoding failed: {error:?}"))?;

    Ok(STANDARD.encode(wire))
}

fn create_attestation(
    client: &Client<PortalTransport>,
    keypair: &Keypair,
) -> Result<StoreAttestation, String> {
    let block = block_on(client.call(&GetBlockData {}))
        .map_err(|error| format!("attestation block data failed: {error:?}"))?;
    let blockhash = block
        .blockhash
        .parse::<Hash>()
        .map_err(|_| "attestation blockhash is invalid")?;
    let request_id = random_digits(32);
    let request =
        RequestId::parse(&request_id).map_err(|_| "attestation request identity is invalid")?;
    let message = AttestationMessage::new(
        block.slot,
        blockhash,
        request,
        CodecLimits::new(ATTESTATION_BYTES, ATTESTATION_ITEMS),
    )
    .map_err(|error| format!("attestation message failed: {error:?}"))?;
    let signature = keypair.sign(message.bytes());
    let payload = message
        .attach_signature(keypair.address(), signature)
        .map_err(|error| format!("attestation signing failed: {error:?}"))?;

    Ok(StoreAttestation {
        payload: payload.payload().to_owned(),
        request_unique_id: request_id,
    })
}

fn funding_preflight(keypair: &Keypair, options: &NewVersionOptions) -> Result<(), String> {
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
                eprintln!("Warning: unable to validate the signer address for funding preflight.");
                Ok(())
            }
        },
        Err(_) => {
            eprintln!(
                "Warning: unable to confirm the signer balance; continuing without a SOL preflight check."
            );
            Ok(())
        }
    }
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
        .timeout_global(Some(Duration::from_secs(15)))
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
        deadline: 120_000,
        max_polls: 60,
        poll_interval_ms: 2_000,
        source_bytes: 64 * 1024 * 1024,
        media_bytes: 16 * 1024 * 1024,
        media_items: 32,
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

fn mint_of(hydrated: &HydratedPublication) -> Option<String> {
    text(&hydrated.backend_session().expected_mint_address)
        .or_else(|| {
            option(&hydrated.backend_bundle().release)
                .and_then(|release| text(&release.nft_mint_address))
        })
        .map(str::to_owned)
}

fn option<T>(value: &solana_kit_client::Presence<Option<T>>) -> Option<&T> {
    match value {
        solana_kit_client::Presence::Present(Some(value)) => Some(value),
        _ => None,
    }
}

fn text(value: &solana_kit_client::Presence<Option<String>>) -> Option<&str> {
    option(value)
        .map(String::as_str)
        .filter(|value| !value.trim().is_empty())
}

fn rank(stage: Stage) -> u8 {
    match stage {
        Stage::PreparedForMint => 0,
        Stage::MintSubmitted => 1,
        Stage::MintSaved => 2,
        Stage::VerificationSubmitted => 3,
        Stage::Verified => 4,
        Stage::Attested => 5,
        Stage::Submitted => 6,
        Stage::Failed => u8::MAX,
    }
}

fn random_token(length: usize) -> String {
    let mut bytes = vec![0; length];
    if getrandom::fill(&mut bytes).is_err() {
        return "0000000000000000".into();
    }
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

fn random_digits(length: usize) -> String {
    let mut bytes = vec![0; length];
    if getrandom::fill(&mut bytes).is_err() {
        return "0".repeat(length);
    }
    bytes
        .iter()
        .map(|byte| char::from(b'0' + (byte % 10)))
        .collect()
}

fn block_on<F: Future>(future: F) -> F::Output {
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
        assert_eq!(random_token(8).len(), 16);
        assert!(random_token(8).bytes().all(|byte| byte.is_ascii_hexdigit()));
        assert_eq!(random_digits(32).len(), 32);
        assert!(random_digits(32).bytes().all(|byte| byte.is_ascii_digit()));
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
