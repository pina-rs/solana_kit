# Token-2022 off-chain client

Use this crate to construct and inspect the complete pinned Token-2022 instruction
schema with Kit-owned addresses and account metas. It provides 100 typed builders,
strict full-frame parsers, three account layouts, 29 extension variants, 69 upstream
custom errors and explicit program identities. Default dependencies are only Kit
core and its pure base58 dependency. The optional `client` feature adds typed RPC
fetching through a caller-owned Kit transport and executor.

This SDK is exclusively for off-chain application clients. On-chain programs should
use Pina or official Solana program libraries.

```rust
use solana_kit_core::{Address, codecs::Limits};
use solana_kit_token_2022::{Authority, Transfer, TransferAccounts};

let instruction = Transfer { amount: 42 }.instruction(
    TransferAccounts {
        source: Address::from_bytes([1; 32]),
        destination: Address::from_bytes([2; 32]),
        authority: Authority {
            address: Address::from_bytes([3; 32]),
            is_signer: true,
        },
        multi_signers: &[],
    },
    Limits::new(1024, 32),
)?;
# Ok::<(), solana_kit_core::codecs::Error>(())
```

Builders preserve positional order and duplicates. Signing or multisignature
authorities are explicit caller inputs. Required static sysvar defaults can be
selected with `None`; optional omitted accounts use `None` for omission. Parsers
check the expected program, complete discriminator and frame, account privileges,
and remaining-account segments. Fee withdrawal uses the decoded source count to
separate multisigners from writable source accounts. Indistinguishable optional
readonly positions remain an exact raw meta list; the parser does not invent
semantic names for them.

Named `MintCodec`, `TokenCodec` and `MultisigCodec` are structural codecs.
`accounts::Token2022Account` additionally checks initialized state, extension
uniqueness and affinity, signer bounds and the source's invalid mint-extension
combinations. Canonical encoding adds two bytes when an extended account would
otherwise collide with the reserved 355-byte Multisig length. Account dispatch
uses that reserved size and the canonical account-type byte. These checks do not
replace the program's authority, balance or cross-account validation.

`ExtensionsCodec` is a public adapter over the same unchanged generated TLV
sequence used by Mint and Token. It retains shared decode budgets, transactional
failure and termination semantics. Explicit zero-tag entries are rejected on
encoding; standalone `Extension::Uninitialized` remains encodable.

The pinned Dart generator omits canonical TLV payload lengths. Its 400 instruction
and 12 base-account frames are compared whole; its 116 extension rows are compared
as bare tags and payloads, with independent canonical framed literals. Canonical
TLV sizes intentionally differ from that Dart defect. Metadata exposes ordered
wire tuples and preserves duplicate keys, rather than pretending to reproduce
Dart Map object semantics.

`extensions::pre_initialize_mint_extensions` implements the 15 source helper
branches in caller order. Account size helpers use canonical framing. The pinned
IDL and original handwritten package define zero PDA recipes, so this crate adds
no speculative PDA derivation or crypto dependency. Confidential proof bytes,
external proof orchestration, keys and entropy remain caller owned.

`errors::Token2022Error` preserves unknown numeric codes and all 69 pinned Rust
labels. The Dart/IDL source names only 20 errors and has a spelling difference at
code 9. Failure classification uses the actual instruction index and explicit
expected program address.

Data codecs regenerate through the existing shared typed IR/emitter. Invocation
wrappers regenerate through the separate, exact-IDL native projection in
`verification/codegen-design/src/client_projection`. All source contracts resolve
and every output formats before any final file replacement. This provides
single-file atomic replacement, not multi-file crash durability. No runtime JSON
model or second data-layout emitter is used. Drift and original-source audits
remain local verification tools.

The upstream IDL is pinned to revision
`e6b4af36a3dbdfe6f66aaf0c564427fa56671bb0`, SHA256
`13cbda7213599f3312e1b7ffd1c7f176f36ebd74dba9feabe6e9a6874dc1d5d2`.
Retain `TOKEN-2022-LICENSE` and `DART-LICENSE` with reused source and fixtures.
Project-authored wrappers use the repository's Unlicense policy. Publishing is
explicitly disabled. Browser, WASI and interpreter execution, validator semantics,
and installed device journeys are separate integration gates.
