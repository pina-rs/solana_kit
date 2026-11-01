//! Every route has an independent expected verb, procedure and input object.

use super::checks::{Outcome, require};
use super::support::*;
use serde_json::{Value, json};
use solana_kit_client::rest;
use solana_kit_dapp_publisher::portal::*;
use std::vec;

pub(super) fn check<O: Operation>(
    operation: O,
    query: bool,
    procedure: &str,
    input: Value,
    payload: &str,
) -> Result<O::Response, &'static str> {
    let (client, state) = client(vec![success(payload)]);
    let result =
        complete(client.call(&operation))?.map_err(|_| "valid portal route response rejected")?;
    let requests = state.requests.borrow();
    require(requests.len() == 1, "portal route exchange count changed")?;
    let request = requests.first().ok_or("portal route exchange missing")?;
    require(
        request.path_segments == ["trpc", procedure],
        "portal procedure/path mismatch",
    )?;
    require(
        request.max_reply_bytes == BUDGETS.bytes,
        "portal response cap changed",
    )?;

    if query {
        check_query(request, input)?;
    } else {
        check_mutation(request, input)?;
    }

    Ok(result)
}

fn check_query(request: &rest::Request<PortalService>, input: Value) -> Outcome {
    require(
        request.method == rest::Method::Get,
        "portal query verb changed",
    )?;
    require(request.query.len() == 1, "portal query pair count changed")?;
    let pair = request.query.first().ok_or("portal input query missing")?;
    require(pair.0 == "input", "portal input query name changed")?;
    require(
        serde_json::from_str::<Value>(&pair.1).map_err(|_| "portal query JSON rejected")? == input,
        "portal query input changed",
    )?;
    require(
        request.body.as_deref().is_none(),
        "portal GET unexpectedly has a body",
    )
}

fn check_mutation(request: &rest::Request<PortalService>, input: Value) -> Outcome {
    require(
        request.method == rest::Method::Post,
        "portal mutation verb changed",
    )?;
    require(
        request.query.is_empty(),
        "portal POST unexpectedly has a query",
    )?;
    require(
        request.body.content_type() == Some("application/json"),
        "portal JSON content type changed",
    )?;

    let body = request
        .body
        .as_deref()
        .ok_or("portal mutation body missing")?;
    require(
        serde_json::from_slice::<Value>(body).map_err(|_| "portal body JSON rejected")? == input,
        "portal mutation input changed",
    )
}

#[cfg_attr(test, test)]
pub fn all_four_query_routes_keep_json_in_query_data_and_never_hydrate_other_resources() -> Outcome
{
    let block = check(
        GetBlockData {},
        true,
        "attestation.getBlockData",
        json!({}),
        r#"{"slot":18446744073709551615,"blockhash":"untrusted"}"#,
    )?;
    require(
        block.slot == u64::MAX,
        "full-width portal block slot changed",
    )?;
    let request = GetIngestionSession {
        session_id: "a/b?c#d".into(),
    };
    check(
        request,
        true,
        "publication.getIngestionSession",
        json!({"sessionId":"a/b?c#d"}),
        r#"{"id":"ingestion"}"#,
    )?;
    let request = GetPublicationSession {
        publication_session_id: Some("session".into()),
        release_id: None,
    };
    check(
        request,
        true,
        "publication.getPublicationSession",
        json!({"publicationSessionId":"session","releaseId":null}),
        r#"{"stage":"future-stage"}"#,
    )?;
    check(
        GetPublicationBundle {
            release_id: "release".into(),
        },
        true,
        "publication.getPublicationBundle",
        json!({"releaseId":"release"}),
        r#"{"releaseId":"release"}"#,
    )?;
    Ok(())
}

#[cfg_attr(test, test)]
pub fn upload_ingestion_cleanup_and_store_routes_have_exact_source_wire_forms() -> Outcome {
    check(
        CreateUploadTarget {
            file_hash: "hash".into(),
            file_extension: "apk".into(),
            content_type: "application/vnd.android.package-archive".into(),
        },
        false,
        "publication.createUploadTarget",
        json!({"fileHash":"hash","fileExtension":"apk","contentType":"application/vnd.android.package-archive"}),
        r#"{"uploadUrl":"https://upload.invalid/?signature=synthetic","publicUrl":"https://public.invalid/file.apk"}"#,
    )?;
    check(
        ingestion(),
        false,
        "publication.createIngestionSession",
        json!({
            "source": {
                "kind":"portalUpload", "releaseFileUrl":"https://public.invalid/file.apk",
                "releaseFileName":"file.apk", "releaseFileSize":123
            },
            "whatsNew":"fixture", "idempotencyKey":"same-key"
        }),
        r#"{"id":"ingestion"}"#,
    )?;
    check(
        CleanupRelease {
            release_id: "release".into(),
        },
        false,
        "publication.cleanupRelease",
        json!({"releaseId":"release"}),
        r#"{"action":"preservedSubmitted"}"#,
    )?;
    let request = SubmitToStore {
        release_id: "release".into(),
        whats_new: "fixture".into(),
        critical_update: false,
        testing_instructions: None,
        is_resubmission: Some(false),
        attestation: StoreAttestation {
            payload: "synthetic-payload".into(),
            request_unique_id: "request".into(),
        },
    };
    check(
        request,
        false,
        "publication.submitToStore",
        json!({
            "releaseId":"release", "whatsNew":"fixture", "criticalUpdate":false,
            "isResubmission":false,
            "attestation": {"payload":"synthetic-payload","requestUniqueId":"request"}
        }),
        r#"{"hubspotTicketId":null}"#,
    )?;
    Ok(())
}

