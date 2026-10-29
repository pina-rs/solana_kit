# Publisher portal protocol

This source-only stack proposal reconciles portal commits
`7e807c506e286b12a641fe90222cdb470415f895` and
`f9b3414961438df24864adaba29374c58aebbfe3` onto the verified upper-layer tree
`b5994c603b19aa6bdc63234293b7ac1c6c69cc55`. That upper layer's CI remains pending
at this source handoff; this is not a merged-main or runtime receipt. The earlier
Bae-based branch and ignored source evidence remain preserved. Production portal
code still matches retained `3bdb3c2793f84ce10835463d57b28f303731d00d`.
It adds the optional `portal` feature
to the same unpublished `solana_kit_dapp_publisher` crate. Publishing remains
disabled. The existing twenty-two policy bodies and their continuation controls
are unchanged. Retained workflow producer `65d66858` and its thirty-two workflow
bodies remain separate; no workflow, metadata/media or CLI implementation is
included here.

## Public and internal interfaces

The publisher feature adds optional existing-version serde, serde_json and
`solana_kit_client` dependencies. `Client<T>` uses the existing
`rest::Transport<PortalService>`, typed request/response records and caller JSON
budgets. REST and Clock interfaces are unchanged between the retained producer's
base and this policy checkpoint. No compatibility shim or host dependency is
added to core/client. `Client::call` is one exchange. Origin, authentication,
TLS, cancellation and receive-time byte caps remain with the caller's adapter.

Relative path segments are `trpc` and one sealed fixed procedure. GET uses the
`input` query pair containing compact JSON; mutations POST JSON. Request data
cannot choose a procedure or authentication namespace. Binary upload has its
own caller-owned `Uploader` trait because REST has no binary body. It receives
no PortalService or credential object, approves its target independently,
denies redirects, bounds its response and performs one PUT.

| Procedure                                      | Method | Pinned Dart anchor                  |
| ---------------------------------------------- | ------ | ----------------------------------- |
| attestation.getBlockData                       | GET    | portal_client.dart:279–292          |
| publication.createUploadTarget                 | POST   | portal_workflow_client.dart:107     |
| publication.createIngestionSession             | POST   | portal_workflow_client.dart:116–126 |
| publication.getIngestionSession                | GET    | portal_workflow_client.dart:134     |
| publication.getPublicationBundle               | GET    | portal_workflow_client.dart:161     |
| publication.getPublicationSession              | GET    | portal_workflow_client.dart:313     |
| publication.prepareReleaseNftTransaction       | POST   | portal_workflow_client.dart:338     |
| publication.submitSignedTransaction            | POST   | portal_workflow_client.dart:352     |
| publication.saveReleaseNftData                 | POST   | portal_workflow_client.dart:369     |
| publication.prepareVerifyCollectionTransaction | POST   | portal_workflow_client.dart:383     |
| publication.markReleaseCollectionAsVerified    | POST   | portal_workflow_client.dart:397     |
| publication.cleanupRelease                     | POST   | portal_workflow_client.dart:408     |
| publication.submitToStore                      | POST   | portal_workflow_client.dart:419     |

Input records preserve omission versus explicit null, false resubmission and
three remote-source variants from workflow_client.dart:6–209 and portal_types.dart.
Local paths cannot be serialized as an ingestion source. Selectors never fall
back to hidden cached identifiers. Prepared transaction text is untrusted data;
decode does not invoke a signer, validation seal, transaction or store submission.

Envelope selection preserves batch `0`, result/data and tagged Left/Right forms
from portal_client.dart:87–195. Mixed outcomes, null active branches, unknown
tags, arrays at object boundaries, duplicate keys and malformed known fields
reject. Every branch preserves absent versus null. Nullable backend DTO fields
still accept null. Budgets admit JSON before typed allocation; the adapter must
bound bytes during reception. HTTP/provider bodies remain bounded evidence
available only through explicit access. Ordinary diagnostics redact untrusted
messages, bodies and URLs.

Backend bundle, ingestion and publication-session DTOs retain raw fields used by
portal_translators.dart. Each optional field distinguishes missing/null/value,
numbers retain exact tokens above 2^53 and wrong known types reject. Unknown
fields are ignored. Required receipts reject missing fields instead of adopting
empty strings or false. Block slots and uploaded byte counts require unsigned
64-bit integers. These are deliberate stricter Rust boundaries.

The source getPublicationBundle wrapper at portal_workflow_client.dart:157–253
also resolves sessions, hydrates/uploads metadata, caches identifiers and infers
source kind. Raw GET here performs none of those operations. Normalized models,
checkpoints, fallback timestamps, media translation and fetchRemoteFile at :456
remain a later unit. Raw DTO presence is not orchestration parity.

The explicit ingestion helper retains at most three attempts and, with base
1500 ms, source waits of 1500/3000 ms from portal_client.dart:239–263. It requires
a nonempty idempotency key and reuses exact input. Caller Clock and absolute
deadline arbitrate before/after polling timer and operation, including Ready
decode completion; equality loses. HTTP 502–504, syntax errors and pinned
transient gateway-message patterns may retry. Auth, duplicate keys, budgets and
invalid known schemas do not. Other operations never retry automatically.
Drop cancels owned futures but cannot undo an accepted remote effect.

## Shared predicate preservation

