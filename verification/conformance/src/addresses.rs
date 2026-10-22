//! Pinned Dart base58, PDA, and seeded-address vectors and boundary cases.

use alloc::{string::ToString, vec};
use hex_literal::hex;
use solana_kit_core::{
    Address, Hash, ParseError, Signature,
    crypto::{
        MAX_SEEDS, PdaError, create_address_with_seed, create_program_address, find_program_address,
    },
};

use crate::require;

const SIGNATURE_BYTES: usize = 64;

/// Parses the immutable reference address or reports corrupted fixture data.
fn address(text: &str) -> Result<Address, &'static str> {
    text.parse().map_err(|_| "invalid address in fixed fixture")
}

/// The Dart codec maps these independent input bytes to the exact base58 string.
pub(crate) fn golden() -> Result<(), &'static str> {
    let text = "4wBqpZM9xaSheZzJSMawUKKwhdpChKbZ5eu5ky4Vigw";
    let expected = hex!("0102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f20");
    let parsed = address(text)?;

    require(
        parsed.to_bytes() == expected,
        "base58 address bytes mismatch",
    )?;
    require(
        parsed.to_string() == text,
        "base58 address display mismatch",
    )
}

/// Solana base58 excludes ambiguous zero/O/I/l characters and whitespace.
pub(crate) fn invalid_alphabet() -> Result<(), &'static str> {
    for text in ["0", "O", "I", "l", " ", "🚀"] {
        require(
            text.parse::<Address>() == Err(ParseError::InvalidBase58),
            "invalid base58 character was accepted",
        )?;
    }

    Ok(())
}

/// Exact decoded lengths reject empty, truncated, and extra leading-zero bytes.
pub(crate) fn invalid_length() -> Result<(), &'static str> {
    for count in [0, 31, 33] {
        let text = "1".repeat(count);
        require(
            text.parse::<Address>()
                == Err(ParseError::WrongLength {
                    expected: 32,
                    actual: count,
                }),
            "address decoded length was not rejected",
        )?;
    }

    Ok(())
}

/// Text-size bounds apply before allocation or potentially expensive decoding.
pub(crate) fn bounded_input() -> Result<(), &'static str> {
    require(
        "1".repeat(45).parse::<Address>()
            == Err(ParseError::EncodedLengthExceeded {
                maximum: 44,
                actual: 45,
            }),
        "oversized address input was not rejected",
    )?;
    require(
        "1".repeat(89).parse::<Signature>()
            == Err(ParseError::EncodedLengthExceeded {
                maximum: 88,
                actual: 89,
            }),
        "oversized signature input was not rejected",
    )
}

/// A zero-valued 32-byte hash has the canonical 32-leading-ones encoding.
pub(crate) fn hash_value() -> Result<(), &'static str> {
    let text = "11111111111111111111111111111111";
    let hash = text
        .parse::<Hash>()
        .map_err(|_| "zero hash parsing failed")?;

    require(hash.to_bytes() == [0; 32], "zero hash bytes mismatch")?;
    require(hash.to_string() == text, "zero hash display mismatch")
}

/// Base58 leading ones represent zero bytes for signatures as well as addresses.
pub(crate) fn signature_value() -> Result<(), &'static str> {
    let text = "1".repeat(SIGNATURE_BYTES);
    let signature = text
        .parse::<Signature>()
        .map_err(|_| "zero signature parsing failed")?;

    require(
        signature.to_bytes() == [0; 64],
        "zero signature bytes mismatch",
    )?;
    require(
        signature.to_string() == text,
        "zero signature display mismatch",
    )?;
    require(
        "0".parse::<Signature>() == Err(ParseError::InvalidBase58),
        "invalid signature alphabet was accepted",
    )
}

/// Compares canonical bump search and explicit recreation with a pinned oracle.
fn pda(program: &str, seeds: &[&[u8]], expected: &str, bump: u8) -> Result<(), &'static str> {
    let program = address(program)?;
    let expected = address(expected)?;
    let actual = find_program_address(seeds, &program).map_err(|_| "PDA search failed")?;
    let bump_seed = [bump];
    let mut explicit_seeds = seeds.to_vec();
    explicit_seeds.push(&bump_seed);

    require(actual.address == expected, "PDA address mismatch")?;
    require(actual.bump == bump, "PDA bump mismatch")?;
    require(
        create_program_address(&explicit_seeds, &program) == Ok(expected),
        "explicit PDA recreation mismatch",
    )
}

/// Pinned reference's no-seed oracle accepts the first bump.
pub(crate) fn pda_no_seeds() -> Result<(), &'static str> {
    pda(
        "CZ3TbkgUYpDAJVEWpujQhDSgzNTeqbokrJmYa1j4HAZc",
        &[],
        "9tVtkyCGAHSDDBPwz7895aC3p2gJRjpu2v26o35FTUco",
        255,
    )
}

/// Pinned reference rejects four higher bumps before accepting 251.
pub(crate) fn pda_bump_search() -> Result<(), &'static str> {
    pda(
        "EfTbwNBrSqSuCNBhWUHsBoBdSMWgRU1S47daqRNgW7aK",
        &[],
        "CKWT8KZ5GMzKpVRiAULWKPg1LiHt9U3NdAtbuTErHCTq",
        251,
    )
}

