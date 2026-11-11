//! Gnark proof JSON to the compressed `a || b || c` bytes a transact carries.
//!
//! The conversion follows `parseCheckedProof` and `compressProof` in
//! `sdk-libs/ts/src/client/prover/proof.ts`. Proof `A` is negated over the
//! BN254 base field before G1 compression. G2 stays the raw 128-byte point.
//! G1 points are checked against `y² = x³ + 3`. G2 coordinates are checked
//! to be canonical field elements. Twist-subgroup membership stays with the
//! on-chain verifier, as it does in the Dart port.

use serde_json::Value;

use crate::encode::Proof;
use crate::error::ProverFailure;
use crate::prover::keys::proving_key_sha256;

const SHA256_HEX_LEN: usize = 64;
const FIELD_BYTES: usize = 32;
const FIELD_HEX_LEN: usize = 64;
const LAST_FIELD_BYTE: usize = 31;
const BYTE_BITS: u16 = 8;
const HIGH_BIT: u8 = 7;
const WIDE_LIMBS: usize = 64;
const BYTE_BASE: i16 = 256;

/// BN254 base field modulus.
const MODULUS: [u8; 32] = [
    0x30, 0x64, 0x4e, 0x72, 0xe1, 0x31, 0xa0, 0x29, 0xb8, 0x50, 0x45, 0xb6, 0x81, 0x81, 0x58, 0x5d,
    0x97, 0x81, 0x6a, 0x91, 0x68, 0x71, 0xca, 0x8d, 0x3c, 0x20, 0x8c, 0x16, 0xd8, 0x7c, 0xfd, 0x47,
];

/// Parse a prover response into the compressed proof a transact instruction stores.
///
/// `circuit` is the proving-key name, with or without `.key`. The response must
/// report that key's pinned SHA-256 in `provingKeySha256`. BSB22 commitment
/// fields are checked when present and are not copied onto [`Proof`].
///
/// # Errors
/// Rejects an unknown circuit, a missing or foreign proving key, a null proof,
/// a point that is not on G1, or a field with the wrong shape.
pub fn parse_prover_proof(value: &Value, circuit: &str) -> Result<Proof, ProverFailure> {
    let Some(expected) = proving_key_sha256(circuit) else {
        return Err(ProverFailure::UnknownCircuit);
    };

    let proof_value = match value.get("proof") {
        Some(proof) => proof,
        None => value,
    };

    if !proof_value.is_null() {
        check_proving_key(proof_value, expected)?;
    }

    compress(parse_gnark(proof_value)?)
}

fn check_proving_key(proof: &Value, expected: &str) -> Result<(), ProverFailure> {
    let Some(reported) = proof.get("provingKeySha256") else {
        return Err(ProverFailure::ProvingKeyMissing);
    };
    let Some(reported) = reported.as_str() else {
        return Err(ProverFailure::Response);
    };

    if reported.len() != SHA256_HEX_LEN
        || !reported
            .bytes()
            .all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase())
    {
        return Err(ProverFailure::Response);
    }

    if reported != expected {
        return Err(ProverFailure::ProvingKeyMismatch);
    }

    Ok(())
}

struct Point {
    x: Field,
    y: Field,
}

struct Parsed {
    a: Point,
    b: [u8; 128],
    point_c: Point,
}

fn parse_gnark(value: &Value) -> Result<Parsed, ProverFailure> {
    if value.is_null() {
        return Err(ProverFailure::Server);
    }

    let proof = value.as_object().ok_or(ProverFailure::ProofParse)?;
    let a = parse_g1(proof.get("ar"))?;
    let negated_y = if a.y.is_zero() {
        Field::ZERO
    } else {
        Field::MODULUS.sub(a.y)
    };

    let b = parse_g2(proof.get("bs"))?;
    let point_c = parse_g1(proof.get("krs"))?;
    let commitment = present(proof.get("proofCommitment"));
    let pok = present(proof.get("proofCommitmentPok"));

    if commitment || pok {
        parse_g1(proof.get("proofCommitment"))?;
        parse_g1(proof.get("proofCommitmentPok"))?;
    }

    Ok(Parsed {
        a: Point {
            x: a.x,
            y: negated_y,
        },
        b,
        point_c,
    })
}

fn present(value: Option<&Value>) -> bool {
    match value {
        None | Some(Value::Null) => false,

        Some(Value::Array(items)) if items.is_empty() => false,
        Some(_) => true,
    }
}

fn compress(proof: Parsed) -> Result<Proof, ProverFailure> {
    Ok(Proof {
        a: compress_g1(&proof.a)?,
        b: proof.b,
        c: compress_g1(&proof.point_c)?,
    })
}

