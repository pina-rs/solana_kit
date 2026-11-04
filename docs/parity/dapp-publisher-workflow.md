# Publisher ingestion and metadata hydration

This source-only reconciliation restores the workflow-only delta from portal
`3bdb3c2793f84ce10835463d57b28f303731d00d` through retained workflow
`65d66858ca157b717b77a7bd60488204041b694d`, onto portal checkpoint
`a7a3b661b490f55f67d25f09ff76661663031865`. The original branch and source evidence
remain preserved. Its obsolete portal layer is not reapplied. The optional
`workflow` feature is the next bounded client slice.
It stops at a hydrated bundle and leaves the existing immutable transaction
policy/seal boundary intact. It is not a complete publication runner. No CLI,
signer callback, transaction submission or store submission is implemented by
this unit. The package remains unpublished at its existing 0.1.0 version;
normal publishing remains disabled. Minor release intent targets this package,
independently of the reserved 0.0.0 umbrella and current SDK version group.

## Ownership and flow

`Workflow` borrows the typed portal `Client`, a separate `Uploader`, an exclusive
caller-owned checkpoint `Storage`, a monotonic `Clock` and explicit `Options`.
It owns no executor, host file API, ambient credentials, entropy source or
background task. Concrete associated futures retain local Rc compatibility;
the SDK imposes no universal Send bound. All losing futures are dropped.

`start` requires a caller-issued nonempty idempotency key and a portal-visible
source. It records the exact input's SHA-256 fingerprint and pending create
intent before the single create request. Acknowledged ingestion identifiers are
stored before polling. Polling makes at most the configured number of GETs,
recognizes Ready/ready and Failed/failed, and waits through the caller Clock.
Release and linked-session identities are persisted even when ingestion fails,
so explicit cleanup can address a known failed release.

`resume` accepts one typed target: local snapshot, publication session or release.
Acknowledged ingestion can resume polling; acknowledged release/session state is
read from the portal. The session selector takes precedence over a release
selector as in the source lookup helper. Session, ingestion and bundle IDs must
agree; conflicting IDs fail before media work. Known explicit backend stages
retain precedence. PreparedForMint uses the pinned ticket/attestation/signature
fallbacks. Unknown stages require review instead of becoming an implicit mint
permission. Raw backend DTOs and timestamps remain available without invented
wall-clock fallbacks.

Attached ready bundles/sessions with a metadata URI avoid redundant reads.
Otherwise the coordinator fetches the raw bundle. Metadata URI precedence is
the acknowledged local cache, session URI, then release.nftMetadataUri. Existing
metadata avoids all media fetches/uploads. A submitted release lacking metadata
fails rather than uploading replacement metadata. `HydratedPublication` exposes
the correlated IDs, resolved stage and metadata URI alongside unchanged raw
bundle/session DTOs. It is no transaction-intent or account-authenticity proof.

## Checkpoints and uncertainty

`Storage` is a minimal bounded load/atomic-save capability. The caller grants one
active workflow owner per key and chooses durability, permissions and recovery.
No CAS framework, actual filesystem adapter or shared global state is added.
Versioned checkpoint JSON rejects duplicate keys, unknown fields, invalid known
records, inconsistent pending upload state and unsupported versions under caller
byte/depth/item/string budgets. Checkpoints contain no API key, presigned PUT
URL, signing key or local path. They are recovery hints, not authenticated policy
seals or authorization to publish metadata.

Every durable public artifact location uses one strict HTTPS boundary: no
userinfo, fragment or query. This applies to existing metadata, upload receipts,
pending public targets and install artifacts embedded in metadata. A target's
public URL must also differ from its presigned PUT URL before checkpointing or
upload. Old checkpoints with query-bearing locations reject before reuse.
Query-free syntax is a conservative subset; it does not prove arbitrary path
bytes contain no secret. The caller must still approve disclosure of each URL.

Pending intent is durably acknowledged before ingestion creation, target
creation, binary PUT or explicit cleanup. Only an acknowledged outcome clears
it. An interrupted/lost response or later receipt-write failure leaves an
inspectable `Error::Uncertain` on resume before another network request. A resume
selector cannot bypass that gate. The caller must reconcile the effect using
its provider/storage authority before replacing recovery state. This unit does
not invent an automatic uncertain-effect reconciliation or replay protocol.

