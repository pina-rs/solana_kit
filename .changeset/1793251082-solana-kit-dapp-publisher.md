---
solana_kit_dapp_publisher: minor
---

# Add resumable ingestion and metadata hydration

Add an optional caller-driven ingestion/hydration coordinator with bounded
start/resume polling, atomic caller-owned checkpoints, pending-effect non-replay,
immutable local APK reads, typed 0.4.0 metadata and bounded media rehosting.
Reuse Kit hashing/codecs and one shared completion-time deadline boundary.
Preserve signer/policy separation and explicit single cleanup. This unit stops
before mint/save/verify/attest/store completion; native adapters and CLI remain
future work. Thirty-two synthetic test bodies are authored, with all new
compiler/runtime/portable-host verification unexecuted by this source worker.
