//! Bundle, install-file, dApp, publisher, signer and release projection.

use super::support::{
    bounded, file_name_from_url, first_non_empty, first_present, list, number_text,
    optional_number, optional_text, required_text, string_or_empty, text, value,
};
use crate::publication::{
    BundleLegal, BundleMedia, BundleMetadata, DappInfo, InstallFileDetails,
    LocalizedReleaseStrings, MediaPurpose, PortalSourceKind, PublicationBundle, PublisherInfo,
    PublisherKind, ReleaseInfo, SignerAuthority,
};
use crate::{
    mime::infer_mime,
    portal::{
        BackendDapp, BackendInstallFile, BackendPublicationBundle, BackendPublisher,
        BackendRelease, BackendSignerAuthority,
    },
};
use serde_json::Number;
use std::{string::String, vec::Vec};

struct Descriptions {
    release_name: String,
    dapp_name: String,
    short_description: String,
    localized_short_description: String,
    long_description: String,
    new_in_version: String,
}

/// Projects a typed backend bundle without fetching, uploading or validating it.
///
/// `release_metadata_uri` is the higher-priority session location. Blank values
/// are ignored by the source's `firstNonEmpty` fallback, so the release's
/// `nftMetadataUri` may supply the final value.
#[must_use]
pub fn map_backend_bundle_to_publication_bundle(
    bundle: &BackendPublicationBundle,
    release_metadata_uri: Option<&str>,
    source_kind: PortalSourceKind,
) -> PublicationBundle {
    let release = value(&bundle.release);
    let dapp = value(&bundle.dapp);
    let publisher = value(&bundle.publisher);
    let install = value(&bundle.install_file);
    let install_file = InstallFileDetails::from_backend(release, install, source_kind);
    let descriptions = descriptions(release, dapp);
    let metadata_uri = metadata_uri(release_metadata_uri, release);
    let metadata = bundle_metadata(
        dapp,
        publisher,
        &descriptions,
        &install_file,
        metadata_uri.clone(),
    );
    let release_record = release_info(
        bundle,
        release,
        dapp,
        &install_file,
        &descriptions,
        metadata_uri,
    );

    PublicationBundle {
        ingestion_session_id: string_or_empty(&bundle.ingestion_session_id),
        publication_session_id: string_or_empty(&bundle.publication_session_id),
        release_id: first_present(&[text(&bundle.release_id), release.and_then(|r| text(&r.id))])
            .unwrap_or_default()
            .to_owned(),
        dapp: dapp_info(dapp, release, publisher, descriptions.dapp_name),
        publisher: publisher_info(publisher, dapp),
        install_file,
        metadata,
        signer_authority: signer_authority(value(&bundle.signer_authority), dapp),
        release: release_record,
    }
}

impl InstallFileDetails {
    /// Projects install-file details from typed release and install records.
    ///
    /// The release record supplies a location and file name when the install
    /// record omits them. Names are inferred from the literal final URI path
    /// segment; the source's URI parser also percent-decodes segments, which
    /// this projection deliberately does not do.
    #[must_use]
    pub fn from_backend(
        release: Option<&BackendRelease>,
        install_file: Option<&BackendInstallFile>,
        origin: PortalSourceKind,
    ) -> Self {
        let uri = first_present(&[
            install_file.and_then(|install| text(&install.uri)),
            release.and_then(|release| text(&release.release_file_url)),
        ])
        .unwrap_or_default()
        .to_owned();
        let file_name = optional_text(release, |release| &release.release_file_name)
            .or_else(|| file_name_from_url(&uri));
        let mime_type = optional_text(install_file, |install| &install.mime_type)
            .unwrap_or_else(|| infer_mime(file_name.as_deref().unwrap_or("")).into());
        let size = optional_number(install_file, |install| &install.size)
            .unwrap_or_else(|| Number::from(0));
        let canonical_url = optional_text(install_file, |install| &install.canonical_url)
            .unwrap_or_else(|| uri.clone());

        Self {
            uri,
            mime_type,
            size,
            sha256: optional_text(install_file, |install| &install.sha256),
            file_name,
            canonical_url,
            origin,
        }
    }
}