Acknowledged upload receipts bind SHA-256, media type, extension and public URI.
The first caller-issued metadata timestamp is retained. These facts let a later
invocation reuse completed uploads after a subsequent write failure instead of
creating duplicate files. The implementation does not infer success from an
absent response, a presigned target alone or a local timestamp.

`cleanup_failed` is explicitly invoked and sends once. It can abandon a known
failed release, records deleted or preservedSubmitted, and never runs from Drop
or broad failure handling. Deleted checkpoints close resume. PreservedSubmitted
requires an authoritative submitted session on resume; no store request follows.
An uncertain cleanup cannot be repeated automatically. The original failure is
returned before cleanup, so a separate cleanup failure cannot hide it.

One absolute deadline bounds storage, file reads, portal exchanges, decoding,
hashing and uploads, including ready completions. Equality loses to the deadline.
The private deadline primitive is shared with the repaired portal retry helper.
The coordinator may hold three Clock sleep futures during a poll delay; callers
must provision their readiness capacity accordingly. Poll count, media count,
decoded-byte and JSON admission bounds are independent. APK/media source caps
are nonzero and at most 64 MiB. No operation silently retries here; the earlier
explicit protocol ingestion retry helper remains independently available.

## Files and metadata

`prepare_local_apk` accepts a local-only reader token and public file name. One
capped immutable read supplies the SHA-256, length and exact uploaded bytes,
removing the Dart stat/hash/read race. Directory separators cannot become the
public file name. The helper uses one target request and one independent PUT;
it owns no checkpoint or automatic recovery. An interrupted local upload is
caller-owned uncertainty and must be reconciled before explicit repetition.

Hydration adds the fixed GET `fetchRemoteFile` operation with typed required
data/fileName/mimeType fields. No portal credentials enter the uploader request
or its origin namespace. All metadata inputs and media counts are resolved before
the first fetch. Query-free public R2 locations are fetched, decoded, hashed and
inspected but avoid re-upload. Other media, including query-bearing R2 sources,
is rehosted. The typed metadata document
preserves schema_version 0.4.0, creator/share, publisher/release details, media
order, install file, Android fields and the pinned en-US resource keys 1/2/4/5.
Optional fields remain omitted. Translator null-coalescing choices are resolved
before builder nonempty fallbacks, including publisher support/contact precedence.
A selected empty required field fails before I/O rather than being hidden by a
different raw fallback. Numeric metadata fields require unsigned integer
values and retain values above 2^53 without floating-point conversion.

Kit SHA-256, Base16 and Base64 are reused. No handwritten hash, new codec
dependency or decompressor is introduced. Header inspection covers PNG, JPEG
SOF0–3, GIF and WebP VP8X/VP8L/VP8; unsupported, truncated and zero dimensions
reject. This is the source's header-only contract and does not establish image
integrity, MIME authenticity, valid compressed contents or safe pixel allocation.

The only new optional dependency is the already pinned portable `http` crate for
`Uri` parsing, with no HTTP client. Missing URL schemes gain HTTPS; explicit HTTP,
userinfo, fragments, control bytes and backslashes reject. Remote source queries
remain fetch inputs. Presigned PUT queries remain exact and in memory; durable
public artifact locations require an explicit query-free HTTPS URL. R2 reuse
requires an exact recognized host and ordinary HTTPS port, preventing
suffix/port lookalikes. These deliberate
refinements are stricter than Dart's permissive URL/coercion helpers. UTF-16 name
and description bounds retain valid Unicode scalar boundaries rather than
manufacturing an unpaired surrogate at a truncation boundary. The caller supplies
and validates publishing authority for every artifact/contact field; porting an
SDK does not authorize outbound publication of the user's data.

## Pinned source mapping

The reference is immutable Dart revision
`0138bb956d2d81ccc0c56edc602401c0df2f2b46`, archive SHA-256
`8d2578053f9a679f87f9c6cce601f3db4c0049583a632f684d925bc9e785980a`.
The original package MIT notice remains in `MIT-DART.txt`.

