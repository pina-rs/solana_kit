use solana_kit_core::{
    Address, Hash,
    transaction::{Lifetime, Message, Transaction, Version},
};

pub fn transaction(payer: Address, token: u8) -> Transaction {
    let lifetime = Lifetime::Blockhash {
        blockhash: Hash::from_bytes([token; 32]),
        last_valid_block_height: 100,
    };
    Transaction::new(Message::compile(Version::V0, payer, lifetime, &[], &[]).unwrap())
}
