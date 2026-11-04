//! Caller-driven mint, verification, attestation and store completion stages.

use super::{
    FileStorage, HttpUploader, PortalTransport, SystemClock, block_on, mint_of, option,
    random_digits, rank, text,
};
use base64::{Engine as _, engine::general_purpose::STANDARD};
use solana_kit_core::{Address, Hash, codecs::Limits as CodecLimits, crypto::Keypair};
use solana_kit_dapp_publisher::{
    AttestationMessage, ExpectedTransaction, PublicationPolicy, RequestId,
    portal::{
        Client, GetBlockData, PreparedReleaseTransaction, PreparedVerifyCollectionTransaction,
        StoreAttestation,
    },
    validate_prepared_transaction,
    workflow::{
        HydratedPublication, MintPreparation, MintSave, ResumeTarget, Stage, StoreSubmission,
        TransactionPurpose, Workflow,
    },
};
use std::{string::String, time::Duration};

const TRANSACTION_BYTES: usize = 4096;
const TRANSACTION_ITEMS: usize = 12_288;
const ATTESTATION_BYTES: usize = 4096;
const ATTESTATION_ITEMS: usize = 8192;
const STAGE_ATTEMPTS: usize = 30;
const STAGE_POLL_SECONDS: u64 = 2;

struct Authorities {
    app_mint_text: String,
    publisher_text: String,
    payer_text: String,
    app_mint: Address,
    publisher: Address,
    payer: Address,
    collection_authority: Option<String>,
}

struct ReleaseDetails {
    name: String,
    version: String,
    android_package: String,
    notes: Option<String>,
}

/// Advances one hydrated publication through every unacknowledged stage.
pub(super) fn complete(
    workflow: &mut Workflow<'_, PortalTransport, HttpUploader, FileStorage, SystemClock>,
    mut hydrated: HydratedPublication,
    client: &Client<PortalTransport>,
    keypair: &Keypair,
    whats_new: Option<&str>,
) -> Result<super::Summary, String> {
    let authorities = authorities(&hydrated)?;
    let details = release_details(&hydrated)?;
    let release_mint = advance_release(workflow, &mut hydrated, &details, &authorities, keypair)?;
    let ticket = advance_store(
        workflow,
        &hydrated,
        client,
        keypair,
        whats_new,
        details.notes.as_deref(),
    )?;

    Ok(super::Summary {
        release_mint,
        collection_mint: Some(authorities.app_mint.to_string()),
        ticket,
    })
}

fn advance_release(
    workflow: &mut Workflow<'_, PortalTransport, HttpUploader, FileStorage, SystemClock>,
    hydrated: &mut HydratedPublication,
    details: &ReleaseDetails,
    authorities: &Authorities,
    keypair: &Keypair,
) -> Result<Option<String>, String> {
    let mut release_mint = mint_of(hydrated);

    if rank(hydrated.stage()) < rank(Stage::MintSubmitted) {
        let (next, mint) = submit_release_mint(workflow, hydrated, details, authorities, keypair)?;
        *hydrated = next;
        release_mint = Some(mint);
    }

    if rank(hydrated.stage()) < rank(Stage::MintSaved) {
        *hydrated = save_release_data(
            workflow,
            hydrated,
            details,
            authorities,
            release_mint.as_deref(),
        )?;
    }

    if rank(hydrated.stage()) < rank(Stage::VerificationSubmitted) {
        *hydrated = verify_collection(
            workflow,
            hydrated,
            authorities,
            keypair,
            release_mint.as_deref(),
        )?;
    }

    if rank(hydrated.stage()) < rank(Stage::Verified) {
        *hydrated = mark_verified(workflow, hydrated)?;
    }

    Ok(release_mint)
}

fn advance_store(
    workflow: &mut Workflow<'_, PortalTransport, HttpUploader, FileStorage, SystemClock>,
    hydrated: &HydratedPublication,
    client: &Client<PortalTransport>,
    keypair: &Keypair,
    whats_new: Option<&str>,
    release_notes: Option<&str>,
) -> Result<Option<String>, String> {
    if rank(hydrated.stage()) >= rank(Stage::Submitted) {
        return Ok(None);
    }

    submit_store(
        workflow,
        hydrated,
        client,
        keypair,
        whats_new,
        release_notes,
    )
}

fn authorities(hydrated: &HydratedPublication) -> Result<Authorities, String> {
    let bundle = hydrated.backend_bundle();
    let authority =
        option(&bundle.signer_authority).ok_or("publication bundle lacks a signer authority")?;

    let app_mint_text = text(&authority.app_mint_address)
        .ok_or("publication bundle lacks an app mint")?
        .to_owned();
    let publisher_text = text(&authority.required_signer)
        .or_else(|| text(&authority.dapp_wallet_address))
        .ok_or("publication bundle lacks a publisher")?
        .to_owned();
    let payer_text = text(&authority.fee_payer)
        .unwrap_or(&publisher_text)
        .to_owned();

    let collection_authority = text(&authority.collection_authority).map(str::to_owned);
    let app_mint = app_mint_text
        .parse::<Address>()
        .map_err(|_| "publication bundle has an invalid app mint")?;
    let publisher = publisher_text
        .parse::<Address>()
        .map_err(|_| "publication bundle has an invalid publisher")?;
    let payer = payer_text
        .parse::<Address>()
        .map_err(|_| "publication bundle has an invalid payer")?;

    Ok(Authorities {
        app_mint_text,
        publisher_text,
        payer_text,
        app_mint,
        publisher,
        payer,
        collection_authority,
    })
}