| Dart source contract                                             | Rust boundary                                                                             |
| ---------------------------------------------------------------- | ----------------------------------------------------------------------------------------- |
| publication_workflow.dart:86–184, :239–273                       | start/resume through ingestion and hydrated bundle; completion is deferred                |
| publication_workflow.dart:302–398                                | typed selectors, attached bundle/session reuse and ID resolution                          |
| publication_workflow.dart:402–454                                | bounded Ready/Failed polling through caller Clock                                         |
| publication_workflow.dart:479–535, files.dart:163–164, :260–296  | one immutable capped local APK read/hash/upload with typed source result                  |
| publication_workflow.dart:581–605, :185–235                      | explicit single cleanup and preservedSubmitted reconciliation; no automatic catch cleanup |
| portal_workflow_client.dart:157–304, :456–480                    | bundle hydration, cached metadata, typed remote fetch and independent upload              |
| release_metadata.dart:96–342, :403–493                           | closed 0.4.0 document, media resolution and external rehosting                            |
| release_metadata.dart:499–724                                    | R2/MIME/extension fallback and header dimensions                                          |
| portal_translators.dart:12–64, workflow_state.dart:234–267       | stage/receipt precedence and normalized session projection; unknown stages retained       |
| portal_translators.dart:147–337, publication_models.dart:4–417   | normalized bundle, install-file, declared media and signer-authority projection           |
| portal_translators.dart:363–425, publication_models.dart:419–535 | checkpoint/status wire translators and coarse stage mapping                               |
| portal_translators.dart:64–144, publication_models.dart:608–687  | ingestion source/status projection and attached bundle/session records                    |

Audited members under `packages/solana_kit_dapp_publisher_cli/lib/src/`:

| Member                    | SHA-256                                                            |
| ------------------------- | ------------------------------------------------------------------ |
| publication_workflow.dart | `4babf5556ab63c17463e682bbe4b130ca8338fd9de4a76ca61cea5dcb63703c2` |
| workflow_state.dart       | `f8687273ec6ff94ccc3f4e426f68c42886fd872c9ea7b8366749373fe5e9b3d4` |
| release_metadata.dart     | `c85b4572d499ada8f5df6671f61ccf2a6dcf6ac0fc8c58457db2b17d4a1037e2` |
| files.dart                | `14cb1663fc458a482d4da8777fb4e512095c7266aa84ccebe8b12e7513055ce0` |
| publication_models.dart   | `d96e980f4fb2d388d0fb135813398cc4c26ecf071030dd5719d07d81edb0df5b` |

## Completion methods

The workflow now exposes the Dart core's explicit completion steps without
owning signing, transaction submission or store authorization:

- `prepare_release_mint` validates the prepared-mint stage, the bundle's app mint
  and every caller input before requesting an unsigned release transaction.
- `prepare_collection_verification` requires the mint-saved stage and resolves the
  dApp, release mint, collection mint and collection authority from the persisted
  session and bundle before requesting an unsigned verification transaction.
- `submit_signed_transaction` accepts a fully signed caller transaction, records
  `SubmitTransaction` before the call and clears it only after an acknowledged
  response. A failure leaves the effect pending and blocks replay.
- `save_release_nft_data` requires the mint-submitted stage, checks the mint and
  app mint against the session/bundle and requires a true acknowledgment.
- `mark_collection_verified` requires the verification-submitted stage and rejects
  a response for another release.
- `submit_to_store` requires the verified or attested stage and a caller-signed
  attestation; a failure leaves `SubmitStore` pending for reconciliation.

Prepared transactions are still untrusted proposals. No method signs, submits a
transaction automatically, retries, creates an attestation or enables normal
publishing. Callers must re-hydrate after each acknowledged mutation to read the
backend stage.

## Normalized publication projection

`solana_kit_dapp_publisher::publication` is available with the `portal` feature.
It projects `publication_models.dart` and `portal_translators.dart` into typed,
deterministic records without network, filesystem, entropy, signing, registry or
clock access:

- `DappInfo`, `PublisherInfo`/`PublisherKind`, `BundleLegal`,
  `LocalizedReleaseStrings`, `BundleMetadata`, `BundleMedia`/`MediaPurpose`,
  `SignerAuthority`, `InstallFileDetails`, `ReleaseInfo`, `PublicationBundle`.
- `PublicationCheckpoint`, `PublicationSessionStage`,
  `PublicationSessionStatus`, `PublicationSession`, `SessionTimestamps` and
  `SessionMarkers`.
- `PublicationIngestionSession`, `PublicationSource` and `PortalSourceKind`.
- `infer_publication_source_kind`, `map_backend_bundle_to_publication_bundle`,
  `translate_backend_publication_session`,
  `translate_backend_ingestion_session` and its explicit-attachment variant.
- `normalize_publication_checkpoint` and `normalize_publication_status`.

