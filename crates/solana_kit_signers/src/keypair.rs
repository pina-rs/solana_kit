use core::future::{Ready, ready};

use solana_kit_core::{Address, crypto::Keypair, transaction::Transaction};

use crate::{
    Error, MessageSigner, SignatureOutput, Signer, SignerConfig, TransactionPartialSigner,
    TransactionSignerConfig,
};

impl Signer for Keypair {
    fn address(&self) -> Address {
        self.address()
    }
}

impl MessageSigner for Keypair {
    type Sign<'a> = Ready<Result<SignatureOutput, Error>>;

    fn sign_message<'a>(&'a self, bytes: &'a [u8], config: SignerConfig) -> Self::Sign<'a> {
        if config.aborted {
            return ready(Err(Error::Cancelled));
        }

        ready(Ok(SignatureOutput::new(
            self.address(),
            bytes,
            self.sign(bytes),
        )))
    }
}

impl TransactionPartialSigner for Keypair {
    type Sign<'a> = Ready<Result<SignatureOutput, Error>>;

    fn sign_transaction<'a>(
        &'a self,
        transaction: &'a Transaction,
        config: TransactionSignerConfig,
    ) -> Self::Sign<'a> {
        if config.aborted {
            return ready(Err(Error::Cancelled));
        }

        // A local keypair does not simulate, so the optional slot hint has no
        // meaning here and is deliberately ignored.
        let bytes = transaction.message().bytes();
        let result = if transaction.message().signers().contains(&self.address()) {
            Ok(SignatureOutput::new(
                self.address(),
                bytes,
                self.sign(bytes),
            ))
        } else {
            Err(solana_kit_core::transaction::Error::UnknownSigner)
        };

        ready(result.map_err(Error::from))
    }
}
