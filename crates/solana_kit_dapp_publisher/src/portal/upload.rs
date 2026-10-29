//! Presigned binary uploads have a separate caller-owned authority and transport.

use super::{Error, PortalUploadTarget};
use solana_kit_client::{TransportError, rest};
use std::future::Future;

/// One bounded binary PUT request. This contains no portal credential or PortalService.
pub struct UploadRequest<'a> {
    /// Presigned target independently approved by the caller's uploader.
    pub target: &'a PortalUploadTarget,
    /// Exact caller-owned file bytes.
    pub body: &'a [u8],
    /// Explicit upload content type.
    pub content_type: &'a str,
    /// Maximum body bytes admitted before the upload begins.
    pub max_bytes: usize,
}

/// Independently configured binary uploader; portal credentials must never be forwarded.
///
/// Implementations approve the target origin, deny redirects, bound any response,
/// send one PUT with the supplied media type and cancel owned work on future Drop.
/// No concrete network uploader is included in this slice.
pub trait Uploader {
    /// Concrete local or native Send-capable future, without an imposed executor.
    type Upload<'a>: Future<Output = Result<(), TransportError>> + 'a
    where
        Self: 'a;
    /// Uploads exact bytes with no automatic retry or detached task.
    fn upload<'a>(&'a self, request: UploadRequest<'a>) -> Self::Upload<'a>;
}

/// Performs one explicitly requested upload through a separately granted uploader.
pub async fn upload_bytes<U: Uploader>(
    uploader: &U,
    request: UploadRequest<'_>,
) -> Result<(), Error> {
    if request.body.len() > request.max_bytes {
        return Err(Error::Request("upload byte limit exceeded"));
    }

    if request
        .content_type
        .bytes()
        .any(|byte| byte == b'\r' || byte == b'\n')
    {
        return Err(Error::Request("invalid upload content type"));
    }

    uploader
        .upload(request)
        .await
        .map_err(|error| Error::Transport(rest::redact_error(error)))
}
