//! Sender address observations remain per slot and precede submission effects.

use core::future::{Ready, ready};
use solana_kit_core::{
    AccountMeta, AccountRole, Address, Hash, Instruction, Signature,
    transaction::{Lifetime, Message, Transaction, Version},
};
use solana_kit_signers::{
    Error, SignatureOutput, Signer, SignerCapability, TransactionPartialSigner,
    TransactionSendingSigner, TransactionSigners,
};
use std::cell::Cell;

const PAYER: Address = Address::from_bytes([1; 32]);
const OTHER_SIGNER: Address = Address::from_bytes([2; 32]);
const PROGRAM: Address = Address::from_bytes([3; 32]);

#[derive(Default)]
struct ObservedSender {
    address_reads: Cell<usize>,
    submissions: Cell<usize>,
}

impl Signer for ObservedSender {
    fn address(&self) -> Address {
        self.address_reads.set(self.address_reads.get() + 1);
        PAYER
    }
}

impl TransactionPartialSigner for ObservedSender {
    type Sign<'a> = Ready<Result<SignatureOutput, Error>>;

    fn sign_transaction<'a>(&'a self, _: &'a Transaction) -> Self::Sign<'a> {
        ready(Err(Error::Rejected))
    }
}

impl TransactionSendingSigner for ObservedSender {
    type Send<'a> = Ready<Result<Signature, Error>>;

    fn send_transaction<'a>(&'a self, _: &'a Transaction) -> Self::Send<'a> {
        self.submissions.set(self.submissions.get() + 1);
        ready(Err(Error::Rejected))
    }
}

fn two_signer_transaction() -> Transaction {
    let instruction = Instruction {
        program_address: PROGRAM,
        accounts: vec![AccountMeta {
            address: OTHER_SIGNER,
            role: AccountRole::ReadonlySigner,
        }],
        data: Vec::new(),
    };
    let message = Message::compile(
        Version::Legacy,
        PAYER,
        Lifetime::Blockhash {
            blockhash: Hash::from_bytes([4; 32]),
            last_valid_block_height: 100,
        },
        &[instruction],
        &[],
    )
    .unwrap();
    assert_eq!(message.signers(), &[PAYER, OTHER_SIGNER]);

    Transaction::new(message)
}

#[tokio::test(flavor = "current_thread")]
async fn sender_address_is_observed_per_slot_before_missing_signature_rejects() {
    let sender = ObservedSender::default();
    let capability = SignerCapability::new(&sender);
    let partials = TransactionSigners::<ObservedSender>::new(&[]).unwrap();
    let transaction = two_signer_transaction();
    let result = partials.sign_and_send(&transaction, &[&capability]).await;

    assert_eq!(
        result,
        Err(Error::Transaction(
            solana_kit_core::transaction::Error::MissingSignature
        ))
    );
    assert_eq!(sender.submissions.get(), 0);
    assert_eq!(
        sender.address_reads.get(),
        transaction.message().signers().len()
    );
}