fn descriptions(release: Option<&BackendRelease>, dapp: Option<&BackendDapp>) -> Descriptions {
    let release_name = first_present(&[
        release.and_then(|release| text(&release.localized_name)),
        dapp.and_then(|dapp| text(&dapp.dapp_name)),
    ])
    .unwrap_or("Release update")
    .to_owned();
    let long_description = required_text(dapp, |dapp| &dapp.description);
    let short_description = optional_text(release, |release| &release.short_description)
        .or_else(|| optional_text(dapp, |dapp| &dapp.subtitle))
        .unwrap_or_else(|| bounded(&long_description));
    let localized_short_description = optional_text(release, |release| &release.short_description)
        .unwrap_or_else(|| bounded(&long_description));
    let dapp_name =
        optional_text(dapp, |dapp| &dapp.dapp_name).unwrap_or_else(|| release_name.clone());
    let long_description =
        optional_text(release, |release| &release.long_description).unwrap_or(long_description);

    Descriptions {
        release_name,
        dapp_name,
        short_description,
        localized_short_description,
        long_description,
        new_in_version: required_text(release, |release| &release.new_in_version),
    }
}

fn metadata_uri(session_uri: Option<&str>, release: Option<&BackendRelease>) -> Option<String> {
    first_non_empty(&[
        session_uri,
        release.and_then(|release| text(&release.nft_metadata_uri)),
    ])
    .map(str::to_owned)
}

fn bundle_metadata(
    dapp: Option<&BackendDapp>,
    publisher: Option<&BackendPublisher>,
    descriptions: &Descriptions,
    install_file: &InstallFileDetails,
    release_metadata_uri: Option<String>,
) -> BundleMetadata {
    BundleMetadata {
        localized_name: descriptions.release_name.clone(),
        short_description: descriptions.short_description.clone(),
        long_description: descriptions.long_description.clone(),
        new_in_version: descriptions.new_in_version.clone(),
        publisher_website: optional_text(publisher, |publisher| &publisher.website),
        support_email: optional_text(publisher, |publisher| &publisher.support_email),
        website: optional_text(dapp, |dapp| &dapp.app_website),
        locales: list(dapp, |dapp| &dapp.languages),
        legal: legal(dapp),
        media: media(dapp),
        install_file: install_file.clone(),
        localized_strings: localized_strings(descriptions),
        release_metadata_uri,
    }
}

fn legal(dapp: Option<&BackendDapp>) -> BundleLegal {
    BundleLegal {
        license_url: optional_text(dapp, |dapp| &dapp.license_url),
        copyright_url: optional_text(dapp, |dapp| &dapp.copyright_url),
        privacy_policy_url: optional_text(dapp, |dapp| &dapp.privacy_policy_url),
    }
}

fn localized_strings(descriptions: &Descriptions) -> Vec<LocalizedReleaseStrings> {
    vec![LocalizedReleaseStrings {
        locale: "en-US".into(),
        name: descriptions.release_name.clone(),
        short_description: descriptions.localized_short_description.clone(),
        long_description: descriptions.long_description.clone(),
        new_in_version: descriptions.new_in_version.clone(),
    }]
}

fn publisher_info(
    publisher: Option<&BackendPublisher>,
    dapp: Option<&BackendDapp>,
) -> PublisherInfo {
    PublisherInfo {
        id: required_text(publisher, |publisher| &publisher.id),
        kind: PublisherKind::from_reported(
            optional_text(publisher, |publisher| &publisher.publisher_type).as_deref(),
        ),
        name: required_text(publisher, |publisher| &publisher.name),
        website: required_text(publisher, |publisher| &publisher.website),
        email: required_text(publisher, |publisher| &publisher.email),
        support_email: first_present(&[
            publisher.and_then(|publisher| text(&publisher.support_email)),
            dapp.and_then(|dapp| text(&dapp.support_email)),
            publisher.and_then(|publisher| text(&publisher.email)),
        ])
        .unwrap_or_default()
        .to_owned(),
    }
}