Checkpoint precedence is the pinned source's: explicit known stages win;
`Attested`, `Verified` and `VerificationSubmitted` all map to `Verified`;
`PreparedForMint` maps to `BundleReady`; other stages use ticket, attestation,
verification signature, mint signature, then expected-mint/metadata evidence.
Status is failed first, then completed for `Submitted` or a nonempty ticket, then
running. Blank receipt text is ignored exactly like the source's `firstNonEmpty`;
it is not trimmed into absence. Absent and explicit null stage values default to
`PreparedForMint`; absent/null timestamps remain absent unless the caller supplies
`SessionTimestamps`.

Deliberate deviations are explicit:

- Unknown stage names become `PublicationSessionStage::Unknown` instead of the
  source's `PreparedForMint`. Checkpoint/status fallback still follows the source,
  so callers must review the retained stage rather than treat it as mint readiness.
- Unknown ingestion `sourceKind` values become `PortalSourceKind::Unrecognized`
  or `PublicationSource::Unrecognized` instead of being presented as a portal
  upload. Missing/null kinds still default to `portalUpload`.
- Unknown publisher `type` values become `PublisherKind::Unrecognized` instead of
  `organization`.
- `BundleMetadata.media` is a typed projection of declared dApp media in icon,
  preview, banner, editors'-choice/feature order. The source leaves its untyped
  list empty because media hydration is a later stage; Rust deliberately exposes
  the declared locations without fetching or approving them.
- `InstallFileDetails::url`, `ReleaseInfo::nft_mint_address` and
  `PublicationSession::error` are methods publishing the source's aliases rather
  than duplicate stored fields.
- File names use the literal final URI path segment and are not percent-decoded
  or URL-validated. No URL in this layer is approved for disclosure or fetch.

Diagnostics stay bounded. Normalized models that carry application, contact, URL
or provider fields do not derive `Debug`; publication and ingestion sessions have
manual `Debug` output that omits identifiers and error text. `Provider` error text
remains available only through the explicit `last_error`/`error()` and
`processing_*` fields. `ProgressStep`/`ProgressEvent` emit the source's static
messages, `IngestionProgress` replaces provider detail/stage text with a fixed
running message, and `CleanupOutcome` classifies actions without echoing them.
No raw portal body, presigned URL, APK bytes, credential or application metadata
enters these ordinary diagnostics. The Dart `PublisherCliException` message
wrapper has no new Rust counterpart because `cli::CliError`, `portal::Error` and
`workflow::Error` already own bounded caller-facing messages, and projection is
infallible.

The workflow uses `CleanupOutcome` for explicit cleanup classification. The
native CLI keeps its existing raw-record composition; the projection is a library
read model, not an orchestration or publication enablement path.

## Verification and remaining work

The retained 41 synthetic test bodies exercise start/resume, cached and R2
hydration, complete document fields, exact large numbers, six header layouts,
one immutable local file read, poll bounds, typed invalid metadata/checkpoints,
cancelled reads/exchanges/uploads, pending non-replay, failed durable receipts,
explicit cleanup and storage/upload deadline ties. The ten-byte GIF fixture's
SHA-256 was independently computed with the system hash utility; its base64 and
all JSON/header literals are independently authored, not generated by SDK codecs.
No actual portal request, upload or publication was performed, and the SDK's
SourceReader was not used to read an application artifact.

These include six durable-URL repair bodies rejecting public/PUT aliases and query-bearing targets,
local APK targets, old metadata/receipt/pending checkpoints and backend metadata
locations; they also retain a positive query-free upload control. Assertions
inspect every recorded durable write as well as PUT observations. All original
twenty-six workflow bodies remain; their R2 matrix now rehosts query-bearing
inputs instead of treating them as durable public locations.

All 41 native entry points retain their behavioral conditions and return
`Result<(), &'static str>`. The 170 original assertions become fallible predicates;
fixture construction, missing observations and SDK errors propagate without
unwinding a host. Fake portal/storage/upload guards retain their first violation,
and every owned rig validates it before successful completion or shadowing. A negative SDK
error cannot conceal an earlier failed durable-intent or byte-cap predicate.
No workflow body imposes Send; the earlier portal's two native witnesses remain
explicitly native-only and unchanged.

| Globally appended group                             | Retained bodies |
| --------------------------------------------------- | --------------: |
| publisher-workflow-new-resume-and-metadata          |               6 |
| publisher-workflow-durable-effects-and-cancellation |              11 |
| publisher-workflow-apk-headers-and-admission        |               9 |
| publisher-workflow-public-url-boundaries            |               6 |
| publisher-workflow-completion-stages                |               9 |

