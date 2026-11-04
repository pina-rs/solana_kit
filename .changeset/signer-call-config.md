---
solana_kit_signers: minor
solana_kit_browser: minor
---

Add explicit per-call `SignerConfig` and `TransactionSignerConfig` values to every
message, partial, modifying and sending capability method, typed pipeline,
heterogeneous local/native composition and the singular message adapter. Aborted
calls fail with the existing cancellation error before backend or wallet effects.
Transaction backends receive the exact optional `min_context_slot` hint; the
browser wallet forwards it as Wallet Standard `options.minContextSlot`, omits it
for `None`, and rejects slots outside the JavaScript safe integer range before
prompting. Repository callers, tests, fixtures and documentation now pass the
configuration explicitly; `Default::default()` preserves prior behavior.