pub(super) fn ingestion() -> CreateIngestionSession {
    CreateIngestionSession {
        source: IngestionSource::PortalUpload {
            release_file_url: "https://public.invalid/file.apk".into(),
            release_file_name: "file.apk".into(),
            release_file_size: 123,
            release_file_hash: None,
        },
        whats_new: "fixture".into(),
        idempotency_key: "same-key".into(),
        dapp_id: None,
    }
}

#[cfg_attr(test, test)]
pub fn transaction_mutations_require_explicit_calls_and_return_untrusted_proposals() -> Outcome {
    let request = PrepareReleaseNftTransaction {
        release_id: "release".into(),
        release_name: "Fixture".into(),
        release_metadata_uri: "https://public.invalid/meta".into(),
        app_mint_address: "collection".into(),
        publisher_address: "publisher".into(),
        payer_address: "payer".into(),
    };
    let result = check(
        request,
        false,
        "publication.prepareReleaseNftTransaction",
        json!({
            "releaseId":"release", "releaseName":"Fixture",
            "releaseMetadataUri":"https://public.invalid/meta", "appMintAddress":"collection",
            "publisherAddress":"publisher", "payerAddress":"payer"
        }),
        r#"{"transaction":"not-yet-decoded","mintAddress":"proposal","blockhash":"proposal"}"#,
    )?;
    require(
        result.transaction == "not-yet-decoded",
        "prepared transaction proposal changed",
    )?;

    let request = PrepareVerifyCollectionTransaction {
        dapp_id: "dapp".into(),
        nft_mint_address: "nft".into(),
        collection_mint_address: "collection".into(),
        collection_authority: "authority".into(),
        payer_address: "payer".into(),
    };
    check(
        request,
        false,
        "publication.prepareVerifyCollectionTransaction",
        json!({"dappId":"dapp","nftMintAddress":"nft","collectionMintAddress":"collection","collectionAuthority":"authority","payerAddress":"payer"}),
        r#"{"transaction":"proposal","blockhash":"proposal"}"#,
    )?;
    let request = SubmitSignedTransaction {
        signed_transaction: "explicitly-selected-wire".into(),
        publication_session_id: None,
    };
    check(
        request,
        false,
        "publication.submitSignedTransaction",
        json!({"signedTransaction":"explicitly-selected-wire","publicationSessionId":null}),
        r#"{"transactionSignature":"unconfirmed"}"#,
    )?;
    check(
        MarkReleaseCollectionAsVerified {
            release_id: "release".into(),
        },
        false,
        "publication.markReleaseCollectionAsVerified",
        json!({"releaseId":"release"}),
        r#"{"success":false,"releaseId":"release"}"#,
    )?;
    save_route()
}

fn save_route() -> Outcome {
    let request = SaveReleaseNftData {
        release_id: "r".into(),
        mint_address: "m".into(),
        transaction_signature: "s".into(),
        metadata_uri: "u".into(),
        owner_address: "o".into(),
        release_name: "n".into(),
        release_version: "v".into(),
        android_package: "package".into(),
        app_mint_address: "a".into(),
    };
    check(
        request,
        false,
        "publication.saveReleaseNftData",
        json!({
            "releaseId":"r", "mintAddress":"m", "transactionSignature":"s",
            "metadataUri":"u", "ownerAddress":"o", "releaseName":"n",
            "releaseVersion":"v", "androidPackage":"package", "appMintAddress":"a"
        }),
        r#"{"success":false}"#,
    )?;
    Ok(())
}

#[cfg_attr(test, test)]
pub fn remote_source_variants_omit_only_the_source_optional_fields() -> Outcome {
    for (source, expected) in [
        (
            IngestionSource::ExternalUrl {
                apk_url: "https://apk.invalid/a".into(),
                release_file_name: None,
            },
            json!({"kind":"externalUrl","apkUrl":"https://apk.invalid/a"}),
        ),
        (
            IngestionSource::ExistingRelease {
                source_release_id: "old".into(),
            },
            json!({"kind":"existingRelease","sourceReleaseId":"old"}),
        ),
    ] {
        let request = CreateIngestionSession {
            source,
            whats_new: "fixture".into(),
            idempotency_key: "same-key".into(),
            dapp_id: Some("dapp".into()),
        };
        check(
            request,
            false,
            "publication.createIngestionSession",
            json!({"source":expected,"whatsNew":"fixture","idempotencyKey":"same-key","dappId":"dapp"}),
            r#"{}"#,
        )?;
    }
    Ok(())
}
