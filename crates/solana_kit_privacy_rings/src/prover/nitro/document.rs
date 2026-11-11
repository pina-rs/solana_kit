//! Nitro attestation document: COSE_Sign1, CBOR fields, and the AWS chain.

use std::collections::{BTreeMap, HashMap, HashSet};
use std::time::Duration;

use ciborium::value::{Integer, Value};
use coset::{Algorithm, CborSerializable, CoseSign1, TaggedCborSerializable, iana};
use p384::ecdsa::signature::Verifier;
use p384::ecdsa::{Signature, VerifyingKey};
use x509_cert::Certificate;
use x509_cert::der::oid::{AssociatedOid, ObjectIdentifier};
use x509_cert::der::{Decode, Encode};
use x509_cert::ext::pkix::{BasicConstraints, KeyUsage};
use x509_cert::time::Time;

use super::scan::scan;
use crate::Error;

const COSE_SIGN1_TAG: u64 = 18;
const MAX_FIELDS: usize = 16;
const MAX_PCRS: usize = 32;
const MAX_BUNDLE: usize = 8;
const NOT_BEFORE_SKEW: u64 = 300;
const PCR_LEN: usize = 48;
const REPORT_LEN: usize = 64;

pub(super) struct MeasuredDocument {
    pub pcr0: [u8; PCR_LEN],
    pub pcr1: [u8; PCR_LEN],
    pub pcr2: [u8; PCR_LEN],
    pub nonce: Option<Vec<u8>>,
    pub public_key: Option<Vec<u8>>,
    pub report_data: [u8; REPORT_LEN],
}

pub(super) fn inspect_document(
    bytes: &[u8],
    root: &[u8],
    now_secs: u64,
) -> Result<MeasuredDocument, Error> {
    scan(bytes, Some(COSE_SIGN1_TAG))?;
    let sign1 = read_sign1(bytes)?;
    let protected = sign1.protected.original_data.as_deref().unwrap_or(&[]);
    scan(protected, None)?;

    if sign1.protected.header.alg != Some(Algorithm::Assigned(iana::Algorithm::ES384)) {
        return Err(Error::Attestation("algorithm"));
    }

    let payload = sign1
        .payload
        .as_deref()
        .ok_or(Error::Attestation("payload"))?;
    let document = AttestationDocument::decode(payload)?;
    let signer = document.leaf_key(root, now_secs)?;
    let signature =
        Signature::from_slice(&sign1.signature).map_err(|_| Error::Attestation("signature"))?;
    signer
        .verify(&sign1.tbs_data(b""), &signature)
        .map_err(|_| Error::Attestation("signature"))?;

    if document.digest != "SHA384" {
        return Err(Error::Attestation("digest"));
    }

    let pcr0 = document.pcr(0)?;

    if pcr0 == [0; PCR_LEN] {
        return Err(Error::Attestation("debug"));
    }

    let report_data = document
        .user_data
        .as_deref()
        .and_then(|bytes| bytes.try_into().ok())
        .ok_or(Error::Attestation("report"))?;
    Ok(MeasuredDocument {
        pcr0,
        pcr1: document.pcr(1)?,
        pcr2: document.pcr(2)?,
        nonce: document.nonce,
        public_key: document.public_key,
        report_data,
    })
}

fn read_sign1(bytes: &[u8]) -> Result<CoseSign1, Error> {
    CoseSign1::from_tagged_slice(bytes)
        .or_else(|_| CoseSign1::from_slice(bytes))
        .map_err(|_| Error::Attestation("cose"))
}

struct AttestationDocument {
    digest: String,
    pcrs: BTreeMap<usize, Vec<u8>>,
    certificate: Vec<u8>,
    cabundle: Vec<Vec<u8>>,
    public_key: Option<Vec<u8>>,
    user_data: Option<Vec<u8>>,
    nonce: Option<Vec<u8>>,
}

impl AttestationDocument {
    fn decode(document: &[u8]) -> Result<Self, Error> {
        scan(document, None)?;
        let value: Value =
            ciborium::from_reader(document).map_err(|_| Error::Attestation("document"))?;
        let Value::Map(entries) = value else {
            return Err(Error::Attestation("document"));
        };
        let mut fields = Fields::new(entries)?;

        if fields.text("module_id")?.is_empty() {
            return Err(Error::Attestation("module"));
        }

        match fields.take("timestamp")? {
            Value::Integer(timestamp)
                if u64::try_from(timestamp).is_ok_and(|value| value < (1 << 53)) => {}
            _ => return Err(Error::Attestation("timestamp")),
        }

        let document = Self {
            digest: fields.text("digest")?,
            pcrs: pcrs(fields.take("pcrs")?)?,
            certificate: fields.bytes("certificate")?,
            cabundle: cabundle(fields.take("cabundle")?)?,
            public_key: fields.optional_bytes("public_key")?,
            user_data: fields.optional_bytes("user_data")?,
            nonce: fields.optional_bytes("nonce")?,
        };

        if !fields
            .named
            .values()
            .chain(&fields.numbered)
            .all(keys_unique)
        {
            return Err(Error::Attestation("keys"));
        }
        Ok(document)
    }

