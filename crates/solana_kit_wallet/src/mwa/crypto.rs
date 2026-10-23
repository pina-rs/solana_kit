use alloc::{format, string::String, vec::Vec};
use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use hkdf::Hkdf;
use p256::{
    PublicKey, SecretKey,
    ecdh::diffie_hellman,
    ecdsa::{
        Signature, SigningKey, VerifyingKey,
        signature::{Signer, Verifier},
    },
    elliptic_curve::sec1::ToEncodedPoint,
};
use sha2::Sha256;
use zeroize::Zeroizing;

use super::Error;

/// A validated, finite, canonical uncompressed P-256 public point.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PublicPoint(PublicKey);

impl PublicPoint {
    /// Parses exactly 65 X9.62 bytes. Compressed and hybrid forms are rejected.
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, Error> {
        if bytes.len() != 65 || bytes.first() != Some(&4) {
            return Err(Error::PublicKey);
        }

        PublicKey::from_sec1_bytes(bytes)
            .map(Self)
            .map_err(|_| Error::PublicKey)
    }

    /// Exports the canonical uncompressed point.
    pub fn to_bytes(&self) -> [u8; 65] {
        let mut output = [0; 65];
        output.copy_from_slice(self.0.to_encoded_point(false).as_bytes());
        output
    }

    /// Returns the unpadded base64url association token.
    pub fn association_token(&self) -> String {
        URL_SAFE_NO_PAD.encode(self.to_bytes())
    }

    /// Verifies SHA-256 ECDSA with a 64-byte P1363 signature.
    pub fn verify(&self, message: &[u8], signature: &[u8]) -> Result<(), Error> {
        let signature = Signature::from_slice(signature).map_err(|_| Error::Signature)?;
        VerifyingKey::from(self.0)
            .verify(message, &signature)
            .map_err(|_| Error::Signature)
    }

    /// Authenticates compact JWS using only the pinned ES256 header contract.
    /// Unsupported algorithms, critical extensions, and padded encodings fail.
    pub fn verify_jws(
        &self,
        compact: &str,
    ) -> Result<serde_json::Map<String, serde_json::Value>, Error> {
        let mut parts = compact.split('.');
        let header = parts.next().ok_or(Error::Signature)?;
        let payload = parts.next().ok_or(Error::Signature)?;
        let signature = parts.next().ok_or(Error::Signature)?;

        if header != "eyJhbGciOiJFUzI1NiJ9" || parts.next().is_some() {
            return Err(Error::Signature);
        }

        let signature = URL_SAFE_NO_PAD
            .decode(signature)
            .map_err(|_| Error::Signature)?;
        self.verify(format!("{header}.{payload}").as_bytes(), &signature)?;
        let payload = URL_SAFE_NO_PAD
            .decode(payload)
            .map_err(|_| Error::Signature)?;
        serde_json::from_slice(&payload).map_err(|_| Error::Json)
    }
}

/// Association signing key. Its private scalar zeroizes on drop and has no Debug.
pub struct AssociationKey(SigningKey);

impl AssociationKey {
    /// Imports a caller-owned 32-byte P-256 private scalar.
    pub fn from_scalar(bytes: &[u8; 32]) -> Result<Self, Error> {
        SigningKey::from_slice(bytes)
            .map(Self)
            .map_err(|_| Error::PrivateKey)
    }

    /// Returns the association verification point.
    pub fn public_key(&self) -> PublicPoint {
        PublicPoint(PublicKey::from(self.0.verifying_key()))
    }

    /// Signs with deterministic RFC6979 ECDSA-SHA256, encoded as r || s.
    pub fn sign(&self, bytes: &[u8]) -> [u8; 64] {
        let signature: Signature = self.0.sign(bytes);
        signature.to_bytes().into()
    }

    /// Creates compact ES256 JWS over an exact JSON payload.
    pub fn jws(
        &self,
        payload: &serde_json::Map<String, serde_json::Value>,
    ) -> Result<String, Error> {
        let payload = serde_json::to_vec(payload).map_err(|_| Error::Json)?;
        let signing_input = format!("eyJhbGciOiJFUzI1NiJ9.{}", URL_SAFE_NO_PAD.encode(payload));
        let signature = URL_SAFE_NO_PAD.encode(self.sign(signing_input.as_bytes()));
        Ok(format!("{signing_input}.{signature}"))
    }
}

