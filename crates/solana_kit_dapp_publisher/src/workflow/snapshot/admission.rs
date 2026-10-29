//! Checkpoint bytes are recovery hints, never permission to skip identity or effect validation.

use super::{Effect, Snapshot};
use crate::workflow::{
    Error,
    media::{mime, urls},
    timestamp,
};

const SHA256_HEX_DIGITS: usize = 64;

pub(super) fn validate(snapshot: &Snapshot) -> Result<(), Error> {
    identities(snapshot)?;
    chronology(snapshot)?;
    intent(snapshot)?;

    for receipt in &snapshot.uploads {
        artifact(&receipt.hash, &receipt.mime, &receipt.extension)?;
        urls::public(&receipt.public_url)?;
    }

    if let Some(uri) = &snapshot.metadata_uri {
        urls::public(uri)?;
    }

    Ok(())
}

fn identities(snapshot: &Snapshot) -> Result<(), Error> {
    let identities = [
        &snapshot.idempotency_key,
        &snapshot.ingestion_id,
        &snapshot.release_id,
        &snapshot.publication_id,
    ];

    if identities.iter().any(|field| {
        field
            .as_deref()
            .is_some_and(|value| value.trim().is_empty())
    }) {
        return Err(Error::Invalid("empty checkpoint identity"));
    }

    if snapshot.input_hash.is_some() != snapshot.idempotency_key.is_some() {
        return Err(Error::Invalid("incomplete ingestion fingerprint"));
    }

    if let Some(hash) = &snapshot.input_hash {
        digest(hash)?;
    }

    Ok(())
}

fn chronology(snapshot: &Snapshot) -> Result<(), Error> {
    if snapshot
        .updated_on
        .as_deref()
        .is_some_and(|value| !timestamp::valid(value))
    {
        return Err(Error::Invalid("invalid checkpoint timestamp"));
    }

    if snapshot.deleted && snapshot.preserved_submitted {
        return Err(Error::Invalid("conflicting cleanup outcomes"));
    }

    Ok(())
}

/// A pending PUT needs a known public target; no pending branch authorizes another mutation.
fn intent(snapshot: &Snapshot) -> Result<(), Error> {
    match (snapshot.pending, &snapshot.pending_upload) {
        (Some(Effect::CreateUploadTarget | Effect::Upload), Some(upload)) => {
            artifact(&upload.hash, &upload.mime, &upload.extension)?;

            if let Some(url) = &upload.public_url {
                urls::public(url)?;
            }

            if snapshot.pending == Some(Effect::Upload) && upload.public_url.is_none() {
                return Err(Error::Invalid("upload intent lacks public target"));
            }
        }
        (Some(Effect::CreateUploadTarget | Effect::Upload), None) | (_, Some(_)) => {
            return Err(Error::Invalid("conflicting upload intent"));
        }
        _ => {}
    }

    Ok(())
}

fn artifact(hash: &str, content_type: &str, extension: &str) -> Result<(), Error> {
    digest(hash)?;
    mime::validate(content_type)?;

    if extension.is_empty() || !extension.bytes().all(|byte| byte.is_ascii_alphanumeric()) {
        return Err(Error::Invalid("invalid checkpoint file extension"));
    }

    Ok(())
}

fn digest(value: &str) -> Result<(), Error> {
    if value.len() != SHA256_HEX_DIGITS
        || !value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
    {
        return Err(Error::Invalid("invalid checkpoint digest"));
    }

    Ok(())
}
