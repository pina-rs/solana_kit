//! Enhanced queries retain their distinct object and positional parameter forms.

mod accounts;
mod history;
mod pages;
mod pagination;
mod transfers;

pub use accounts::*;
pub use history::*;
pub use pages::*;
pub use pagination::{
    Pagination, PaginationError, all_program_accounts, all_token_accounts_by_owner,
};
pub use transfers::*;

method!(
    GetProgramAccountsV2Request,
    "getProgramAccountsV2",
    GetProgramAccountsV2Response
);
method!(
    GetTokenAccountsByOwnerV2Request,
    "getTokenAccountsByOwnerV2",
    GetTokenAccountsByOwnerV2Response
);
impl solana_kit_client::methods::Method for GetTransactionsForAddressRequest {
    type Response = GetTransactionsForAddressResponse;
    const NAME: &'static str = "getTransactionsForAddress";

    fn decode(&self, result: &str) -> Result<Self::Response, solana_kit_client::Error> {
        let response: Self::Response =
            serde_json::from_str(result).map_err(solana_kit_client::Error::Schema)?;
        let details = self
            .config
            .as_ref()
            .and_then(|config| config.transaction_details)
            .unwrap_or(TransactionDetails::Signatures);
        let encoding = self
            .config
            .as_ref()
            .and_then(|config| config.encoding)
            .unwrap_or(TransactionEncoding::Json);
        let valid = response.data.iter().all(|record| match (details, record) {
            (TransactionDetails::Signatures, TransactionForAddress::Signature(_)) => true,
            (TransactionDetails::Full, TransactionForAddress::Full(record)) => {
                record.transaction.matches_encoding(encoding)
            }
            _ => false,
        });

        if !valid {
            return Err(solana_kit_client::Error::Schema(
                <serde_json::Error as serde::de::Error>::custom(
                    "history record does not match requested details or encoding",
                ),
            ));
        }

        Ok(response)
    }
}
method!(
    GetTransfersByAddressRequest,
    "getTransfersByAddress",
    GetTransfersByAddressResponse
);
