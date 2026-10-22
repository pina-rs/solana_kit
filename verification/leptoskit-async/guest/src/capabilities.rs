//! Private capability semantics layered on the actual upstream guest Call future.

use crate::application::record;
use futures::future::poll_fn;
use leptoskit_bridge::guest::{Call, Client};
use leptoskit_bridge::{Bytes, Error, MethodId};
use std::future::Future;
use std::pin::Pin;

/// Keeps transport failure distinct from an incomplete entropy response.
#[derive(Debug, PartialEq, Eq)]
pub(super) enum EntropyError {
    Bridge(Error),
    Length,
}

/// Requires actual Pending before allowing a host completion to count as proof.
pub(super) async fn pending(mut call: Call, marker: &'static str) -> Result<Bytes, Error> {
    let mut first = true;

    poll_fn(move |context| {
        let result = Pin::new(&mut call).poll(context);

        if first {
            assert!(result.is_pending(), "host completed synchronously");
            first = false;
            record(marker);
        }

        result
    })
    .await
}

/// Writes the caller's complete slice only after exact-length validation.
pub(super) async fn entropy(
    client: Client,
    output: &mut [u8; 4],
    marker: &'static str,
) -> Result<(), EntropyError> {
    let call = client
        .call(MethodId(0), &[4])
        .map_err(EntropyError::Bridge)?;
    let bytes = pending(call, marker).await.map_err(EntropyError::Bridge)?;

    if bytes.len() != output.len() {
        return Err(EntropyError::Length);
    }

    output.copy_from_slice(&bytes);
    Ok(())
}
