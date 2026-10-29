---
solana_kit_dapp_publisher: minor
---

# Add publisher attestation and prepared transaction policy

Add portable publication attestation messages and sealed prepared-transaction
policy with explicit cryptographic verification. Keep signing, portal calls,
uploads and publication caller-owned. All runtime verification remains pending.
Reject conflicting bytes in an occupied caller signature slot while preserving
matching signatures and filling only empty slots.
Share all 22 original publisher assertion bodies with five portable conformance
groups without enabling host services or performing publication.
