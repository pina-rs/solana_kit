//! A single capped read supplies both APK hash and upload bytes, avoiding stat/read races.

use super::{Error, Limits, SourceReader, media};
use crate::{
    deadline,
    portal::{
        self, Client, CreateUploadTarget, IngestionSource, PortalService, PortalUploadTarget,
        UploadRequest, Uploader,
    },
};
use solana_kit_client::{rest::Transport, subscriptions::Clock};
use solana_kit_core::{
    codecs::{Base16, Decoder, Limits as CodecLimits},
    crypto,
};

const SHA256_HEX_DIGITS: usize = 64;

/// Caller-approved local source. Its reader token is never serialized into a portal request.
pub struct LocalApk {
    /// Local-only opaque file token, interpreted exclusively by SourceReader.
    pub token: String,
    /// Public file name, without a directory or path separator.
    pub file_name: String,
    /// Optional media type, otherwise the pinned APK media type.
    pub content_type: Option<String>,
}

pub(super) fn sha256(bytes: &[u8]) -> Result<String, Error> {
    Base16
        .decode(
            crypto::hash(&[bytes]).as_bytes(),
            CodecLimits::new(SHA256_HEX_DIGITS, SHA256_HEX_DIGITS),
        )
        .map_err(Error::Codec)
}

/// Explicitly reads and uploads one immutable APK. No retry, storage or signer is implied.
///
/// An interrupted target/PUT may have taken effect. The caller must reconcile it
/// before explicitly invoking this helper again; no local path enters diagnostics.
pub async fn prepare_local_apk<T, U, R, C>(
    client: &Client<T>,
    uploader: &U,
    reader: &R,
    clock: &C,
    source: &LocalApk,
    limits: Limits,
) -> Result<IngestionSource, Error>
where
    T: Transport<PortalService>,
    U: Uploader,
    R: SourceReader,
    C: Clock,
{
    deadline::within(
        prepare_inner(client, uploader, reader, clock, source, limits),
        clock,
        limits.deadline,
    )
    .await
    .map_err(Error::from)?
}

async fn prepare_inner<T, U, R, C>(
    client: &Client<T>,
    uploader: &U,
    reader: &R,
    clock: &C,
    source: &LocalApk,
    limits: Limits,
) -> Result<IngestionSource, Error>
where
    T: Transport<PortalService>,
    U: Uploader,
    R: SourceReader,
    C: Clock,
{
    limits.validate()?;
    let (name, mime) = header(source, limits.metadata.string_bytes)?;
    let read = read_and_hash(reader, &source.token, limits.source_bytes);
    let (bytes, hash) = deadline::within(read, clock, limits.deadline)
        .await
        .map_err(Error::from)??;

    let request = CreateUploadTarget {
        file_hash: hash.clone(),
        file_extension: "apk".into(),
        content_type: mime.into(),
    };
    let target = request_upload_target(client, clock, &request, limits.deadline).await?;
    let upload = portal::upload_bytes(
        uploader,
        UploadRequest {
            target: &target,
            body: &bytes,
            content_type: mime,
            max_bytes: limits.source_bytes,
        },
    );
    deadline::within(upload, clock, limits.deadline)
        .await
        .map_err(Error::from)?
        .map_err(Error::Portal)?;

    Ok(IngestionSource::PortalUpload {
        release_file_url: target.public_url,
        release_file_name: name,
        release_file_size: u64::try_from(bytes.len())
            .map_err(|_| Error::Invalid("APK size overflow"))?,
        release_file_hash: Some(hash),
    })
}

/// Target validation is shared by local APK, media and metadata uploads; signed PUT bytes stay exact.
pub(super) async fn request_upload_target<T: Transport<PortalService>, C: Clock>(
    client: &Client<T>,
    clock: &C,
    request: &CreateUploadTarget,
    limit: u64,
) -> Result<PortalUploadTarget, Error> {
    let mut target = deadline::within(client.call(request), clock, limit)
        .await
        .map_err(Error::from)?
        .map_err(Error::Portal)?;
    media::urls::signed_target(&target.upload_url)?;
    target.public_url = media::urls::public(&target.public_url)?;

    if target.public_url == target.upload_url {
        return Err(Error::Invalid("public URL aliases upload capability"));
    }

    Ok(target)
}

async fn read_and_hash<R: SourceReader>(
    reader: &R,
    token: &str,
    maximum: usize,
) -> Result<(Vec<u8>, String), Error> {
    let bytes = reader.read(token, maximum).await.map_err(Error::Source)?;

    if bytes.is_empty() || bytes.len() > maximum {
        return Err(Error::Invalid("APK source byte limit"));
    }

    let hash = sha256(&bytes)?;
    Ok((bytes, hash))
}

fn header(source: &LocalApk, maximum: usize) -> Result<(String, &str), Error> {
    let name = source.file_name.trim();

    if name.is_empty() || name.contains(['/', '\\']) || name.len() > maximum {
        return Err(Error::Invalid("APK name must not contain a local path"));
    }

    let name = if name.to_ascii_lowercase().ends_with(".apk") {
        name.to_owned()
    } else {
        format!("{name}.apk")
    };

    let mime = source
        .content_type
        .as_deref()
        .unwrap_or("application/vnd.android.package-archive");

    if mime.len() > maximum {
        return Err(Error::Invalid("APK media type limit"));
    }

    media::mime::validate(mime)?;
    Ok((name, mime))
}
