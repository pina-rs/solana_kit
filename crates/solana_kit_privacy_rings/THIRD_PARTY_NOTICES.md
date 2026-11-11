# Third-party notices

`solana_kit_privacy_rings` is original Kit code. It does not vendor Helius's
Rust or TypeScript SDKs. Protocol constants, instruction tags, account order,
and ciphertext layout follow the published Helius sources below. Those
repositories keep their own licenses.

- Helius, Solana Privacy Rings documentation, <https://www.helius.dev/docs/privacy>
- Helius, Privacy Rings API reference, version 0.51.2,
  <https://www.helius.dev/docs/api-reference/privacy>
- `helius-labs/zolana` tag `v0.8.0` (`6660d9839714`), in particular
  `program-libs/interface`, `program-libs/event`, `sdk-libs/instruction`,
  `sdk-libs/keypair`, and `sdk-libs/transaction`

Instruction payloads are encoded to match `wincode` 0.5 `SchemaWrite` for the
v0.8.0 field order. The golden byte tests in this crate were produced by
serializing those field layouts with `wincode` 0.5.1.
