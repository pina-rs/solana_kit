---
solana_kit_native: minor
solana_kit_leptoskit: patch
---

Expose the failed private-backup operation through KeyFileError::stage while
preserving the original I/O cause. Caller-supplied I/O conversions retain an
unknown stage, and library failures identify their exact operation.

Deliver the actual wallet service failure before waking the released lease so
failed backups reach the guest as native failures.
