# Subscriptions program lifecycle current receipt

Date: 2026-11-04\
Package: `verification/program-integration` (`solana_kit_program_integration`, private, own
workspace and lock)\
Node: `solana-test-validator 4.2.0 (src:ac82b5d4; feat:21b0d33a, client:Agave)`\
Fixture source: Dart revision `0138bb956d2d81ccc0c56edc602401c0df2f2b46`, `config/programs`

## Scope and result

This receipt adds the RW-09 Subscriptions lifecycle journey to the existing owned-validator suite:
real `Subscribe`, `CancelSubscription` and `ResumeSubscription` transactions against the pinned
`subscriptions-v0.5.0.so`, independent typed account and SPL Token reads, exact token-delegation
assertions, decoded events from confirmed transaction inner instructions, and four confirmed
rejection transitions with byte-identical unchanged state.

The executed run, from the repository root:

```text
running 18 tests
test result: ok. 18 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 90.56s
```

Two existing harness unit regressions also passed (`2 passed`, 0.16 s). The previous 17 program
journeys are unchanged; the new case is
`subscriptions_lifecycle::executes_subscribe_cancel_resume_with_exact_state_and_events`.

```sh
CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=2 \
SOLANA_KIT_PROGRAM_FIXTURES=<pinned>/config/programs \
SOLANA_TEST_VALIDATOR=$HOME/.local/share/solana/install/active_release/bin/solana-test-validator \
cargo +1.99.1 test --locked --offline \
  --manifest-path verification/program-integration/Cargo.toml -- --test-threads=1
```

No SDK production crate, root manifest, root lockfile, workflow, fixture, canonical ledger, shared
conformance or other program case was changed.

## Source contract at the fixture pin

The pinned artifact is the `subscriptions-v0.5.0.so` recorded in the fixture manifest; the executed
behavior follows the pinned program source:

- `program/src/instructions/initialize_subscription_authority.rs` creates the authority PDA and
  approves it as the user ATA's SPL Token delegate with `u64::MAX`.
- `program/src/instructions/subscribe.rs` validates the plan PDA, live plan status, expiry and
  approved terms; validates the authority's user, mint and `init_id`; derives the subscription PDA
  from `["subscription", plan, subscriber]`; creates the 155-byte account with a 107-byte header,
  snapshotted terms, `amount_pulled_in_period = 0`, `current_period_start_ts = Clock::unix_timestamp`
  and `expires_at_ts = 0`; then emits `SubscriptionCreated` through a self-CPI.
- `program/src/instructions/cancel_subscription.rs` rejects a non-subscriber with `Unauthorized`
  (130) and a second cancellation with `SubscriptionAlreadyCancelled` (509); sets `expires_at_ts`
  to the end of the current period (period start plus 86,400 s for a 24-hour plan with `end_ts = 0`);
  then emits `SubscriptionCancelled`.
- `program/src/instructions/resume_subscription.rs` rejects an active subscription with
  `SubscriptionNotCancelled` (510), a wrong approval echo with `StaleSubscriptionApproval` (521),
  and only then clears `expires_at_ts` while preserving period accounting; then emits
  `SubscriptionResumed`.
- `program/src/event_engine.rs` emits events by invoking the program's own `EmitEvent`
  instruction, so the wire payload is inner instruction data, not a `Program data:` log line.

The generated Rust client's source-literal checks are asserted too: instruction discriminators
11/12/13, subscribe data length 74, resume data length 9, account sizes 491/106/155,
discriminator 4, account version 1 and plan status `Active = 1`.

## Executed lifecycle

Fresh funded parties, a fresh 9-decimal mint and a fresh canonical ATA were created. The merchant
is a separate signer from the subscriber; the subscriber, merchant and a deliberately different
funded signer are all distinct addresses.

| Object           | Observed address                                           |
| ---------------- | ---------------------------------------------------------- |
| Subscriber       | `9Z4HgYnYd3bMxrJ2ufNL9mWDtjKhUiqp3UL6ARZFLTWK`             |
| Merchant         | `BsiEk5kQFTbzdvFA3CVw2W8fJLvogkv1YUqDBAQkwzGN`             |
| Different signer | `HmCazJfcXTS1pVRwxEYwDQuMWJjeSHq1vf11tbMWZY3H`             |
| Mint             | `7B2egtCTpdghwtEFDWEQx3ukANupMGH73XrGCjnNiss3`             |
| Subscriber ATA   | `F8qYdcoMPcJnxDEu4JBVoiZMXoYzAbaEGAB451vNeyDp`             |
| Authority PDA    | `2szi3iWJLEH2aJptjj7uKciHYjVtHqP4VKWFwqmuqqiM`             |
| Plan PDA         | `3CaduDmDvCyWWj2KhEGyCcykoRhdScCtXuAEGCxfQXko`             |
| Subscription PDA | `8rZ7ADeTEPCpwsrcJBU4qTB9MNEx8hPLNUSu1wJ9Wr1f`             |
| Event authority  | `3Hnj4BYoDgtpBuqXfiy7Y8cNa3jXaNd4oqgSXBzkMcH7` (canonical) |

