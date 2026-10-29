//! Independent raw Solana envelopes and Borsh bytes; no Kit compiler or metadata encoder.
use alloc::{vec, vec::Vec};
use solana_kit_core::{
    Address, Hash, Signature, codecs::Limits, crypto::Keypair, transaction::Version,
};
use solana_kit_dapp_publisher::{
    Error, ExpectedTransaction, PublicationPolicy, validate_prepared_transaction,
};
use solana_kit_mpl_token_metadata::{PROGRAM, find_master_edition, find_metadata};

pub const LIMITS: Limits = Limits::new(4096, 16384);
pub const COLLECTION: Address = Address::from_bytes([7; 32]);
pub const BLOCKHASH: Hash = Hash::from_bytes([3; 32]);

pub fn assert_rejected(bytes: &[u8], policy: PublicationPolicy, limits: Limits, error: Error) {
    let actual = validate_prepared_transaction(bytes, policy, limits).unwrap_err();
    assert_eq!(actual, error);
}

pub fn payer() -> Keypair {
    Keypair::from_seed(&[1; 32])
}

pub fn mint() -> Keypair {
    Keypair::from_seed(&[2; 32])
}

pub fn expected() -> ExpectedTransaction {
    ExpectedTransaction {
        blockhash: BLOCKHASH,
        payer: payer().address(),
        signer: payer().address(),
    }
}

pub fn release_policy() -> PublicationPolicy {
    PublicationPolicy::ReleaseMint {
        expected: expected(),
        mint: mint().address(),
        collection: COLLECTION,
    }
}

pub fn verification_policy(authority: Address) -> PublicationPolicy {
    PublicationPolicy::VerifyCollection {
        expected: expected(),
        nft_mint: mint().address(),
        collection: COLLECTION,
        authority,
    }
}

pub fn create_data(collection: Address) -> Vec<u8> {
    let mut bytes = vec![
        42, 0, // instruction and CreateArgs V1 tags
        0, 0, 0, 0, // empty name
        0, 0, 0, 0, // empty symbol
        0, 0, 0, 0, // empty URI
        0, 0, // seller fee basis points
        0, 0, 1, 0, // no creators, no primary sale, mutable, non-fungible
        1, 0, // collection present, unverified
    ];
    bytes.extend_from_slice(collection.as_bytes());
    bytes.extend_from_slice(&[0, 0, 0, 0, 0]); // uses, details, rule set, decimals, print supply
    bytes
}

pub struct RawTransaction {
    pub accounts: Vec<Address>,
    pub signers: u8,
    pub program: u8,
    pub data: Vec<u8>,
    pub version: Version,
    pub lookup: bool,
}

impl RawTransaction {
    pub fn release() -> Self {
        Self {
            accounts: vec![payer().address(), mint().address(), PROGRAM],
            signers: 2,
            program: 2,
            data: create_data(COLLECTION),
            version: Version::Legacy,
            lookup: false,
        }
    }

    pub fn verification() -> Self {
        Self {
            accounts: vec![
                payer().address(),
                PROGRAM,
                find_metadata(mint().address()).unwrap().address(),
                COLLECTION,
                find_metadata(COLLECTION).unwrap().address(),
                find_master_edition(COLLECTION).unwrap().address(),
            ],
            signers: 1,
            program: 1,
            data: vec![18],
            version: Version::Legacy,
            lookup: false,
        }
    }

    pub fn message(&self) -> Vec<u8> {
        let readonly = u8::try_from(self.accounts.len()).unwrap() - self.signers;
        let header = [self.signers, 0, readonly];
        let mut bytes = Vec::new();

        match self.version {
            Version::Legacy => bytes.extend_from_slice(&header),
            Version::V0 => {
                bytes.push(0x80);
                bytes.extend_from_slice(&header);
            }
            Version::V1 => {
                bytes.push(0x81);
                bytes.extend_from_slice(&header);
                bytes.extend_from_slice(&[0; 4]); // inline-config mask
                bytes.extend_from_slice(BLOCKHASH.as_bytes());
                bytes.push(1); // instruction count
            }
        }

        bytes.push(u8::try_from(self.accounts.len()).unwrap());

        for address in &self.accounts {
            bytes.extend_from_slice(address.as_bytes());
        }

        if self.version == Version::V1 {
            bytes.extend_from_slice(&[self.program, 0]);
            bytes.extend_from_slice(&u16::try_from(self.data.len()).unwrap().to_le_bytes());
        } else {
            bytes.extend_from_slice(BLOCKHASH.as_bytes());
            bytes.extend_from_slice(&[1, self.program, 0, u8::try_from(self.data.len()).unwrap()]);
        }

        bytes.extend_from_slice(&self.data);

        if self.version == Version::V0 {
            self.lookups(&mut bytes);
        }

        bytes
    }

    fn lookups(&self, bytes: &mut Vec<u8>) {
        bytes.push(u8::from(self.lookup));

        if self.lookup {
            bytes.extend_from_slice(&[9; 32]);
            bytes.extend_from_slice(&[1, 0, 0]); // one writable index, no readonly indices
        }
    }

    pub fn wire(&self, signatures: &[(usize, Signature)]) -> Vec<u8> {
        let message = self.message();
        let mut bytes = Vec::new();

        if self.version == Version::V1 {
            bytes.extend_from_slice(&message);
        } else {
            bytes.push(self.signers);
        }

        for index in 0..usize::from(self.signers) {
            let signature = signatures
                .iter()
                .find(|(slot, _)| *slot == index)
                .map_or([0; 64], |(_, signature)| signature.to_bytes());
            bytes.extend_from_slice(&signature);
        }

        if self.version != Version::V1 {
            bytes.extend_from_slice(&message);
        }

        bytes
    }

    pub fn signed_mint(&self) -> Vec<u8> {
        self.wire(&[(1, mint().sign(&self.message()))])
    }
}
