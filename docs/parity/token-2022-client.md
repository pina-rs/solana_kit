# Token-2022 deterministic client scope

The public crate consumes the exact shared generated data codecs for all 100
instruction variants and all three account layouts. A separate source-pinned
projection emits typed account inputs, builders, strict invocation parsers and
100 independent original Dart builder literals. No shared IR/emitter file is
changed by the public client unit.

Every canonical declaration remains in `token-2022.tsv`, with its original 14
columns unchanged and a concrete Rust owner, bounded disposition, evidence target
and residual. These dispositions are not full ecosystem or device parity.

The original Dart instruction/data decoder does not enforce account semantics or
program identity. The Rust invocation parser enforces both and preserves the exact
account meta list. Two original fee-withdrawal oracle rows specify nonzero source
counts while supplying zero source accounts; data bytes remain valid structural
frames while strict invocation parsing rejects those inconsistent accounts.

Canonical extension framing includes u16 tags and u16 payload lengths. The pinned
Dart extension generator omitted those lengths; comparisons preserve 400 complete
instruction frames, 12 complete base frames and 116 separate bare extension
payloads. Independent canonical literals establish framing. Named structural
codecs and canonical initialized-account wrappers remain distinct.

The pinned upstream extension module assigns each extension to its base account,
reserves 355 bytes for Multisig, and defines invalid mint-extension combinations.
Public canonical wrappers apply those contracts. They do not prove every
cross-account program invariant or validator transition.

The original package and IDL define no PDA recipes. Confidential proof generation,
host execution, validator integration and installed application journeys remain
outside this deterministic client. Optional fetching uses the existing typed Kit
client with caller-owned transport, executor and read configuration.

The fifteen mint pre-initialization branches preserve caller extension order.
Independent original Dart literals cover their instruction bytes, with explicit
program and writable mint assertions, truncated frames, invalid tags, invalid
account roles, absent required authority and byte/item limits. Unmatched extension
variants retain the source helper's empty result.

The invocation projector validates and renders every contract before replacing
output. Malformed IDL and unsupported account policy preserve existing output.
Each final file replacement is atomic; replacing the complete generated file set
is not transactional if a later filesystem rename fails.

This SDK is exclusively for off-chain application clients. On-chain programs use
Pina or official Solana program libraries. Component publishing remains disabled.
