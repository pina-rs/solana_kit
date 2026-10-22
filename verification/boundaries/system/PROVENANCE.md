# CreateAccount oracle

Reference: `openbudgetfun/solana_kit` commit
`0138bb956d2d81ccc0c56edc602401c0df2f2b46`,
`packages/solana_kit_system/lib/src/generated/instructions/create_account.dart`.
The source selects discriminator 0, u32/u64/u64/address encoding and payer then
new account, both writable signers. Numeric codecs use little endian by default.
Its license is retained at `../../conformance/fixtures/DART-LICENSE`.

This experiment manually fixes the independent wire facts, rather than using
the Rust constructor to generate its own expected bytes. For lamports
`0x0102030405060708`, space `0x1112131415161718`, and owner `[0x33; 32]`,
the 52-byte payload is `0000000008070605040302011817161514131211` followed by
32 bytes of `33`. The System Program address is 32 zero bytes; payer `[0x11; 32]`
precedes new account `[0x22; 32]`. Both must be signers and writable.

`verify(true)` changes the expected discriminator and must return the exact
serialization error after calling the same official constructor. The oracle
does not run Dart, submit a transaction, or claim validator acceptance.

The implementation under test is the registry crate
`solana-system-interface =3.3.0`, with default features off and only `bincode`
selected; `solana-address =2.7.0` also has default features off. Cargo.lock
preserves every registry checksum. No official implementation source is copied.
