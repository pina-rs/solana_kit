# Program integration verification

Date: 2026-11-04\
Package: `verification/program-integration` (`solana_kit_program_integration`, private, own workspace and lock)\
Node: `solana-test-validator 4.2.0 (src:ac82b5d4; feat:21b0d33a, client:Agave)`\
Fixture source: Dart revision `0138bb956d2d81ccc0c56edc602401c0df2f2b46`, `config/programs`.

## Scope and result

This receipt is the executed acceptance slice for the existing Rust program clients against one owned
local Agave 4.2 validator per test. It does not claim whole-SDK, Phase 4–8, release, registry, browser,
WASI, LeptosKit, device or mainnet completion.

The executed run:

```text
running 18 tests
test result: ok. 18 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 90.56s
```

The new case is the Subscriptions subscribe/cancel/resume lifecycle; its literal addresses,
signatures, state and rejection evidence are in
[the Subscriptions lifecycle receipt](subscriptions-program-current.md). The previous 17 program
journeys are unchanged.

Two existing harness unit regressions for bounded reaping and confirmed read ordering also passed.
Strict all-target Clippy (`-D warnings`), warning-free docs, formatting and the repository style gate
passed locally with Rust 1.99.1 (2,265 files, readability 97.24, complexity 98.53).

Command, from the repository root after extracting the pinned artifacts as described in the
[workspace README](../../verification/program-integration/README.md):

```sh
CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=2 \
SOLANA_KIT_PROGRAM_FIXTURES=<pinned>/config/programs \
SOLANA_TEST_VALIDATOR=$HOME/.local/share/solana/install/active_release/bin/solana-test-validator \
cargo +1.99.1 test --locked --offline \
  --manifest-path verification/program-integration/Cargo.toml -- --test-threads=1
```

No canonical `docs/parity/capabilities.tsv` ID, status, checker or threshold was edited. No production
crate, root manifest, root lockfile, workflow, fixture or other verification directory was changed.

## Owned node, identity and cleanup

`verification/program-integration/src/node.rs` owns one direct child process:

- a fresh directory under the platform temp directory, created with `create_new` semantics and mode
  `0700`; the node never reuses or resumes another ledger;
- `--ledger <owned>/ledger`, `--reset`, `--quiet`, `--bind-address 127.0.0.1`, an OS-reserved numeric
  RPC pair, an OS-reserved faucet port, an OS-reserved gossip port, and a bounded
  `--dynamic-port-range` that avoids those ports;
- an owned CLI config (`--config`) and explicit `--mint`, so neither `~/.config/solana/cli/config.yml`
  nor an ambient identity/mint keypair is consulted;
- `.env_clear()` with owned `HOME`, `TMPDIR`, `PATH` and `NO_DNA`;
- bounded readiness (120 s) that polls `getHealth` and `getVersion` while checking `try_wait()` on the
  exact child, then compares `getIdentity` with the `validator-keypair.json` written into the owned
  ledger;
- final `getIdentity` and `getGenesisHash` challenges before shutdown;
- awaited `SIGTERM`, a bounded wait, `SIGKILL` fallback, `try_wait()` reaping and removal of only the owned
  root. A cleanup failure retains the directory and reports the path.

Parent review removed an unconditional blocking `wait()` after the kill deadline and in the Drop
fallback. A direct-child regression verifies that both bounded observers return while a child is
still alive and report reaping only after explicit termination. Spawn failures also remove their
owned root, and fixture selection now requires an explicit directory.

A fresh payer, mint and every disposable signer come from `NativeEntropy` (`solana_kit_native::keys`).
Transactions use `NativeHttp` + `Client`, the core compiler/signers, and
`solana_kit_client::confirmation::send_and_confirm` with `--test-threads=1`. Submission uses
`skip_preflight: true` and `max_retries: 0`, so a rejected transaction is observed as its real confirmed
status failure and is never retried. All RPC traffic is numeric loopback.

The observer withholds processed statuses, including execution failures, until confirmed or finalized.
It records the transaction slot; subsequent account, balance, slot and blockhash requests send that
slot as `minContextSlot`. This prevents a pre-transaction snapshot from invalidating exact post-state
assertions. The loader close checks the full returned rent and the actual fee debit, rather than only
checking that the payer's balance increased.

