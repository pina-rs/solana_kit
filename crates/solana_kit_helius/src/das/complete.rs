//! Complete application records retain the provider's optional metadata.

use super::{NftEdition, TokenAccount, TokenAccountList};
use crate::Field;

model!(
    "An NFT edition with the mint and edition required by the Dart Kit contract.",
    CompleteNftEdition {
        mint: String => "mint",
        edition: u64 => "edition",
    } optional {
        edition_address: String => "edition_address",
        burnt: bool => "burnt",
    }
);
model!(
    "A token account with its identity and exact raw balance present.",
    CompleteTokenAccount {
        address: String => "address",
        mint: String => "mint",
        owner: String => "owner",
        amount: u64 => "amount",
    } optional {
        delegated_amount: u64 => "delegated_amount",
        frozen: bool => "frozen",
        burnt: bool => "burnt",
        token_extensions: serde_json::Value => "token_extensions",
    }
);
model!(
    "A token-account page with counts and a list of complete accounts present.",
    CompleteTokenAccountList {
        total: u64 => "total",
        limit: u64 => "limit",
        token_accounts: Vec<CompleteTokenAccount> => "token_accounts",
    } optional {
        last_indexed_slot: u64 => "last_indexed_slot",
        page: u64 => "page",
        cursor: String => "cursor",
    }
);

/// A partial provider record cannot satisfy a complete application contract.
///
/// Field names are static wire keys; errors never include account values.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IncompleteDasRecord {
    /// A required field was absent from the provider record.
    Missing {
        /// The missing JSON field's wire name.
        field: &'static str,
        /// The zero-based token-account position when converting a page.
        token_account_index: Option<usize>,
    },
    /// A required field was explicitly null in the provider record.
    Null {
        /// The null JSON field's wire name.
        field: &'static str,
        /// The zero-based token-account position when converting a page.
        token_account_index: Option<usize>,
    },
}

impl IncompleteDasRecord {
    fn at_token_account(mut self, index: usize) -> Self {
        match &mut self {
            Self::Missing {
                token_account_index,
                ..
            }
            | Self::Null {
                token_account_index,
                ..
            } => {
                *token_account_index = Some(index);
            }
        }

        self
    }
}

impl core::fmt::Display for IncompleteDasRecord {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let (field, presence, index) = match self {
            Self::Missing {
                field,
                token_account_index,
            } => (field, "missing", token_account_index),
            Self::Null {
                field,
                token_account_index,
            } => (field, "null", token_account_index),
        };

        if let Some(index) = index {
            write!(formatter, "token account {index}: ")?;
        }

        write!(formatter, "{presence} required field `{field}`")
    }
}

impl std::error::Error for IncompleteDasRecord {}

impl TryFrom<NftEdition> for CompleteNftEdition {
    type Error = IncompleteDasRecord;

    /// Requires the mint and edition, moving optional metadata without reencoding.
    fn try_from(record: NftEdition) -> Result<Self, Self::Error> {
        Ok(Self {
            mint: required(record.mint, "mint")?,
            edition: required(record.edition, "edition")?,
            edition_address: record.edition_address,
            burnt: record.burnt,
        })
    }
}

impl TryFrom<TokenAccount> for CompleteTokenAccount {
    type Error = IncompleteDasRecord;

    /// Requires account identity and amount, retaining exact integers and metadata.
    fn try_from(record: TokenAccount) -> Result<Self, Self::Error> {
        Ok(Self {
            address: required(record.address, "address")?,
            mint: required(record.mint, "mint")?,
            owner: required(record.owner, "owner")?,
            amount: required(record.amount, "amount")?,
            delegated_amount: record.delegated_amount,
            frozen: record.frozen,
            burnt: record.burnt,
            token_extensions: record.token_extensions,
        })
    }
}

impl TryFrom<TokenAccountList> for CompleteTokenAccountList {
    type Error = IncompleteDasRecord;

    /// Requires page counts and every account; a partial account rejects the page.
    fn try_from(record: TokenAccountList) -> Result<Self, Self::Error> {
        let total = required(record.total, "total")?;
        let limit = required(record.limit, "limit")?;
        let token_accounts = required(record.token_accounts, "token_accounts")?
            .into_iter()
            .enumerate()
            .map(|(index, account)| {
                CompleteTokenAccount::try_from(account)
                    .map_err(|error| error.at_token_account(index))
            })
            .collect::<Result<_, _>>()?;

        Ok(Self {
            total,
            limit,
            token_accounts,
            last_indexed_slot: record.last_indexed_slot,
            page: record.page,
            cursor: record.cursor,
        })
    }
}

fn required<T>(field: Field<T>, name: &'static str) -> Result<T, IncompleteDasRecord> {
    match field {
        Field::Missing => Err(IncompleteDasRecord::Missing {
            field: name,
            token_account_index: None,
        }),
        Field::Null => Err(IncompleteDasRecord::Null {
            field: name,
            token_account_index: None,
        }),
        Field::Value(value) => Ok(value),
    }
}
