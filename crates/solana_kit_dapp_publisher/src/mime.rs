//! File-name MIME inference shared by normalized projections and metadata generation.

/// Infers a media type from the final name extension, defaulting to bytes.
///
/// The table mirrors the pinned publisher source. Names are inspected literally:
/// no URL decoding, filesystem access or content sniffing occurs.
pub(crate) fn infer_mime(name: &str) -> &'static str {
    match name
        .rsplit('.')
        .next()
        .unwrap_or("")
        .to_ascii_lowercase()
        .as_str()
    {
        "apk" => "application/vnd.android.package-archive",
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "webp" => "image/webp",
        "gif" => "image/gif",
        "svg" => "image/svg+xml",
        "mp4" => "video/mp4",
        "webm" => "video/webm",
        "mov" => "video/quicktime",
        "json" => "application/json",
        _ => "application/octet-stream",
    }
}