The same twenty-three retained native functions remain test entry points and
now return `Result<(), &'static str>`. Every original route, equality, rejection,
budget, redaction, timeout and cancellation predicate remains. SDK/helper
failures propagate instead of panicking. Checked request/slice access prevents
a malformed observed result from trapping the guest. The uploader's original
three adapter assertions are recorded for every invocation and checked fallibly
by both original upload bodies; the pending poll also requires exactly one call.
A CI unit control records a wrong URL, body or MIME followed by a valid upload
and requires the earlier mismatch to remain visible. Both native Send-bound
compile contracts remain intact. There is no unwind catcher or feature no-op.

| Appended portable group                          | Retained bodies | Coverage                                                                  |
| ------------------------------------------------ | --------------: | ------------------------------------------------------------------------- |
| publisher-portal-thirteen-exact-routes           |               4 | All thirteen verbs/procedures, JSON/query/body forms and source variants  |
| publisher-portal-envelopes-admission-and-absence |               9 | Seven boundary bodies plus null conflict and legitimate-null controls     |
| publisher-portal-retry-deadline-and-cancellation |               7 | Six retry bodies plus Ready clock-advance deadline arbitration            |
| publisher-portal-independent-binary-upload       |               3 | Independent uploader data, limits, cancellation and Send compile contract |

The portal adapter nests under the existing publisher adapter and imports its
single canonical private checks module. No duplicate file-loading declaration
or second runner is introduced. Each actual group result becomes CaseResult;
one failure leaves subsequent independent groups runnable. A conformance unit
control changes only the expected procedure passed to the real shared route
checker, requires its specific failure, then inspects results from the real
envelope, retry and upload groups. Existing workspace all-features CI includes it.
The four groups register globally at the end of conformance::run, after every
one of the base's 439 IDs, including all seven contextual slot-store cases. They
are not inserted inside publisher::cases. The unchanged global registry source
and all existing case-producing modules preserve the entire base order as a prefix.

The earlier Bae-based proposal declared v52/436 and remains frozen on its old
branch. This stack proposal declares v53/443 by adding exactly four groups after
the upper layer's v52/439 registry. Its 439-row golden reports remain untouched.
These declarations are not execution receipts or claims that current main has
that identity. All new portal tests, compiler checks and
portable runtime checks remain **UNEXECUTED** by this source worker.

## Root execution handoff

Root first resolves the changed root and independent consumer dependency graphs
authentically through Cargo under its resource lease; no lock is hand-edited.
The next behavioral checks after that resolution are:

```sh
cargo +1.99.1 test --locked -p solana_kit_dapp_publisher --all-features
cargo +1.99.1 test --locked -p solana_kit_conformance --all-features
cargo +1.99.1 clippy --locked -p solana_kit_dapp_publisher -p solana_kit_conformance --all-targets --all-features -- -D warnings
```

For golden regeneration, run the actual conformance executable twice into
separate retained receipt files using these exact producer commands:

```sh
cargo +1.99.1 run --quiet -p solana_kit_conformance --locked
cargo +1.99.1 run --quiet -p solana_kit_conformance --locked -- --corrupt-fixture
```

Require canonical exit 0, corrupt exit 1, all retained prefix case names in order,
and the four appended publisher portal IDs. The declared v53 proposal expects
443/443 and 442/443; these are targets, not observed results. Compare the first
439 case IDs against the immutable b5994c60 canonical/corrupt receipts before
promotion, including the seven slot-store groups. Inspect real stdout before
copying it verbatim into
verification/conformance/evidence/native-success.txt and native-corruption.txt.
Do not generate or substitute case text manually. Then `cargo +1.99.1 xtask all`
executes browser/both WASI/production wasmi consumers and their corruption
controls against the integrated suite. Root owns those runs, all receipts and
actual golden promotion. No actual service request, upload, credential read or
publisher/contact metadata disclosure is authorized by this source port.

## Pinned source and licence

The reference remains Dart revision 0138bb956d2d81ccc0c56edc602401c0df2f2b46.
Archive SHA-256:
`8d2578053f9a679f87f9c6cce601f3db4c0049583a632f684d925bc9e785980a`.
Member paths are under packages/solana_kit_dapp_publisher_cli/lib/src/.

| Member                      | SHA-256                                                            |
| --------------------------- | ------------------------------------------------------------------ |
| portal_client.dart          | `1302e8c41cf9bb6941f0ac06dc84f7798c50de741a5df9eaeb9101cbce9ea3f7` |
| portal_workflow_client.dart | `6eae8b420fe0b93a5f3235728dcc4cc40f8b807a5c2a6097a8e4a31b45430abc` |
| workflow_client.dart        | `1725ff3411a921ca83d240d1d2c6f72d0054373bc2760af7b0173aa58fe038e8` |
| portal_types.dart           | `f5f7360247c64c05778097afef93489e88ed200ef2d867155dd47d180e4520bd` |
| portal_translators.dart     | `cf2d748f734d558cd0cd3d9672e9d0ca220ab8ec20e5abdfa7117899de669b29` |

Original MIT attribution remains in MIT-DART.txt and THIRD_PARTY_NOTICES.md.
The retained 0325b429 review remains NOTVERIFIED for the fixed null-branch and
completion-deadline defects. The subsequent 3bdb source verdict is historical;
it does not qualify this shared-test reconciliation or claim runtime execution.
Canonical declaration IDs remain pending and no derived parity verdict is edited.
