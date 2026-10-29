# Publisher attestation and prepared-transaction policy

The immutable reference is Dart Solana Kit revision
`0138bb956d2d81ccc0c56edc602401c0df2f2b46`. Canonical IDs in `capabilities.tsv`
remain unchanged and pending. The package disposition ledger refines ownership
of the publisher package from generic workspace tooling to the unpublished
`solana_kit_dapp_publisher` library and its future optional native CLI.

This source slice implements `attestation.dart:28`, `publication_signer.dart:61`,
`:159`, `:239`, `:354` and `:383` through exact JSON messages, detached signature
verification and typed publication policy. `signPreparedTransaction` at `:136`
becomes caller-owned signing between a private validation seal and verified
attachment. It invokes no signer, network, upload or submission. Core and client
gain no publisher or host dependency. There is no registry reservation or
publication enablement.

Rust deliberately requires a real 32-byte `Hash`, an unsigned 64-bit slot and
exactly 32 ASCII request-id digits. It does not preserve the Dart override's
arbitrary strings, fractional slots or acceptance of a longer unrelated signed
message. Attestation signatures must authenticate the returned exact JSON.

Prepared policy preserves canonical program IDs, exact signer address sets,
verification of every present signature, the first Create's collection and required verification
accounts. Strict full Create consumption is preserved from generated
`create.dart:61–67`, which already rejects trailing bytes. Rust adds explicit
caller budgets, unresolved-lookup rejection and complete signatures after caller
attachment. A conflicting supplied signature cannot replace any occupied caller
slot, including duplicate signer identities; equal bytes are verified again.
The seal authenticates the documented policy, not complete instruction intent,
PDA account state, validator acceptance, freshness or economic safety.

Authored tests use independent ordered JSON, Borsh payload and all three envelope
layouts without the Kit message compiler or Metadata encoder. They cover each
policy invariant, malformed/trailing payloads, stale signatures, byte/item limits
and immutable signing/attachment. Additional occupied-slot controls cover
conflicting bytes, identical complete wire preservation and filling an empty
duplicate identity slot. Cryptographic fixtures use deterministic test
keys and existing Kit Ed25519. A separately executed Dart signature oracle has
not yet been produced for these new messages.

Root verification unit 840 executed the frozen `cff45f7` producer: all 22 native
test bodies, one Rustdoc example, strict Clippy and documentation checks passed.
That historical result does not qualify the subsequent shared-source refactor.

## Shared conformance

The native test entry points include private assertion source modules, retaining
the same 22 bodies and assertions. `verification/conformance/src/publisher.rs`
includes those exact modules with an unconditional `crypto` dependency and calls
every body in five groups appended after the verified parent's 427-case prefix:

| Group                                                    | Complete bodies | Contracts                                                                                 |
| -------------------------------------------------------- | --------------: | ----------------------------------------------------------------------------------------- |
| `publisher-attestation-exact-json-and-verified-base64`   |               4 | Ordered JSON, request IDs, signature prefix and signature rejection                       |
| `publisher-prepared-three-envelope-bytes-and-signatures` |               3 | Legacy/v0/v1 bytes, existing signatures, completeness, unsigned slots                     |
| `publisher-prepared-policy-invariants-and-rejections`    |               9 | Common expectations, allowlist, collections, account presence, malformed data and lookups |
| `publisher-occupied-signature-preservation`              |               3 | Conflicting occupied bytes, identical complete wire and duplicate empty slots             |
| `publisher-shared-caller-budgets`                        |               3 | Wire/tree reservation, nested metadata claims and signed attestation output               |

Only native test shims select optional crypto; the shared cryptographic bodies
cannot silently disappear through conformance feature mismatches. These bodies
return `Result<(), &'static str>` for their original behavioral conditions.
Fixture construction, SDK failures and mismatched conditions propagate to the
corresponding `CaseResult`; a failed group does not suppress later independent
groups. Native test entry points use Rust's Result-returning test contract.
No unwind catcher, runtime, network or program implementation is duplicated.

Historical verification unit 891 executed the shared-source refactor as
`solana-kit-core-v51` with 432 groups: all 22 native bodies, one Rustdoc example,
strict Clippy and documentation checks passed. The complete suite passed all
432 groups; deliberate corruption passed 431 groups and exited one. Checked-in
golden reports at that checkpoint were copied from those executions. Those
identifiers describe that historical execution, not the current integrated suite
or this subsequent failure-reporting repair.

Two authored conformance unit controls deliberately change the independent
expected attestation literal and require the real shared assertion to return its
specific failure. They then execute later real envelope/policy/occupied/budget
groups and inspect their retained results. The existing workspace CI test command
includes these controls. This source-only repair has not executed the controls,
native bodies or portable hosts; its runtime and golden qualification remain
pending Root/CI verification.

Unit 887 resolved the root and independent wasmi caller graphs through Cargo.
The genuine inventory tool regenerated and checked the canonical ledgers from
the clean pinned Dart checkout. Unit 886 verified exactly 573 owner-only changes;
all declaration IDs, pending states and other fields remain unchanged.

Portable-host execution and new cross-language oracles remain unexecuted.
Source formatting and per-file Monostyle checks establish no behavioral result.

The next portal-only proposal is documented in [Publisher portal protocol](dapp-publisher-portal.md).
It adds raw wire contracts and explicit retry/upload boundaries without claiming
workflow orchestration. Resumable sessions/checkpoints, normalized translations,
metadata/media, funding preflight and native CLI remain separate work. Existing
canonical declaration IDs remain pending.
