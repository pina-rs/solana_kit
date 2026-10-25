//! Isolated pure nonce checks for off-chain clients, ready for later host integration.
#![no_std]

extern crate alloc;

#[cfg(feature = "crypto")]
use solana_kit_core::crypto::Keypair;
use solana_kit_core::transaction::{
    DurableNonce, Error, Lifetime, Message, NonceAdvance, Transaction, Version,
};
use solana_kit_core::{AccountRole, Address, Hash, Instruction};

const PREFIX_BYTE: u8 = 42;

fn nonce() -> DurableNonce {
    DurableNonce::new(
        Hash::from_bytes([7; 32]),
        Address::from_bytes([2; 32]),
        Address::from_bytes([3; 32]),
    )
}

/// Exercises idempotent preparation, retention, replacement and exact prerequisite checks.
///
/// # Errors
/// Returns a descriptive failure when observed public behavior differs.
pub fn construction() -> Result<(), &'static str> {
    let nonce = nonce();
    let instructions = nonce.prepare_instructions(&[]);
    assert_eq!(instructions, nonce.prepare_instructions(&instructions));
    assert_eq!(instructions[0].data, [4, 0, 0, 0]);
    assert_eq!(instructions[0].accounts[0].role, AccountRole::Writable);
    assert_eq!(instructions[0].accounts[1].role, AccountRole::Readonly);
    assert_eq!(
        instructions[0].accounts[2].role,
        AccountRole::ReadonlySigner
    );

    let mut writable = instructions.clone();
    writable[0].accounts[2].role = AccountRole::WritableSigner;
    assert_eq!(writable, nonce.prepare_instructions(&writable));

    let other = DurableNonce::new(
        nonce.token(),
        Address::from_bytes([4; 32]),
        Address::from_bytes([5; 32]),
    );
    assert_eq!(
        other.prepare_instructions(&instructions),
        other.prepare_instructions(&[])
    );

    let prefix = Instruction {
        program_address: Address::from_bytes([9; 32]),
        accounts: alloc::vec![],
        data: alloc::vec![PREFIX_BYTE],
    };
    let prefixed = nonce.prepare_instructions(core::slice::from_ref(&prefix));
    assert_eq!(prefixed[1], prefix);
    assert_eq!(
        nonce.validate_instructions(&prefixed[1..]),
        Err(Error::InvalidNonceAdvance)
    );
    assert_eq!(
        other.validate_instructions(&instructions),
        Err(Error::InvalidNonceAdvance)
    );

    Ok(())
}

/// Exercises every supported wire version with distinct and shared payer-authority roles.
///
/// # Errors
/// Returns a descriptive failure when public construction, decode, or encoding fails.
pub fn wire() -> Result<(), &'static str> {
    for version in [Version::Legacy, Version::V0, Version::V1] {
        for payer in [
            Address::from_bytes([1; 32]),
            nonce().advance().nonce_authority,
        ] {
            wire_pair(version, payer)?;
        }
    }

    Ok(())
}

fn wire_pair(version: Version, payer: Address) -> Result<(), &'static str> {
    let nonce = nonce();
    let expected_signers = 1 + usize::from(payer != nonce.advance().nonce_authority);
    let message = Message::compile(
        version,
        payer,
        Lifetime::DurableNonce(nonce),
        &nonce.prepare_instructions(&[]),
        &[],
    )
    .map_err(|_| "compile")?;
    assert_eq!(message.lifetime(), Lifetime::DurableNonce(nonce));
    assert_eq!(message.signers()[0], payer);
    assert_eq!(message.signers().len(), expected_signers);
    assert_eq!(message.nonce_advance(), Ok(Some(nonce.advance())));

    let transaction = Transaction::new(message);
    let bytes = transaction.encode(false).map_err(|_| "encode")?;
    let decoded = Transaction::decode(&bytes).map_err(|_| "decode")?;
    assert_eq!(
        decoded.message().lifetime(),
        Lifetime::Unresolved(nonce.token())
    );
    assert_eq!(decoded.message().nonce_advance(), Ok(Some(nonce.advance())));
    assert_eq!(decoded.encode(false).map_err(|_| "reencode")?, bytes);

    for end in 0..bytes.len() {
        assert!(Transaction::decode(&bytes[..end]).is_err());
    }

    let mut trailing = bytes;
    trailing.push(0);
    assert!(Transaction::decode(&trailing).is_err());

    Ok(())
}

/// Rejects malformed nonce prerequisites and invalid textual hashes without relaxing blockhash behavior.
///
/// # Errors
/// Returns a descriptive failure when the blockhash control fails to compile.
pub fn negatives() -> Result<(), &'static str> {
    let nonce = nonce();
    let valid = nonce.advance().instruction();

    for wrong in invalid_advances(&valid) {
        assert_eq!(NonceAdvance::recognize(&wrong), None);
        assert_eq!(
            Message::compile(
                Version::V0,
                Address::from_bytes([1; 32]),
                Lifetime::DurableNonce(nonce),
                &[wrong],
                &[]
            ),
            Err(Error::InvalidNonceAdvance)
        );
    }

    hash_and_blockhash_control(nonce)
}

