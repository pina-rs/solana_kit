//! Explicit Android platform registration. This does not install a socket owner.
use leptoskit_bridge::{Error as BridgeError, MethodId, guest::Client};
use solana_kit_core::crypto::Entropy;
use solana_kit_wallet::mwa::{Association, parse_association_uri};
use std::{error::Error, fmt, future::Future, pin::Pin};

const SERVICE: &str = "solana-wallet-platform";
const MAX_ENTROPY_BYTES: usize = 4096;
const MAX_URI_BYTES: usize = 4096;

/// Platform failures retain the exact upstream bridge code or protocol cause.
#[derive(Debug)]
pub enum WalletPlatformError {
    /// Native capability unavailable, denied, cancelled or failed.
    Bridge(BridgeError),
    /// Association data is outside the supported local v1 contract.
    Association(solana_kit_wallet::mwa::Error),
    /// The native reply does not match the fixed method contract.
    Malformed,
}

impl fmt::Display for WalletPlatformError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("wallet platform operation failed")
    }
}

impl Error for WalletPlatformError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Bridge(error) => Some(error),
            Self::Association(error) => Some(error),
            Self::Malformed => None,
        }
    }
}

/// Application-selected Android Intent and entropy capability.
/// Other platforms must select their supported wallet backend explicitly.
#[derive(Clone, Copy)]
pub struct GuestWalletPlatform(Client);

impl GuestWalletPlatform {
    /// Resolves the real registered Android plugin before any prompt or entropy.
    pub fn new() -> Result<Self, WalletPlatformError> {
        Client::new(SERVICE)
            .map(Self)
            .map_err(WalletPlatformError::Bridge)
    }

    /// Reports a foreground Activity and a resolvable installed wallet.
    pub async fn available(&self) -> Result<bool, WalletPlatformError> {
        match self.request(0, &[]).await?.as_slice() {
            [0] => Ok(false),
            [1] => Ok(true),
            _ => Err(WalletPlatformError::Malformed),
        }
    }

    /// Launches a validated local association from an explicit user action.
    /// Cancelling the call cannot undo an Activity already launched by Android.
    pub async fn launch(&self, uri: &str) -> Result<(), WalletPlatformError> {
        let association = parse_association_uri(uri).map_err(WalletPlatformError::Association)?;

        if uri.len() > MAX_URI_BYTES
            || !matches!(association, Association::Local { protocol, .. } if protocol == "v1")
        {
            return Err(WalletPlatformError::Malformed);
        }

        if !self.request(1, uri.as_bytes()).await?.is_empty() {
            return Err(WalletPlatformError::Malformed);
        }

        Ok(())
    }

    async fn request(&self, method: u32, payload: &[u8]) -> Result<Vec<u8>, WalletPlatformError> {
        let bytes = self
            .0
            .call(MethodId(method), payload)
            .map_err(WalletPlatformError::Bridge)?
            .await
            .map_err(WalletPlatformError::Bridge)?;
        Ok(bytes.as_ref().to_vec())
    }
}

impl Entropy for GuestWalletPlatform {
    type Error = WalletPlatformError;
    type Fill<'a> = Pin<Box<dyn Future<Output = Result<(), Self::Error>> + 'a>>;

    fn fill<'a>(&'a mut self, output: &'a mut [u8]) -> Self::Fill<'a> {
        Box::pin(async move {
            for chunk in output.chunks_mut(MAX_ENTROPY_BYTES) {
                let count =
                    u32::try_from(chunk.len()).map_err(|_| WalletPlatformError::Malformed)?;
                let mut bytes = self.request(2, &count.to_be_bytes()).await?;

                let copied = copy_entropy(chunk, &bytes);
                bytes.fill(0);
                copied?;
            }

            Ok(())
        })
    }
}

fn copy_entropy(output: &mut [u8], bytes: &[u8]) -> Result<(), WalletPlatformError> {
    if bytes.len() != output.len() {
        return Err(WalletPlatformError::Malformed);
    }

    output.copy_from_slice(bytes);
    Ok(())
}