fn compress_g1(point: &Point) -> Result<[u8; 32], ProverFailure> {
    if point.x.is_zero() && point.y.is_zero() {
        return Ok([0; 32]);
    }

    validate_g1(point)?;
    let mut bytes = point.x.to_bytes();

    if point.y.is_largest() {
        bytes[0] |= 0x80;
    }

    Ok(bytes)
}

fn validate_g1(point: &Point) -> Result<(), ProverFailure> {
    let y_squared = point.y.mul(point.y);
    let x_cubed = point.x.mul(point.x).mul(point.x);

    if y_squared != x_cubed.add(Field::from_u64(3)) {
        return Err(ProverFailure::ProofPoint);
    }

    Ok(())
}

fn parse_g1(value: Option<&Value>) -> Result<Point, ProverFailure> {
    let items = value
        .and_then(Value::as_array)
        .ok_or(ProverFailure::ProofParse)?;

    if items.len() != 2 {
        return Err(ProverFailure::ProofParse);
    }

    let x = coordinate(&items[0])?;
    let y = coordinate(&items[1])?;
    let point = Point { x, y };

    if !x.is_zero() || !y.is_zero() {
        validate_g1(&point)?;
    }

    Ok(point)
}

/// Reads G2 as two rows of two field elements, stored in JSON order.
///
/// Twist-subgroup membership is left to the on-chain verifier. This parser
/// only checks that each coordinate is a canonical field element.
fn parse_g2(value: Option<&Value>) -> Result<[u8; 128], ProverFailure> {
    let rows = value
        .and_then(Value::as_array)
        .ok_or(ProverFailure::ProofParse)?;

    if rows.len() != 2 {
        return Err(ProverFailure::ProofParse);
    }

    let mut bytes = [0u8; 128];

    for (row_index, row) in rows.iter().enumerate() {
        let values = row.as_array().ok_or(ProverFailure::ProofParse)?;

        if values.len() != 2 {
            return Err(ProverFailure::ProofParse);
        }

        for (index, item) in values.iter().enumerate() {
            let coordinate = coordinate(item)?;
            let offset = (row_index * 2 + index) * FIELD_BYTES;
            bytes[offset..offset + FIELD_BYTES].copy_from_slice(&coordinate.to_bytes());
        }
    }

    Ok(bytes)
}