/// Raw-byte seed oracle prevents accidental UTF-8 or integer reinterpretation.
pub(crate) fn pda_byte_seed() -> Result<(), &'static str> {
    pda(
        "FD3PDEvpQ9JXq8tv7FpJPyZrCjWkCnAaTju16gFPdpqP",
        &[&[1, 2, 3]],
        "9Tj3hpMWacDiZoBe94sjwJQ72zsUVvEQYsrqyy2CfHky",
        255,
    )
}

/// UTF-8 seed oracle uses four bytes for the rocket scalar value.
pub(crate) fn pda_utf8_seed() -> Result<(), &'static str> {
    pda(
        "H8gBP21L5ietkHgXcGbgQBCVVEdPUQyuP9Q5MPRLLSJu",
        &["🚀".as_bytes()],
        "46v3JvPtEPeQmH3euXydEbxYD6yfxeZjWSzkkYvvM5Pp",
        251,
    )
}

/// Direct creation permits 16 seeds; bump search reserves the final slot.
pub(crate) fn seed_count() -> Result<(), &'static str> {
    let program = address("CZ3TbkgUYpDAJVEWpujQhDSgzNTeqbokrJmYa1j4HAZc")?;
    let empty: &[u8] = &[];
    let seventeen = [empty; 17];
    let bump_seed = [255];
    let mut sixteen = [empty; MAX_SEEDS];
    sixteen[MAX_SEEDS - 1] = &bump_seed;
    let expected = address("9tVtkyCGAHSDDBPwz7895aC3p2gJRjpu2v26o35FTUco")?;

    require(
        create_program_address(&seventeen, &program) == Err(PdaError::TooManySeeds),
        "17 seeds were accepted",
    )?;
    require(
        find_program_address(&seventeen[..MAX_SEEDS], &program) == Err(PdaError::TooManySeeds),
        "bump search accepted 16 input seeds",
    )?;
    require(
        find_program_address(&seventeen[..MAX_SEEDS - 1], &program).is_ok(),
        "bump search rejected 15 valid seeds",
    )?;
    require(
        create_program_address(&sixteen, &program) == Ok(expected),
        "direct creation rejected 16 valid seeds",
    )
}

/// The seed limit counts bytes, including UTF-8 bytes, rather than characters.
pub(crate) fn seed_length() -> Result<(), &'static str> {
    let program = Address::from_bytes([0; 32]);

    require(
        find_program_address(&[&[0; 33]], &program) == Err(PdaError::SeedTooLong),
        "33-byte seed was accepted",
    )?;
    require(
        find_program_address(&[&[0; 32]], &program).is_ok(),
        "32-byte seed was rejected",
    )?;
    require(
        create_address_with_seed(&program, &[0; 33], &program) == Err(PdaError::SeedTooLong),
        "33-byte seeded address seed was accepted",
    )
}

/// Higher bumps in the pinned 251 oracle must fail the actual curve test.
pub(crate) fn on_curve() -> Result<(), &'static str> {
    let program = address("EfTbwNBrSqSuCNBhWUHsBoBdSMWgRU1S47daqRNgW7aK")?;

    for bump in 252..=255 {
        require(
            create_program_address(&[&[bump]], &program) == Err(PdaError::OnCurve),
            "on-curve PDA candidate was accepted",
        )?;
    }

    Ok(())
}

/// Pinned Dart base/seed/owner concatenation oracle.
pub(crate) fn seeded_address() -> Result<(), &'static str> {
    let base = address("Bh1uUDP3ApWLeccVNHwyQKpnfGQbuE2UECbGA6M4jiZJ")?;
    let owner = address("FGrddpvjBUAG6VdV4fR8Q2hEZTHS6w4SEveVBgfwbfdm")?;
    let expected = address("HUKxCeXY6gZohFJFARbLE6L6C9wDEHz1SfK8ENM7QY7z")?;

    require(
        create_address_with_seed(&base, b"seed", &owner) == Ok(expected),
        "seeded address mismatch",
    )
}

/// Pinned owner whose suffix is exactly the PDA marker must be forbidden.
pub(crate) fn illegal_owner() -> Result<(), &'static str> {
    let owner = address("4vJ9JU1bJJE96FbKdjWme2JfVK1knU936FHTDZV7AC2")?;

    require(
        create_address_with_seed(&Address::from_bytes([0; 32]), b"seed", &owner)
            == Err(PdaError::IllegalOwner),
        "PDA marker owner was accepted",
    )
}

/// All fixed values reject trailing bytes as well as truncation.
pub(crate) fn byte_lengths() -> Result<(), &'static str> {
    for length in [0, 31, 33] {
        let bytes = vec![0; length];
        require(
            Address::try_from(bytes.as_slice()).is_err(),
            "invalid address byte count",
        )?;
        require(
            Hash::try_from(bytes.as_slice()).is_err(),
            "invalid hash byte count",
        )?;
    }

    for length in [0, 63, 65] {
        require(
            Signature::try_from(vec![0; length].as_slice()).is_err(),
            "invalid signature byte count",
        )?;
    }

    Ok(())
}
