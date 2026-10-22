---
solana_kit_core: minor
---

# Add bounded deterministic binary codecs

Encode exact-width integers through 256 bits, IEEE 754 floats, canonical
short-u16, raw bytes, strict UTF-8, sequences, tuples, explicit option layouts,
and constant or length framing. Full decoding rejects trailing data; prefix
decoding returns the next offset. Caller-selected byte and allocation budgets
apply across nested decoders. The codec layer adds no runtime dependencies.

The shared portable conformance suite grows to 31 cases. This records release
intent for off-chain client applications; component publication stays disabled.
