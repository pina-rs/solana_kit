---
solana_kit_privacy_rings: minor
---

# Enforce Nitro attestation freshness and bind safe challenges

`NitroPolicy` gains `max_age_secs` and `max_future_secs` (seconds), and `verify_nitro` now compares the document's signed NSM timestamp with the caller's `now_secs` in exact milliseconds after the quote and its certificate chain authenticate, so a stale or excessively future document is rejected even while its certificates are still valid; zero bounds are the strictest setting, never a disabled check. The signed timestamp is retained on `VerifiedNitro::timestamp_ms` so callers can decide any later reuse themselves. `PrivacyProverClient::attest` runs one attestation round: it draws a fresh 32-byte nonce from a caller-supplied `rand_core` generator per attempt, fetches the quote, and verifies exactly that nonce — an entropy failure returns before any request, and a quote from a previous challenge cannot verify. The raw fetch helper is renamed to `fetch_unverified_attestation` to say what it does not do; callers that use it directly still own CSPRNG nonce generation and the one-attempt fetch/verify pairing.
