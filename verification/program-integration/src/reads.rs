//! Typed RPC reads and deployed-image verification.

use crate::fixtures;
use crate::{Error, IntegrationEnv};
use solana_kit_client::Presence;
use solana_kit_client::methods::history::HistoricalCommitment;
use solana_kit_client::methods::{
    AccountInfo, Balance,
    accounts::{
        AccountConfig, AccountData, AccountEncoding, AccountInfoWithConfig,
        MinimumBalanceForRentExemption, ParsedAccount,
    },
    node::Slot,
    transaction_results::{
        ConfirmedTransaction, Transaction as TransactionRequest, TransactionEncoding,
        TransactionMeta, TransactionReadConfig,
    },
};
use solana_kit_core::{
    Address, Signature,
    accounts::{EncodedAccount, MaybeAccount},
};

const PROGRAM_DATA_HEADER: usize = 45;

impl IntegrationEnv {
    /// Confirmed lamport balance from an independent typed read.
    pub async fn balance(&self, address: Address) -> Result<u64, Error> {
        let balance = self
            .client()
            .call(Balance(address, Some(self.read_config())))
            .await?;

        Ok(balance.value)
    }

    /// Confirmed rent-exempt minimum for an explicit allocation.
    pub async fn rent(&self, size: u64) -> Result<u64, Error> {
        Ok(self
            .client()
            .call(MinimumBalanceForRentExemption(size, None))
            .await?)
    }

    /// Current confirmed slot from the node.
    pub async fn slot(&self) -> Result<u64, Error> {
        Ok(self.client().call(Slot(Some(self.read_config()))).await?)
    }

    /// Confirmed account snapshot, or `None` when the node reports absence.
    pub async fn account(&self, address: Address) -> Result<Option<EncodedAccount>, Error> {
        let response = self
            .client()
            .call(AccountInfo(address, self.read_config()))
            .await?;

        match response.value {
            MaybeAccount::Missing(_) => Ok(None),
            MaybeAccount::Present(account) => Ok(Some(account)),
        }
    }

    /// Confirmed `jsonParsed` snapshot, rejecting an encoded fallback.
    pub async fn parsed_account(&self, address: Address) -> Result<Option<ParsedAccount>, Error> {
        let response = self
            .client()
            .call(AccountInfoWithConfig(
                address,
                Some(AccountConfig {
                    read: self.read_config(),
                    encoding: Some(AccountEncoding::JsonParsed),
                    data_slice: None,
                }),
            ))
            .await?;

        match response.value {
            MaybeAccount::Missing(_) => Ok(None),

            MaybeAccount::Present(account) => match account.data {
                AccountData::Parsed(parsed) => Ok(Some(parsed)),
                _ => Err(Error::Client(
                    "the node returned an encoded account instead of jsonParsed",
                )),
            },
        }
    }

    /// Fetches one confirmation transaction including its logs and compute use.
    pub async fn transaction(
        &self,
        signature: Signature,
    ) -> Result<Option<ConfirmedTransaction>, Error> {
        Ok(self
            .client()
            .call(TransactionRequest(
                signature,
                Some(TransactionReadConfig {
                    encoding: Some(TransactionEncoding::Json),
                    commitment: Some(HistoricalCommitment::Confirmed),
                    max_supported_transaction_version: Some(1),
                }),
            ))
            .await?)
    }

    /// Execution logs from the confirmed transaction, rejecting absent metadata.
    pub async fn logs(&self, signature: Signature) -> Result<Vec<String>, Error> {
        let meta = self.execution_meta(signature).await?;

        match meta.log_messages {
            Presence::Present(Some(logs)) => Ok(logs),
            _ => Err(Error::Client("transaction logs are unavailable")),
        }
    }

    /// Compute units actually consumed by a confirmed transaction.
    pub async fn compute_units(&self, signature: Signature) -> Result<u64, Error> {
        let meta = self.execution_meta(signature).await?;

        match meta.compute_units_consumed {
            Presence::Present(Some(units)) => Ok(units),
            _ => Err(Error::Client(
                "transaction compute-unit metadata is unavailable",
            )),
        }
    }

    async fn execution_meta(&self, signature: Signature) -> Result<TransactionMeta, Error> {
        let transaction = self.transaction(signature).await?.ok_or(Error::Client(
            "transaction is not available at confirmed commitment",
        ))?;

        transaction
            .details
            .meta
            .ok_or(Error::Client("transaction has no execution metadata"))
    }
}

/// Proves every deployed image is executable and byte-identical to its file.
pub(crate) async fn verify_loaded_programs(env: &IntegrationEnv) -> Result<(), Error> {
    for loaded in env.loaded_programs() {
        verify_program(env, loaded).await?;
    }

    Ok(())
}

async fn verify_program(
    env: &IntegrationEnv,
    loaded: &fixtures::LoadedProgram,
) -> Result<(), Error> {
    let address = loaded.deployed_at;
    let account = env
        .account(address)
        .await?
        .ok_or_else(|| Error::ProgramImage(format!("{address} is absent")))?;
    require_external_program(address, &account)?;

    let program_data = crate::rpc::program_data_address(&account.data)
        .ok_or_else(|| Error::ProgramImage(format!("{address} has no program state")))?;
    let data_account = env.account(program_data).await?.ok_or_else(|| {
        Error::ProgramImage(format!("{address} ProgramData {program_data} is absent"))
    })?;
    let elf = data_account
        .data
        .get(PROGRAM_DATA_HEADER..)
        .filter(|bytes| !bytes.is_empty())
        .ok_or_else(|| Error::ProgramImage(format!("{address} ProgramData has no ELF bytes")))?;
    let deployed = fixtures::sha256_hex(elf);

    if deployed != loaded.sha256 {
        return Err(Error::ProgramImage(format!(
            "{address} deployed hash {deployed} differs from fixture {}",
            loaded.sha256
        )));
    }

    Ok(())
}

fn require_external_program(address: Address, account: &EncodedAccount) -> Result<(), Error> {
    if !account.executable {
        return Err(Error::ProgramImage(format!("{address} is not executable")));
    }

    if account.owner != fixtures::BPF_LOADER_UPGRADEABLE {
        return Err(Error::ProgramImage(format!(
            "{address} has owner {}, expected the upgradeable loader",
            account.owner
        )));
    }

    Ok(())
}