fn release_details(hydrated: &HydratedPublication) -> Result<ReleaseDetails, String> {
    let bundle = hydrated.backend_bundle();
    let release = option(&bundle.release).ok_or("publication bundle lacks a release")?;

    Ok(ReleaseDetails {
        name: option(&bundle.dapp)
            .and_then(|dapp| text(&dapp.dapp_name))
            .unwrap_or("Release update")
            .to_owned(),
        version: text(&release.version_name).unwrap_or("1.0").to_owned(),
        android_package: text(&release.android_package)
            .unwrap_or_default()
            .to_owned(),
        notes: text(&release.new_in_version).map(str::to_owned),
    })
}

fn submit_release_mint(
    workflow: &mut Workflow<'_, PortalTransport, HttpUploader, FileStorage, SystemClock>,
    hydrated: &HydratedPublication,
    details: &ReleaseDetails,
    authorities: &Authorities,
    keypair: &Keypair,
) -> Result<(HydratedPublication, String), String> {
    let prepared = block_on(workflow.prepare_release_mint(
        hydrated,
        MintPreparation {
            release_name: &details.name,
            app_mint_address: &authorities.app_mint_text,
            publisher_address: &authorities.publisher_text,
            payer_address: &authorities.payer_text,
        },
    ))
    .map_err(|error| format!("release mint preparation failed: {error:?}"))?;
    let mint = prepared.mint_address.clone();
    let signed = sign_release_mint(
        &prepared,
        keypair,
        authorities.payer,
        authorities.publisher,
        authorities.app_mint,
    )?;

    block_on(workflow.submit_signed_transaction(
        hydrated,
        TransactionPurpose::ReleaseMint,
        &signed,
    ))
    .map_err(|error| format!("release mint submission failed: {error:?}"))?;

    Ok((wait_for(workflow, Stage::MintSubmitted)?, mint))
}

fn save_release_data(
    workflow: &mut Workflow<'_, PortalTransport, HttpUploader, FileStorage, SystemClock>,
    hydrated: &HydratedPublication,
    details: &ReleaseDetails,
    authorities: &Authorities,
    release_mint: Option<&str>,
) -> Result<HydratedPublication, String> {
    let mint = required_mint(release_mint)?;
    let signature = text(&hydrated.backend_session().mint_transaction_signature)
        .ok_or("publication session lacks the mint signature")?;

    block_on(workflow.save_release_nft_data(
        hydrated,
        MintSave {
            mint_address: mint,
            transaction_signature: signature,
            owner_address: &authorities.publisher_text,
            release_name: &details.name,
            release_version: &details.version,
            android_package: &details.android_package,
            app_mint_address: &authorities.app_mint_text,
        },
    ))
    .map_err(|error| format!("release data save failed: {error:?}"))?;

    wait_for(workflow, Stage::MintSaved)
}

fn verify_collection(
    workflow: &mut Workflow<'_, PortalTransport, HttpUploader, FileStorage, SystemClock>,
    hydrated: &HydratedPublication,
    authorities: &Authorities,
    keypair: &Keypair,
    release_mint: Option<&str>,
) -> Result<HydratedPublication, String> {
    let prepared =
        block_on(workflow.prepare_collection_verification(hydrated, &authorities.payer_text))
            .map_err(|error| format!("collection verification preparation failed: {error:?}"))?;
    let mint = required_mint(release_mint)?
        .parse::<Address>()
        .map_err(|_| "publication has an invalid release mint")?;
    let authority = authorities
        .collection_authority
        .as_deref()
        .ok_or("publication lacks a collection authority")?
        .parse::<Address>()
        .map_err(|_| "publication has an invalid collection authority")?;
    let signed = sign_verification(
        &prepared,
        keypair,
        authorities.payer,
        authorities.publisher,
        mint,
        authorities.app_mint,
        authority,
    )?;

    block_on(workflow.submit_signed_transaction(
        hydrated,
        TransactionPurpose::CollectionVerification,
        &signed,
    ))
    .map_err(|error| format!("collection verification submission failed: {error:?}"))?;

    wait_for(workflow, Stage::VerificationSubmitted)
}

fn mark_verified(
    workflow: &mut Workflow<'_, PortalTransport, HttpUploader, FileStorage, SystemClock>,
    hydrated: &HydratedPublication,
) -> Result<HydratedPublication, String> {
    block_on(workflow.mark_collection_verified(hydrated))
        .map_err(|error| format!("collection verification acknowledgement failed: {error:?}"))?;

    wait_for(workflow, Stage::Verified)
}

fn submit_store(
    workflow: &mut Workflow<'_, PortalTransport, HttpUploader, FileStorage, SystemClock>,
    hydrated: &HydratedPublication,
    client: &Client<PortalTransport>,
    keypair: &Keypair,
    whats_new: Option<&str>,
    release_notes: Option<&str>,
) -> Result<Option<String>, String> {
    let attestation = create_attestation(client, keypair)?;
    let result = block_on(workflow.submit_to_store(
        hydrated,
        StoreSubmission {
            whats_new: whats_new.or(release_notes).unwrap_or("New version"),
            critical_update: false,
            testing_instructions: None,
            is_resubmission: None,
            attestation,
        },
    ))
    .map_err(|error| format!("store submission failed: {error:?}"))?;

    Ok(match result.hubspot_ticket_id {
        solana_kit_client::Presence::Present(Some(value)) => Some(value),
        _ => None,
    })
}

fn required_mint(release_mint: Option<&str>) -> Result<&str, String> {
    release_mint.ok_or_else(|| "publication did not resolve a release mint".into())
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

        std::thread::sleep(Duration::from_secs(STAGE_POLL_SECONDS));
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
    let request_id = random_digits(super::REQUEST_ID_DIGITS)?;
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