fn dapp_info(
    dapp: Option<&BackendDapp>,
    release: Option<&BackendRelease>,
    publisher: Option<&BackendPublisher>,
    dapp_name: String,
) -> DappInfo {
    DappInfo {
        id: required_text(dapp, |dapp| &dapp.id),
        dapp_name,
        subtitle: optional_text(dapp, |dapp| &dapp.subtitle),
        description: required_text(dapp, |dapp| &dapp.description),
        android_package: first_present(&[
            dapp.and_then(|dapp| text(&dapp.android_package)),
            release.and_then(|release| text(&release.android_package)),
        ])
        .unwrap_or_default()
        .to_owned(),
        dapp_icon_url: optional_text(dapp, |dapp| &dapp.dapp_icon_url),
        dapp_preview_urls: list(dapp, |dapp| &dapp.dapp_preview_urls),
        banner_url: optional_text(dapp, |dapp| &dapp.banner_url),
        feature_graphic_url: optional_text(dapp, |dapp| &dapp.feature_graphic_url),
        editors_choice_graphic_url: optional_text(dapp, |dapp| &dapp.editors_choice_graphic_url),
        app_website: optional_text(dapp, |dapp| &dapp.app_website),
        contact_email: optional_text(dapp, |dapp| &dapp.contact_email),
        support_email: first_present(&[
            dapp.and_then(|dapp| text(&dapp.support_email)),
            publisher.and_then(|publisher| text(&publisher.support_email)),
        ])
        .unwrap_or_default()
        .to_owned(),
        languages: list(dapp, |dapp| &dapp.languages),
        license_url: optional_text(dapp, |dapp| &dapp.license_url),
        copyright_url: optional_text(dapp, |dapp| &dapp.copyright_url),
        privacy_policy_url: optional_text(dapp, |dapp| &dapp.privacy_policy_url),
        wallet_address: required_text(dapp, |dapp| &dapp.wallet_address),
        nft_mint_address: required_text(dapp, |dapp| &dapp.nft_mint_address),
        last_approved_release_id: optional_text(dapp, |dapp| &dapp.last_approved_release_id),
        website: first_present(&[
            dapp.and_then(|dapp| text(&dapp.website)),
            dapp.and_then(|dapp| text(&dapp.app_website)),
        ])
        .map(str::to_owned),
    }
}

fn signer_authority(
    signer: Option<&BackendSignerAuthority>,
    dapp: Option<&BackendDapp>,
) -> SignerAuthority {
    let fallback = signer_fallback(signer, dapp);

    SignerAuthority {
        dapp_wallet_address: fallback.clone(),
        collection_authority: collection_authority(signer, dapp),
        app_mint_address: app_mint_address(signer, dapp),
        same_signer_required: same_signer_required(signer),
        accepted_signer_roles: accepted_signer_roles(signer),
        dapp_id: required_text(dapp, |dapp| &dapp.id),
        required_signer: optional_text(signer, |signer| &signer.required_signer)
            .unwrap_or_else(|| fallback.clone()),
        mint_signer: optional_text(signer, |signer| &signer.mint_signer).unwrap_or(fallback),
        fee_payer: optional_text(signer, |signer| &signer.fee_payer),
    }
}

fn signer_fallback(signer: Option<&BackendSignerAuthority>, dapp: Option<&BackendDapp>) -> String {
    first_present(&[
        signer.and_then(|signer| text(&signer.dapp_wallet_address)),
        signer.and_then(|signer| text(&signer.collection_authority)),
        dapp.and_then(|dapp| text(&dapp.wallet_address)),
    ])
    .unwrap_or_default()
    .to_owned()
}

fn collection_authority(
    signer: Option<&BackendSignerAuthority>,
    dapp: Option<&BackendDapp>,
) -> String {
    first_present(&[
        signer.and_then(|signer| text(&signer.collection_authority)),
        signer.and_then(|signer| text(&signer.dapp_wallet_address)),
        dapp.and_then(|dapp| text(&dapp.wallet_address)),
    ])
    .unwrap_or_default()
    .to_owned()
}

fn app_mint_address(signer: Option<&BackendSignerAuthority>, dapp: Option<&BackendDapp>) -> String {
    first_present(&[
        signer.and_then(|signer| text(&signer.app_mint_address)),
        dapp.and_then(|dapp| text(&dapp.nft_mint_address)),
    ])
    .unwrap_or_default()
    .to_owned()
}

fn same_signer_required(signer: Option<&BackendSignerAuthority>) -> bool {
    signer
        .and_then(|signer| value(&signer.same_signer_required))
        .is_none_or(|required| *required)
}

