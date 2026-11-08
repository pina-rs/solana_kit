---
solana_kit_memo: major
---

# solana_kit_memo: build and parse Memo instructions

A handwritten `no_std + alloc` client for the Memo program. Build raw UTF-8
invocations with ordered signers, parse complete invocations, and extract memos
across the v1, v3, and v4 identities.

UTF-8 is strict, nulls stay present, and decoding removes a leading BOM exactly
as the reference codec does. `Limits` bounds input bytes, decoded text, signer
positions, and extracted payloads without inventing validator or transaction
limits.
