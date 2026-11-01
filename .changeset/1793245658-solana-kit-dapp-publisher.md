---
solana_kit_dapp_publisher: minor
---

# Add typed portal protocol and bounded ingestion retry

Add an optional runtime-neutral portal client with thirteen fixed typed
operations, raw backend DTOs and bounded duplicate-aware JSON admission. Keep
origin/authentication with the caller's transport, presigned binary uploads with
a separate caller-owned uploader, and signing/submission explicitly caller-owned.
Only explicit idempotent ingestion creation supports the bounded three-attempt
retry policy with caller clock/deadline. Normalized workflows, metadata/media and
native CLI remain future work. Preserve null envelope branch presence and let
the deadline win completion ties after polling and decoding. Twenty-three
synthetic protocol test bodies retain their predicates through four fallible
shared native/portable groups and a real failure-continuation unit control;
all new portal compiler/runtime verification is unexecuted by this source worker.
