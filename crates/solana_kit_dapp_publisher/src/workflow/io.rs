//! File and checkpoint authority stays with the application, independently of portal credentials.

use solana_kit_client::TransportError;
use std::future::Future;

/// Reads one immutable local source snapshot; the token is never sent to the portal.
/// Task-owning adapters retain capacity until actual termination is joined after cancellation.
pub trait SourceReader {
    /// Concrete local or Send-capable read, cancelled on future Drop.
    type Read<'a>: Future<Output = Result<Vec<u8>, TransportError>> + 'a
    where
        Self: 'a;
    /// Enforces the byte cap while reading, before allocation beyond it.
    fn read<'a>(&'a self, token: &'a str, maximum: usize) -> Self::Read<'a>;
}

/// Exclusive caller-owned checkpoint slot, independent of files and network authentication.
///
/// The caller grants one active workflow owner per key. Save atomically replaces
/// one complete document; successful completion means durable acknowledgment.
/// Adapters enforce caps while reading/writing and cancel owned work on Drop.
/// A cancelled or failed save may have committed, so callers reload before recovery.
/// Task-owning adapters retain capacity until actual termination is joined;
/// dropping a future cannot release a permit while its host work still runs.
pub trait Storage {
    /// One bounded snapshot load.
    type Load<'a>: Future<Output = Result<Option<Vec<u8>>, TransportError>> + 'a
    where
        Self: 'a;
    /// One atomic, durably acknowledged snapshot replacement.
    type Save<'a>: Future<Output = Result<(), TransportError>> + 'a
    where
        Self: 'a;
    /// Reads an absent or complete prior checkpoint, without creating one.
    fn load<'a>(&'a self, key: &'a str, maximum: usize) -> Self::Load<'a>;
    /// Replaces only this caller-approved key, with no retry or detached task.
    fn save<'a>(&'a mut self, key: &'a str, bytes: &'a [u8], maximum: usize) -> Self::Save<'a>;
}
