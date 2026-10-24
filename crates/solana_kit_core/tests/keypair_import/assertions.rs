//! Public import boundaries using RFC 8032 section 7.1 tests 1 and 2.

use alloc::{format, string::ToString};

use solana_kit_core::{
    Signature,
    crypto::{Keypair, KeypairImportError, verify},
};

struct Vector {
    seed: &'static str,
    public: &'static str,
    message: &'static [u8],
    signature: &'static str,
}

// These are the fixed standards vectors already owned by conformance/cryptography.rs.
const VECTORS: [Vector; 2] = [
    Vector {
        seed: "9d61b19deffd5a60ba844af492ec2cc44449c5697b326919703bac031cae7f60",
        public: "d75a980182b10ab7d54bfed3c964073a0ee172f3daa62325af021a68f707511a",
        message: b"",
        signature: concat!(
            "e5564300c360ac729086e2cc806e828a84877f1eb8e5d974d873e06522490155",
            "5fb8821590a33bacc61e39701cf9b46bd25bf5f0595bbe24655141438e7a100b",
        ),
    },
    Vector {
        seed: "4ccd089b28ff96da9db6c346ec114e0f5b8a319f35aba624da8cf6ed4fb8a6fb",
        public: "3d4017c3e843895a92b70aa74d1b7ebc9c982ccf2ec4968cc0cd55f12af4660c",
        message: &[0x72],
        signature: concat!(
            "92a009a9f0d4cab8720e820b5f642540a2b27b5416503f8fb3762223ebdb69da",
            "085ac1e43e15996e458f3613d0f11d8c387b2eaeb4302aeeb00d291612bb0c00",
        ),
    },
];

fn bytes<const N: usize>(text: &str) -> [u8; N] {
    let mut output = [0; N];
    assert_eq!(text.len(), N * 2);

    let (pairs, remainder) = text.as_bytes().as_chunks::<2>();
    assert!(remainder.is_empty());

    for (byte, pair) in output.iter_mut().zip(pairs) {
        *byte = u8::from_str_radix(core::str::from_utf8(pair).unwrap(), 16).unwrap();
    }

    output
}

fn encoded_pair(seed: &[u8; 32], public: &[u8; 32]) -> [u8; 64] {
    let mut input = [0; 64];
    input[..32].copy_from_slice(seed);
    input[32..].copy_from_slice(public);

    input
}

pub(crate) fn imports_both_rfc8032_vectors_and_matches_seed_signatures() {
    for vector in VECTORS {
        let seed = bytes(vector.seed);
        let public = bytes(vector.public);
        let input = encoded_pair(&seed, &public);
        let imported = Keypair::from_bytes(&input).unwrap();
        let seeded = Keypair::from_seed(&seed);
        let expected = Signature::from_bytes(bytes(vector.signature));

        assert_eq!(imported.address().to_bytes(), public);
        assert_eq!(imported.address(), seeded.address());
        assert_eq!(imported.sign(vector.message), expected);
        assert_eq!(imported.sign(vector.message), seeded.sign(vector.message));
        verify(&imported.address(), vector.message, &expected).unwrap();
    }
}

pub(crate) fn rejects_wrong_lengths_before_public_key_validation() {
    let input = [0xa5; 128];

    for length in [0, 1, 31, 32, 33, 63, 65, 128] {
        assert_eq!(
            Keypair::from_bytes(&input[..length]).err(),
            Some(KeypairImportError::InvalidLength { actual: length }),
        );
    }

    assert_eq!(input, [0xa5; 128]);
}

pub(crate) fn rejects_each_public_half_position_for_both_vectors() {
    for vector in VECTORS {
        let input = encoded_pair(&bytes(vector.seed), &bytes(vector.public));

        for position in 0..32 {
            let mut altered = input;
            altered[32 + position] ^= 1;
            let original = altered;

            assert_eq!(
                Keypair::from_bytes(&altered).err(),
                Some(KeypairImportError::PublicKeyMismatch),
                "accepted altered public-half position {position}",
            );
            assert_eq!(altered, original);
        }
    }
}

pub(crate) fn rejects_a_different_valid_public_key_in_both_directions() {
    let pairs = [(&VECTORS[0], &VECTORS[1]), (&VECTORS[1], &VECTORS[0])];

    for (seed_vector, public_vector) in pairs {
        let input = encoded_pair(&bytes(seed_vector.seed), &bytes(public_vector.public));
        let original = input;

        assert_eq!(
            Keypair::from_bytes(&input).err(),
            Some(KeypairImportError::PublicKeyMismatch),
        );
        assert_eq!(input, original);
    }
}

pub(crate) fn import_preserves_input_and_owns_its_signer_independently() {
    for vector in VECTORS {
        let mut input = encoded_pair(&bytes(vector.seed), &bytes(vector.public));
        let original = input;
        let imported = Keypair::from_bytes(&input).unwrap();

        assert_eq!(input, original);
        input.fill(0);
        assert_eq!(input, [0; 64]);
        assert_eq!(imported.address().to_bytes(), bytes::<32>(vector.public));
        assert_eq!(
            imported.sign(vector.message),
            Signature::from_bytes(bytes(vector.signature)),
        );
    }
}

pub(crate) fn errors_expose_only_length_or_a_public_key_mismatch() {
    for vector in VECTORS {
        let seed = bytes(vector.seed);
        let public = bytes(vector.public);
        let mut input = encoded_pair(&seed, &public);
        input[32] ^= 1;
        let mismatch = Keypair::from_bytes(&input).err().unwrap();
        let length = Keypair::from_bytes(&input[..63]).err().unwrap();
        let error: &dyn core::error::Error = &mismatch;

        assert_eq!(format!("{mismatch:?}"), "PublicKeyMismatch");
        assert_eq!(
            mismatch.to_string(),
            "Ed25519 public key does not match the seed"
        );
        assert_eq!(format!("{length:?}"), "InvalidLength { actual: 63 }");
        assert_eq!(
            length.to_string(),
            "expected 64 Ed25519 keypair bytes, received 63"
        );
        assert!(error.source().is_none());
        assert!(core::error::Error::source(&length).is_none());
    }
}
