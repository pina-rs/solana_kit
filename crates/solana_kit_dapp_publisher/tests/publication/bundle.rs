//! Independently authored normalized-projection behavior.

use std::{borrow::ToOwned, string::ToString, vec, vec::Vec};

use super::support::*;

const RELEASE_FILE_BYTES: u64 = 4096;
const INSTALL_FILE_BYTES: u64 = 12;
const FALLBACK_INSTALL_BYTES: u64 = 2048;
const EXPLICIT_VERSION_CODE: u64 = 7;

#[cfg_attr(test, test)]
pub fn maps_the_full_source_bundle() -> Outcome {
    let projected = map_backend_bundle_to_publication_bundle(
        &bundle(source_bundle()),
        Some("https://meta.example.com/rel-1.json"),
        PortalSourceKind::PortalUpload,
    );

    assert_source_identity(&projected)?;
    assert_source_release(&projected)?;

    Ok(())
}

fn assert_source_identity(projected: &PublicationBundle) -> Outcome {
    assert_eq!(projected.ingestion_session_id, "ing-1");
    assert_eq!(projected.publication_session_id, "pub-1");
    assert_eq!(projected.release_id, "rel-1");
    assert_eq!(projected.dapp.id, "dapp-1");
    assert_eq!(projected.dapp.dapp_name, "My App");
    assert_eq!(projected.dapp.subtitle.as_deref(), Some("Short"));

    assert_eq!(
        projected.dapp.dapp_icon_url.as_deref(),
        Some("https://img.example.com/icon.png")
    );
    assert_eq!(
        projected.dapp.dapp_preview_urls,
        vec!["https://img.example.com/1.png".to_owned()]
    );
    assert_eq!(projected.publisher.name, "Example Inc");
    assert_eq!(projected.publisher.kind, PublisherKind::Organization);
    assert_eq!(projected.publisher.website, "https://example.com");

    assert_eq!(
        projected.install_file.uri,
        "https://files.example.com/app.apk"
    );
    assert_eq!(projected.install_file.sha256.as_deref(), Some("deadbeef"));
    assert_eq!(projected.metadata.localized_name, "My App");
    assert_eq!(projected.metadata.short_description, "Short");
    assert_eq!(projected.metadata.long_description, "Long description");

    assert_eq!(projected.metadata.new_in_version, "Faster");
    assert_eq!(
        projected.metadata.release_metadata_uri.as_deref(),
        Some("https://meta.example.com/rel-1.json")
    );
    assert_eq!(projected.metadata.localized_strings.len(), 1);
    assert_eq!(projected.metadata.localized_strings[0].locale, "en-US");

    Ok(())
}

fn assert_source_release(projected: &PublicationBundle) -> Outcome {
    assert_eq!(
        projected.signer_authority.app_mint_address,
        "App111111111111111111111111111111111111111"
    );
    assert_eq!(
        projected.signer_authority.fee_payer.as_deref(),
        Some("Payer111111111111111111111111111111111111111")
    );
    assert_eq!(projected.release.version_name, "1.2.0");
    assert_eq!(projected.release.version_code.as_u64(), Some(VERSION_CODE));

    assert_eq!(
        projected
            .release
            .min_sdk_version
            .as_ref()
            .and_then(|value| value.as_u64()),
        Some(MIN_SDK_VERSION)
    );
    assert_eq!(
        projected
            .release
            .target_sdk_version
            .as_ref()
            .and_then(|value| value.as_u64()),
        Some(TARGET_SDK_VERSION)
    );
    assert!(projected.release.release_mint_address.is_some());
    assert!(projected.release.release_metadata_uri.is_some());

    assert_eq!(
        projected.release.nft_mint_address(),
        projected.release.release_mint_address.as_deref()
    );

    Ok(())
}