The node passes `--deactivate-feature B8JJXCy5amZyWG9r7EnUYLwzXSXTxG7GZ1qZ1qggo83g`
(`disable_sbpf_v0_v1_v2_deployment`) on the owned validator. The pinned `noop` loader ELF is an SBPF v0
image; Agave 4.2 otherwise rejects its **loader deployment** with
`Detected sbpf_version required by the executable which are not enabled`. Genesis `--bpf-program`
loads it without that deployment gate. This is a local verification configuration, not a production
or SDK change; it is required only to exercise the loader deploy/upgrade path.

After the final suite run, `pgrep -fl solana-test-validator` printed no process and no
`solana-kit-program-integration-*` directory remained under the temp directory.

## Pinned external artifacts

`verification/program-integration/src/fixtures.rs` records every complete-file SHA-256 and checks:

1. the file bytes against the recorded hash before spawn;
2. the canonical ID against `artifacts.json` for artifacts listed there (the local Anchor fixtures are
   not in `artifacts.json`, so their canonical ID is checked against the IDL in the Anchor test);
3. after readiness, that the `--bpf-program` account is executable and owned by the upgradeable loader;
4. that the deployed ProgramData bytes hash to the same fixture hash (45-byte header stripped).

| Artifact                            | Canonical ID                                   | SHA-256                                                            |
| ----------------------------------- | ---------------------------------------------- | ------------------------------------------------------------------ |
| `config-v3.0.0.so`                  | `Config1111111111111111111111111111111111111`  | `78c5f88e528337a33937da050a8d883175ec02ec39e27a9b21ec34f72465a5bd` |
| `subscriptions-v0.5.0.so`           | `De1egAFMkMWZSN5rYXRj9CAdheBamobVNubTsi9avR44` | `b398830321078473ac728435d8c11f667b95d1db342635ab3df892662b351d8d` |
| `mpl_bubblegum-v0.12.0.so`          | `BGUMAp9Gq7iTEuizy4pqaxsTyUCBK68MDfK752saRPUY` | `5b86d4092287e989893ef8bc039a7f02229cd65ac1d1aa96201a1ead99ef51ff` |
| `spl_account_compression-v0.3.3.so` | `cmtDvXumGCrqC1Age74AVPhSRVXJMd8PJS91L8KbNCK`  | `8b1bbb64e086e96fcf4a7db8bebd2db661eaf61d5692e0330517f1a96e75ee1f` |
| `noop-v0.2.0.so`                    | `noopb9bkMVfRPU8AsbpTUg8AQkHtKwMYZiFUjNRtMmV`  | `7e5c4e0bc5eec7f1b0aa273f55f3c86b12c1c1de40e2a3d43f3e9624dba737dc` |
| `mpl_core-v0.2.0.so`                | `CoREENxT6tW1HoK8ypY1SxRMZTcVPm7R94rH4PZNhX7d` | `431b31ed52280f87f78e53369eb8c43f9f687becc14e07cd1f82f11823aae4f6` |
| `mpl_token_metadata-v1.14.0.so`     | `metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s`  | `5603d356486af7f7c4f94cc8f1a57f1028478e0b597f0d08b5598e7e7fdd39a6` |
| `squads_multisig-v2.1.0.so`         | `SQDS4ep65T869zMMBKyuUq6aD6EgTu8psMjkvj52pCf`  | `26e2ba150db168da272b350b01aac45f20d2f3443fa4cebb9a0eb406c49b4769` |
| `anchor_compatibility-v0.1.0.so`    | `Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS` | `11df9f013b8e757ae128a929091b524d7cf3c0f172fd46b1484b640576cacaf8` |
| `anchor_event_imposter-v0.1.0.so`   | `8MyZkLi7NVstEPYwQoSS9VtKAcaRzGNLqxwQX4VtwW1e` | `b201f9eec0ab9084de0ba7eaf89876542948396f30bfb8ebbf9083ff32c404b5` |

`config-v3.0.0.so` is recorded but not re-deployed: the Agave 4.2 genesis already provides the Config
program used by the built-in Config test. The Anchor imposter is hash-recorded but not exercised by
this slice. No artifact is downloaded, rebuilt or copied into the repository.

A negative fixture run with a deliberately modified copy of the Anchor artifact failed before spawn:

```text
Fixture(Hash { artifact: "anchor_compatibility-v0.1.0.so",
  expected: "11df9f013b8e757ae128a929091b524d7cf3c0f172fd46b1484b640576cacaf8",
  actual:   "40dc14cc0ac1b53fc1e8c35bb907cc86042e5900fe461f419dc2e44d1cfa5bb7" })
```

