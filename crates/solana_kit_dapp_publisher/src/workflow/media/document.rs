//! Closed Solana dApp Store NFT 0.4.0 document; no arbitrary JSON map enters the API.

use super::spec::{Spec, truncate};
use crate::workflow::Error;
use serde::Serialize;

#[derive(Serialize)]
pub(crate) struct ResolvedMedia {
    pub mime: String,
    pub purpose: &'static str,
    pub width: u32,
    pub height: u32,
    pub sha256: String,
    pub uri: String,
}

#[derive(Serialize)]
pub(crate) struct Document<'a> {
    schema_version: &'static str,
    name: &'a str,
    description: &'a str,
    image: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    external_url: Option<&'a str>,
    properties: Properties<'a>,
    extensions: Extensions<'a>,
}
#[derive(Serialize)]
struct Properties<'a> {
    category: &'static str,
    creators: [Creator<'a>; 1],
}
#[derive(Serialize)]
struct Creator<'a> {
    address: &'a str,
    share: u8,
}
#[derive(Serialize)]
struct Extensions<'a> {
    solana_dapp_store: Store<'a>,
    i18n: I18n<'a>,
}
#[derive(Serialize)]
struct Store<'a> {
    publisher_details: Publisher<'a>,
    release_details: Release<'a>,
    media: &'a [ResolvedMedia],
    files: [Install<'a>; 1],
    android_details: Android<'a>,
}
#[derive(Serialize)]
struct Publisher<'a> {
    name: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    website: Option<&'a str>,
    contact: &'a str,
    support_email: &'a str,
}
#[derive(Serialize)]
struct Release<'a> {
    updated_on: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    license_url: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    copyright_url: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    privacy_policy_url: Option<&'a str>,
    localized_resources: Resources,
}
#[derive(Serialize)]
struct Resources {
    long_description: &'static str,
    new_in_version: &'static str,
    name: &'static str,
    short_description: &'static str,
}
#[derive(Serialize)]
struct Install<'a> {
    mime: &'a str,
    purpose: &'static str,
    size: u64,
    sha256: &'a str,
    uri: &'a str,
}
#[derive(Serialize)]
struct Android<'a> {
    android_package: &'a str,
    version: &'a str,
    version_code: u64,
    min_sdk: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_sdk: Option<u64>,
    cert_fingerprint: &'a str,
    permissions: &'a [String],
    locales: &'a [String],
}
#[derive(Serialize)]
struct I18n<'a> {
    #[serde(rename = "en-US")]
    en_us: Locale<'a>,
}
#[derive(Serialize)]
struct Locale<'a> {
    #[serde(rename = "1")]
    long: &'a str,
    #[serde(rename = "2")]
    whats_new: &'a str,
    #[serde(rename = "4")]
    name: &'a str,
    #[serde(rename = "5")]
    short: String,
}

impl<'a> Document<'a> {
    pub fn new(spec: &'a Spec, media: &'a [ResolvedMedia]) -> Result<Self, Error> {
        let image = &media
            .first()
            .ok_or(Error::Invalid("missing resolved icon"))?
            .uri;
        let website = spec.website.as_deref();
        Ok(Self {
            schema_version: "0.4.0",
            name: &spec.name,
            description: &spec.short,
            image,
            external_url: website,
            properties: Properties {
                category: "dApp",
                creators: [Creator {
                    address: &spec.address,
                    share: 100,
                }],
            },
            extensions: Extensions {
                solana_dapp_store: Store {
                    publisher_details: Publisher {
                        name: &spec.publisher_name,
                        website,
                        contact: &spec.contact,
                        support_email: &spec.support,
                    },
                    release_details: Release {
                        updated_on: &spec.updated_on,
                        license_url: spec.license.as_deref(),
                        copyright_url: spec.copyright.as_deref(),
                        privacy_policy_url: spec.privacy.as_deref(),
                        localized_resources: Resources {
                            long_description: "1",
                            new_in_version: "2",
                            name: "4",
                            short_description: "5",
                        },
                    },
                    media,
                    files: [Install {
                        mime: &spec.install_mime,
                        purpose: "install",
                        size: spec.install_size,
                        sha256: &spec.install_hash,
                        uri: &spec.install_uri,
                    }],
                    android_details: Android {
                        android_package: &spec.android.android_package,
                        version: &spec.android.version_name,
                        version_code: spec.android.version_code,
                        min_sdk: spec.android.min_sdk,
                        target_sdk: spec.android.target_sdk,
                        cert_fingerprint: &spec.android.certificate,
                        permissions: &spec.android.permissions,
                        locales: &spec.android.locales,
                    },
                },
                i18n: I18n {
                    en_us: Locale {
                        long: &spec.long,
                        whats_new: &spec.whats_new,
                        name: &spec.name,
                        short: truncate(&spec.short, 50),
                    },
                },
            },
        })
    }
}