    fn leaf_key(&self, root: &[u8], now_secs: u64) -> Result<VerifyingKey, Error> {
        if self.cabundle.first().map(Vec::as_slice) != Some(root) {
            return Err(Error::Attestation("root"));
        }

        let leaf = certificate(&self.certificate)?;
        let bundle = self
            .cabundle
            .iter()
            .map(|der| certificate(der))
            .collect::<Result<Vec<_>, _>>()?;
        let mut path = Vec::with_capacity(bundle.len() + 1);
        path.push(leaf);
        path.extend(bundle.into_iter().rev());

        for cert in &path {
            within_validity(cert, now_secs)?;
            known_critical(cert)?;
        }

        if key_usage(&path[0])?.is_some_and(|usage| !usage.digital_signature()) {
            return Err(Error::Attestation("key usage"));
        }

        for (below, pair) in path.iter().zip(path.iter().skip(1)).enumerate() {
            may_issue(pair.1, below)?;
            verify_signed_by(pair.0, pair.1)?;
        }

        public_key(&path[0])
    }

    fn pcr(&self, index: usize) -> Result<[u8; PCR_LEN], Error> {
        self.pcrs
            .get(&index)
            .and_then(|value| value.as_slice().try_into().ok())
            .ok_or(Error::Attestation("pcr"))
    }
}

struct Fields {
    named: HashMap<String, Value>,
    numbered: Vec<Value>,
}

impl Fields {
    fn new(entries: Vec<(Value, Value)>) -> Result<Self, Error> {
        if entries.len() > MAX_FIELDS {
            return Err(Error::Attestation("fields"));
        }

        if !unique_keys(&entries) {
            return Err(Error::Attestation("keys"));
        }

        let mut fields = Self {
            named: HashMap::with_capacity(entries.len()),
            numbered: Vec::new(),
        };

        for (key, value) in entries {
            match key {
                Value::Text(key) => {
                    fields.named.insert(key, value);
                }
                _ => fields.numbered.push(value),
            }
        }

        Ok(fields)
    }

    fn take(&mut self, name: &str) -> Result<Value, Error> {
        self.named.remove(name).ok_or(Error::Attestation("field"))
    }

    fn text(&mut self, name: &str) -> Result<String, Error> {
        match self.take(name)? {
            Value::Text(text) => Ok(text),
            _ => Err(Error::Attestation("field")),
        }
    }

    fn bytes(&mut self, name: &str) -> Result<Vec<u8>, Error> {
        byte_string(self.take(name)?, name)
    }

    fn optional_bytes(&mut self, name: &str) -> Result<Option<Vec<u8>>, Error> {
        match self.named.remove(name) {
            None | Some(Value::Null) => Ok(None),
            Some(value) => byte_string(value, name).map(Some),
        }
    }
}

fn pcrs(value: Value) -> Result<BTreeMap<usize, Vec<u8>>, Error> {
    let Value::Map(entries) = value else {
        return Err(Error::Attestation("pcrs"));
    };

    if entries.len() > MAX_PCRS {
        return Err(Error::Attestation("pcrs"));
    }

    let mut pcrs = BTreeMap::new();

    for (index, pcr) in entries {
        let Some(index) = pcr_index(index) else {
            return Err(Error::Attestation("pcrs"));
        };

        if pcrs.insert(index, byte_string(pcr, "pcr")?).is_some() {
            return Err(Error::Attestation("pcrs"));
        }
    }

    Ok(pcrs)
}

fn pcr_index(index: Value) -> Option<usize> {
    let Value::Integer(index) = index else {
        return None;
    };
    let index = u64::try_from(index).ok()?;
    usize::try_from(index)
        .ok()
        .filter(|index| *index < MAX_PCRS)
}

fn cabundle(value: Value) -> Result<Vec<Vec<u8>>, Error> {
    let Value::Array(certificates) = value else {
        return Err(Error::Attestation("cabundle"));
    };

    if certificates.len() > MAX_BUNDLE {
        return Err(Error::Attestation("cabundle"));
    }

    certificates
        .into_iter()
        .map(|certificate| byte_string(certificate, "cabundle"))
        .collect()
}

fn byte_string(value: Value, _name: &str) -> Result<Vec<u8>, Error> {
    match value {
        Value::Bytes(bytes) => Ok(bytes),
        _ => Err(Error::Attestation("bytes")),
    }
}

#[derive(PartialEq, Eq, Hash)]
enum Key<'a> {
    Integer(Integer),
    Text(&'a str),
}