## Executed families

Each row is a real instruction transaction. State assertions use an independent typed account read (and
a `jsonParsed` projection where noted), never the instruction's own expected bytes.

| Family                  | Positive transition observed                                                                                                                                                                                                                       | Rejection observed                                                                                                        |
| ----------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------- |
| System                  | create 64-byte account (owner `111…`); transfer 1 SOL exact; nonce create/init, durable-nonce advance replaces the token, withdraw 1 SOL keeps 1,447,680 rent-exempt                                                                               | `u64::MAX` transfer → custom 1                                                                                            |
| Compute Budget          | limit 100,000; price 1,000 microLamports; 24,655 units consumed; fee 5,100; memo log present                                                                                                                                                       | limit 1 → `ComputationalBudgetExceeded` at instruction 0                                                                  |
| Memo                    | Memo v3 text appears in confirmed logs                                                                                                                                                                                                             | readonly non-signer account → `MissingRequiredSignature`                                                                  |
| SPL Token plans         | `create_mint_plan`; `mint_to_ata_derived_plan` (idempotent ATA, mint 1,000,000); second idempotent run with 0; `transfer_to_ata_derived_plan` (250,000). Decoded supply 1,000,000; balances 750,000 / 250,000; parsed mint and token account agree | over-balance plan transfer → custom 1; non-idempotent duplicate ATA → `IllegalOwner`                                      |
| Token-2022              | TransferFeeConfig + InterestBearingConfig pre-init; 334-byte mint; mint 1,000,000; 100 bps fee → recipient net 99,000 and withheld 1,000                                                                                                           | foreign mint authority → custom 4 (`OwnerMismatch`)                                                                       |
| Address Lookup Table    | create, extend two addresses, deactivate at slot 4; binary and parsed `AddressLookupTable` agree                                                                                                                                                   | foreign authority extend → `IncorrectAuthority`                                                                           |
| Config                  | store one key and 12 bytes; 954 bytes reserved zero padding; decoded keys/data                                                                                                                                                                     | unsigned store → `MissingRequiredSignature`                                                                               |
| Stake                   | 3,762-byte vote fixture; 200-byte stake; delegate 5 SOL; deactivate at epoch 0                                                                                                                                                                     | second deactivate → custom 2 (`AlreadyDeactivated`)                                                                       |
| Loader buffer           | 97-byte Buffer, write 60 bytes, decoded `Buffer { authority: Some(payer) }`, close returns 1,566,000 lamports                                                                                                                                      | foreign authority write → `IncorrectAuthority`                                                                            |
| Loader deploy/upgrade   | `noop` ELF (18,432 bytes) deployed through loader v3: Program + ProgramData, executable, invoked; upgraded with the same pinned bytes and invoked again                                                                                            | n/a in this path; see Anchor ID guard                                                                                     |
| Anchor compatibility    | IDL-encoded initialize (41), increment (46), one `CounterChanged` event each; decoded account fields                                                                                                                                               | foreign authority → 2001 (`ConstraintHasOne`), state unchanged; mismatched deploy ID → 4100 (`DeclaredProgramIdMismatch`) |
| Subscriptions           | authority, plan, 155-byte delegation; subscribe/cancel/resume with exact `u64::MAX` delegate, period-end expiry and decoded events                                                                                                                 | duplicate plan 518; resume-before-cancel 510; wrong signer 130; duplicate cancel 509; stale resume 521                    |
| MPL Core                | create asset with name/URI, transfer to a new owner; decoded `AssetV1` owner changes, metadata retained                                                                                                                                            | foreign transfer authority → 26 (`NoApprovals`)                                                                           |
| Metadata                | create Metadata for an SPL mint (fee 250), update name/URI; decoded `Metadata` fields                                                                                                                                                              | foreign update authority → 7 (`UpdateAuthorityIncorrect`)                                                                 |
| Squads V4               | injected `ProgramConfig` (fee 0), create 1-of-1 multisig, add a second member with reallocation; decoded account members                                                                                                                           | foreign config authority → 6004 (`Unauthorized`)                                                                          |
| Bubblegum + compression | 31,800-byte tree (`maxDepth 14`, buffer 64); `createTree`; `mintV1` leaf 0; transfer leaf 0 to a new owner; burn leaf 0. Tree root changes and `TreeConfig.num_minted` moves 0→1                                                                   | foreign delegate → 6001 (`PublicKeyMismatch`)                                                                             |