#[cfg_attr(test, test)]
pub fn maps_an_empty_bundle_with_source_defaults() -> Outcome {
    let projected = map_backend_bundle_to_publication_bundle(
        &bundle(json!({})),
        None,
        PortalSourceKind::ExternalUrl,
    );

    assert_eq!(projected.release_id, "");
    assert_eq!(projected.release.release_name, "Release update");
    assert_eq!(projected.release.version_name, "1");
    assert_eq!(projected.release.version_code.as_u64(), Some(1));
    assert_eq!(projected.metadata.short_description, "");
    assert_eq!(projected.publisher.kind, PublisherKind::Organization);

    assert!(projected.signer_authority.same_signer_required);
    assert_eq!(
        projected.signer_authority.accepted_signer_roles,
        vec!["publisher".to_owned(), "payer".to_owned()]
    );
    assert!(projected.release.release_mint_address.is_none());

    assert_eq!(projected.install_file.origin, PortalSourceKind::ExternalUrl);
    assert_eq!(projected.install_file.uri, "");
    assert_eq!(projected.install_file.mime_type, "application/octet-stream");
    assert_eq!(projected.install_file.size.as_u64(), Some(0));

    Ok(())
}

#[cfg_attr(test, test)]
pub fn publisher_kind_preserves_individual_and_unknown_values() -> Outcome {
    let individual = map_backend_bundle_to_publication_bundle(
        &bundle(json!({"publisher": {"type": "individual"}})),
        None,
        PortalSourceKind::PortalUpload,
    );
    assert_eq!(individual.publisher.kind, PublisherKind::Individual);

    let unknown = map_backend_bundle_to_publication_bundle(
        &bundle(json!({"publisher": {"type": "collective"}})),
        None,
        PortalSourceKind::PortalUpload,
    );
    assert_eq!(
        unknown.publisher.kind,
        PublisherKind::Unrecognized("collective".into())
    );

    let absent = map_backend_bundle_to_publication_bundle(
        &bundle(json!({"publisher": {"type": null}})),
        None,
        PortalSourceKind::PortalUpload,
    );
    assert_eq!(absent.publisher.kind, PublisherKind::Organization);

    Ok(())
}

#[cfg_attr(test, test)]
pub fn description_and_contact_precedence_matches_the_source() -> Outcome {
    let projected = map_backend_bundle_to_publication_bundle(
        &bundle(json!({
            "release": {
                "localizedName": "Release name",
                "shortDescription": "Release short"
            },
            "dapp": {
                "dappName": "Dapp name",
                "subtitle": "Dapp subtitle",
                "description": "1234567890123456789012345678901234567890123456789012345",
                "supportEmail": "dapp@example.com",
                "website": "https://website.example",
                "appWebsite": "https://app.example"
            },
            "publisher": {
                "email": "contact@example.com",
                "website": "https://publisher.example"
            }
        })),
        None,
        PortalSourceKind::PortalUpload,
    );

    assert_eq!(projected.metadata.localized_name, "Release name");
    assert_eq!(projected.metadata.short_description, "Release short");
    assert_eq!(
        projected.metadata.localized_strings[0].short_description,
        "Release short"
    );
    assert_eq!(projected.metadata.localized_strings[0].name, "Release name");

    assert_eq!(
        projected.dapp.website.as_deref(),
        Some("https://website.example")
    );
    assert_eq!(
        projected.dapp.app_website.as_deref(),
        Some("https://app.example")
    );
    assert_eq!(
        projected.metadata.website.as_deref(),
        Some("https://app.example")
    );

    assert_eq!(
        projected.metadata.publisher_website.as_deref(),
        Some("https://publisher.example")
    );
    assert_eq!(projected.dapp.support_email, "dapp@example.com");
    assert_eq!(projected.publisher.support_email, "dapp@example.com");
    assert!(projected.metadata.support_email.is_none());

    Ok(())
}

#[cfg_attr(test, test)]
pub fn blank_contact_fields_remain_distinct_from_missing_values() -> Outcome {
    let blank = map_backend_bundle_to_publication_bundle(
        &bundle(json!({
            "publisher": {"supportEmail": "", "email": "contact@example.com"},
            "dapp": {"supportEmail": ""}
        })),
        None,
        PortalSourceKind::PortalUpload,
    );

    assert_eq!(blank.publisher.support_email, "");
    assert_eq!(blank.dapp.support_email, "");
    assert_eq!(blank.metadata.support_email.as_deref(), Some(""));

    Ok(())
}

#[cfg_attr(test, test)]
pub fn description_fallback_uses_a_unicode_safe_utf16_bound() -> Outcome {
    let projected = map_backend_bundle_to_publication_bundle(
        &bundle(json!({
            "dapp": {"description": "a😀bcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"}
        })),
        None,
        PortalSourceKind::PortalUpload,
    );

    assert_eq!(
        projected.metadata.short_description,
        "a😀bcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUV"
    );
    assert_eq!(
        projected.metadata.localized_strings[0].short_description,
        projected.metadata.short_description
    );
    assert_eq!(
        projected.metadata.localized_strings[0]
            .short_description
            .len(),
        52
    );

    Ok(())
}