/// Ephemeral agreement key. Its private scalar zeroizes on drop and has no Debug.
pub struct AgreementKey(SecretKey);

impl AgreementKey {
    /// Imports a caller-owned scalar. Generate a fresh scalar for each session.
    pub fn from_scalar(bytes: &[u8; 32]) -> Result<Self, Error> {
        SecretKey::from_slice(bytes)
            .map(Self)
            .map_err(|_| Error::PrivateKey)
    }

    /// Returns this agreement key's public point.
    pub fn public_key(&self) -> PublicPoint {
        PublicPoint(self.0.public_key())
    }

    /// Derives the AES-128 session key with the association point as HKDF salt.
    pub fn session_key(&self, peer: &PublicPoint, association: &PublicPoint) -> SessionKey {
        let shared = diffie_hellman(self.0.to_nonzero_scalar(), peer.0.as_affine());
        let hkdf = Hkdf::<Sha256>::new(Some(&association.to_bytes()), shared.raw_secret_bytes());
        let mut output = Zeroizing::new([0; 16]);
        hkdf.expand(&[], output.as_mut())
            .expect("16 bytes is within HKDF output limit");
        SessionKey(output)
    }

    /// Signs the ephemeral point to produce the exact 129-byte HELLO_REQ.
    pub fn hello_request(&self, association: &AssociationKey) -> [u8; 129] {
        let public = self.public_key().to_bytes();
        let mut output = [0; 129];
        output[..65].copy_from_slice(&public);
        output[65..].copy_from_slice(&association.sign(&public));
        output
    }

    /// Consumes the ephemeral key after importing the peer HELLO_RSP point.
    /// Remaining bytes are returned for authenticated session-property parsing.
    pub fn accept_hello(
        self,
        payload: &[u8],
        association: &PublicPoint,
    ) -> Result<(SessionKey, Option<Vec<u8>>), Error> {
        let public = payload.get(..65).ok_or(Error::PublicKey)?;
        let key = self.session_key(&PublicPoint::from_bytes(public)?, association);
        let properties = (payload.len() > 65).then(|| payload[65..].to_vec());
        Ok((key, properties))
    }
}

/// AES-128 key material. It has no export, clone, Display, or Debug contract.
pub struct SessionKey(pub(super) Zeroizing<[u8; 16]>);

impl SessionKey {
    /// Imports a caller-owned key, for restored sessions or independent fixtures.
    pub fn from_bytes(bytes: &[u8; 16]) -> Self {
        Self(Zeroizing::new(*bytes))
    }
}

/// Authenticates and parses a HELLO_REQ before using its agreement point.
pub fn verify_hello_request(
    payload: &[u8],
    association: &PublicPoint,
) -> Result<PublicPoint, Error> {
    if payload.len() != 129 {
        return Err(Error::Signature);
    }

    association.verify(&payload[..65], &payload[65..])?;
    PublicPoint::from_bytes(&payload[..65])
}

#[cfg(test)]
mod tests {
    use super::*;

    fn scalar(value: u8) -> [u8; 32] {
        let mut bytes = [0; 32];
        bytes[31] = value;
        bytes
    }

    fn hex(bytes: &[u8]) -> String {
        bytes.iter().map(|byte| format!("{byte:02x}")).collect()
    }

    #[test]
    fn raw_ecdh_and_session_key_match_independent_dart_oracle() {
        let vectors: serde_json::Value =
            serde_json::from_str(include_str!("../../tests/fixtures/mwa/dart.json")).unwrap();
        let alice = AgreementKey::from_scalar(&scalar(2)).unwrap();
        let bob = AgreementKey::from_scalar(&scalar(3)).unwrap();
        let association = AssociationKey::from_scalar(&scalar(1))
            .unwrap()
            .public_key();
        let shared = diffie_hellman(alice.0.to_nonzero_scalar(), bob.public_key().0.as_affine());
        assert_eq!(hex(shared.raw_secret_bytes()), vectors["shared"]);
        assert_eq!(
            hex(alice
                .session_key(&bob.public_key(), &association)
                .0
                .as_ref()),
            vectors["key"]
        );
    }
}
