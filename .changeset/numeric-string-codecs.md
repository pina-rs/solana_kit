---
solana_kit_core: minor
---

# Add bounded numeric and string codec capabilities

Provide exact 256-bit numeric text conversion, explicit/default numeric byte
order, integer radix and bit-reslicing string codecs, hexadecimal pairs and odd
nibbles, configured strict/lossy UTF-8, and bounded null helpers. The optional
`text-codecs` feature adds audited Base64 conversion with pinned Dart leniency.
Default and existing capability dependency graphs remain unchanged. This is an
off-chain client SDK; no component publication is enabled.
