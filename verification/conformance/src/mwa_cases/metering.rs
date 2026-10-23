//! Fixed backend call-site entries of the original public MWA tests.

const P256_SIGNING_SCALAR_IMPORT_PUBLIC_DERIVE: u64 = 43;
const P256_AGREEMENT_SCALAR_IMPORT: u64 = 12;
const P256_AGREEMENT_PUBLIC_DERIVE: u64 = 14;
const P256_POINT_PARSE: u64 = 32;
const P256_SIGN: u64 = 16;
const P256_VERIFY: u64 = 16;
const P256_ECDH: u64 = 7;
const HKDF_EXTRACT: u64 = 7;
const HKDF_EXPAND: u64 = 7;
const AES_KEY_INIT: u64 = 204;
const AES_GCM_ENCRYPT: u64 = 20;
const AES_GCM_DECRYPT: u64 = 184;
const ED25519_KEY_DERIVE: u64 = 3;
const ED25519_PUBLIC_PARSE: u64 = 3;
const ED25519_SIGN: u64 = 2;
const ED25519_VERIFY: u64 = 3;

/// Fixed backend call-site entries measured before adapting the original tests.
/// Constructor attempts, initialization, and subsequent operations are separate.
pub const BACKEND_WORK: [(&str, u64); 16] = [
    (
        "p256-signing-scalar-import-public-derive",
        P256_SIGNING_SCALAR_IMPORT_PUBLIC_DERIVE,
    ),
    ("p256-agreement-scalar-import", P256_AGREEMENT_SCALAR_IMPORT),
    ("p256-agreement-public-derive", P256_AGREEMENT_PUBLIC_DERIVE),
    ("p256-point-parse", P256_POINT_PARSE),
    ("p256-sign", P256_SIGN),
    ("p256-verify", P256_VERIFY),
    ("p256-ecdh", P256_ECDH),
    ("hkdf-extract", HKDF_EXTRACT),
    ("hkdf-expand", HKDF_EXPAND),
    ("aes-key-init", AES_KEY_INIT),
    ("aes-gcm-encrypt", AES_GCM_ENCRYPT),
    ("aes-gcm-decrypt", AES_GCM_DECRYPT),
    ("ed25519-key-derive", ED25519_KEY_DERIVE),
    ("ed25519-public-parse", ED25519_PUBLIC_PARSE),
    ("ed25519-sign", ED25519_SIGN),
    ("ed25519-verify", ED25519_VERIFY),
];
