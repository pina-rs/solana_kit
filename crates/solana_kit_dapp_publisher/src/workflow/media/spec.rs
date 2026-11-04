//! Validate and resolve all metadata inputs before the first media fetch or upload.

use super::{mime, urls};
use crate::{
    portal::{
        BackendDapp, BackendInstallFile, BackendPublicationBundle, BackendPublisher,
        BackendRelease, BackendSignerAuthority,
    },
    workflow::{Error, text, value},
};

const NAME_UTF16_UNITS: usize = 32;
const SHORT_UTF16_UNITS: usize = 50;

struct Descriptions {
    name: String,
    short: String,
    long: String,
    whats_new: String,
}

struct PublisherData {
    publisher_name: String,
    website: Option<String>,
    contact: String,
    support: String,
}
struct InstallData {
    install_uri: String,
    install_mime: String,
    install_size: u64,
    install_hash: String,
}

pub(crate) struct MediaInput {
    pub uri: String,
    pub purpose: &'static str,
    pub fallback_name: String,
    pub default_mime: Option<&'static str>,
}

pub(crate) struct Spec {
    pub name: String,
    pub short: String,
    pub long: String,
    pub whats_new: String,
    pub address: String,
    pub publisher_name: String,
    pub website: Option<String>,
    pub contact: String,
    pub support: String,
    pub license: Option<String>,
    pub copyright: Option<String>,
    pub privacy: Option<String>,
    pub updated_on: String,
    pub install_uri: String,
    pub install_mime: String,
    pub install_size: u64,
    pub install_hash: String,
    pub android: AndroidData,
    pub media: Vec<MediaInput>,
}

pub(crate) struct AndroidData {
    pub android_package: String,
    pub version_name: String,
    pub version_code: u64,
    pub min_sdk: u64,
    pub target_sdk: Option<u64>,
    pub certificate: String,
    pub permissions: Vec<String>,
    pub locales: Vec<String>,
}

fn pick<'a>(fields: &[Option<&'a str>]) -> Option<&'a str> {
    fields
        .iter()
        .copied()
        .flatten()
        .find(|field| !field.is_empty())
}
fn string(field: &crate::portal::Optional<String>) -> Option<&str> {
    value(field).map(String::as_str)
}
fn required(value: Option<&str>, reason: &'static str) -> Result<String, Error> {
    value
        .filter(|value| !value.trim().is_empty())
        .map(str::to_owned)
        .ok_or(Error::Invalid(reason))
}
fn optional_url(value: Option<&str>) -> Result<Option<String>, Error> {
    value
        .filter(|value| !value.trim().is_empty())
        .map(urls::https)
        .transpose()
}
fn number(value: Option<&serde_json::Number>, fallback: u64) -> Result<u64, Error> {
    value.map_or(Ok(fallback), |value| {
        value
            .as_u64()
            .ok_or(Error::Invalid("metadata integer must be unsigned"))
    })
}

/// A Unicode boundary is never split when applying the source's UTF-16 length bound.
pub(super) use crate::text::truncate_utf16 as truncate;

impl Spec {
    pub fn new(
        bundle: &BackendPublicationBundle,
        updated_on: &str,
        maximum: usize,
    ) -> Result<Self, Error> {
        let release = value(&bundle.release).ok_or(Error::Invalid("missing release record"))?;
        let dapp = value(&bundle.dapp).ok_or(Error::Invalid("missing dApp record"))?;
        let signer =
            value(&bundle.signer_authority).ok_or(Error::Invalid("missing signer authority"))?;
        let publisher = value(&bundle.publisher);
        let install = value(&bundle.install_file);
        let address = publication_address(dapp, signer)?;
        let Descriptions {
            name,
            short,
            long,
            whats_new,
        } = descriptions(release, dapp)?;
        let PublisherData {
            publisher_name,
            website,
            contact,
            support,
        } = publisher_contacts(publisher, dapp)?;
        let InstallData {
            install_uri,
            install_mime,
            install_size,
            install_hash,
        } = install_file(install, release)?;

        let android = android_details(release, dapp)?;
        let media = media_inputs(bundle, maximum)?;

        Ok(Self {
            name,
            short,
            long,
            whats_new,
            address,
            publisher_name,
            website,
            contact,
            support,
            license: optional_url(text(&dapp.license_url))?,
            copyright: pick(&[string(&dapp.copyright_url)]).map(str::to_owned),
            privacy: optional_url(text(&dapp.privacy_policy_url))?,
            updated_on: updated_on.into(),
            install_uri,
            install_mime,
            install_size,
            install_hash,
            android,
            media,
        })
    }
}

