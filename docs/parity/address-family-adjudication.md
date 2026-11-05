# Address-family adjudication

This note records the bounded RW-01 review of the 49 canonical declarations in
`solana_kit_address_constants` and the ten `solana_kit_addresses` curve and
program-derived-address declarations. It does not complete either package,
Phase 2, or the SDK.

## Scope and identity

The manifest is keyed by complete canonical ID and the defining source hash from
[`source-files.tsv`](source-files.tsv). The review spans seven pinned files:

| Source                                                                    | SHA256                                                             |
| ------------------------------------------------------------------------- | ------------------------------------------------------------------ |
| `packages/solana_kit_address_constants/lib/src/metaplex_addresses.dart`   | `7974b89c6c9d6fef94af97f1c32d309ea60fcbc74ed66542b0b409db0793f7d2` |
| `packages/solana_kit_address_constants/lib/src/program_addresses.dart`    | `240759e33e7f5a8914c52b92e10d6a385e6e804d9a648da7036bc15bfb71864c` |
| `packages/solana_kit_address_constants/lib/src/spl_addresses.dart`        | `fc2d6d3b3c88ff54065443242e672dbfacff865c8d982cd246992f6c863b913b` |
| `packages/solana_kit_address_constants/lib/src/sysvar_addresses.dart`     | `904d22be8ab5b5e6fe97b288b2322741acedbfd88d543464b4d629a4da51864f` |
| `packages/solana_kit_address_constants/lib/src/well_known_addresses.dart` | `b597789c975d4d390af813a95d0374e07e7c79bcc6f3a816d4c2fbbe4700956a` |
| `packages/solana_kit_addresses/lib/src/curve.dart`                        | `3f688a5001d30847ab47e1b9f6df14e86f964a8b0e598c6dc7cd75847223b567` |
| `packages/solana_kit_addresses/lib/src/program_derived_address.dart`      | `1d689cdef09e23895ac8a8768891909dfe89cbba6557bd232372a9bc22d95806` |

The per-ID [`address-family.tsv`](address-family.tsv) lists all 59 IDs with the
public Rust operation, executable cases, status and concrete residual.

Regeneration leaves `id`, `package`, `path`, `symbol`, `kind`, `line`, `owner`,
`phase`, `unit`, `sha256` and `provenance` byte-identical for all 17,948 rows.
Only `status`, `evidence` and `notes` on the 59 target rows differ. The 462
earlier manifest claims, `source-files.tsv`, `delivery-units.md` and the package
ledger are unchanged.

## Canonical literals

All 49 declarations map to a public Rust constant. Forty-eight are complete for
the declared value. The `nativeLoaderProgramAddress` declaration is partial
because its pinned literal is not a 32-byte address; the mismatch is recorded
below rather than silently reproducing or hiding it. The executable case
`canonical-address-constant-literals` and `address-sysvar-constant-literals`
compare each public `Address` constant to
its independently transcribed base58 text, in both directions: the text parses
to the exact constant and the constant displays as the exact text. The first 36
pairs are preserved byte-for-byte; the 13 sysvar aliases were appended to the
same table. The aliases are checked individually through
`address_constants::SYSVAR_*`, not inferred from `sysvars::*`.