#[cfg_attr(test, test)]
pub fn media_is_projected_in_deterministic_source_order() -> Outcome {
    let projected = map_backend_bundle_to_publication_bundle(
        &bundle(json!({
            "dapp": {
                "dappIconUrl": "https://img.example/icon.png",
                "dappPreviewUrls": ["https://img.example/one.png", "https://img.example/two.png"],
                "bannerUrl": "https://img.example/banner.png",
                "featureGraphicUrl": "https://img.example/feature.png",
                "editorsChoiceGraphicUrl": "https://img.example/editors.png"
            }
        })),
        None,
        PortalSourceKind::PortalUpload,
    );
    let values: Vec<_> = projected
        .metadata
        .media
        .iter()
        .map(|media| (media.purpose, media.uri.as_str()))
        .collect();

    assert_eq!(
        values,
        vec![
            (MediaPurpose::Icon, "https://img.example/icon.png"),
            (MediaPurpose::Screenshot, "https://img.example/one.png"),
            (MediaPurpose::Screenshot, "https://img.example/two.png"),
            (MediaPurpose::Banner, "https://img.example/banner.png"),
            (
                MediaPurpose::FeatureGraphic,
                "https://img.example/editors.png"
            ),
        ]
    );
    assert_eq!(MediaPurpose::FeatureGraphic.wire_name(), "featureGraphic");

    Ok(())
}

#[cfg_attr(test, test)]
pub fn install_and_release_file_fallbacks_are_independent_literals() -> Outcome {
    let pinned = bundle(json!({
        "release": {
            "id": "rel-2",
            "releaseFileUrl": "https://files.example.com/releases/two.ipa",
            "releaseFileSize": RELEASE_FILE_BYTES,
            "releaseFileHash": "release-hash"
        },
        "installFile": {
            "canonicalUrl": "https://canonical.example.com/two.ipa"
        }
    }));
    let projected =
        map_backend_bundle_to_publication_bundle(&pinned, None, PortalSourceKind::ExternalUrl);

    assert_eq!(
        projected.install_file.uri,
        "https://files.example.com/releases/two.ipa"
    );
    assert_eq!(projected.install_file.file_name.as_deref(), Some("two.ipa"));
    assert_eq!(projected.install_file.mime_type, "application/octet-stream");
    assert_eq!(
        projected.install_file.canonical_url,
        "https://canonical.example.com/two.ipa"
    );
    assert_eq!(projected.install_file.url(), projected.install_file.uri);
    assert_eq!(projected.install_file.origin, PortalSourceKind::ExternalUrl);

    assert_eq!(
        projected.release.release_file_url,
        "https://files.example.com/releases/two.ipa"
    );
    assert_eq!(projected.release.release_file_name, "two.ipa");
    assert_eq!(
        projected.release.release_file_size.as_u64(),
        Some(RELEASE_FILE_BYTES)
    );

    assert_eq!(
        projected.release.release_file_hash.as_deref(),
        Some("release-hash")
    );

    Ok(())
}

#[cfg_attr(test, test)]
pub fn direct_install_file_details_infer_name_and_mime() -> Outcome {
    let direct = InstallFileDetails::from_backend(
        Some(
            &serde_json::from_value::<BackendRelease>(json!({
                "releaseFileUrl": "https://files.example.com/app.apk",
                "releaseFileName": "app.apk"
            }))
            .expect("release fixture"),
        ),
        Some(
            &serde_json::from_value::<BackendInstallFile>(json!({
                "size": INSTALL_FILE_BYTES,
                "sha256": "install-hash"
            }))
            .expect("install fixture"),
        ),
        PortalSourceKind::PortalUpload,
    );

    assert_eq!(direct.file_name.as_deref(), Some("app.apk"));
    assert_eq!(direct.mime_type, "application/vnd.android.package-archive");
    assert_eq!(direct.size.as_u64(), Some(INSTALL_FILE_BYTES));
    assert_eq!(direct.sha256.as_deref(), Some("install-hash"));
    assert_eq!(direct.canonical_url, direct.uri);

    Ok(())
}

