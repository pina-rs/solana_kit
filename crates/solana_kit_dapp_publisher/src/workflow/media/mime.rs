//! MIME and extension fallbacks mirror the pinned source without accepting header injection.

use crate::workflow::Error;

pub(in crate::workflow) use crate::mime::infer_mime as infer;

pub(crate) fn validate(mime: &str) -> Result<(), Error> {
    if mime.is_empty() || mime.bytes().any(|byte| byte.is_ascii_control()) || !mime.contains('/') {
        return Err(Error::Invalid("invalid media type"));
    }

    Ok(())
}

pub(in crate::workflow) fn normalized(value: &str) -> Option<String> {
    let value = value.split(';').next()?.trim().to_ascii_lowercase();
    (!value.is_empty()).then_some(value)
}

/// A plain name extension wins; otherwise a cleaned MIME subtype falls back to bin.
pub(in crate::workflow) fn extension(name: &str, mime: &str) -> String {
    if let Some((_, extension)) = name.rsplit_once('.') {
        let extension = extension.trim();

        if !extension.is_empty() && extension.bytes().all(|byte| byte.is_ascii_alphanumeric()) {
            return extension.to_ascii_lowercase();
        }
    }

    let extension = match mime {
        "image/jpeg" => "jpg".into(),
        "image/svg+xml" => "svg".into(),
        "video/quicktime" => "mov".into(),
        _ => mime
            .split('/')
            .nth(1)
            .unwrap_or("bin")
            .split('+')
            .next()
            .unwrap_or("bin")
            .chars()
            .filter(char::is_ascii_alphanumeric)
            .collect::<String>(),
    };

    if extension.is_empty() {
        return "bin".into();
    }

    extension
}