| Pinned declaration                       | Rust constant                        |
| ---------------------------------------- | ------------------------------------ |
| `tokenMetadataProgramAddress`            | `TOKEN_METADATA_PROGRAM`             |
| `mplBubblegumProgramAddress`             | `MPL_BUBBLEGUM_PROGRAM`              |
| `mplTokenAuthRulesProgramAddress`        | `MPL_TOKEN_AUTH_RULES_PROGRAM`       |
| `mplCoreProgramAddress`                  | `MPL_CORE_PROGRAM`                   |
| `splAccountCompressionProgramAddress`    | `SPL_ACCOUNT_COMPRESSION_PROGRAM`    |
| `noopProgramAddress`                     | `NOOP_PROGRAM`                       |
| `systemProgramAddress`                   | `SYSTEM_PROGRAM`                     |
| `addressLookupTableProgramAddress`       | `ADDRESS_LOOKUP_TABLE_PROGRAM`       |
| `bpfLoaderProgramAddress`                | `BPF_LOADER_PROGRAM`                 |
| `bpfLoaderDeprecatedProgramAddress`      | `BPF_LOADER_DEPRECATED_PROGRAM`      |
| `bpfLoaderUpgradeableProgramAddress`     | `BPF_LOADER_UPGRADEABLE_PROGRAM`     |
| `computeBudgetProgramAddress`            | `COMPUTE_BUDGET_PROGRAM`             |
| `configProgramAddress`                   | `CONFIG_PROGRAM`                     |
| `ed25519ProgramAddress`                  | `ED25519_PROGRAM`                    |
| `featureProgramAddress`                  | `FEATURE_PROGRAM`                    |
| `incineratorAddress`                     | `INCINERATOR`                        |
| `loaderV4ProgramAddress`                 | `LOADER_V4_PROGRAM`                  |
| `nativeLoaderProgramAddress`             | `NATIVE_LOADER_PROGRAM`              |
| `secp256k1ProgramAddress`                | `SECP256K1_PROGRAM`                  |
| `secp256r1ProgramAddress`                | `SECP256R1_PROGRAM`                  |
| `stakeProgramAddress`                    | `STAKE_PROGRAM`                      |
| `stakeConfigAddress`                     | `STAKE_CONFIG`                       |
| `voteProgramAddress`                     | `VOTE_PROGRAM`                       |
| `solanaAttestationServiceProgramAddress` | `SOLANA_ATTESTATION_SERVICE_PROGRAM` |
| `subscriptionsProgramAddress`            | `SUBSCRIPTIONS_PROGRAM`              |
| `zkTokenProofProgramAddress`             | `ZK_TOKEN_PROOF_PROGRAM`             |
| `zkElgamalProofProgramAddress`           | `ZK_ELGAMAL_PROOF_PROGRAM`           |
| `tokenProgramAddress`                    | `TOKEN_PROGRAM`                      |
| `token2022ProgramAddress`                | `TOKEN2022_PROGRAM`                  |
| `associatedTokenProgramAddress`          | `ASSOCIATED_TOKEN_PROGRAM`           |
| `memoProgramAddress`                     | `MEMO_PROGRAM`                       |
| `memoLegacyProgramAddress`               | `MEMO_LEGACY_PROGRAM`                |
| `memoLegacyProgramAddressV3`             | `MEMO_LEGACY_PROGRAM_ADDRESS_V3`     |
| `sysvarOwnerAddress`                     | `SYSVAR_OWNER`                       |
| `sysvarClockAddress`                     | `SYSVAR_CLOCK`                       |
| `sysvarEpochRewardsAddress`              | `SYSVAR_EPOCH_REWARDS`               |
| `sysvarEpochScheduleAddress`             | `SYSVAR_EPOCH_SCHEDULE`              |
| `sysvarFeesAddress`                      | `SYSVAR_FEES`                        |
| `sysvarInstructionsAddress`              | `SYSVAR_INSTRUCTIONS`                |
| `sysvarLastRestartSlotAddress`           | `SYSVAR_LAST_RESTART_SLOT`           |
| `sysvarRecentBlockhashesAddress`         | `SYSVAR_RECENT_BLOCKHASHES`          |
| `sysvarRentAddress`                      | `SYSVAR_RENT`                        |
| `sysvarRewardsAddress`                   | `SYSVAR_REWARDS`                     |
| `sysvarSlotHashesAddress`                | `SYSVAR_SLOT_HASHES`                 |
| `sysvarSlotHistoryAddress`               | `SYSVAR_SLOT_HISTORY`                |
| `sysvarStakeHistoryAddress`              | `SYSVAR_STAKE_HISTORY`               |
| `wrappedSolMintAddress`                  | `WRAPPED_SOL_MINT`                   |
| `usdcMintAddress`                        | `USDC_MINT`                          |
| `usdtMintAddress`                        | `USDT_MINT`                          |

The negative control `address-constant-rejects-wrong-literal` changes the first
character of the pinned `SYSVAR_OWNER` literal and proves the same comparison
rejects it before the curve evidence case runs. The converted Dart `Address`
String extension type is substituted by the typed `Address` newtype; the value,
canonical display and 32 bytes are exact.

One source literal is defective. `nativeLoaderProgramAddress` pins the
40-character `NativeLoader1111111111111111111111111111`, which decodes to 30
bytes and is therefore not a representable `Address`. The canonical Solana
Native Loader address exposed by `solana-sdk-ids` is the 43-character
`NativeLoader1111111111111111111111111111111`, which is what
`NATIVE_LOADER_PROGRAM` keeps. The appended
`native-loader-source-literal-invalid` case executes the public parser and
asserts `WrongLength { expected: 32, actual: 30 }`, then checks the canonical
constant. The row is partial: reproducing the source value would require an
invalid address, and the source intent is satisfied by the canonical literal.

## Curve predicates

`isOnCurveAddress` maps to `crypto::is_on_curve` over a validated `Address`.
The appended `curve-compressed-point-classification` executes seven fixed
compressed Edwards encodings: the RFC 8032 basepoint, identity, `y = 0`,
`y = p - 1`, `y = 2` and the two runtime zero-x sign-bit encodings. The
`curve-classification-rejects-wrong-expected` control flips one expected
classification and proves the comparison fails.