The shared adapter imports the same source and its parent's one canonical private
checks module. Its failure-continuation control changes only the independent
metadata URI oracle passed to the actual create/hydrate body, requires its real
failure and inspects the later durable/media/URL group outcomes. A second control
calls the actual portal method without durable intent and requires the fake's
original violation after the transport error. Both controls are real CI tests.

All prior 443 case IDs and order remain an exact prefix. Five workflow groups
append globally after the portal groups, producing the v55/448 identity. The prior
22 policy bodies and 23 portal native bodies are preserved. Historical Root unit
1074 describes a separate producer; it does not qualify this workflow or the
shared deadline extraction. Portal CI records remain attached to their own source
identity.

The repaired shared source now compiles inside the `no_std` conformance crate,
including explicit `std`-item imports for the host-side test bodies. The native
suite was executed on 2026-11-01:

```sh
cargo +1.99.1 test --locked -p solana_kit_dapp_publisher --all-features
cargo +1.99.1 test --locked -p solana_kit_conformance --all-features
cargo +1.99.1 clippy --locked -p solana_kit_dapp_publisher --all-targets --all-features -- -D warnings
cargo +1.99.1 run --locked -p solana_kit_conformance
cargo +1.99.1 run --locked -p solana_kit_conformance -- --corrupt-fixture
```

All 41 workflow bodies, the policy and portal suites, and the Rustdoc example
passed. The canonical run reported 448/448 with exit 0; the SHA-256-corrupt run
reported 447/448 with exit 1. Both exact outputs are checked in as
`verification/conformance/evidence/native-success.txt` and
`native-corruption.txt`. Strict Clippy passed for the package.

Browser, both WASI targets, production wasmi, independent consumer locks and
three-OS CI remain unexecuted for this producer and are delegated to pull-request
CI. No canonical declaration ID or pending verdict changes.

The completion methods add nine shared test bodies covering exact
prepared request/procedure selection, stage rejection, publisher mismatch, acknowledged submission,
failed-submission uncertainty, negative save acknowledgement, mismatched
verification and store-submission stage/flag behavior. They are part of
the appended `publisher-workflow-completion-stages` group, so every portable
conformance host executes them.

The normalized projection adds 32 native integration bodies covering the full
and empty bundle fixtures, display/contact/support precedence, exact numbers,
media order, install/release fallbacks, checkpoint stage and receipt precedence,
absent/null/blank/unknown values, caller-supplied timestamps, source inference,
attached ingestion records, metadata URI precedence, progress messages,
cleanup outcomes and diagnostic redaction. They live in
`crates/solana_kit_dapp_publisher/tests/publication.rs`, use independently
authored JSON and string literals, and are not registered in the shared
conformance suite. The checked-in v55/448 shared identity and all canonical
declaration IDs/statuses remain unchanged. No Dart oracle was executed for these
bodies, and no actual portal request, upload or publication occurred.

The 2026-11-04 local execution of this source passed:

```sh
cargo +1.99.1 fmt --all --check
cargo +1.99.1 test --locked -p solana_kit_dapp_publisher --all-features
cargo +1.99.1 test --locked -p solana_kit_dapp_publisher_cli --all-features
cargo +1.99.1 clippy --locked -p solana_kit_dapp_publisher -p solana_kit_dapp_publisher_cli --all-targets --all-features -- -D warnings
RUSTDOCFLAGS="-D warnings" cargo +1.99.1 doc --locked -p solana_kit_dapp_publisher -p solana_kit_dapp_publisher_cli --all-features --no-deps
dprint check
monochange check
cargo +1.99.1 xtask style
```

The publisher package executed 131 test bodies (11 unit, 5 attestation, 24
portal, 17 prepared, 32 publication, 41 workflow and one Rustdoc example); the
CLI executed five unit bodies. The style gate scored 2,110 files with repository
readability 97.15 and complexity 98.50, and every file remained at or above 90.
These results qualify the local source only; browser, both WASI targets,
production wasmi, three-OS CI and independent consumer locks remain unexecuted.

Automated uncertain-effect reconciliation remains incomplete. The native CLI
seam is argument/env/stdin resolution, approved credentials, existing Kit
signer/funding preflight, bounded SourceReader, atomic durable Storage, portal
transport, independent PUT adapter and progress/summary output. Normalized
projections remain a library read model and do not enable normal publishing; no
CLI or image framework is represented by a stub or claimed as complete here.
