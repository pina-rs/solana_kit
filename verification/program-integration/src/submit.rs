//! Signed transaction construction, submission and confirmation.

use crate::rpc::StatusObserver;
use crate::{IntegrationEnv, SubmitError};
use solana_kit_client::confirmation::{Policy, send_and_confirm};
use solana_kit_client::methods::submission::SendConfig;
use solana_kit_core::{
    Instruction, Signature,
    crypto::Keypair,
    transaction::{Lifetime, Message, Transaction, Version},
};
use std::time::Instant;

const CONFIRM_DEADLINE: std::time::Duration = std::time::Duration::from_secs(90);

impl IntegrationEnv {
    /// Compiles, signs and confirms one legacy transaction.
    ///
    /// Preflight is skipped so a rejected transaction is observed as its real
    /// confirmed status failure rather than only a simulation error.
    pub async fn submit(
        &self,
        instructions: &[Instruction],
        signers: &[&Keypair],
    ) -> Result<Signature, SubmitError> {
        let latest = self
            .client()
            .call(solana_kit_client::methods::LatestBlockhash(Some(
                self.read_config(),
            )))
            .await
            .map_err(SubmitError::Rpc)?;
        let lifetime = Lifetime::Blockhash {
            blockhash: latest.value.blockhash,
            last_valid_block_height: latest.value.last_valid_block_height,
        };

        self.submit_with_lifetime(lifetime, instructions, signers)
            .await
    }

    /// Submits with a caller-supplied blockhash or durable-nonce lifetime.
    pub async fn submit_with_lifetime(
        &self,
        lifetime: Lifetime,
        instructions: &[Instruction],
        signers: &[&Keypair],
    ) -> Result<Signature, SubmitError> {
        let transaction = self.signed_transaction(lifetime, instructions, signers)?;

        self.confirm_signed(&transaction).await
    }

    fn signed_transaction(
        &self,
        lifetime: Lifetime,
        instructions: &[Instruction],
        signers: &[&Keypair],
    ) -> Result<Transaction, SubmitError> {
        let message = Message::compile(
            Version::Legacy,
            self.payer_address(),
            lifetime,
            instructions,
            &[],
        )
        .map_err(SubmitError::Transaction)?;
        let mut transaction = Transaction::new(message);

        for signer in signers {
            transaction = transaction.sign(signer).map_err(SubmitError::Transaction)?;
        }

        Ok(transaction)
    }

    async fn confirm_signed(&self, transaction: &Transaction) -> Result<Signature, SubmitError> {
        send_and_confirm(
            self.client(),
            transaction,
            SendConfig {
                skip_preflight: Some(true),
                max_retries: Some(0),
                ..SendConfig::default()
            },
            Policy::default(),
            |signature| {
                StatusObserver::new(
                    self.client(),
                    signature,
                    Instant::now() + CONFIRM_DEADLINE,
                    self.observed_slot(),
                )
            },
            core::future::pending::<()>(),
        )
        .await
        .map_err(SubmitError::Wait)
    }
}