The two zero-x encodings with the sign bit set are a genuine source residual.
The Dart BigInt port rejects them because `x == 0` with an odd sign bit is
invalid. `curve25519-dalek` 4.1.3, and therefore the Solana runtime and Agave
`Pubkey::is_on_curve`, accepts them. The Rust predicate was not changed: doing
so would diverge from on-chain PDA derivation for those encodings. The rows stay
partial instead of claiming a source equivalence that is not true.

`compressedPointBytesAreOnCurve` additionally accepts arbitrary-length
`Uint8List` input and returns `false` for non-32-byte input. Rust can only call
the predicate after constructing an `Address`, so that branch is unrepresentable
and the row is partial. `isOffCurveAddress` is the exact `!is_on_curve`
negation on the same domain and shares the sign-bit residual. The two assert
helpers have no Rust assert wrapper, SolanaError code or throw/context contract;
the predicate evidence does not promote them.

## Program-derived addresses

`getProgramDerivedAddress` maps to `crypto::find_program_address`. The executed
cases prove the same 255-down-to-0 search, the reserved bump slot (15 input
seeds), the 16-seed direct-creation limit, 32-byte seed limits including UTF-8
byte counts, the `seeds || program || "ProgramDerivedAddress"` hash order, and
on-curve rejection at the exact pinned bumps.

`createAddressWithSeed` maps to `crypto::create_address_with_seed`. The cases
prove `base || seed || owner` hashing, the 32-byte seed limit, no curve check,
and rejection when the owner ends with the PDA marker.

Both remain partial because Dart accepts `Object` seeds (`String` or
`Uint8List`, otherwise `ArgumentError`) and reports `SolanaError` codes and
context, while Rust accepts typed byte slices and returns `PdaError` variants.
`ProgramDerivedAddress` is a private-field struct rather than an unvalidated
`(Address, int)` record; `isProgramDerivedAddress` and
`assertIsProgramDerivedAddress` have no runtime `Object?` predicate or error
codes because the type cannot represent an invalid pair. The curve sign-bit
residual also applies in the astronomically unlikely case that a candidate hash
uses one of those encodings.

## Executed evidence

Five groups are appended after the preserved 476-case prefix:

| Case                                          | Body                                                |
| --------------------------------------------- | --------------------------------------------------- |
| `address-constant-rejects-wrong-literal`      | `core_parity/constants.rs::rejects_mutated_literal` |
| `curve-classification-rejects-wrong-expected` | `core_parity/curve.rs::rejects_wrong_expected`      |
| `curve-compressed-point-classification`       | `core_parity/curve.rs::classification`              |
| `native-loader-source-literal-invalid`        | `address_family::native_loader_source_literal`      |
| `address-sysvar-constant-literals`            | `core_parity/constants.rs::sysvar_literals`         |

Every mismatch returns a `CaseResult` error with a stable explanation; no guest
panic is added, and later cases still execute. The maintained
`verification/conformance/evidence/native-success.txt` is `solana-kit-core-v60`
with 481/481 cases. The corruption report is 480/481 with only
`sha256-fips-abc` failing.

The five appended groups perform ordinary deterministic work. The fuel
accounting therefore changes only by five ordinary cases at the existing
15,000,000 rate: 15,505,000,000 becomes 15,580,000,000. No crypto-unit rate,
`CRYPTO_WORK` category, negative control or one-fuel trap changed.

## Status and totals

The canonical ledger now has 450 complete, 72 partial and 17,426 pending rows
across 17,948 declarations. Phase 2 is 450 complete, 72 partial and 865
pending. Every package disposition remains `inventoried`; the 48 complete
literal rows do not claim the Dart package as a whole, and the one defective
source literal plus the ten helper rows do not claim the package is fully
substituted.

## Reproduction

```sh
cargo test --locked -p solana_kit_core --all-features --offline --test core_parity
cargo test --locked -p solana_kit_conformance --offline --lib
cargo run --locked --manifest-path tools/capability_inventory/Cargo.toml --offline -- \
  check <clean-pin-checkout> .
cargo run --locked --manifest-path tools/capability_inventory/Cargo.toml --offline -- \
  controls <clean-pin-checkout> .
```

The checked-in reports are the native execution. The
[runtime receipt](../verification/address-family-runtime.md) records the same
481-case suite in Chrome, both Wasmtime 50 previews and the pinned LeptosKit
host. Those runs are not separately checked-in case reports, so this note
claims only the receipt.

| Artifact                           | SHA256                                                             |
| ---------------------------------- | ------------------------------------------------------------------ |
| `native-success.txt`               | `147043fd82c71a505461c387fc76d336e99acd12607e7bcda94276005bda21d3` |
| `native-corruption.txt`            | `8f8e16344e4df1a27f18af4cce1077470fe6bf5499ac509e88ed8b4d9abb6ec6` |
| `parity-completion-prefix-v55.txt` | `6602dd63618190a1fdb81dd25ca2640417fe2b681a1d9cacc1ff691f60bc8dcb` |