fn android_details(release: &BackendRelease, dapp: &BackendDapp) -> Result<AndroidData, Error> {
    let locales = value(&release.locales)
        .filter(|list| !list.is_empty())
        .or_else(|| value(&dapp.languages).filter(|list| !list.is_empty()))
        .cloned()
        .unwrap_or_else(|| vec!["en-US".into()]);
    let version_name = string(&release.version_name)
        .map(str::to_owned)
        .unwrap_or_else(|| {
            value(&release.version_code).map_or_else(|| "1".into(), ToString::to_string)
        });
    if string(&release.version_name).is_none()
        && value(&release.version_code).and_then(serde_json::Number::as_u64) == Some(0)
    {
        return Err(Error::Invalid(
            "zero version code does not supply a version name",
        ));
    }
    Ok(AndroidData {
        android_package: required(
            string(&release.android_package).or_else(|| string(&dapp.android_package)),
            "missing Android package",
        )?,
        version_name: required(Some(&version_name), "missing version name")?,
        version_code: number(value(&release.version_code), 1)?,
        min_sdk: number(value(&release.min_sdk_version), 1)?,
        target_sdk: value(&release.target_sdk_version)
            .map(|value| number(Some(value), 0))
            .transpose()?,
        certificate: string(&release.certificate_fingerprint)
            .unwrap_or("")
            .into(),
        permissions: value(&release.permissions).cloned().unwrap_or_default(),
        locales,
    })
}

fn publisher_contacts(
    publisher: Option<&BackendPublisher>,
    dapp: &BackendDapp,
) -> Result<PublisherData, Error> {
    let website = optional_url(pick(&[
        publisher.and_then(|p| string(&p.website)),
        string(&dapp.app_website),
        string(&dapp.website),
    ]))?;
    let publisher_email = publisher.and_then(|p| string(&p.email));
    let publisher_support = publisher.and_then(|p| string(&p.support_email));
    let dapp_support = string(&dapp.support_email);
    let normalized_support = publisher_support.or(dapp_support).or(publisher_email);
    let normalized_dapp_support = dapp_support.or(publisher_support);
    let contact = pick(&[
        publisher_email,
        string(&dapp.contact_email),
        normalized_dapp_support,
    ])
    .unwrap_or("")
    .to_owned();
    let support = pick(&[
        normalized_support,
        publisher_support,
        normalized_dapp_support,
        string(&dapp.contact_email),
        publisher_email,
    ])
    .unwrap_or(&contact)
    .to_owned();
    Ok(PublisherData {
        publisher_name: publisher.and_then(|p| string(&p.name)).unwrap_or("").into(),
        website,
        contact,
        support,
    })
}

fn install_file(
    install: Option<&BackendInstallFile>,
    release: &BackendRelease,
) -> Result<InstallData, Error> {
    let install_uri = urls::public(&required(
        install
            .and_then(|i| string(&i.uri))
            .or_else(|| string(&release.release_file_url)),
        "missing install URI",
    )?)?;
    let install_name = string(&release.release_file_name)
        .or_else(|| urls::file_name(&install_uri))
        .unwrap_or("");
    let install_mime = install
        .and_then(|i| string(&i.mime_type))
        .unwrap_or_else(|| mime::infer(install_name))
        .to_owned();
    mime::validate(&install_mime)?;
    let install_size = number(install.and_then(|i| value(&i.size)), 0)?;
    let install_size = if install_size == 0 {
        number(value(&release.release_file_size), 0)?
    } else {
        install_size
    };

    let install_hash = pick(&[
        install.and_then(|i| string(&i.sha256)),
        string(&release.release_file_hash),
    ])
    .unwrap_or("")
    .into();
    Ok(InstallData {
        install_uri,
        install_mime,
        install_size,
        install_hash,
    })
}

