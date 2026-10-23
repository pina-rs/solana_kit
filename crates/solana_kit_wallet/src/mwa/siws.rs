use super::{Error, SignInPayload, SignInResult};
use alloc::{format, string::String};
use base64::{Engine, engine::general_purpose::STANDARD};
use solana_kit_core::{Address, Signature, crypto::verify};

impl SignInPayload {
    fn fields(&self) -> [(&str, Option<&str>); 8] {
        [
            ("URI", self.uri.as_deref()),
            ("Version", self.version.as_deref()),
            ("Chain ID", self.chain_id.as_deref()),
            ("Nonce", self.nonce.as_deref()),
            ("Issued At", self.issued_at.as_deref()),
            ("Expiration Time", self.expiration_time.as_deref()),
            ("Not Before", self.not_before.as_deref()),
            ("Request ID", self.request_id.as_deref()),
        ]
    }

    fn validate_lines(&self) -> Result<(), Error> {
        let fields = self.fields();
        let injected = [
            self.domain.as_deref(),
            self.address.as_deref(),
            self.statement.as_deref(),
        ]
        .into_iter()
        .chain(fields.iter().map(|(_, value)| *value))
        .flatten()
        .chain(self.resources.iter().flatten().map(String::as_str))
        .any(|value| value.contains(['\r', '\n']));

        if injected {
            return Err(Error::SignIn);
        }

        Ok(())
    }

    /// Creates the pinned SIWS message, rejecting CR or LF in every field.
    pub fn message(&self) -> Result<String, Error> {
        self.validate_lines()?;
        let mut message = format!(
            "{} wants you to sign in with your Solana account:\n{}\n",
            self.domain.as_deref().unwrap_or(""),
            self.address.as_deref().unwrap_or("")
        );

        if let Some(statement) = &self.statement {
            message.push_str(&format!("\n{statement}\n"));
        }

        message.push('\n');
        let fields = self
            .fields()
            .into_iter()
            .filter_map(|(label, value)| value.map(|value| (label, value)));

        for (label, value) in fields {
            message.push_str(&format!("{label}: {value}\n"));
        }

        append_resources(&mut message, self.resources.as_deref().unwrap_or_default());
        Ok(message.trim_end().into())
    }
}

fn append_resources(message: &mut String, resources: &[String]) {
    if resources.is_empty() {
        return;
    }

    message.push_str("Resources:\n");

    for resource in resources {
        message.push_str(&format!("- {resource}\n"));
    }
}

impl SignInResult {
    /// Verifies exact expected SIWS bytes, expected signer, and strict Ed25519.
    /// The caller also owns nonce issuance, expiry policy, and replay storage.
    pub fn verify(&self, expected: &SignInPayload, signer: Address) -> Result<(), Error> {
        if self
            .signature_type
            .as_deref()
            .is_some_and(|kind| kind != "ed25519")
        {
            return Err(Error::SignIn);
        }

        let claimed = expected
            .address
            .as_deref()
            .map(str::parse::<Address>)
            .transpose()
            .map_err(|_| Error::SignIn)?;

        if claimed.is_some_and(|address| address != signer) {
            return Err(Error::SignIn);
        }

        let address: [u8; 32] = STANDARD
            .decode(&self.address)
            .map_err(|_| Error::SignIn)?
            .try_into()
            .map_err(|_| Error::SignIn)?;
        let signature: [u8; 64] = STANDARD
            .decode(&self.signature)
            .map_err(|_| Error::SignIn)?
            .try_into()
            .map_err(|_| Error::SignIn)?;
        let message = STANDARD
            .decode(&self.signed_message)
            .map_err(|_| Error::SignIn)?;

        if signer.as_bytes() != &address || message != expected.message()?.as_bytes() {
            return Err(Error::SignIn);
        }

        verify(&signer, &message, &Signature::from_bytes(signature)).map_err(|_| Error::SignIn)
    }
}