Token and ATA positives use the merged public `solana_kit_token` `plans` feature
(`create_mint_plan`, `mint_to_ata_derived_plan`, `transfer_to_ata_derived_plan`, merged in
<https://github.com/pina-rs/solana_kit/pull/152>) against the real program, including the authority-signer
roles. Token-2022 also uses the same plan helpers with an explicit `token_program` override.

## `jsonParsed` evidence

`solana_kit_client/parsed-accounts` is enabled only in this private manifest. The source projection is
merged in <https://github.com/pina-rs/solana_kit/pull/153> (source `22e74168`). The suite fetches real
Agave 4.2 `jsonParsed` snapshots and calls `ParsedAccount::decode_typed(Limits)`:

- SPL Token mint: typed `supply`, `decimals`, `mint_authority` and `is_initialized` equal the binary
  `MintCodec` decode.
- SPL Token account: typed `tokenAmount.amount`, `mint`, `owner` and `state: initialized` equal the
  binary `TokenCodec` decode.
- Durable nonce: typed `authority`, `blockhash` and `feeCalculator.lamportsPerSignature` equal the
  binary `decode_nonce` snapshot.
- Address lookup table: typed addresses, authority, deactivation slot, last extended slot and start
  index equal the binary decoder both before and after deactivation.
- Clock sysvar: parsed slot is non-zero and no newer than a separate `getSlot`, epoch is 0 during this
  test and the Unix timestamp is positive.
- Rent sysvar: Agave 4.2 reports the current shape `lamportsPerByte = 6960` with the legacy
  `lamportsPerByteYear`, `exemptionThreshold` and `burnPercent` explicitly absent; the same validator
  returns 890,880 lamports for a zero-byte rent exemption, which is consistent with that per-byte rate.

## SDK observations from real execution

- The handwritten `LeafAccounts::transfer`/`burn` helpers always append the supplied proof nodes as
  remaining accounts. The pinned Bubblegum 0.12 generated client sends no proof accounts, and the
  deployed program rejects the extra tail with 6001. Passing an explicit empty proof slice matches the
  generated client and completes mint/transfer/burn. The helper was not edited; this is a
  version-specific caller contract to verify when its proof tail is used.
- The Squads `Multisig` account carries reallocated spare tail bytes. `decode_multisig` is intentionally
  strict and reports `TrailingBytes`; after explicit owner validation the test uses
  `MultisigCodec::decode_prefix`, which reads the canonical header without dropping the allocation
  metadata.
- The Token Metadata account tail is not all zero padding. `AllocationPolicy::ZeroPadding` rejects it;
  `AllocationPolicy::PreserveTail` accepts the canonical value and retains the tail, which is the
  intended policy for this program.
- A confirmed signature can be readable a moment before a Confirmed `getAccountInfo` snapshot reflects
  a just-deployed program. The loader test polls the executable flag with a bounded deadline instead of
  treating the first read as final.

## Known limits of this slice

- `noop` is the only ELF deployed through the loader; the Anchor artifact is deliberately deployed at a
  non-canonical ID only to prove the ID guard. No claim is made that an SBPF v0 ELF deploys on an Agave
  validator without the local deployment-feature override.
- No Pyth receiver `.so` exists in the pinned fixture set, so there is no positive Pyth receiver case.
- Subscriptions now covers authority, plan, subscribe, cancel, resume and their three event types
  with exact token delegation and unchanged-state rejections; `TransferSubscription`,
  fixed/recurring delegations, revoke, close, `CancelSubscriptionNow`, plan update, sunset and
  deletion are not exercised here.
- Squads covers program config, multisig creation and member expansion; vault transaction, proposal and
  execution flows are not exercised here.
- Bubblegum covers one V1 tree and the mint/transfer/burn lifecycle; V2, collections, DAS and vouchers
  are not exercised here.
- The external test suite requires the pinned fixture directory. Missing directories, missing files,
  hash changes, malformed manifests, a wrong validator version or a missing validator binary fail the
  test loudly; none of them skip.
- This is a local-loopback acceptance slice. It is not a production network, mainnet, registry or
  release claim and does not prove SDK-wide or Phase completion.
