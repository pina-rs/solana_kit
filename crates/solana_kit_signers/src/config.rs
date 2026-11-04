/// Per-call options for message signers.
///
/// A configuration value belongs to one call. Compositions and adapters carry
/// it unchanged to every backend operation; they never store it as constructor
/// state. [`Default`] preserves the behavior of calls made before per-call
/// configuration existed.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SignerConfig {
    /// The caller has already abandoned this signing operation.
    ///
    /// A backend must return its existing cancellation failure before any
    /// signing, prompt, or network effect. This is only an initial signal:
    /// dropping an in-flight future remains the cancellation mechanism for an
    /// operation that has already started.
    pub aborted: bool,
}

/// Per-call options for transaction signers.
///
/// Like [`SignerConfig`], this value is supplied to each backend call. SDK
/// compositions and adapters do not use it as constructor state.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TransactionSignerConfig {
    /// The caller has already abandoned this signing operation.
    ///
    /// A backend must return its existing cancellation failure before any
    /// signing, prompt, simulation, or network effect. Dropping an in-flight
    /// future remains the cancellation mechanism for an operation already
    /// started.
    pub aborted: bool,
    /// Exact slot at or after which a simulating backend may evaluate the
    /// transaction.
    ///
    /// This is an optional simulation hint, not authenticated freshness and not
    /// proof that the transaction is valid at that slot. Every transaction
    /// backend receives the same hint. A local keypair does not simulate and
    /// explicitly ignores it.
    pub min_context_slot: Option<u64>,
}