fn publication_address(
    dapp: &BackendDapp,
    signer: &BackendSignerAuthority,
) -> Result<String, Error> {
    let address = required(
        string(&signer.dapp_wallet_address)
            .or_else(|| string(&signer.collection_authority))
            .or_else(|| string(&dapp.wallet_address)),
        "missing publisher address",
    )?;
    address
        .parse::<solana_kit_core::Address>()
        .map_err(|_| Error::Invalid("invalid publisher address"))?;
    required(
        string(&signer.app_mint_address).or_else(|| string(&dapp.nft_mint_address)),
        "missing collection mint",
    )?;
    required(
        string(&signer.collection_authority)
            .or_else(|| string(&signer.dapp_wallet_address))
            .or_else(|| string(&dapp.wallet_address)),
        "missing collection authority",
    )?;

    if value(&signer.accepted_signer_roles).is_some_and(Vec::is_empty) {
        return Err(Error::Invalid("empty accepted signer roles"));
    }

    Ok(address)
}

fn descriptions(release: &BackendRelease, dapp: &BackendDapp) -> Result<Descriptions, Error> {
    let raw_name = string(&release.localized_name)
        .or_else(|| string(&dapp.dapp_name))
        .unwrap_or("Release update");
    let name = truncate(
        required(Some(raw_name), "missing release name")?.trim(),
        NAME_UTF16_UNITS,
    );
    let short = string(&release.short_description)
        .or_else(|| string(&dapp.subtitle))
        .map(str::to_owned)
        .unwrap_or_else(|| truncate(string(&dapp.description).unwrap_or(""), SHORT_UTF16_UNITS));
    let long = required(
        string(&release.long_description).or_else(|| string(&dapp.description)),
        "missing long description",
    )?;
    let whats_new = required(text(&release.new_in_version), "missing release notes")?;

    if name.is_empty() || short.trim().is_empty() {
        return Err(Error::Invalid("empty metadata name or description"));
    }

    Ok(Descriptions {
        name,
        short,
        long,
        whats_new,
    })
}

/// Count admission and URL validation finish before any media is fetched or rehosted.
fn media_inputs(
    bundle: &BackendPublicationBundle,
    maximum: usize,
) -> Result<Vec<MediaInput>, Error> {
    let dapp = value(&bundle.dapp).ok_or(Error::Invalid("missing dApp record"))?;
    let previews = value(&dapp.dapp_preview_urls)
        .map(Vec::as_slice)
        .unwrap_or(&[]);
    let banner = string(&dapp.banner_url).filter(|value| !value.is_empty());
    let feature = pick(&[
        string(&dapp.editors_choice_graphic_url),
        string(&dapp.feature_graphic_url),
    ]);
    let count = previews
        .len()
        .checked_add(1)
        .and_then(|n| n.checked_add(usize::from(banner.is_some()) + usize::from(feature.is_some())))
        .ok_or(Error::Invalid("media count overflow"))?;

    if count > maximum {
        return Err(Error::Invalid("media item limit"));
    }

    let icon = required(text(&dapp.dapp_icon_url), "missing public app icon")?;
    let mut media = vec![MediaInput {
        uri: urls::https(&icon)?,
        purpose: "icon",
        fallback_name: "release-icon.png".into(),
        default_mime: Some("image/png"),
    }];

    for (index, uri) in previews.iter().enumerate() {
        media.push(MediaInput {
            uri: urls::https(uri)?,
            purpose: "screenshot",
            fallback_name: format!("release-screenshot-{}", index + 1),
            default_mime: None,
        });
    }

    for (uri, purpose, name) in [
        (banner, "banner", "release-banner.png"),
        (feature, "featureGraphic", "release-feature-graphic.png"),
    ] {
        if let Some(uri) = uri {
            media.push(MediaInput {
                uri: urls::https(uri)?,
                purpose,
                fallback_name: name.into(),
                default_mime: Some(mime::infer(urls::file_name(uri).unwrap_or(""))),
            });
        }
    }

    Ok(media)
}
