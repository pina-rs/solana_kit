//! The pinned ZK query catalog. Decoding a proof does not authenticate it.

mod locators;
mod models;
mod requests;
mod response_checks;

pub use crate::das::GetSignaturesForAssetRequest as GetZkSignaturesForAssetRequest;
pub use locators::*;
pub use models::*;
pub use requests::*;
use solana_kit_client::methods::Contextual;

method!(
    GetCompressedAccountRequest,
    "getCompressedAccount",
    GetCompressedAccountResponse,
    response_checks::account
);
method!(
    GetCompressedAccountProofRequest,
    "getCompressedAccountProof",
    Contextual<CompressedAccountProof>,
    response_checks::proof
);
method!(
    GetCompressedAccountsByOwnerRequest,
    "getCompressedAccountsByOwner",
    Contextual<CompressedPage<CompressedAccount>>
);
method!(
    GetCompressedBalanceRequest,
    "getCompressedBalance",
    Contextual<u64>
);
method!(
    GetCompressedBalanceByOwnerRequest,
    "getCompressedBalanceByOwner",
    Contextual<u64>
);
method!(
    GetCompressedMintTokenHoldersRequest,
    "getCompressedMintTokenHolders",
    Contextual<CompressedPage<OwnerBalance>>
);
method!(
    GetCompressedTokenAccountBalanceRequest,
    "getCompressedTokenAccountBalance",
    Contextual<CompressedTokenAccountBalance>
);
method!(
    GetCompressedTokenAccountsByDelegateRequest,
    "getCompressedTokenAccountsByDelegate",
    Contextual<CompressedPage<CompressedTokenAccount>>
);
method!(
    GetCompressedTokenAccountsByOwnerRequest,
    "getCompressedTokenAccountsByOwner",
    Contextual<CompressedPage<CompressedTokenAccount>>
);
method!(
    GetCompressedTokenBalancesByOwnerRequest,
    "getCompressedTokenBalancesByOwner",
    Contextual<CompressedTokenBalanceList>
);
method!(
    GetCompressedTokenBalancesByOwnerV2Request,
    "getCompressedTokenBalancesByOwnerV2",
    Contextual<CompressedPage<CompressedTokenBalance>>
);
method!(
    GetCompressionSignaturesForAccountRequest,
    "getCompressionSignaturesForAccount",
    Contextual<CompressedItems<CompressedSignature>>
);
method!(
    GetCompressionSignaturesForAddressRequest,
    "getCompressionSignaturesForAddress",
    Contextual<CompressedPage<CompressedSignature>>
);
method!(
    GetCompressionSignaturesForOwnerRequest,
    "getCompressionSignaturesForOwner",
    Contextual<CompressedPage<CompressedSignature>>
);
method!(
    GetCompressionSignaturesForTokenOwnerRequest,
    "getCompressionSignaturesForTokenOwner",
    Contextual<CompressedPage<CompressedSignature>>
);
method!(
    GetLatestCompressionSignaturesRequest,
    "getLatestCompressionSignatures",
    Contextual<CompressedPage<CompressedSignature>>
);
method!(
    GetLatestNonVotingSignaturesRequest,
    "getLatestNonVotingSignatures",
    Contextual<CompressedItems<CompressedSignatureWithError>>
);
method!(
    GetMultipleCompressedAccountProofsRequest,
    "getMultipleCompressedAccountProofs",
    Contextual<Vec<CompressedAccountProof>>,
    response_checks::proofs
);
method!(
    GetMultipleCompressedAccountsRequest,
    "getMultipleCompressedAccounts",
    Contextual<CompressedItems<Option<CompressedAccount>>>,
    response_checks::accounts
);
method!(
    GetMultipleNewAddressProofsRequest,
    "getMultipleNewAddressProofs",
    Contextual<Vec<NewAddressProof>>,
    response_checks::new_addresses
);
method!(
    GetMultipleNewAddressProofsV2Request,
    "getMultipleNewAddressProofsV2",
    Contextual<Vec<NewAddressProof>>,
    response_checks::new_addresses_with_trees
);
method!(
    GetTransactionWithCompressionInfoRequest,
    "getTransactionWithCompressionInfo",
    TransactionWithCompressionInfo
);
method!(
    GetValidityProofRequest,
    "getValidityProof",
    Contextual<ValidityProof>
);
method!(GetIndexerHealth, "getIndexerHealth", IndexerHealth);
method!(GetIndexerSlot, "getIndexerSlot", u64);
