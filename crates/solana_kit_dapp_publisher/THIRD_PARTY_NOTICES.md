# Source provenance

Attestation field ordering, publication signer-set checks, canonical outer
program policy, collection checks and prepared-transaction contracts derive
from the MIT-licensed Dart `solana_kit_dapp_publisher_cli` package at immutable
commit `0138bb956d2d81ccc0c56edc602401c0df2f2b46` in
<https://github.com/openbudgetfun/solana_kit>.

The optional portal protocol also derives its fixed procedure/input names,
envelope selection, explicit ingestion retry schedule and raw backend field
catalog from that revision's `portal_client.dart`, `portal_workflow_client.dart`,
`workflow_client.dart`, `portal_types.dart` and `portal_translators.dart`.
Backend normalization, media hydration, session caching and CLI orchestration
are outside this source slice.

The original MIT notice is retained in `MIT-DART.txt`. New original Rust code
uses the Unlicense. No upstream HTTP adapter, secret, CLI, handwritten cryptography
or platform implementation is bundled. Test fixtures independently express the
documented JSON/Borsh/Solana wire formats; no Dart test bodies are copied.

Prepared-message parsing, Ed25519, Base64 and Token Metadata/PDA operations reuse
existing Kit crates and their retained third-party notices. Their upstream
dependencies do not grant any portal or publishing authorization.