fn keys_unique(value: &Value) -> bool {
    match value {
        Value::Array(items) => items.iter().all(keys_unique),
        Value::Map(entries) => {
            unique_keys(entries) && entries.iter().all(|(_, value)| keys_unique(value))
        }
        _ => true,
    }
}

fn unique_keys(entries: &[(Value, Value)]) -> bool {
    let mut seen = HashSet::with_capacity(entries.len());
    entries.iter().all(|(key, _)| {
        let key = match key {
            Value::Integer(integer) => Key::Integer(*integer),
            Value::Text(text) => Key::Text(text),
            _ => return false,
        };

        seen.insert(key)
    })
}

fn certificate(der: &[u8]) -> Result<Certificate, Error> {
    Certificate::from_der(der).map_err(|_| Error::Attestation("certificate"))
}

fn within_validity(cert: &Certificate, now_secs: u64) -> Result<(), Error> {
    let validity = cert.tbs_certificate().validity();
    let now = Duration::from_secs(now_secs);
    let latest = Duration::from_secs(now_secs.saturating_add(NOT_BEFORE_SKEW));
    let not_before = Time::to_unix_duration(validity.not_before);
    let not_after = Time::to_unix_duration(validity.not_after);

    if latest < not_before || now > not_after {
        return Err(Error::Attestation("validity"));
    }

    Ok(())
}

fn known_critical(cert: &Certificate) -> Result<(), Error> {
    let known = [BasicConstraints::OID, KeyUsage::OID];
    let unknown = cert
        .tbs_certificate()
        .extensions()
        .into_iter()
        .flatten()
        .any(|extension| extension.critical && !known.contains(&extension.extn_id));

    if unknown {
        return Err(Error::Attestation("extension"));
    }

    Ok(())
}

fn verify_signed_by(child: &Certificate, parent: &Certificate) -> Result<(), Error> {
    let tbs = child.tbs_certificate();

    if tbs.issuer() != parent.tbs_certificate().subject() {
        return Err(Error::Attestation("issuer"));
    }

    if child.signature_algorithm().oid != ecdsa_oid()
        || tbs.signature().oid != child.signature_algorithm().oid
    {
        return Err(Error::Attestation("certificate signature"));
    }
    let signature = child
        .signature()
        .as_bytes()
        .and_then(|der| Signature::from_der(der).ok())
        .ok_or(Error::Attestation("certificate signature"))?;
    let signed = tbs
        .to_der()
        .map_err(|_| Error::Attestation("certificate"))?;

    public_key(parent)?
        .verify(&signed, &signature)
        .map_err(|_| Error::Attestation("certificate signature"))
}

fn may_issue(parent: &Certificate, below: usize) -> Result<(), Error> {
    let constraints = parent
        .tbs_certificate()
        .get_extension::<BasicConstraints>()
        .map_err(|_| Error::Attestation("constraints"))?;
    let is_ca = constraints.is_some_and(|(_, constraints)| {
        constraints.ca
            && constraints
                .path_len_constraint
                .is_none_or(|limit| usize::from(limit) >= below)
    });

    if !is_ca {
        return Err(Error::Attestation("ca"));
    }

    if key_usage(parent)?.is_some_and(|usage| !usage.key_cert_sign()) {
        return Err(Error::Attestation("key usage"));
    }

    Ok(())
}

fn key_usage(cert: &Certificate) -> Result<Option<KeyUsage>, Error> {
    cert.tbs_certificate()
        .get_extension::<KeyUsage>()
        .map(|usage| usage.map(|(_, usage)| usage))
        .map_err(|_| Error::Attestation("key usage"))
}

fn public_key(cert: &Certificate) -> Result<VerifyingKey, Error> {
    let spki = cert.tbs_certificate().subject_public_key_info();

    if spki.algorithm.oid != ec_oid() || curve_oid(spki) != Some(p384_oid()) {
        return Err(Error::Attestation("p384"));
    }

    spki.subject_public_key
        .as_bytes()
        .and_then(|sec1| VerifyingKey::from_sec1_bytes(sec1).ok())
        .ok_or(Error::Attestation("p384"))
}

fn curve_oid(spki: &x509_cert::spki::SubjectPublicKeyInfoOwned) -> Option<ObjectIdentifier> {
    spki.algorithm
        .parameters
        .as_ref()
        .and_then(|curve| curve.decode_as::<ObjectIdentifier>().ok())
}

fn ecdsa_oid() -> ObjectIdentifier {
    ObjectIdentifier::new("1.2.840.10045.4.3.3").expect("oid")
}

fn ec_oid() -> ObjectIdentifier {
    ObjectIdentifier::new("1.2.840.10045.2.1").expect("oid")
}

fn p384_oid() -> ObjectIdentifier {
    ObjectIdentifier::new("1.3.132.0.34").expect("oid")
}
