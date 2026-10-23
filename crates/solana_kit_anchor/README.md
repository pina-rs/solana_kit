# Portable Anchor runtime

`solana_kit_anchor` codes Anchor IDL 0.30 instructions, accounts and events
at runtime. It owns exact-width dynamic values, preserves explicit IDL
discriminators, attributes encode failures to fields, and resolves standard
and program-defined errors.

This is an off-chain application client library. Use Pina or official Solana
program libraries for on-chain programs. The default and only dependency graph
is `no_std + alloc`; it starts no executor and performs no network operations.
Construct a normal Kit `Instruction` from encoded data and caller-owned account
metadata, then use existing Kit message, signer and client APIs.

Every operation takes explicit byte, item and depth budgets. Field types remain
deferred, so an IDL containing unsupported generic instantiations can be parsed
and inspected; using that field returns an error. One-byte options, all pinned
integer widths through 128 bits, both floats, strings, bytes, addresses,
vectors, arrays, structs and enums are supported.

```rust
use solana_kit_anchor::{Coder, Idl, Limits, Values};
use solana_kit_core::Instruction;

let limits = Limits::new(4096, 1024, 32);
let idl = Idl::parse(r#"{
    "address": "11111111111111111111111111111111",
    "instructions": [{"name": "close", "discriminator": [1, 2, 3]}]
}"#, limits)?;
let data = Coder::new(&idl, limits).encode_instruction("close", &Values::new())?;
let instruction = Instruction {
    program_address: idl.address.parse()?,
    accounts: vec![],
    data,
};
assert_eq!(instruction.data, [1, 2, 3]);
# Ok::<(), Box<dyn std::error::Error>>(())
```

See `docs/parity/anchor.md` for the pinned source contract, exact differences,
behavior evidence and remaining portability/runtime limits.