fn accepted_signer_roles(signer: Option<&BackendSignerAuthority>) -> Vec<String> {
    signer
        .and_then(|signer| value(&signer.accepted_signer_roles))
        .cloned()
        .unwrap_or_else(|| vec!["publisher".into(), "payer".into()])
}

fn release_info(
    bundle: &BackendPublicationBundle,
    release: Option<&BackendRelease>,
    dapp: Option<&BackendDapp>,
    install_file: &InstallFileDetails,
    descriptions: &Descriptions,
    release_metadata_uri: Option<String>,
) -> ReleaseInfo {
    ReleaseInfo {
        id: first_present(&[
            release.and_then(|release| text(&release.id)),
            text(&bundle.release_id),
        ])
        .unwrap_or_default()
        .to_owned(),
        dapp_id: first_present(&[
            release.and_then(|release| text(&release.dapp_id)),
            dapp.and_then(|dapp| text(&dapp.id)),
        ])
        .unwrap_or_default()
        .to_owned(),
        release_file_url: optional_text(release, |release| &release.release_file_url)
            .unwrap_or_else(|| install_file.uri.clone()),
        release_file_name: optional_text(release, |release| &release.release_file_name)
            .unwrap_or_else(|| install_file.file_name.clone().unwrap_or_default()),
        release_file_size: optional_number(release, |release| &release.release_file_size)
            .unwrap_or_else(|| install_file.size.clone()),
        release_file_hash: optional_text(release, |release| &release.release_file_hash)
            .or_else(|| install_file.sha256.clone()),
        release_name: descriptions.release_name.clone(),
        version_name: version_name(release),
        version_code: optional_number(release, |release| &release.version_code)
            .unwrap_or_else(|| Number::from(1)),
        android_package: first_present(&[
            release.and_then(|release| text(&release.android_package)),
            dapp.and_then(|dapp| text(&dapp.android_package)),
        ])
        .unwrap_or_default()
        .to_owned(),
        min_sdk_version: optional_number(release, |release| &release.min_sdk_version),
        target_sdk_version: optional_number(release, |release| &release.target_sdk_version),
        permissions: list(release, |release| &release.permissions),
        locales: list(release, |release| &release.locales),
        certificate_fingerprint: optional_text(release, |release| &release.certificate_fingerprint),
        short_description: optional_text(release, |release| &release.short_description),
        long_description: optional_text(release, |release| &release.long_description),
        localized_name: optional_text(release, |release| &release.localized_name)
            .unwrap_or_else(|| descriptions.release_name.clone()),
        new_in_version: descriptions.new_in_version.clone(),
        saga_features: optional_text(release, |release| &release.saga_features),
        release_mint_address: optional_text(release, |release| &release.nft_mint_address),
        release_metadata_uri,
        nft_metadata_uri: optional_text(release, |release| &release.nft_metadata_uri),
    }
}

fn version_name(release: Option<&BackendRelease>) -> String {
    optional_text(release, |release| &release.version_name).unwrap_or_else(|| {
        optional_number(release, |release| &release.version_code)
            .map_or_else(|| "1".to_owned(), |code| number_text(&code))
    })
}

fn media(dapp: Option<&BackendDapp>) -> Vec<BundleMedia> {
    let Some(dapp) = dapp else {
        return Vec::new();
    };
    let mut media = Vec::new();

    if let Some(icon) = text(&dapp.dapp_icon_url).filter(|value| !value.is_empty()) {
        media.push(BundleMedia {
            purpose: MediaPurpose::Icon,
            uri: icon.to_owned(),
        });
    }

    for preview in value(&dapp.dapp_preview_urls).map_or(&[][..], Vec::as_slice) {
        media.push(BundleMedia {
            purpose: MediaPurpose::Screenshot,
            uri: preview.clone(),
        });
    }

    if let Some(banner) = text(&dapp.banner_url).filter(|value| !value.is_empty()) {
        media.push(BundleMedia {
            purpose: MediaPurpose::Banner,
            uri: banner.to_owned(),
        });
    }

    if let Some(feature) = first_non_empty(&[
        text(&dapp.editors_choice_graphic_url),
        text(&dapp.feature_graphic_url),
    ]) {
        media.push(BundleMedia {
            purpose: MediaPurpose::FeatureGraphic,
            uri: feature.to_owned(),
        });
    }

    media
}
