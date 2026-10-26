//! Cursor collection adds explicit caller budgets to the reference convenience loop.

use super::{accounts::*, pages::*};
use solana_kit_client::{Client, Transport, methods::Method};
use std::{collections::BTreeSet, fmt};

/// Bounds for one all-pages operation. Dropping the future cancels its active call.
#[derive(Debug, Clone, Copy)]
pub struct Pagination {
    /// Maximum exchanges, including the first page.
    pub max_pages: usize,
    /// Maximum retained records across all pages.
    pub max_items: usize,
}

/// A collection failure never returns a silently truncated successful result.
#[derive(Debug)]
pub enum PaginationError {
    /// An exchange or typed response failed.
    Rpc(solana_kit_client::Error),
    /// The next page would exceed the caller's exchange budget.
    Pages,
    /// The next page exceeds the caller's retained item budget.
    Items,
    /// The server repeated a cursor, which would otherwise loop indefinitely.
    RepeatedCursor,
}

impl fmt::Display for PaginationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "Helius pagination error: {self:?}")
    }
}

impl std::error::Error for PaginationError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Rpc(error) => Some(error),
            _ => None,
        }
    }
}

trait PageRequest: Method + Clone {
    type Item;
    fn cursor(&self) -> Option<&str>;
    fn set_cursor(&mut self, cursor: String);
    fn page(response: Self::Response) -> (Vec<Self::Item>, Option<String>);
}

impl PageRequest for GetProgramAccountsV2Request {
    type Item = ProgramAccountV2;
    fn cursor(&self) -> Option<&str> {
        self.config.as_ref()?.pagination_key.as_deref()
    }
    fn set_cursor(&mut self, cursor: String) {
        self.config
            .get_or_insert_with(Default::default)
            .pagination_key = Some(cursor);
    }
    fn page(response: Self::Response) -> (Vec<Self::Item>, Option<String>) {
        let page = response.into_page();
        (page.accounts, page.pagination_key)
    }
}

impl PageRequest for GetTokenAccountsByOwnerV2Request {
    type Item = TokenAccountV2;
    fn cursor(&self) -> Option<&str> {
        self.config.as_ref()?.pagination_key.as_deref()
    }
    fn set_cursor(&mut self, cursor: String) {
        self.config
            .get_or_insert_with(Default::default)
            .pagination_key = Some(cursor);
    }
    fn page(response: Self::Response) -> (Vec<Self::Item>, Option<String>) {
        let page = response.into_page();
        (page.accounts, page.pagination_key)
    }
}

async fn collect<T: Transport, R: PageRequest>(
    client: &Client<T>,
    mut request: R,
    limits: Pagination,
) -> Result<Vec<R::Item>, PaginationError> {
    let mut collection = Collection::new(request.cursor(), limits.max_items);

    for _ in 0..limits.max_pages {
        let response = client
            .call(request.clone())
            .await
            .map_err(PaginationError::Rpc)?;
        let (page, cursor) = R::page(response);

        match collection.append(page, cursor)? {
            Some(cursor) => request.set_cursor(cursor),
            None => return Ok(collection.items),
        }
    }

    Err(PaginationError::Pages)
}

/// Records obey the item ceiling; cursor history is bounded by the exchange ceiling.
struct Collection<T> {
    items: Vec<T>,
    seen: BTreeSet<String>,
    maximum: usize,
}

impl<T> Collection<T> {
    fn new(initial_cursor: Option<&str>, maximum: usize) -> Self {
        Self {
            items: Vec::new(),
            seen: initial_cursor.into_iter().map(str::to_owned).collect(),
            maximum,
        }
    }

    fn append(
        &mut self,
        page: Vec<T>,
        cursor: Option<String>,
    ) -> Result<Option<String>, PaginationError> {
        if page.len() > self.maximum.saturating_sub(self.items.len()) {
            return Err(PaginationError::Items);
        }

        self.items.extend(page);

        let Some(cursor) = cursor else {
            return Ok(None);
        };

        if !self.seen.insert(cursor.clone()) {
            return Err(PaginationError::RepeatedCursor);
        }

        Ok(Some(cursor))
    }
}

/// Collects program accounts with finite exchange and retained-record budgets.
pub async fn all_program_accounts<T: Transport>(
    client: &Client<T>,
    request: GetProgramAccountsV2Request,
    limits: Pagination,
) -> Result<Vec<ProgramAccountV2>, PaginationError> {
    collect(client, request, limits).await
}

/// Collects token accounts with finite exchange and retained-record budgets.
pub async fn all_token_accounts_by_owner<T: Transport>(
    client: &Client<T>,
    request: GetTokenAccountsByOwnerV2Request,
    limits: Pagination,
) -> Result<Vec<TokenAccountV2>, PaginationError> {
    collect(client, request, limits).await
}