/// Mutations independently cover program, data, account count, identities and roles.
fn invalid_advances(valid: &Instruction) -> alloc::vec::Vec<Instruction> {
    let mut invalid = alloc::vec![];
    let mut wrong = valid.clone();
    wrong.program_address = Address::from_bytes([9; 32]);
    invalid.push(wrong);

    let mut wrong = valid.clone();
    wrong.data[0] = 5;
    invalid.push(wrong);

    let mut wrong = valid.clone();
    wrong.data.push(0);
    invalid.push(wrong);

    let mut wrong = valid.clone();
    wrong.data.pop();
    invalid.push(wrong);

    let mut wrong = valid.clone();
    wrong.accounts.pop();
    invalid.push(wrong);

    let mut wrong = valid.clone();
    wrong.accounts.push(wrong.accounts[0]);
    invalid.push(wrong);

    let mut wrong = valid.clone();
    wrong.accounts[1].address = Address::from_bytes([9; 32]);
    invalid.push(wrong);

    for (index, role) in [
        (0, AccountRole::Readonly),
        (0, AccountRole::WritableSigner),
        (1, AccountRole::Writable),
        (2, AccountRole::Readonly),
    ] {
        let mut wrong = valid.clone();
        wrong.accounts[index].role = role;
        invalid.push(wrong);
    }

    invalid
}

/// Textual hash rejection must not relax the separate ordinary-blockhash control.
fn hash_and_blockhash_control(nonce: DurableNonce) -> Result<(), &'static str> {
    for text in [
        "",
        "123",
        "0OIl0OIl0OIl0OIl0OIl0OIl0OIl0OIl",
        "2222222222222222222222222222222222222222",
        "111111111111111111111111111111111",
    ] {
        assert!(text.parse::<Hash>().is_err());
    }

    let lifetime = Lifetime::Blockhash {
        blockhash: nonce.token(),
        last_valid_block_height: u64::MAX,
    };
    let control = Message::compile(
        Version::V0,
        Address::from_bytes([1; 32]),
        lifetime,
        &[],
        &[],
    )
    .map_err(|_| "blockhash")?;
    assert_eq!(control.lifetime(), lifetime);
    assert_eq!(control.nonce_advance(), Ok(None));

    Ok(())
}

/// Signs exact nonce bytes and rejects stale signatures and cryptographic tampering.
///
/// # Errors
/// Returns a descriptive failure when signing or verification fails.
#[cfg(feature = "crypto")]
pub fn signatures() -> Result<(), &'static str> {
    let payer = Keypair::from_seed(&[1; 32]);
    let authority = Keypair::from_seed(&[3; 32]);

    for version in [Version::Legacy, Version::V0, Version::V1] {
        signed_version(version, &payer, &authority)?;
    }

    Ok(())
}

/// Every wire version signs the same nonce ownership contract before tamper checks.
#[cfg(feature = "crypto")]
fn signed_version(
    version: Version,
    payer: &Keypair,
    authority: &Keypair,
) -> Result<(), &'static str> {
    let nonce = DurableNonce::new(
        Hash::from_bytes([7; 32]),
        Address::from_bytes([2; 32]),
        authority.address(),
    );
    let message = Message::compile(
        version,
        payer.address(),
        Lifetime::DurableNonce(nonce),
        &nonce.prepare_instructions(&[]),
        &[],
    )
    .map_err(|_| "compile")?;

    let signed = Transaction::new(message)
        .sign(payer)
        .map_err(|_| "payer")?
        .sign(authority)
        .map_err(|_| "authority")?;
    signed.verify().map_err(|_| "verify")?;

    let bytes = signed.encode(true).map_err(|_| "encode")?;
    let received = Transaction::decode(&bytes).map_err(|_| "decode")?;
    received.verify().map_err(|_| "received verify")?;
    assert_eq!(received.encode(true).map_err(|_| "reencode")?, bytes);

    reject_stale_nonce(version, payer, authority, nonce, &signed)?;
    reject_tamper(version, bytes)
}

/// Replacing nonce bytes invalidates retained signature provenance even before crypto.
#[cfg(feature = "crypto")]
fn reject_stale_nonce(
    version: Version,
    payer: &Keypair,
    authority: &Keypair,
    nonce: DurableNonce,
    signed: &Transaction,
) -> Result<(), &'static str> {
    let changed = DurableNonce::new(
        Hash::from_bytes([8; 32]),
        nonce.advance().nonce_account,
        authority.address(),
    );
    let message = Message::compile(
        version,
        payer.address(),
        Lifetime::DurableNonce(changed),
        &changed.prepare_instructions(&[]),
        &[],
    )
    .map_err(|_| "changed compile")?;
    let fresh = Transaction::new(message);
    assert_eq!(fresh.verify(), Err(Error::MissingSignature));

    assert_eq!(
        fresh.with_signature(
            payer.address(),
            signed.message().bytes(),
            signed.signatures()[0].ok_or("missing")?
        ),
        Err(Error::ChangedMessage)
    );

    Ok(())
}

/// Version-specific signed-message corruption must fail cryptographic validation.
#[cfg(feature = "crypto")]
fn reject_tamper(version: Version, bytes: alloc::vec::Vec<u8>) -> Result<(), &'static str> {
    let mut corrupt = bytes;
    let index = if version == Version::V1 {
        corrupt.len() - 1
    } else {
        1
    };
    corrupt[index] ^= 1;
    assert_eq!(
        Transaction::decode(&corrupt)
            .map_err(|_| "tampered decode")?
            .verify(),
        Err(Error::InvalidSignature)
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn construction() {
        super::construction().unwrap();
    }
    #[test]
    fn wire() {
        super::wire().unwrap();
    }
    #[test]
    fn negatives() {
        super::negatives().unwrap();
    }
    #[cfg(feature = "crypto")]
    #[test]
    fn signatures() {
        super::signatures().unwrap();
    }
}
