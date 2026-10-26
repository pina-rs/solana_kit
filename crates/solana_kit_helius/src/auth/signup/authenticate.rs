//! Authentication finishes before the caller binds the resulting developer session.

use super::SignupError;
use crate::{
    auth::{AuthenticatedIdentity, SignupAuthentication},
    service::{Service, ServiceClient, ServiceTransport},
};

pub(super) async fn authenticate<T: ServiceTransport<Service>>(
    client: &ServiceClient<T>,
    authentication: &SignupAuthentication,
    timestamp: u64,
) -> Result<AuthenticatedIdentity, SignupError> {
    match authentication {
        SignupAuthentication::Authenticated {
            jwt,
            ref_id,
            wallet_address,
        } => {
            if jwt.expose().is_empty() || ref_id.is_empty() {
                return Err(SignupError::InvalidIdentity);
            }

            Ok(AuthenticatedIdentity {
                jwt: jwt.clone(),
                ref_id: ref_id.clone(),
                wallet_address: *wallet_address,
            })
        }
        SignupAuthentication::SecretKey(secret) => secret_identity(client, secret, timestamp).await,
    }
}

#[cfg(not(feature = "crypto"))]
async fn secret_identity<T: ServiceTransport<Service>>(
    _client: &ServiceClient<T>,
    _secret: &crate::SensitiveString,
    _timestamp: u64,
) -> Result<AuthenticatedIdentity, SignupError> {
    Err(SignupError::CryptoUnavailable)
}

#[cfg(feature = "crypto")]
async fn secret_identity<T: ServiceTransport<Service>>(
    client: &ServiceClient<T>,
    secret: &crate::SensitiveString,
    timestamp: u64,
) -> Result<AuthenticatedIdentity, SignupError> {
    let (wallet_address, signed) = {
        let bytes = key_bytes(secret)?;
        let keypair = solana_kit_core::crypto::Keypair::from_bytes(bytes.as_ref())
            .map_err(|_| SignupError::InvalidSecretKey)?;
        let signed = crate::auth::sign_auth_message(&keypair, None, timestamp)
            .map_err(crate::service::Error::Schema)?;
        (keypair.address(), signed)
    };
    let response = client
        .call(crate::auth::DeveloperWalletSignup {
            message: signed.message,
            signature: signed.signature.to_string(),
            wallet_address: wallet_address.to_string(),
        })
        .await?;

    Ok(AuthenticatedIdentity {
        jwt: response.token,
        ref_id: response.ref_id,
        wallet_address,
    })
}

#[cfg(feature = "crypto")]
fn key_bytes(secret: &crate::SensitiveString) -> Result<zeroize::Zeroizing<[u8; 64]>, SignupError> {
    use base64::{Engine, alphabet, engine::general_purpose};

    let mut encoded = zeroize::Zeroizing::new([0u8; 88]);
    let mut bytes = zeroize::Zeroizing::new([0u8; 64]);
    let mut length = 0;

    for (index, byte) in percent_encoding::percent_decode_str(secret.expose()).enumerate() {
        let slot = encoded
            .get_mut(index)
            .ok_or(SignupError::InvalidSecretKey)?;
        *slot = match byte {
            b'-' => b'+',
            b'_' => b'/',
            byte => byte,
        };
        length = index + 1;
    }

    let config = general_purpose::GeneralPurposeConfig::new()
        .with_decode_padding_mode(base64::engine::DecodePaddingMode::Indifferent);
    let engine = general_purpose::GeneralPurpose::new(&alphabet::STANDARD, config);
    let decoded = engine
        .decode_slice(&encoded[..length], bytes.as_mut())
        .map_err(|_| SignupError::InvalidSecretKey)?;

    if decoded != bytes.len() {
        return Err(SignupError::InvalidSecretKey);
    }

    Ok(bytes)
}
