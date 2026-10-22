---
solana_kit_compute_budget: minor
---

Add portable off-chain Compute Budget data codecs, canonical instruction builders
and parsers, plus instruction-list update helpers with exact unsigned prices.

Discovery rejects unknown qualifying Compute Budget tags before a selected match
and tolerates trailing limit and price bytes, matching the pinned Dart helpers.
The unreleased limit lookup now returns a Result so callers can handle malformed
tags. Canonical whole-instruction parsing remains strict.
