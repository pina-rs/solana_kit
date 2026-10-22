---
solana_kit_core: minor
---

# Add optional exact decimal and binary fixed points

The `fixed-points` feature adds validated shapes, checked arithmetic, explicit
rounding, exact decimal parsing and formatting, rescaling, comparisons, ranges,
and bounded little/big-endian codecs using optional no_std BigInt primitives.
The default graph remains bs58-only. Widths and scales are bounded at 4096,
input at 8192 bytes, and ratio/scalar operands at 32768 magnitude bits.

The portable suite adds three fixed-point cases. Custom callback constructor
parity and extreme lossy float conversion parity remain explicitly open in the
126-entry capability receipt; this is not complete package parity.