| Step                       | Confirmed signature                                                                        | Independent observation                                                                                                                                                                                                                                  |
| -------------------------- | ------------------------------------------------------------------------------------------ | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Mint to the subscriber ATA | `2TduLdytSLDGe3sQzqNoZhf15Ndprb4hFLrnSQ5vvXWVswqsAQmUaSGyVgP8XwrLfqgcpbXNvyNWd5ER2K7Vx2yM` | binary `Mint` decode: supply 1,000,000, decimals 9                                                                                                                                                                                                       |
| Authority                  | `5bFWJvqypbLwKvdZdbarvYeCcFNfK1Y1uDnwR5trQaccseUymTRVBVh7kmRZCrrd2YJngCpPL6DkD4sAqQ5bsE9n` | 106-byte authority; user = payer = subscriber; mint equal; bump 250; `init_id = 7`                                                                                                                                                                       |
| Plan                       | `3L9cBMSvMn4cLCA9JK12vQ6uPACQaAvDhqS9A6NaF36tbGpeXTBzjh57hLWLTjEpfZJLVWiEJqL9uM6yyqoxsnD1` | 491-byte plan; owner = merchant; bump 254; status 1; amount 42; 24 h; `created_at = 1793792701`; `end_ts = 0`                                                                                                                                            |
| Subscribe                  | `3AQ1McoGB7HopSoFJyQfNyv63rpekZVXWdTbd5xM7pw2G2vQSSJjDC3fztxz3Yk7z8S4T6zJWgZcweRWE6DUivhk` | 155-byte delegation; discriminator 4; version 1; bump 251; delegator = subscriber; delegatee = plan; payer = subscriber; `init_id = 7`; terms equal the plan; `amount_pulled_in_period = 0`; `current_period_start_ts = 1793792702`; `expires_at_ts = 0` |
| Cancel                     | `5uHeSgG2FxHQ45rKRU9NZpNEQYhwSoK6LXiJkvroF6Hqxn4Asaa9Z4pSTkGocprDPmARHxXGGMf1WLsK8EpVnm5B` | `expires_at_ts = 1793879102` = `1793792702 + 86,400`; period start and pulled amount unchanged                                                                                                                                                           |
| Resume                     | `5ZNhbDKkNT7WqKjLzMUYFSnCxUxTRyrC9kwT8mvuyHtLbRa2ipDa4ShomR5Hk58wmKFKq9Gh5Ek2KTDhmYYtW7wU` | `expires_at_ts = 0`; period start `1793792702` and pulled amount 0 preserved; terms equal the plan                                                                                                                                                       |

The subscriber ATA was decoded before and after the lifecycle through the binary `TokenCodec` and
through the node's `jsonParsed` projection. Both agreed at each of those checks:

```text
amount = 1000000
owner = 9Z4HgYnYd3bMxrJ2ufNL9mWDtjKhUiqp3UL6ARZFLTWK
mint = 7B2egtCTpdghwtEFDWEQx3ukANupMGH73XrGCjnNiss3
delegate = 2szi3iWJLEH2aJptjj7uKciHYjVtHqP4VKWFwqmuqqiM
delegated_amount = 18446744073709551615 (u64::MAX)
state = initialized
```

The ATA decode before and after the lifecycle proves the exact unchanged balance and `u64::MAX`
delegate across subscribe, cancel and resume; it is not merely a check that some delegate exists.

## Confirmed rejections

Every rejection is a real confirmed transaction submitted with `skip_preflight: true` and
`max_retries: 0`. The full encoded subscription account (owner, lamports, executable, allocation,
rent epoch and bytes) was captured before and after each failure and compared exactly.

| Journey                               | Expected error                            | Observed                           |
| ------------------------------------- | ----------------------------------------- | ---------------------------------- |
| Resume before cancel                  | custom 510 `SubscriptionNotCancelled`     | custom 510; account byte-identical |
| Cancel signed by the different signer | custom 130 `Unauthorized`                 | custom 130; account byte-identical |
| Cancel a second time                  | custom 509 `SubscriptionAlreadyCancelled` | custom 509; account byte-identical |
| Resume with a stale expiry approval   | custom 521 `StaleSubscriptionApproval`    | custom 521; account byte-identical |