#[cfg_attr(test, test)]
pub fn install_file_size_and_url_supply_release_fallbacks() -> Outcome {
    let projected = map_backend_bundle_to_publication_bundle(
        &bundle(json!({
            "release": {"releaseFileName": "app.apk"},
            "installFile": {
                "uri": "https://files.example.com/app.apk",
                "size": FALLBACK_INSTALL_BYTES
            }
        })),
        None,
        PortalSourceKind::PortalUpload,
    );

    assert_eq!(
        projected.install_file.uri,
        "https://files.example.com/app.apk"
    );
    assert_eq!(
        projected.install_file.size.as_u64(),
        Some(FALLBACK_INSTALL_BYTES)
    );
    assert_eq!(
        projected.release.release_file_url,
        "https://files.example.com/app.apk"
    );
    assert_eq!(projected.release.release_file_name, "app.apk");
    assert_eq!(
        projected.release.release_file_size.as_u64(),
        Some(FALLBACK_INSTALL_BYTES)
    );
    assert!(projected.release.release_file_hash.is_none());

    Ok(())
}

#[cfg_attr(test, test)]
pub fn signer_authority_preserves_booleans_roles_and_fallbacks() -> Outcome {
    let projected = map_backend_bundle_to_publication_bundle(
        &bundle(json!({
            "dapp": {
                "id": "dapp-1",
                "walletAddress": "Dapp111111111111111111111111111111111111111",
                "nftMintAddress": "Mint111111111111111111111111111111111111111"
            },
            "signerAuthority": {
                "sameSignerRequired": false,
                "acceptedSignerRoles": [],
                "feePayer": ""
            }
        })),
        None,
        PortalSourceKind::PortalUpload,
    );
    let signer = &projected.signer_authority;

    assert_eq!(
        signer.dapp_wallet_address,
        "Dapp111111111111111111111111111111111111111"
    );
    assert_eq!(
        signer.collection_authority,
        "Dapp111111111111111111111111111111111111111"
    );
    assert_eq!(
        signer.app_mint_address,
        "Mint111111111111111111111111111111111111111"
    );
    assert_eq!(signer.dapp_id, "dapp-1");

    assert_eq!(
        signer.required_signer,
        "Dapp111111111111111111111111111111111111111"
    );
    assert_eq!(
        signer.mint_signer,
        "Dapp111111111111111111111111111111111111111"
    );

    assert!(!signer.same_signer_required);
    assert!(signer.accepted_signer_roles.is_empty());
    assert_eq!(signer.fee_payer.as_deref(), Some(""));

    let nulled = map_backend_bundle_to_publication_bundle(
        &bundle(json!({
            "signerAuthority": {
                "sameSignerRequired": null,
                "acceptedSignerRoles": null
            }
        })),
        None,
        PortalSourceKind::PortalUpload,
    );

    assert!(nulled.signer_authority.same_signer_required);
    assert_eq!(
        nulled.signer_authority.accepted_signer_roles,
        vec!["publisher".to_owned(), "payer".to_owned()]
    );

    Ok(())
}

#[cfg_attr(test, test)]
pub fn numbers_and_version_names_keep_exact_source_text() -> Outcome {
    let version_code = bundle(json!({
        "release": {"versionCode": 9007199254740993_i64}
    }));
    let projected = map_backend_bundle_to_publication_bundle(
        &version_code,
        None,
        PortalSourceKind::PortalUpload,
    );
    assert_eq!(projected.release.version_name, "9007199254740993");
    assert_eq!(
        projected.release.version_code.to_string(),
        "9007199254740993"
    );

    let zero = bundle(json!({"release": {"versionCode": 0}}));
    let projected =
        map_backend_bundle_to_publication_bundle(&zero, None, PortalSourceKind::PortalUpload);
    assert_eq!(projected.release.version_name, "");
    assert_eq!(projected.release.version_code.as_u64(), Some(0));

    let explicit_name = bundle(json!({
        "release": {"versionName": "", "versionCode": EXPLICIT_VERSION_CODE}
    }));
    let projected = map_backend_bundle_to_publication_bundle(
        &explicit_name,
        None,
        PortalSourceKind::PortalUpload,
    );
    assert_eq!(projected.release.version_name, "");

    Ok(())
}