/// Parses one `0x` field element and rejects a non-canonical encoding.
///
/// Gnark prints coordinates in hex. A value that needs more than the field
/// width, or that is not reduced, is not a proof point.
fn coordinate(value: &Value) -> Result<Field, ProverFailure> {
    let text = value.as_str().ok_or(ProverFailure::ProofParse)?;
    let digits = text
        .strip_prefix("0x")
        .or_else(|| text.strip_prefix("0X"))
        .unwrap_or(text);

    if digits.is_empty() || !digits.bytes().all(|byte| byte.is_ascii_hexdigit()) {
        return Err(ProverFailure::ProofParse);
    }

    if digits.len() > FIELD_HEX_LEN {
        return Err(ProverFailure::ProofParse);
    }

    let mut bytes = [0u8; 32];

    for (index, digit) in digits.bytes().rev().enumerate() {
        let nibble = match digit {
            b'0'..=b'9' => digit - b'0',
            b'a'..=b'f' => digit - b'a' + 10,
            b'A'..=b'F' => digit - b'A' + 10,
            _ => return Err(ProverFailure::ProofParse),
        };

        let place = LAST_FIELD_BYTE - index / 2;

        if index % 2 == 0 {
            bytes[place] |= nibble;
        } else {
            bytes[place] |= nibble << 4;
        }
    }

    Field::canonical(bytes).ok_or(ProverFailure::ProofParse)
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Field([u8; 32]);

impl Field {
    const ZERO: Self = Self([0; 32]);
    const MODULUS: Self = Self(MODULUS);

    fn from_u64(value: u64) -> Self {
        let mut bytes = [0; 32];
        bytes[24..].copy_from_slice(&value.to_be_bytes());
        Self(bytes)
    }

    fn canonical(bytes: [u8; 32]) -> Option<Self> {
        let field = Self(bytes);

        if field.cmp_bytes(Self::MODULUS) == std::cmp::Ordering::Less {
            Some(field)
        } else {
            None
        }
    }

    fn is_zero(self) -> bool {
        self.0 == [0; 32]
    }

    fn is_largest(self) -> bool {
        let half = Self::MODULUS.sub(Self::from_u64(1)).shr1();
        self.cmp_bytes(half) == std::cmp::Ordering::Greater
    }

    fn to_bytes(self) -> [u8; 32] {
        self.0
    }

    fn cmp_bytes(self, other: Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }

    fn add(self, other: Self) -> Self {
        let mut out = [0u8; 32];
        let mut carry = 0u16;

        for index in (0..FIELD_BYTES).rev() {
            let sum = u16::from(self.0[index]) + u16::from(other.0[index]) + carry;
            out[index] = sum as u8;
            carry = sum >> BYTE_BITS;
        }

        let sum = Self(out);

        if carry > 0 || sum.cmp_bytes(Self::MODULUS) != std::cmp::Ordering::Less {
            sum.sub(Self::MODULUS)
        } else {
            sum
        }
    }

    fn sub(self, other: Self) -> Self {
        let base = if self.cmp_bytes(other) == std::cmp::Ordering::Less {
            self.add_modulus()
        } else {
            self
        };

        let mut out = [0u8; 32];
        let mut borrow = 0i16;

        for index in (0..FIELD_BYTES).rev() {
            let diff = i16::from(base.0[index]) - i16::from(other.0[index]) - borrow;
            let (limb, next_borrow) = signed_limb(diff);
            out[index] = limb;
            borrow = next_borrow;
        }

        Self(out)
    }

    /// `self + modulus`, which still fits in 32 bytes because `2p < 2^256`.
    fn add_modulus(self) -> Self {
        let mut out = [0u8; 32];
        let mut carry = 0u16;

        for index in (0..FIELD_BYTES).rev() {
            let sum = u16::from(self.0[index]) + u16::from(MODULUS[index]) + carry;
            out[index] = sum as u8;
            carry = sum >> BYTE_BITS;
        }

        debug_assert_eq!(carry, 0);
        Self(out)
    }

    fn mul(self, other: Self) -> Self {
        let mut wide = [0u32; 64];

        for left in 0..FIELD_BYTES {
            for right in 0..FIELD_BYTES {
                wide[left + right + 1] += u32::from(self.0[left]) * u32::from(other.0[right]);
            }
        }

        for index in (1..WIDE_LIMBS).rev() {
            wide[index - 1] += wide[index] >> BYTE_BITS;
            wide[index] &= 0xff;
        }

        debug_assert!(wide[0] < 256);

        let mut bytes = [0u8; 64];

        for (index, limb) in wide.iter().enumerate() {
            bytes[index] = *limb as u8;
        }

        reduce(bytes)
    }

    fn shr1(self) -> Self {
        let mut out = [0u8; 32];
        let mut carry = 0u8;

        for (index, byte) in self.0.iter().enumerate() {
            out[index] = (byte >> 1) | carry;
            carry = (byte & 1) << HIGH_BIT;
        }

        Self(out)
    }
}

fn reduce(wide: [u8; 64]) -> Field {
    let mut remainder = [0u8; 33];

    for byte in wide {
        let mut carry = u16::from(byte);

        for limb in remainder.iter_mut().rev() {
            let shifted = (u16::from(*limb) << 8) | carry;
            *limb = shifted as u8;
            carry = shifted >> BYTE_BITS;
        }

        debug_assert_eq!(carry, 0);
        subtract_modulus(&mut remainder);
    }

    let mut bytes = [0u8; 32];
    bytes.copy_from_slice(&remainder[1..]);
    Field(bytes)
}

/// Subtracts the field modulus until the 33-byte remainder is reduced.
///
/// Each input byte is shifted in before this call, so the value can be larger
/// than the modulus more than once.
fn subtract_modulus(remainder: &mut [u8; 33]) {
    let mut modulus = [0u8; 33];
    modulus[1..].copy_from_slice(&MODULUS);

    while greater_or_equal(remainder, &modulus) {
        let mut borrow = 0i16;

        for index in (0..33).rev() {
            let diff = i16::from(remainder[index]) - i16::from(modulus[index]) - borrow;
            let (limb, next_borrow) = signed_limb(diff);
            remainder[index] = limb;
            borrow = next_borrow;
        }
    }
}

fn signed_limb(diff: i16) -> (u8, i16) {
    if diff < 0 {
        ((diff + BYTE_BASE) as u8, 1)
    } else {
        (diff as u8, 0)
    }
}

fn greater_or_equal(left: &[u8; 33], right: &[u8; 33]) -> bool {
    left.cmp(right) != std::cmp::Ordering::Less
}

#[cfg(test)]
mod field_tests {
    use super::Field;

    #[test]
    fn two_squared_is_four_and_two_negates_to_zero() {
        let two = Field::from_u64(2);
        assert_eq!(two.mul(two), Field::from_u64(4));
        assert!(Field::MODULUS.sub(two).add(two).is_zero());
    }
}