The existing authority/plan case still rejects a duplicate plan with custom 518
(`PlanAlreadyExists`) and decodes `init_id = 4` in that independent run.

## Event evidence

The confirmed transaction response was fetched at `confirmed` commitment. For each lifecycle
transaction, the test resolved the subscriptions program index from the JSON message account keys,
required no loaded addresses, read `meta.innerInstructions`, and selected the single compiled inner
instruction whose program id is `De1egAFMkMWZSN5rYXRj9CAdheBamobVNubTsi9avR44`. Each payload was
then decoded with the generated event codec, which re-checks the 9-byte anchor tag and rejects
trailing bytes.

| Event                   | Payload size | Assertions                                                                                                                                     |
| ----------------------- | ------------ | ---------------------------------------------------------------------------------------------------------------------------------------------- |
| `SubscriptionCreated`   | 145 bytes    | plan, subscriber, mint and payer equal the observed accounts; `created_ts` exactly equals the account's `current_period_start_ts`              |
| `SubscriptionCancelled` | 81 bytes     | plan and subscriber equal the observed accounts; `expires_at_ts` exactly equals the account value `1793879102`                                 |
| `SubscriptionResumed`   | 81 bytes     | plan and subscriber equal the observed accounts; `resumed_ts` is positive, not earlier than subscribe and strictly before the cancelled expiry |

No event is inferred from the client's encoder. The event bytes are the transaction's actual
confirmed inner-call data.

## Terms and delegation shape

Subscription plans in this program snapshot are recurring-period terms (`amount`, `period_hours`,
`created_at`), not fixed one-shot or recurring-delegation account types. The separate
`FixedDelegation` and `RecurringDelegation` account families and their `Create*Delegation`
instructions are not exercised by the subscribe/cancel/resume lifecycle and are not claimed here.
`TransferSubscription` is also not executed in this receipt, so no pull, destination-whitelist or
per-period counter transition is claimed; `amount_pulled_in_period` is asserted only at its exact
zero value across the lifecycle.

## Owned node, ports and cleanup

`verification/program-integration/src/node.rs` still owns one direct Agave 4.2 child with a fresh
ledger, `--reset`, an owned CLI config and explicit mint, and `.env_clear()` with owned
`HOME`/`TMPDIR`/`PATH`. This unit additionally reserves the gossip port with a held UDP socket and
passes `--gossip-port`, so a concurrent validator holding the default UDP 8000 cannot make the
owned child exit before readiness. RPC pair, faucet and gossip are OS-selected numeric ports; all
client RPC traffic is numeric loopback; no ambient CLI config, network program clone or user key
is used.

Pinned inputs and executable, as measured after the run:

```text
b398830321078473ac728435d8c11f667b95d1db342635ab3df892662b351d8d  config/programs/subscriptions-v0.5.0.so
c18d89627acc0f2c6546d890fd9c3bcc731649a23bd449d4cb3f6e381bfabc0f  $HOME/.local/share/solana/install/active_release/bin/solana-test-validator
solana-test-validator 4.2.0 (src:ac82b5d4; feat:21b0d33a, client:Agave)
```

The harness hashes the file before spawn and re-hashes the deployed ProgramData (45-byte header
stripped) after readiness; both equal the recorded `b3988303…1d8d` value. The `--gossip-port`
override was exercised while another worker's validator held the default gossip socket, and the
suite still completed 18/18.

After the suite, no `solana-kit-program-integration-*` directory remained under the platform temp
directory and no validator process belonging to this worktree remained. The only
`solana-test-validator` process visible was the separately owned
`/tmp/solana-kit-rw03-20261104/manual/ledger` worker, which this unit did not start, borrow or stop.

## Known limits of this unit

- No `TransferSubscription`, fixed delegation, recurring delegation, revoke, close,
  `CancelSubscriptionNow`, plan update, plan sunset or plan deletion claim.
- No Pyth, Squads, Bubblegum or other program-family claim; their prior journeys are unchanged.
- This is a local loopback acceptance slice against one pinned binary. It is not a mainnet,
  devnet, release, registry, SDK-wide or Phase completion claim.
- Missing fixture directories, hash changes, a wrong validator version or a missing binary still
  fail loudly; nothing is skipped or suppressed.
