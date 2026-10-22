# Account, sysvar and program-error contracts

These deterministic APIs serve off-chain clients. They perform no RPC, use no
host services and add no dependencies. On-chain programs should use Pina or the
official Solana program libraries.

`accounts::Account<T>` owns the address, owner, lamports, executable flag,
reported allocation, optional rent epoch and data. `MaybeAccount<T>` explicitly
represents missing addresses. Encoded snapshots decode only when the owner and
reported data size match; the decoder preserves every metadata field and reports
the account address alongside the original codec failure. Full decoding rejects
trailing bytes. Use a decoder's prefix operation explicitly for framed streams.

`sysvars::SysvarCodec<T>::NEW` implements the shared `Encoder` and `Decoder`
traits for Clock, Rent, EpochSchedule, EpochRewards, LastRestartSlot, SlotHashes,
RecentBlockhashes, StakeHistory and SlotHistory. Fields use the exact pinned LE
widths. History vectors have u64 counts, checked conversion/multiplication and
shared element budgets. SlotHistory requires discriminator 1, 16,384 u64 blocks
and 1,048,576 bits. Signed timestamps and 128-bit reward points remain exact.
Rent preserves all IEEE float bit patterns and reference u8 burn values; this is
a wire codec, not a policy validator. Historical RecentBlockhashes is retained
for snapshots; current transaction fees should come from RPC fee estimation.
Thirteen sysvar address constants include historical Fees and Rewards and the
Instructions address; the pinned package defines no codecs for those three.

`program_error::InstructionFailure::is_program_error` matches a custom code only
at a valid instruction position targeting the requested program. Failure log
parsing preserves the exact source line and program address, recognizes exact
known descriptions, retains unknown descriptions, and explicitly rejects invalid
addresses and overflowing custom codes. It never infers instruction indices or
outer-program attribution from log order.

The pinned source is Dart commit
`0138bb956d2d81ccc0c56edc602401c0df2f2b46`. The Clock byte array is taken from
`packages/solana_kit_sysvars/test/clock_test.dart`; its MIT notice is retained at
`verification/conformance/fixtures/DART-LICENSE`. Other fixtures explicitly
specify the field widths/order in that commit using fixed bytes and boundary
values, independently of Kit encoders. No Dart execution is claimed.

`crates/solana_kit_core/tests/accounts_sysvars.rs` executes independent encoding
and decoding, all truncated prefixes for small layouts, trailing rejection,
nonzero offsets, malformed tags/counts/bit-vector lengths, allocation budgets,
metadata preservation, missing accounts, wrong owners/sizes, custom matching and
lossless unknown logs. `verification/conformance/src/accounts_sysvars.rs` adds
four host-neutral cases. The integrated `solana-kit-core-v4` suite preserves all
35 preceding codec/transaction cases and adds those four cases. The
[runtime receipt](runtimes.md) records actual execution on every core host and
the corrupt-hash control. Earlier v3/35 and v2/31 evidence remains historical.
Target compilation alone is not runtime execution.

The [capability receipt](../parity/accounts-sysvars.tsv) preserves all 229
canonical declarations from the three source packages, including exact IDs,
source digests and provenance. Its [remaining rows](../parity/accounts-sysvars-remaining.tsv)
explicitly retain every unresolved ID. RPC fetch/config/JSON parsing and Dart
hash/display/runtime-assertion adaptations remain open; no package-wide parity
completion is claimed.

Fixture provenance SHA-256:

- Clock test file: `eb55e2b1aefa24debfe2bdf03dfee112979d0a4fc7c26a19de850b0fd2c115bc`.
- Sysvar address source: `904d22be8ab5b5e6fe97b288b2322741acedbfd88d543464b4d629a4da51864f`.

The receipt maps 151 declarations to implemented wire/idiomatic Rust behavior
and retains 78 pending declarations. Matching a source declaration to a Rust
field or enum does not claim Dart object formatting or hash compatibility.
