//! Independently authored wire fixtures and the system-SHA-256 digest of a ten-byte GIF header.

use super::support::{Step, json};
use serde_json::{Value, json};
use solana_kit_dapp_publisher::portal::{CreateIngestionSession, IngestionSource};
use std::{string::ToString, vec, vec::Vec};

pub const GIF: &[u8] = b"GIF89a\x02\x00\x03\x00";
pub const GIF_HASH: &str = "a39ba6aebc26432b0b646b0ca5b569fa7c114063f8afaf052744f1554d28cca8";
pub const ABC_HASH: &str = "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad";
pub const ADDRESS: &str = "11111111111111111111111111111111";
pub const ICON: &str = "https://external.invalid/icon.gif";
pub const PUBLIC_ICON: &str = "https://public.invalid/icon.gif";
pub const METADATA: &str = "https://public.invalid/metadata.json";

const MIN_SDK: u64 = 21;
const TARGET_SDK: u64 = 35;

pub fn input() -> CreateIngestionSession {
    CreateIngestionSession {
        source: IngestionSource::ExternalUrl {
            apk_url: "https://apk.invalid/app.apk".into(),
            release_file_name: None,
        },
        whats_new: "New version".into(),
        idempotency_key: "same-idempotency-key".into(),
        dapp_id: None,
    }
}

pub fn session(metadata: Option<&str>, stage: &str) -> Step {
    let mut session =
        json!({"id":"pub","ingestionSessionId":"ing","releaseId":"rel","stage":stage});

    if let Some(uri) = metadata {
        session["metadataUri"] = uri.into();
    }

    json(&session.to_string())
}

pub fn bundle(icon: &str) -> Value {
    json!({
        "releaseId":"rel", "ingestionSessionId":"ing", "publicationSessionId":"pub",
        "dapp": {"id":"dapp","dappName":"Fixture", "description":"Long fixture", "subtitle":"Short fixture", "androidPackage":"test.fixture",
            "dappIconUrl":icon, "walletAddress":ADDRESS, "nftMintAddress":ADDRESS, "languages":["en-US"]},
        "publisher": {
            "name":"Synthetic Publisher", "website":"publisher.invalid/",
            "email":"contact@publisher.invalid", "supportEmail":"support@publisher.invalid"
        },
        "signerAuthority": {"dappWalletAddress":ADDRESS,"collectionAuthority":ADDRESS,"appMintAddress":ADDRESS,"acceptedSignerRoles":["publisher","payer"]},
        "installFile": {"uri":"https://public.invalid/app.apk","mimeType":"application/vnd.android.package-archive","size":3,"sha256":ABC_HASH},
        "release": {"id":"rel","androidPackage":"test.fixture","versionName":"2.0","versionCode":9007199254740993u64,
            "minSdkVersion":MIN_SDK,"targetSdkVersion":TARGET_SDK,"certificateFingerprint":"synthetic-fingerprint",
            "permissions":["INTERNET"],"locales":["en-US","fr"],"newInVersion":"New version","nftMintAddress":ADDRESS}
    })
}

pub fn remote_file() -> Step {
    json(
        r#"{"data":"R0lGODlhAgADAA==","fileName":"icon.gif","mimeType":" IMAGE/GIF; charset=binary "}"#,
    )
}
pub fn target(public: &str) -> Step {
    json(
        &json!({"uploadUrl":"https://upload.invalid/put?signature=synthetic","publicUrl":public})
            .to_string(),
    )
}
pub fn created() -> Step {
    json(r#"{"id":"ing","status":"created"}"#)
}
pub fn ready() -> Step {
    json(
        r#"{"id":"ing","idempotencyKey":"same-idempotency-key","releaseId":"rel","publicationSessionId":"pub","status":"Ready"}"#,
    )
}
pub fn queued() -> Step {
    json(r#"{"id":"ing","idempotencyKey":"same-idempotency-key","status":"queued"}"#)
}

pub fn hydrate_steps(icon: &str) -> Vec<Step> {
    vec![
        session(None, "PreparedForMint"),
        json(&bundle(icon).to_string()),
        remote_file(),
        target(PUBLIC_ICON),
        target(METADATA),
    ]
}

pub fn expected_document(image: &str) -> Value {
    json!({
        "schema_version":"0.4.0", "name":"Fixture", "description":"Short fixture", "image":image,"external_url":"https://publisher.invalid",
        "properties":{"category":"dApp","creators":[{"address":ADDRESS,"share":100}]},
        "extensions":{"solana_dapp_store":{
            "publisher_details":{
                "name":"Synthetic Publisher","website":"https://publisher.invalid",
                "contact":"contact@publisher.invalid","support_email":"support@publisher.invalid"
            },
            "release_details":{"updated_on":"2026-10-29T12:00:00.000Z","localized_resources":{"long_description":"1","new_in_version":"2","name":"4","short_description":"5"}},
            "media":[{"mime":"image/gif","purpose":"icon","width":2,"height":3,"sha256":GIF_HASH,"uri":image}],
            "files":[{"mime":"application/vnd.android.package-archive","purpose":"install","size":3,"sha256":ABC_HASH,"uri":"https://public.invalid/app.apk"}],
            "android_details":{"android_package":"test.fixture","version":"2.0","version_code":9007199254740993u64,
                "min_sdk":MIN_SDK,"target_sdk":TARGET_SDK,"cert_fingerprint":"synthetic-fingerprint",
                "permissions":["INTERNET"],"locales":["en-US","fr"]}},
            "i18n":{"en-US":{"1":"Long fixture","2":"New version","4":"Fixture","5":"Short fixture"}}}
    })
}
