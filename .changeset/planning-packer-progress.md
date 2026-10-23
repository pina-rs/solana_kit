---
solana_kit_core: patch
---

# Reject completed packers that append no instructions

Planning now rejects a successful custom packer that reports completion without
appending work. Synchronous planning, asynchronous planning, and append return
`NoProgress` instead of producing an empty transaction or silently losing work.
